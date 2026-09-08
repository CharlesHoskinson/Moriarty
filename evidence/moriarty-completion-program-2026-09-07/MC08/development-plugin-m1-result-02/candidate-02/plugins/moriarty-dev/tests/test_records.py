"""Exact-source and authority reader tests for load_snapshot()."""
from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

from moriarty_dev.policy import assess
from moriarty_dev.records import load_snapshot, read_actions


SOURCE_ROOT = Path(__file__).resolve().parents[3]
PROGRAM_SRC = SOURCE_ROOT / "openspec" / "moriarty-completion-program.json"
SPRINTS_SRC = SOURCE_ROOT / "openspec" / "sprints" / "sprints.json"
CAMPAIGN_SRC = (
    SOURCE_ROOT
    / "evidence"
    / "moriarty-completion-program-2026-09-07"
    / "report-reconciliation"
    / "campaign-admission.json"
)
LOAN_DIR = (
    SOURCE_ROOT
    / "evidence"
    / "moriarty-completion-program-2026-09-07"
    / "SP01"
    / "loan-swap-subset-01"
)
LOAN_BINDING_SRC = LOAN_DIR / "binding.json"
LOAN_RESOURCE_SRC = LOAN_DIR / "resource-amendment.json"
LOAN_FREEZE_SRC = LOAN_DIR / "candidate-04-freeze.json"
LOAN_ACCEPT_SRC = LOAN_DIR / "acceptance.json"
ATOMIC_BINDING_SRC = (
    SOURCE_ROOT
    / "evidence"
    / "moriarty-completion-program-2026-09-07"
    / "SP01"
    / "atomic-integration-01"
    / "binding.json"
)
ATOMIC_RESOURCE_SRC = (
    SOURCE_ROOT
    / "evidence"
    / "moriarty-completion-program-2026-09-07"
    / "SP01"
    / "atomic-integration-01"
    / "resource-amendment.json"
)
MARKER = Path("/tmp/moriarty-dev-m1-command-must-not-run")
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
LOAN_CANDIDATE = "982062069f2146b98598e5a17ef214c4a0745b7adef4d00c0470575bd2fd748f"
BUDGET_REL = (
    "evidence/moriarty-completion-program-2026-09-07/SP01/"
    "loan-swap-subset-01/current-runtime-budget.json"
)


def sha256_bytes(data):
    return hashlib.sha256(data).hexdigest()


def sha256_file(path):
    return sha256_bytes(Path(path).read_bytes())


def write_json(path, payload):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=False) + "\n")
    return path


def git_init(root):
    subprocess.run(
        ["git", "init"],
        cwd=root,
        check=True,
        capture_output=True,
        text=True,
    )


def make_repo():
    temp = tempfile.TemporaryDirectory(prefix="moriarty-dev-m1-", dir="/tmp")
    root = Path(temp.name)
    git_init(root)
    return temp, root


def copy_relative(root, relative):
    src = SOURCE_ROOT / relative
    if not src.is_file():
        return None
    dest = root / relative
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dest)
    return dest


def copy_genuine_registers(root):
    dest_program = root / "openspec" / "moriarty-completion-program.json"
    dest_sprints = root / "openspec" / "sprints" / "sprints.json"
    dest_campaign = (
        root
        / "evidence"
        / "moriarty-completion-program-2026-09-07"
        / "report-reconciliation"
        / "campaign-admission.json"
    )
    dest_program.parent.mkdir(parents=True, exist_ok=True)
    dest_sprints.parent.mkdir(parents=True, exist_ok=True)
    dest_campaign.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(PROGRAM_SRC, dest_program)
    shutil.copy2(SPRINTS_SRC, dest_sprints)
    shutil.copy2(CAMPAIGN_SRC, dest_campaign)
    for src in (
        LOAN_BINDING_SRC,
        LOAN_RESOURCE_SRC,
        LOAN_FREEZE_SRC,
        LOAN_ACCEPT_SRC,
        ATOMIC_BINDING_SRC,
        ATOMIC_RESOURCE_SRC,
    ):
        dest = root / src.relative_to(SOURCE_ROOT)
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dest)
    freeze = json.loads(LOAN_FREEZE_SRC.read_text())
    acceptance = json.loads(LOAN_ACCEPT_SRC.read_text())
    binding = json.loads(LOAN_BINDING_SRC.read_text())
    campaign = json.loads(CAMPAIGN_SRC.read_text())
    loan = campaign["campaigns"]["sp01-loan-swap-grok-01"]
    refs = [
        binding.get("authority"),
        loan.get("review"),
        loan.get("currentFindingReview"),
        BUDGET_REL,
        acceptance.get("publishedControlPath"),
        acceptance.get("review"),
        acceptance.get("sourceCandidate"),
    ]
    refs.extend(loan.get("successorAmendments") or [])
    refs.extend(freeze.get("ownedFiles", {}))
    refs.extend(freeze.get("inputs", {}))
    refs.extend(binding.get("inputs", {}))
    if type(binding.get("ownedFiles")) is list:
        refs.extend(binding["ownedFiles"])
    elif type(binding.get("ownedFiles")) is dict:
        refs.extend(binding["ownedFiles"])
    refs.extend((binding.get("correctionInputs") or {}))
    for rel in refs:
        if type(rel) is str and rel != "":
            copy_relative(root, rel)
    return dest_program, dest_sprints, dest_campaign


