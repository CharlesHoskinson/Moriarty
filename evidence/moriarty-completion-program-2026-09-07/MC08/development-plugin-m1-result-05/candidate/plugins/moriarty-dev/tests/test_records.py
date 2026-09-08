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
LOAN_VERIFY3_SRC = LOAN_DIR / "verify3-receipt.json"
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
CURRENT_ACCOUNTING_REL = ".moriarty-dev/current-accounting.json"
MASTER_BUDGET_SRC = Path("/home/charl/.local/state/moriarty/mc01-supervised-20260907/budget.json")
PASS_BINDING_SRC = Path(
    "/home/charl/.local/state/moriarty/moriarty-dev-plugin-20260908/"
    "successor-admission-01/binding.json"
)
PASS_LEDGER_SRC = Path(
    "/home/charl/.local/state/moriarty/moriarty-dev-plugin-20260908/"
    "successor-admission-01/ledger.json"
)
PARENT_BINDING_SRC = Path(
    "/home/charl/.local/state/moriarty/moriarty-dev-plugin-20260908/binding.json"
)
PARENT_LEDGER_SRC = Path(
    "/home/charl/.local/state/moriarty/moriarty-dev-plugin-20260908/ledger.json"
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
        LOAN_VERIFY3_SRC,
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
    freeze_name = LOAN_FREEZE_SRC.name
    if freeze_name.endswith("-freeze.json"):
        archive_control = (
            LOAN_DIR
            / freeze_name[: -len("-freeze.json")]
            / Path(acceptance.get("publishedControlPath") or "")
        )
        refs.append(str(archive_control.relative_to(SOURCE_ROOT)))
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
        "accounting": "historical integer snapshot; not live runner credit",
    }
    return write_json(root / BUDGET_REL, payload)


def write_current_accounting(root, patch=None):
    payload = {
        "schema": "moriarty.supervised-accounting/1",
        "authority_sha256": "f6b7f17d1bda437d5f83abc656ccf4a0d9c7a57a83995668387c29bd0328f492",
        "master_limit_seconds": 20000,
        "planning_charge_seconds": 2323.0473305040214,
        "planning_overhead_reserved_seconds": 300,
        "planning_source_sha256": "bfb8e0f54c95580a7a498f2d42b9a81ef36016aa66f536839b4c296f7d81c770",
        "package": "MC01",
        "package_limit_seconds": 18000,
        "worker_dispatch_limit": 8,
        "worker_dispatches": 1,
        "charges": [{"id": "prior", "seconds": 600}],
        "reserved": [{"id": "host_verification", "seconds": 300}],
        "externalPackageCharges": [],
        "externalWorkerDispatches": 0,
        "externalWorkerDispatchLimits": {},
        "accounting": "contained current-runner family derived from supervised-accounting/1",
        "amendment_sha256": "ae2f517852e0f691f21acb005d5836085d3e8dfec2c271e7819f140cab86ae16",
        "delegated_authority": "raw/assignments/moriarty-autonomous-execution-2026-09-07.md",
        "dispatch_amendment_sha256": "f52b5f09c65303c41e17b579965157a6aac8d4f0327d47fd182a516955cc4575",
        "gross_tnight_debit": 0,
        "master_worker_dispatch_limit": 79,
        "native_campaigns": 0,
        "preview_submissions": 0,
        "successorEnvelopes": {},
    }
    if patch:
        payload.update(patch)
    return write_json(root / CURRENT_ACCOUNTING_REL, payload)


def attach_current_accounting(root, campaigns, campaign_id="sp01-loan-swap-grok-01", extra=None):
    spec = {
        "schema": "moriarty.supervised-accounting/1",
        "path": CURRENT_ACCOUNTING_REL,
    }
    if extra:
        spec.update(extra)
    campaigns["campaigns"][campaign_id]["currentAccounting"] = spec
    dest = (
        root
        / "evidence"
        / "moriarty-completion-program-2026-09-07"
        / "report-reconciliation"
        / "campaign-admission.json"
    )
    dump_json(dest, campaigns)
    return spec


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