def load_json(path):
    return json.loads(Path(path).read_text())


def dump_json(path, payload):
    Path(path).write_text(json.dumps(payload, indent=2) + "\n")


def stage_by_id(program, stage_id):
    for stage in program["reportReconciliation"]["stageAdmission"]["stages"]:
        if stage["id"] == stage_id:
            return stage
    raise KeyError(stage_id)


def sp05_action(kind, candidate, admission_ref, command_ref="commands.json#driver"):
    return {
        "id": "sp05-ledger-" + kind,
        "requirement": "SP05.2",
        "capability": "fixed-financial-driver",
        "kind": kind,
        "candidate": candidate,
        "admissionRef": admission_ref,
        "commandRef": command_ref,
        "evidenceProfile": "local-runtime",
    }


def loan_action(kind, command_ref=""):
    return {
        "id": "sp01-loan-" + kind,
        "requirement": "SP01.6",
        "capability": "loan-swap-subset",
        "kind": kind,
        "candidate": LOAN_CANDIDATE,
        "admissionRef": "campaign:sp01-loan-swap-grok-01",
        "commandRef": command_ref,
        "evidenceProfile": "syntax",
    }


def write_actions(root, actions):
    write_json(
        root / ".moriarty-dev" / "actions.json",
        {"schema": "moriarty-dev.actions/1", "actions": actions},
    )


def write_commands(root, name="driver"):
    write_json(
        root / "commands.json",
        {
            "schema": "moriarty-dev.commands/1",
            "commands": {
                name: {
                    "argv": [
                        "python3",
                        "-c",
                        "from pathlib import Path; Path(%r).write_text('ran')"
                        % str(MARKER),
                    ]
                }
            },
        },
    )


def write_binding(root, relative, candidate, extra=None):
    payload = {
        "schema": "moriarty.source-ledger-integration/1",
        "status": "admitted-source-implementation",
        "candidateHash": candidate,
        "scope": "Fixed local driver source only",
        "resources": {"allocationSeconds": 600},
    }
    if extra:
        payload.update(extra)
    path = root / relative
    write_json(path, payload)
    return sha256_file(path)


def write_resource(root, relative):
    payload = {
        "schema": "moriarty.delegated-resource-amendment/1",
        "authority": "raw/assignments/moriarty-twelve-sprint-afk-execution-2026-09-07.md",
        "allocationSeconds": 600,
        "scope": "local source implementation only",
    }
    path = root / relative
    write_json(path, payload)
    return path


def write_integer_budget(root, package="MC01", limit=20000, charges=None):
    payload = {
        "schema": "moriarty.supervised-accounting/1",
        "master_limit_seconds": limit,
        "package": package,
        "package_limit_seconds": limit,
        "charges": charges or [],
        "reserved": [],
        "accounting": "derived integer remaining for reader tests",
    }
    return write_json(root / BUDGET_REL, payload)


def add_i2_campaign(program, campaigns, candidate, binding, binding_hash, resource):
    stage_by_id(program, "i2")["campaignRecordId"] = "sp05-ledger-driver-01"
    campaigns["campaigns"]["sp05-ledger-driver-01"] = {
        "stage": "i2",
        "owner": "MC02",
        "status": "admitted-source-implementation",
        "binding": binding,
        "bindingSha256": binding_hash,
        "resourceAmendment": resource,
        "candidateHash": candidate,
        "scope": "local ledger driver source only",
    }


def block_f0(program):
    stage = stage_by_id(program, "f0")
    stage["status"] = "blocked"
    stage["acceptedProfile"] = None
    stage["candidateHash"] = None
    stage["campaignRecordId"] = None


def verified_history(failures=0, **extra):
    payload = {
        "sameDefectFailures": failures,
        "adminCycles": 0,
        "adminSeconds": 0,
        "primaryActive": False,
        "reproducerVerified": False,
        "approachChanged": False,
    }
    payload.update(extra)

    def reader(lineage):
        if set(lineage) != {"repository", "requirement", "capability"}:
            raise AssertionError("history reader received non-lineage keys")
        return dict(payload)

    return reader


def snapshot_shape(test, snap):
    test.assertEqual(tuple(snap), SNAPSHOT_KEYS)
    for key in (
        "authorityCurrent",
        "entryEligible",
        "candidateCurrent",
        "resourceAdmitted",
        "primaryActive",
        "reproducerVerified",
        "approachChanged",
    ):
        test.assertIs(type(snap[key]), bool, key)
    for key in ("sameDefectFailures", "adminCycles", "adminSeconds"):
        test.assertIs(type(snap[key]), int, key)
        test.assertFalse(isinstance(snap[key], bool))
    test.assertTrue(snap["nextActionId"] is None or type(snap["nextActionId"]) is str)
    test.assertIs(type(snap["missingEvidence"]), list)
    test.assertTrue(all(type(item) is str for item in snap["missingEvidence"]))