def require_atomic_prereq(program_path, sprints_path):
    program = load_json(program_path)
    stage_by_id(program, "rp01-mc02")["requires"] = ["atomic-accept"]
    dump_json(program_path, program)
    sprints = load_json(sprints_path)
    for sprint in sprints["sprints"]:
        for gate in sprint.get("entryGates", []):
            if "SP01.6" in gate.get("tasks", []):
                gate["requires"] = ["atomic-accept"]
    dump_json(sprints_path, sprints)
    return program


def overlay_supported_prereq(campaigns, program, loan):
    profile = loan["acceptedProfile"]
    digest = loan["candidateHash"]
    for stage_id in ("atomic-prepare", "atomic-accept"):
        stage = stage_by_id(program, stage_id)
        campaign_id = stage["campaignRecordId"]
        record = dict(campaigns["campaigns"][campaign_id])
        for key in (
            "binding",
            "bindingSha256",
            "candidateHash",
            "candidateManifest",
            "acceptance",
            "review",
            "acceptedProfile",
            "scope",
        ):
            record[key] = loan[key]
        record["stage"] = stage_id
        record["status"] = "complete"
        record["owner"] = "MC01"
        campaigns["campaigns"][campaign_id] = record
        stage["candidateHash"] = digest
        stage["acceptedProfile"] = profile


def use_supported_i2_prereq(program, sprints):
    stage_by_id(program, "i2")["requires"] = ["rp01-mc02"]
    for sprint in sprints["sprints"]:
        for gate in sprint.get("entryGates", []):
            tasks = gate.get("tasks", [])
            if type(tasks) is list and any(
                type(item) is str and item.startswith("SP05.") for item in tasks
            ):
                gate["requires"] = ["rp01-mc02"]


def require_supported_prereq(program_path, sprints_path):
    program = load_json(program_path)
    sprints = load_json(sprints_path)
    for sprint in sprints["sprints"]:
        for gate in sprint.get("entryGates", []):
            if "SP01.6" in gate.get("tasks", []):
                gate["requires"] = ["rp01-mc02"]
    dump_json(sprints_path, sprints)
    dump_json(program_path, program)
    return program


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
        dump_json(self.program_path, self.program)
        dump_json(self.campaign_path, self.campaigns)

    def _isolate_i2(self):
        sprints = load_json(self.sprints_path)
        use_supported_i2_prereq(self.program, sprints)
        dump_json(self.program_path, self.program)
        dump_json(self.sprints_path, sprints)

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
        self._isolate_i2()
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

    def _install_funded_loan_review(self, accounting_patch=None, extra_accounting=None):
        self.program_path, self.sprints_path, self.campaign_path = copy_genuine_registers(
            self.root
        )
        self.program = load_json(self.program_path)
        self.campaigns = load_json(self.campaign_path)
        write_commands(self.root)
        write_current_accounting(self.root, accounting_patch)
        attach_current_accounting(
            self.root, self.campaigns, extra=extra_accounting
        )
        self.campaigns = load_json(self.campaign_path)
        write_actions(
            self.root,
            [loan_action("review", "commands.json#driver"), loan_action("report")],
        )

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
        self._isolate_i2()
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

    def test_historical_integer_snapshot_is_not_live_credit(self):
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
        self.assertFalse(snap["resourceAdmitted"])
        self.assertTrue(
            any("resource-live-state-unavailable" in item for item in snap["missingEvidence"])
        )
        action = [row for row in read_actions(str(self.root)) if row["id"] == "sp01-loan-review"][0]
        self.assertFalse(assess(snap, action)["allow"])
        report = [row for row in read_actions(str(self.root)) if row["kind"] == "report"][0]
        report_snap = load_snapshot(str(self.root), "sp01-loan-report")
        self.assertTrue(assess(report_snap, report)["allow"])

    def test_current_runner_family_admits_matching_review(self):
        self._install_funded_loan_review()
        snap = load_snapshot(
            str(self.root),
            "sp01-loan-review",
            history_reader=verified_history(),
        )
        snapshot_shape(self, snap)
        self.assertTrue(snap["entryEligible"])
        self.assertTrue(snap["candidateCurrent"])
        self.assertTrue(snap["authorityCurrent"])
        self.assertTrue(snap["resourceAdmitted"])
        action = [row for row in read_actions(str(self.root)) if row["id"] == "sp01-loan-review"][0]
        self.assertTrue(assess(snap, action)["allow"])
        self.assertEqual(snap["nextActionId"], "sp01-loan-review")
        self.assertFalse(MARKER.exists())

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
        current = {}

        def reader(lineage):
            if lineage.get("requirement") == "SP05.2":
                current.update(lineage)
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
        self.assertEqual(current["requirement"], "SP05.2")
        self.assertEqual(current["capability"], "fixed-financial-driver")
        self.assertNotIn("candidate", current)
        implement = [row for row in read_actions(str(self.root)) if row["kind"] == "implement"][0]
        self.assertFalse(assess(second, implement)["allow"])

    def test_missing_history_is_not_verified_zero(self):
        self._install_funded_loan_review()
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
        self._isolate_i2()
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
        self._isolate_i2()
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