class GenuineRegisters(unittest.TestCase):
    def setUp(self):
        self.temp, self.root = make_repo()
        self.addCleanup(self.temp.cleanup)
        if MARKER.exists():
            MARKER.unlink()
        self.program_path, self.sprints_path, self.campaign_path = copy_genuine_registers(
            self.root
        )
        self.program = load_json(self.program_path)
        self.campaigns = load_json(self.campaign_path)
        self.candidate_path = self.root / "experiments" / "driver-candidate.txt"
        self.candidate_path.parent.mkdir(parents=True, exist_ok=True)
        self.candidate_path.write_bytes(b"ledger-driver-candidate-v1\n")
        self.candidate = sha256_file(self.candidate_path)
        self.binding = "evidence/moriarty-completion-program-2026-09-07/SP05/ledger-source-packet-01/binding.json"
        self.resource = "evidence/moriarty-completion-program-2026-09-07/SP05/ledger-source-packet-01/resource-amendment.json"
        self.binding_hash = write_binding(self.root, self.binding, self.candidate)
        write_resource(self.root, self.resource)
        write_commands(self.root)
        copy_relative(
            self.root,
            "raw/assignments/moriarty-twelve-sprint-afk-execution-2026-09-07.md",
        )

    def _install_admitted_sp05(self, *, f0_blocked=True):
        if f0_blocked:
            block_f0(self.program)
        add_i2_campaign(
            self.program,
            self.campaigns,
            self.candidate,
            self.binding,
            self.binding_hash,
            self.resource,
        )
        dump_json(self.program_path, self.program)
        dump_json(self.campaign_path, self.campaigns)
        write_actions(
            self.root,
            [
                sp05_action("implement", self.candidate, "campaign:sp05-ledger-driver-01"),
                sp05_action("reproduce", self.candidate, "campaign:sp05-ledger-driver-01"),
                sp05_action("repair", self.candidate, "campaign:sp05-ledger-driver-01"),
                sp05_action("report", self.candidate, "campaign:sp05-ledger-driver-01", ""),
                {
                    "id": "sp02-frontend-implement",
                    "requirement": "SP02.1",
                    "capability": "successor-frontend",
                    "kind": "implement",
                    "candidate": self.candidate,
                    "admissionRef": "campaign:sp05-ledger-driver-01",
                    "commandRef": "commands.json#driver",
                    "evidenceProfile": "syntax",
                },
                {
                    "id": "sp04-f0a-implement",
                    "requirement": "SP04.1",
                    "capability": "native-component-authorship",
                    "kind": "implement",
                    "candidate": self.candidate,
                    "admissionRef": "campaign:sp05-ledger-driver-01",
                    "commandRef": "commands.json#driver",
                    "evidenceProfile": "local-runtime",
                },
                loan_action("report"),
            ],
        )

    def _install_loan_family(self, actions=None, integer_budget=False):
        if integer_budget:
            write_integer_budget(self.root)
        write_actions(self.root, actions or [loan_action("report")])

    def test_blocked_f0_does_not_block_admitted_sp05(self):
        self._install_admitted_sp05(f0_blocked=True)
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        snapshot_shape(self, snap)
        self.assertTrue(snap["entryEligible"])
        self.assertFalse(snap["authorityCurrent"])
        self.assertFalse(snap["candidateCurrent"])
        self.assertFalse(snap["resourceAdmitted"])
        decision = assess(snap, read_actions(str(self.root))[0])
        self.assertFalse(decision["allow"])
        f0 = stage_by_id(load_json(self.program_path), "f0")
        self.assertEqual(f0["status"], "blocked")

    def test_successor_still_requires_rp01_full(self):
        self._install_admitted_sp05()
        snap = load_snapshot(
            str(self.root),
            "sp02-frontend-implement",
            history_reader=verified_history(),
        )
        self.assertFalse(snap["entryEligible"])
        action = [row for row in read_actions(str(self.root)) if row["id"] == "sp02-frontend-implement"][0]
        self.assertFalse(assess(snap, action)["allow"])
        rp01 = stage_by_id(load_json(self.program_path), "rp01-full")
        self.assertEqual(rp01["status"], "specified-only")

    def test_blocked_f0_blocks_f0a_not_sp05(self):
        self._install_admitted_sp05()
        sp05 = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        f0a = load_snapshot(
            str(self.root),
            "sp04-f0a-implement",
            history_reader=verified_history(),
        )
        self.assertTrue(sp05["entryEligible"])
        self.assertFalse(f0a["entryEligible"])

    def test_dispatch_enabled_alone_does_not_admit(self):
        self.program["reportReconciliation"]["stageAdmission"]["dispatchEnabled"] = True
        self.program["sprintPlan"]["status"] = "complete"
        dump_json(self.program_path, self.program)
        sprints = load_json(self.sprints_path)
        sprints["dispatchEnabled"] = True
        sprints["resourceAllocationGranted"] = True
        dump_json(self.sprints_path, sprints)
        write_actions(
            self.root,
            [sp05_action("implement", self.candidate, "campaign:sp05-ledger-driver-01")],
        )
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertTrue(snap["entryEligible"])
        self.assertFalse(snap["resourceAdmitted"])
        self.assertTrue(
            any("campaign" in item or "admission" in item for item in snap["missingEvidence"])
        )

    def test_hash_match_alone_does_not_admit(self):
        write_json(
            self.root / "candidate-manifest.json",
            {
                "schema": "moriarty-dev.unauthoritative-digest/1",
                "files": {
                    "experiments/driver-candidate.txt": self.candidate,
                },
            },
        )
        write_actions(
            self.root,
            [
                sp05_action(
                    "implement",
                    self.candidate,
                    "candidate-manifest.json",
                )
            ],
        )
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertFalse(snap["resourceAdmitted"])
        self.assertFalse(snap["authorityCurrent"] and snap["resourceAdmitted"])
        action = read_actions(str(self.root))[0]
        self.assertFalse(assess(snap, action)["allow"])

    def test_genuine_loan_swap_report_is_current(self):
        self._install_loan_family()
        snap = load_snapshot(str(self.root), "sp01-loan-report")
        snapshot_shape(self, snap)
        self.assertTrue(snap["entryEligible"])
        self.assertTrue(snap["candidateCurrent"])
        self.assertTrue(snap["authorityCurrent"])
        self.assertFalse(snap["resourceAdmitted"])
        self.assertIn("operational-history", snap["missingEvidence"])
        self.assertTrue(any("resource-live-state-unavailable" in item for item in snap["missingEvidence"]))
        self.assertTrue(assess(snap, read_actions(str(self.root))[0])["allow"])

    def test_genuine_rp01_campaign_cannot_admit_sp05_implement(self):
        write_actions(
            self.root,
            [sp05_action("implement", LOAN_CANDIDATE, "campaign:sp01-loan-swap-grok-01")],
        )
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertFalse(snap["authorityCurrent"])
        self.assertFalse(assess(snap, read_actions(str(self.root))[0])["allow"])
        self.assertTrue(
            any("campaign-stage-mismatch" in item for item in snap["missingEvidence"])
        )

    def test_integer_remaining_admits_matching_review(self):
        self._install_loan_family(
            [loan_action("review", "commands.json#driver"), loan_action("report")],
            integer_budget=True,
        )
        snap = load_snapshot(
            str(self.root),
            "sp01-loan-review",
            history_reader=verified_history(),
        )
        self.assertTrue(snap["entryEligible"])
        self.assertTrue(snap["candidateCurrent"])
        self.assertTrue(snap["authorityCurrent"])
        self.assertTrue(snap["resourceAdmitted"])
        action = [row for row in read_actions(str(self.root)) if row["id"] == "sp01-loan-review"][0]
        self.assertTrue(assess(snap, action)["allow"])
        self.assertEqual(snap["nextActionId"], "sp01-loan-review")

    def test_complete_campaign_cannot_admit_implementation(self):
        self._install_loan_family(
            [loan_action("implement", "commands.json#driver")],
            integer_budget=True,
        )
        snap = load_snapshot(
            str(self.root),
            "sp01-loan-implement",
            history_reader=verified_history(),
        )
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(
            any("campaign-status-unsupported" in item for item in snap["missingEvidence"])
        )
        self.assertFalse(assess(snap, read_actions(str(self.root))[0])["allow"])

    def test_changed_owned_bytes_invalidate_candidate(self):
        self._install_loan_family()
        design = (
            self.root
            / "evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/design.md"
        )
        design.write_bytes(design.read_bytes() + b"\nchanged\n")
        snap = load_snapshot(str(self.root), "sp01-loan-report")
        self.assertFalse(snap["candidateCurrent"])
        self.assertTrue(any("stale" in item for item in snap["missingEvidence"]))

    def test_missing_owned_bytes_invalidate_candidate(self):
        self._install_loan_family()
        traces = (
            self.root
            / "evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/traces.json"
        )
        traces.unlink()
        snap = load_snapshot(str(self.root), "sp01-loan-report")
        self.assertFalse(snap["candidateCurrent"])

    def test_empty_and_negative_resource_rejected(self):
        self._install_loan_family([loan_action("review", "commands.json#driver")], True)
        resource = self.root / LOAN_RESOURCE_SRC.relative_to(SOURCE_ROOT)
        dump_json(resource, {})
        empty = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(empty["resourceAdmitted"])
        dump_json(resource, {"authority": "raw/assignments/moriarty-grok-high-gpt6-execution-2026-09-07.md", "additionalSeconds": -1})
        negative = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(negative["resourceAdmitted"])

    def test_genuine_loan_campaign_hash_is_checked(self):
        self._install_loan_family()
        snap = load_snapshot(str(self.root), "sp01-loan-report")
        self.assertTrue(snap["entryEligible"])
        self.assertTrue(snap["candidateCurrent"])
        self.assertIn("operational-history", snap["missingEvidence"])
        self.assertTrue(assess(snap, read_actions(str(self.root))[0])["allow"])
        tampered = self.root / LOAN_BINDING_SRC.relative_to(SOURCE_ROOT)
        tampered.write_bytes(tampered.read_bytes() + b"\n")
        stale = load_snapshot(str(self.root), "sp01-loan-report")
        self.assertFalse(stale["authorityCurrent"])
        self.assertTrue(
            any("bindingSha256" in item or "stale" in item for item in stale["missingEvidence"])
        )

    def test_finding_persists_across_renamed_candidate_and_action(self):
        self._install_admitted_sp05()
        recorded = {}

        def reader(lineage):
            recorded["lineage"] = dict(lineage)
            return {
                "sameDefectFailures": 2,
                "adminCycles": 0,
                "adminSeconds": 0,
                "primaryActive": False,
                "reproducerVerified": False,
                "approachChanged": False,
            }

        first = load_snapshot(str(self.root), "sp05-ledger-implement", history_reader=reader)
        catalog = load_json(self.root / ".moriarty-dev" / "actions.json")
        catalog["actions"][0]["id"] = "packet-renamed-implement"
        catalog["actions"][0]["candidate"] = "renamed-candidate"
        dump_json(self.root / ".moriarty-dev" / "actions.json", catalog)
        second = load_snapshot(
            str(self.root), "packet-renamed-implement", history_reader=reader
        )
        self.assertEqual(first["sameDefectFailures"], 2)
        self.assertEqual(second["sameDefectFailures"], 2)
        self.assertEqual(recorded["lineage"]["requirement"], "SP05.2")
        self.assertEqual(recorded["lineage"]["capability"], "fixed-financial-driver")
        self.assertNotIn("candidate", recorded["lineage"])
        implement = [row for row in read_actions(str(self.root)) if row["kind"] == "implement"][0]
        self.assertFalse(assess(second, implement)["allow"])

    def test_missing_history_is_not_verified_zero(self):
        self._install_loan_family(
            [loan_action("review", "commands.json#driver"), loan_action("report")],
            integer_budget=True,
        )
        snap = load_snapshot(str(self.root), "sp01-loan-review")
        self.assertIn("operational-history", snap["missingEvidence"])
        self.assertEqual(snap["sameDefectFailures"], 0)
        action = [row for row in read_actions(str(self.root)) if row["id"] == "sp01-loan-review"][0]
        self.assertTrue(assess(snap, action)["allow"])
        report = [row for row in read_actions(str(self.root)) if row["kind"] == "report"][0]
        report_snap = load_snapshot(str(self.root), "sp01-loan-report")
        self.assertTrue(assess(report_snap, report)["allow"])
        self.assertIn("operational-history", report_snap["missingEvidence"])

    def test_linked_worktree_uses_git_common_dir(self):
        self._install_loan_family()
        subprocess.run(["git", "add", "."], cwd=self.root, check=True, capture_output=True)
        subprocess.run(
            ["git", "-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid", "commit", "-m", "fixture"],
            cwd=self.root,
            check=True,
            capture_output=True,
        )
        seen = []

        def reader(lineage):
            seen.append(lineage["repository"])
            return verified_history()(lineage)

        with tempfile.TemporaryDirectory(dir="/tmp") as tmp:
            linked = Path(tmp) / "linked"
            subprocess.run(
                ["git", "worktree", "add", "--detach", str(linked), "HEAD"],
                cwd=self.root,
                check=True,
                capture_output=True,
            )
            load_snapshot(str(self.root), "sp01-loan-report", history_reader=reader)
            load_snapshot(str(linked), "sp01-loan-report", history_reader=reader)
        self.assertEqual(seen[0], seen[1])
        self.assertNotEqual(seen[0], str(self.root.resolve()))

    def test_next_action_names_absence_when_none_qualify(self):
        write_actions(
            self.root,
            [sp05_action("implement", self.candidate, "campaign:sp05-ledger-driver-01")],
        )
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertIsNone(snap["nextActionId"])
        self.assertIn("next-action-none", snap["missingEvidence"])

    def test_reads_do_not_write(self):
        self._install_admitted_sp05()
        watched = [
            self.program_path,
            self.sprints_path,
            self.campaign_path,
            self.root / ".moriarty-dev" / "actions.json",
            self.root / self.binding,
        ]
        before = {path: (path.stat().st_mtime_ns, sha256_file(path)) for path in watched}
        load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        after = {path: (path.stat().st_mtime_ns, sha256_file(path)) for path in watched}
        self.assertEqual(before, after)
        self.assertFalse(MARKER.exists())

    def test_command_is_not_executed(self):
        self._install_admitted_sp05()
        load_snapshot(
            str(self.root),
            "sp05-ledger-reproduce",
            history_reader=verified_history(2),
        )
        self.assertFalse(MARKER.exists())

    def test_unknown_schema_names_unresolved_shape(self):
        write_json(
            self.root / "mystery.json",
            {"schema": "moriarty.not-a-supported-authority/9", "ok": True},
        )
        write_actions(
            self.root,
            [sp05_action("implement", self.candidate, "mystery.json")],
        )
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(
            any("moriarty.not-a-supported-authority/9" in item for item in snap["missingEvidence"])
        )

    def test_duplicate_json_keys_rejected(self):
        path = self.root / ".moriarty-dev" / "actions.json"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(
            '{"schema":"moriarty-dev.actions/1","schema":"moriarty-dev.actions/1","actions":[]}\n'
        )
        snap = load_snapshot(str(self.root), "anything")
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("duplicate" in item for item in snap["missingEvidence"]))

    def test_unknown_action_field_rejected(self):
        write_json(
            self.root / ".moriarty-dev" / "actions.json",
            {
                "schema": "moriarty-dev.actions/1",
                "actions": [
                    dict(
                        sp05_action("implement", self.candidate, "campaign:sp05-ledger-driver-01"),
                        mutationId="case-9",
                    )
                ],
            },
        )
        snap = load_snapshot(str(self.root), "sp05-ledger-implement")
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("mutationId" in item or "unknown" in item for item in snap["missingEvidence"]))

    def test_malformed_numeric_in_stage_record(self):
        self.program["schemaVersion"] = "1"
        dump_json(self.program_path, self.program)
        write_actions(
            self.root,
            [sp05_action("report", self.candidate, "campaign:sp01-loan-swap-grok-01", "")],
        )
        snap = load_snapshot(str(self.root), "sp05-ledger-report")
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("schemaVersion" in item for item in snap["missingEvidence"]))

    def test_missing_action_is_unresolved_not_throw(self):
        write_actions(self.root, [sp05_action("report", self.candidate, "campaign:sp01-loan-swap-grok-01", "")])
        snap = load_snapshot(str(self.root), "does-not-exist")
        snapshot_shape(self, snap)
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("does-not-exist" in item for item in snap["missingEvidence"]))

    def test_missing_ref_named(self):
        write_actions(
            self.root,
            [sp05_action("implement", self.candidate, "campaign:missing-campaign")],
        )
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertTrue(any("missing-campaign" in item for item in snap["missingEvidence"]))
        self.assertFalse(snap["resourceAdmitted"])

    def test_stale_binding_hash(self):
        self._install_admitted_sp05()
        self.campaigns["campaigns"]["sp05-ledger-driver-01"]["bindingSha256"] = "0" * 64
        dump_json(self.campaign_path, self.campaigns)
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(
            any("bindingSha256" in item or "stale" in item for item in snap["missingEvidence"])
        )

    def test_path_escape_rejected(self):
        outside = Path("/tmp/moriarty-dev-m1-outside.json")
        write_json(outside, {"schema": "moriarty.source-ledger-integration/1", "status": "admitted-x"})
        self.addCleanup(lambda: outside.exists() and outside.unlink())
        write_actions(
            self.root,
            [sp05_action("implement", self.candidate, "../moriarty-dev-m1-outside.json")],
        )
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("path" in item or "escape" in item or "containment" in item for item in snap["missingEvidence"]))

    def test_secret_path_not_inspected(self):
        secret = self.root / "wallet" / "seed.json"
        secret.parent.mkdir(parents=True, exist_ok=True)
        secret.write_text("SUPERSECRET\n")
        write_actions(
            self.root,
            [sp05_action("implement", self.candidate, "wallet/seed.json")],
        )
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertTrue(any("secret" in item for item in snap["missingEvidence"]))
        self.assertFalse(snap["authorityCurrent"])

    def test_absolute_command_ref_rejected(self):
        write_actions(
            self.root,
            [sp05_action("implement", self.candidate, "campaign:sp01-loan-swap-grok-01", "/bin/true")],
        )
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertTrue(any("commandRef" in item for item in snap["missingEvidence"]))

    def test_bool_dispatch_enabled_is_not_int(self):
        self.program["reportReconciliation"]["stageAdmission"]["dispatchEnabled"] = 0
        dump_json(self.program_path, self.program)
        write_actions(
            self.root,
            [sp05_action("report", self.candidate, "campaign:sp01-loan-swap-grok-01", "")],
        )
        snap = load_snapshot(str(self.root), "sp05-ledger-report")
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("dispatchEnabled" in item for item in snap["missingEvidence"]))

    def test_report_names_missing_prerequisites_honestly(self):
        write_actions(
            self.root,
            [sp05_action("report", self.candidate, "campaign:sp05-ledger-driver-01", "")],
        )
        snap = load_snapshot(str(self.root), "sp05-ledger-report")
        self.assertTrue(snap["entryEligible"])
        self.assertFalse(snap["resourceAdmitted"])
        self.assertIn("operational-history", snap["missingEvidence"])
        self.assertTrue(any("sp05-ledger-driver-01" in item or "campaign" in item for item in snap["missingEvidence"]))
        self.assertTrue(assess(snap, read_actions(str(self.root))[0])["allow"])

    def test_counterfeit_stage_complete_without_campaign_does_not_admit(self):
        stage_by_id(self.program, "i2")["status"] = "complete"
        stage_by_id(self.program, "i2")["candidateHash"] = self.candidate
        stage_by_id(self.program, "i2")["campaignRecordId"] = None
        self.program["reportReconciliation"]["stageAdmission"]["dispatchEnabled"] = True
        dump_json(self.program_path, self.program)
        write_actions(
            self.root,
            [sp05_action("implement", self.candidate, "campaign:sp05-ledger-driver-01")],
        )
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertTrue(snap["entryEligible"])
        self.assertFalse(snap["resourceAdmitted"])

    def test_fixed_program_symlink_is_contained(self):
        outside = Path("/tmp/moriarty-dev-m1-program-outside.json")
        outside.write_bytes(self.program_path.read_bytes())
        self.addCleanup(lambda: outside.exists() and outside.unlink())
        self.program_path.unlink()
        self.program_path.symlink_to(outside)
        self._install_loan_family()
        snap = load_snapshot(str(self.root), "sp01-loan-report")
        self.assertTrue(
            any("containment" in item or "symlink" in item for item in snap["missingEvidence"])
        )
        self.assertFalse(snap["authorityCurrent"])

    def test_resolved_secret_alias_is_refused(self):
        self._install_loan_family([loan_action("review", "commands.json#driver")], True)
        resource = self.root / LOAN_RESOURCE_SRC.relative_to(SOURCE_ROOT)
        secret = self.root / "credentials.json"
        secret.write_bytes(resource.read_bytes())
        resource.unlink()
        resource.symlink_to(secret)
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertTrue(any("secret" in item for item in snap["missingEvidence"]))
        self.assertFalse(snap["resourceAdmitted"])

    def test_list_binding_schema_is_unresolved_not_exception(self):
        self._install_admitted_sp05()
        dump_json(self.root / self.binding, {"schema": []})
        self.campaigns["campaigns"]["sp05-ledger-driver-01"]["bindingSha256"] = sha256_file(
            self.root / self.binding
        )
        dump_json(self.campaign_path, self.campaigns)
        snap = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(),
        )
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("binding-schema" in item for item in snap["missingEvidence"]))

    def test_load_snapshot_callable_and_input_dependent(self):
        self.assertTrue(callable(load_snapshot))
        self.assertTrue(callable(read_actions))
        self._install_admitted_sp05()
        implement = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(0),
        )
        failed = load_snapshot(
            str(self.root),
            "sp05-ledger-implement",
            history_reader=verified_history(2),
        )
        self.assertEqual(implement["sameDefectFailures"], 0)
        self.assertEqual(failed["sameDefectFailures"], 2)
        self.assertNotEqual(implement, failed)


class UnsupportedShapes(unittest.TestCase):
    def setUp(self):
        self.temp, self.root = make_repo()
        self.addCleanup(self.temp.cleanup)

    def test_missing_registers_named(self):
        write_actions(
            self.root,
            [
                {
                    "id": "x",
                    "requirement": "SP05",
                    "capability": "c",
                    "kind": "report",
                    "candidate": "c",
                    "admissionRef": "campaign:none",
                    "commandRef": "",
                    "evidenceProfile": "syntax",
                }
            ],
        )
        snap = load_snapshot(str(self.root), "x")
        joined = " ".join(snap["missingEvidence"])
        self.assertIn("openspec/moriarty-completion-program.json", joined)
        self.assertTrue(assess(snap, read_actions(str(self.root))[0])["allow"])

    def test_symlink_escape_rejected(self):
        outside = Path("/tmp/moriarty-dev-m1-symlink-target.json")
        outside.write_text('{"schema":"moriarty.source-ledger-integration/1","status":"admitted-x"}\n')
        self.addCleanup(lambda: outside.exists() and outside.unlink())
        link = self.root / "admission.json"
        os.symlink(outside, link)
        write_actions(
            self.root,
            [
                {
                    "id": "x",
                    "requirement": "SP05.2",
                    "capability": "c",
                    "kind": "implement",
                    "candidate": "c",
                    "admissionRef": "admission.json",
                    "commandRef": "",
                    "evidenceProfile": "local-runtime",
                }
            ],
        )
        snap = load_snapshot(str(self.root), "x")
        self.assertTrue(
            any("containment" in item or "symlink" in item or "path" in item for item in snap["missingEvidence"])
        )


if __name__ == "__main__":
    unittest.main()