class AdmissionTraversal(unittest.TestCase):
    setUp = GenuineRegisters.setUp
    _install_funded_loan_review = GenuineRegisters._install_funded_loan_review
    def _positive_review(self):
        self._install_funded_loan_review()
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        action = [row for row in read_actions(str(self.root)) if row["id"] == "sp01-loan-review"][0]
        self.assertTrue(assess(snap, action)["allow"], snap["missingEvidence"])
        self.assertTrue(snap["resourceAdmitted"])
        self.assertTrue(snap["candidateCurrent"])
        return snap

    def test_actual_master_family_admits_review(self):
        self._install_funded_loan_review()
        dest = self.root / CURRENT_ACCOUNTING_REL
        dest.write_bytes(MASTER_BUDGET_SRC.read_bytes())
        pass_bind = self.root / ".moriarty-dev" / "pass-binding.json"
        pass_led = self.root / ".moriarty-dev" / "pass-ledger.json"
        pass_bind.write_bytes(PASS_BINDING_SRC.read_bytes())
        pass_led.write_bytes(PASS_LEDGER_SRC.read_bytes())
        parent_bind = self.root / ".moriarty-dev" / "parent-binding.json"
        parent_led = self.root / ".moriarty-dev" / "parent-ledger.json"
        parent_bind.write_bytes(PARENT_BINDING_SRC.read_bytes())
        parent_led.write_bytes(PARENT_LEDGER_SRC.read_bytes())
        attach_current_accounting(
            self.root,
            load_json(self.campaign_path),
            extra={
                "passBinding": ".moriarty-dev/pass-binding.json",
                "passLedger": ".moriarty-dev/pass-ledger.json",
            },
        )
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        action = [row for row in read_actions(str(self.root)) if row["id"] == "sp01-loan-review"][0]
        self.assertTrue(snap["candidateCurrent"])
        self.assertTrue(snap["authorityCurrent"])
        self.assertTrue(snap["resourceAdmitted"], snap["missingEvidence"])
        self.assertTrue(assess(snap, action)["allow"])
        self.assertTrue(parent_bind.is_file())
        self.assertTrue(parent_led.is_file())

    def test_planning_charge_exhausts_remaining(self):
        self._positive_review()
        write_current_accounting(self.root, {"planning_charge_seconds": 20000})
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["resourceAdmitted"])
        self.assertFalse(
            assess(snap, [row for row in read_actions(str(self.root)) if row["id"] == "sp01-loan-review"][0])["allow"]
        )

    def test_negative_planning_is_unavailable(self):
        self._positive_review()
        write_current_accounting(self.root, {"planning_charge_seconds": -1})
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["resourceAdmitted"])
        self.assertTrue(any("planning" in item for item in snap["missingEvidence"]))

    def test_unknown_accounting_key_rejected(self):
        self._positive_review()
        write_current_accounting(self.root, {"inventedAuthority": True})
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["resourceAdmitted"])
        self.assertTrue(any("inventedAuthority" in item for item in snap["missingEvidence"]))

    def test_missing_master_and_package_caps_unavailable(self):
        self._positive_review()
        write_current_accounting(
            self.root, {"master_limit_seconds": None, "package_limit_seconds": None}
        )
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["resourceAdmitted"])
        self.assertTrue(
            any("resource-live-state-unavailable" in item for item in snap["missingEvidence"])
        )

    def test_schema_only_accounting_unavailable(self):
        self._positive_review()
        dump_json(self.root / CURRENT_ACCOUNTING_REL, {"schema": "moriarty.supervised-accounting/1"})
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["resourceAdmitted"])

    def test_historical_snapshot_path_is_not_current_accounting(self):
        self._positive_review()
        write_integer_budget(self.root)
        attach_current_accounting(
            self.root,
            load_json(self.campaign_path),
            extra={"path": BUDGET_REL},
        )
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["resourceAdmitted"])
        self.assertTrue(any("historical" in item for item in snap["missingEvidence"]))

    def test_action_candidate_must_match_manifest(self):
        self._positive_review()
        catalog = load_json(self.root / ".moriarty-dev" / "actions.json")
        catalog["actions"][0]["candidate"] = "0" * 64
        dump_json(self.root / ".moriarty-dev" / "actions.json", catalog)
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("action-candidate-conflict" in item for item in snap["missingEvidence"]))
        self.assertFalse(
            assess(snap, read_actions(str(self.root))[0])["allow"]
        )

    def test_action_evidence_profile_cannot_claim_settlement(self):
        self._positive_review()
        catalog = load_json(self.root / ".moriarty-dev" / "actions.json")
        catalog["actions"][0]["evidenceProfile"] = "finalized-financial-settlement"
        dump_json(self.root / ".moriarty-dev" / "actions.json", catalog)
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("evidenceProfile" in item for item in snap["missingEvidence"]))

    def test_unrelated_capability_rejected(self):
        self._positive_review()
        catalog = load_json(self.root / ".moriarty-dev" / "actions.json")
        catalog["actions"][0]["capability"] = "native-recursive-financial-proof"
        dump_json(self.root / ".moriarty-dev" / "actions.json", catalog)
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("capability" in item for item in snap["missingEvidence"]))

    def test_stage_candidate_must_match_action(self):
        self._positive_review()
        stage_by_id(self.program, "rp01-mc02")["candidateHash"] = "0" * 64
        dump_json(self.program_path, self.program)
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["authorityCurrent"])
        self.assertTrue(any("stage-candidate-conflict" in item for item in snap["missingEvidence"]))

    def test_blocked_stage_and_null_profiles_rejected(self):
        self._positive_review()
        stage_by_id(self.program, "rp01-mc02")["status"] = "blocked"
        dump_json(self.program_path, self.program)
        blocked = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(blocked["authorityCurrent"])
        self._positive_review()
        stage_by_id(self.program, "rp01-mc02")["acceptedProfile"] = None
        dump_json(self.program_path, self.program)
        null_stage = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(null_stage["authorityCurrent"])
        self._positive_review()
        self.campaigns["campaigns"]["sp01-loan-swap-grok-01"]["acceptedProfile"] = None
        dump_json(self.campaign_path, self.campaigns)
        null_camp = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(null_camp["authorityCurrent"])

    def test_acceptance_cannot_rewrite_frozen_payload(self):
        self._positive_review()
        design = (
            self.root
            / "evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/design.md"
        )
        design.write_bytes(design.read_bytes() + b"\nreview mutation\n")
        acceptance = self.root / LOAN_ACCEPT_SRC.relative_to(SOURCE_ROOT)
        payload = load_json(acceptance)
        rel = "evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/design.md"
        payload["unchangedPayloadHashes"][rel] = sha256_file(design)
        dump_json(acceptance, payload)
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["candidateCurrent"])
        self.assertTrue(any("unchanged-conflict" in item for item in snap["missingEvidence"]))

    def test_review_and_acceptance_contradictions(self):
        self._positive_review()
        review = self.root / Path(
            "evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/gpt6-design-review-04.json"
        )
        body = load_json(review)
        body["verdict"] = "BLOCKED"
        dump_json(review, body)
        blocked = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(blocked["candidateCurrent"])
        self._positive_review()
        body = load_json(review)
        body.pop("scope")
        dump_json(review, body)
        noscope = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(noscope["candidateCurrent"])
        self._positive_review()
        body = load_json(review)
        body["reviewer"] = "author-grok"
        dump_json(review, body)
        author = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(author["candidateCurrent"])
        self._positive_review()
        acceptance = self.root / LOAN_ACCEPT_SRC.relative_to(SOURCE_ROOT)
        body = load_json(acceptance)
        body["status"] = "revoked"
        dump_json(acceptance, body)
        revoked = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(revoked["candidateCurrent"])
        self._positive_review()
        body = load_json(acceptance)
        body["candidateHash"] = None
        dump_json(acceptance, body)
        null_hash = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(null_hash["candidateCurrent"])

    def test_manifest_inputs_and_unknown_schemas(self):
        self._positive_review()
        freeze = self.root / LOAN_FREEZE_SRC.relative_to(SOURCE_ROOT)
        body = load_json(freeze)
        body.pop("inputs")
        dump_json(freeze, body)
        missing_inputs = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(missing_inputs["candidateCurrent"])
        self._positive_review()
        body = load_json(freeze)
        body["schema"] = "unsupported/999"
        dump_json(freeze, body)
        unknown_man = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(unknown_man["candidateCurrent"])
        self._positive_review()
        self.program["reportReconciliation"]["stageAdmission"]["schema"] = "unsupported/999"
        dump_json(self.program_path, self.program)
        unknown_stage = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(unknown_stage["authorityCurrent"])
        self._positive_review()
        self.program["revoked"] = True
        dump_json(self.program_path, self.program)
        unknown_prog = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(unknown_prog["authorityCurrent"])
        self.assertTrue(any("unknown-program-field" in item for item in unknown_prog["missingEvidence"]))
        self._positive_review()
        self.campaigns["campaigns"]["sp01-loan-swap-grok-01"]["revoked"] = True
        dump_json(self.campaign_path, self.campaigns)
        unknown_camp = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(unknown_camp["authorityCurrent"])
        self._positive_review()
        resource = self.root / LOAN_RESOURCE_SRC.relative_to(SOURCE_ROOT)
        body = load_json(resource)
        body["schema"] = "unsupported/999"
        dump_json(resource, body)
        unknown_res = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(unknown_res["resourceAdmitted"])

    def test_prerequisite_campaign_and_hash_are_verified(self):
        self._positive_review()
        loan = load_json(self.campaign_path)["campaigns"]["sp01-loan-swap-grok-01"]
        require_supported_prereq(self.program_path, self.sprints_path)
        ok = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertTrue(ok["entryEligible"], ok["missingEvidence"])
        self.assertTrue(ok["candidateCurrent"], ok["missingEvidence"])
        self.program = require_atomic_prereq(self.program_path, self.sprints_path)
        self.campaigns = load_json(self.campaign_path)
        unavailable = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(unavailable["entryEligible"])
        self.assertTrue(unavailable["candidateCurrent"])
        self.assertTrue(
            any("required-stage-profile" in item for item in unavailable["missingEvidence"])
        )
        overlay_supported_prereq(self.campaigns, self.program, loan)
        dump_json(self.program_path, self.program)
        dump_json(self.campaign_path, self.campaigns)
        reused = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(reused["entryEligible"])
        self.assertTrue(reused["candidateCurrent"])
        self.assertTrue(any("binding-stage" in item for item in reused["missingEvidence"]))
        for sid in ("atomic-prepare", "atomic-accept"):
            st = stage_by_id(self.program, sid)
            st["acceptedProfile"] = "finalized-financial-settlement"
            self.campaigns["campaigns"][st["campaignRecordId"]]["acceptedProfile"] = (
                "finalized-financial-settlement"
            )
        dump_json(self.program_path, self.program)
        dump_json(self.campaign_path, self.campaigns)
        finalized = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(finalized["entryEligible"])
        self.assertTrue(finalized["candidateCurrent"])
        prepare_id = stage_by_id(self.program, "atomic-prepare")["campaignRecordId"]
        kept = dict(self.campaigns["campaigns"][prepare_id])
        self.campaigns["campaigns"][prepare_id] = {
            "candidateHash": kept["candidateHash"],
            "status": "complete",
        }
        dump_json(self.campaign_path, self.campaigns)
        labels = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(labels["entryEligible"])
        self.assertTrue(labels["candidateCurrent"])
        self.campaigns["campaigns"][prepare_id] = kept
        dump_json(self.campaign_path, self.campaigns)
        accept_id = stage_by_id(self.program, "atomic-accept")["campaignRecordId"]
        self.campaigns["campaigns"][accept_id]["status"] = "blocked"
        dump_json(self.campaign_path, self.campaigns)
        blocked = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(blocked["entryEligible"])
        self.assertTrue(blocked["candidateCurrent"])
        self.assertTrue(any("campaign-status" in item for item in blocked["missingEvidence"]))

    def test_published_control_cannot_replace_frozen_commitment(self):
        snap = self._positive_review()
        self.assertTrue(snap["candidateCurrent"])
        loan = load_json(self.campaign_path)["campaigns"]["sp01-loan-swap-grok-01"]
        acceptance_path = self.root / loan["acceptance"]
        acceptance = load_json(acceptance_path)
        published = self.root / acceptance["publishedControlPath"]
        original = published.read_bytes()
        published.write_text("arbitrary replacement: all financial requirements accepted\n")
        acceptance["publishedControlSha256"] = sha256_file(published)
        dump_json(acceptance_path, acceptance)
        rewritten = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(rewritten["candidateCurrent"])
        self.assertTrue(
            any("publication" in item for item in rewritten["missingEvidence"]),
            rewritten["missingEvidence"],
        )
        published.write_bytes(original)
        dump_json(acceptance_path, load_json(LOAN_ACCEPT_SRC))
        control = json.loads(original.decode("utf-8"))
        control["findings"] = [{"id": "forged", "kind": "substitution", "statement": "x"}]
        published.write_text(json.dumps(control) + "\n")
        acceptance = load_json(acceptance_path)
        acceptance["publishedControlSha256"] = sha256_file(published)
        dump_json(acceptance_path, acceptance)
        forbidden = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(forbidden["candidateCurrent"])
        self.assertTrue(any("publication" in item for item in forbidden["missingEvidence"]))

    def test_publication_nested_acceptance_and_closed_status(self):
        snap = self._positive_review()
        self.assertTrue(snap["candidateCurrent"])
        self.assertTrue(snap["entryEligible"])
        loan = load_json(self.campaign_path)["campaigns"]["sp01-loan-swap-grok-01"]
        acceptance_path = self.root / loan["acceptance"]
        published = self.root / load_json(acceptance_path)["publishedControlPath"]
        original = published.read_bytes()

        def mutate_control(patch):
            published.write_bytes(original)
            control = json.loads(original.decode("utf-8"))
            patch(control)
            published.write_text(json.dumps(control) + "\n")
            acceptance = load_json(acceptance_path)
            acceptance["publishedControlSha256"] = sha256_file(published)
            dump_json(acceptance_path, acceptance)

        def snapshot():
            return load_snapshot(
                str(self.root), "sp01-loan-review", history_reader=verified_history()
            )

        def extra(control):
            subset = control["subsets"]["RP01-MC02"]
            subset["acceptance"]["allSprintsAccepted"] = True
            subset["acceptance"]["nativeProofsAccepted"] = True

        mutate_control(extra)
        added = snapshot()
        self.assertFalse(added["candidateCurrent"])
        self.assertTrue(added["entryEligible"])
        self.assertTrue(any("publication" in item for item in added["missingEvidence"]))

        def escalate(control):
            subset = control["subsets"]["RP01-MC02"]
            subset["acceptance"]["scope"] = "Full RP01 native proof and financial Preview acceptance"
            subset["acceptance"]["networkMilestones"] = (
                "complete; all SP05 financial settlement gates accepted"
            )
            subset["acceptance"]["staticVerification"] = "missing-forged-verification.json"

        mutate_control(escalate)
        scoped = snapshot()
        self.assertFalse(scoped["candidateCurrent"])
        self.assertTrue(scoped["entryEligible"])

        def close_all(control):
            subset = control["subsets"]["RP01-MC02"]
            for row in subset["rowSummaries"].values():
                row["closureTask"] = (
                    "All financial settlement, native proof and corpus "
                    "acceptance obligations are closed."
                )

        mutate_control(close_all)
        closed = snapshot()
        self.assertFalse(closed["candidateCurrent"])
        self.assertTrue(closed["entryEligible"])
        self.assertTrue(any("closure" in item for item in closed["missingEvidence"]))

        published.write_bytes(original)
        dump_json(acceptance_path, load_json(LOAN_ACCEPT_SRC))
        body = load_json(acceptance_path)
        body["status"] = "complete-NOT_REVIEWED"
        dump_json(acceptance_path, body)
        unknown = snapshot()
        self.assertFalse(unknown["candidateCurrent"])
        self.assertTrue(unknown["entryEligible"])
        self.assertTrue(any("acceptance-status" in item for item in unknown["missingEvidence"]))
        body["status"] = "complete-but-REVOKED"
        dump_json(acceptance_path, body)
        revoked = snapshot()
        self.assertFalse(revoked["candidateCurrent"])
        self.assertTrue(revoked["entryEligible"])
        self.assertTrue(any("acceptance-status" in item for item in revoked["missingEvidence"]))

    def test_not_reviewed_verdict_is_unresolved(self):
        self._positive_review()
        loan = load_json(self.campaign_path)["campaigns"]["sp01-loan-swap-grok-01"]
        review_path = self.root / loan["review"]
        body = load_json(review_path)
        body["verdict"] = "NOT_REVIEWED"
        dump_json(review_path, body)
        snap = load_snapshot(
            str(self.root), "sp01-loan-review", history_reader=verified_history()
        )
        self.assertFalse(snap["candidateCurrent"])
        self.assertTrue(any("verdict" in item for item in snap["missingEvidence"]))

    def test_next_selection_queries_other_lineage_history(self):
        self._install_funded_loan_review()
        write_actions(
            self.root,
            [
                {
                    "id": "sp02-frontend-reproduce",
                    "requirement": "SP02.1",
                    "capability": "successor-frontend",
                    "kind": "reproduce",
                    "candidate": LOAN_CANDIDATE,
                    "admissionRef": "campaign:sp01-loan-swap-grok-01",
                    "commandRef": "commands.json#driver",
                    "evidenceProfile": "syntax",
                },
                loan_action("review", "commands.json#driver"),
                loan_action("report"),
            ],
        )
        seen = []

        def reader(lineage):
            seen.append((lineage["requirement"], lineage["capability"]))
            return {
                "sameDefectFailures": 0,
                "adminCycles": 0,
                "adminSeconds": 0,
                "primaryActive": False,
                "reproducerVerified": False,
                "approachChanged": False,
            }

        load_snapshot(str(self.root), "sp01-loan-review", history_reader=reader)
        self.assertIn(("SP01.6", "loan-swap-subset"), seen)
        self.assertIn(("SP02.1", "successor-frontend"), seen)


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
