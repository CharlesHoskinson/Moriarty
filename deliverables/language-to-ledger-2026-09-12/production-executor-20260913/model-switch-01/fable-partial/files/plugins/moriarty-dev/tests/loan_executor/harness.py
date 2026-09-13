"""Isolated fixture for the installed loan executor, runner and CLI consumer.

Builds a disposable git repository with: the genuine SP01 registers, an
isolated SQLite store carrying an adopted identity token and receipt-bound
history rows, an external retained predecessor master and its derived current
master with one unclaimed MC02 runtime debit, the immutable
projection/votes/context/history/observation/parent-evidence/correspondence/
ownership records the real accounting verifier authenticates, a reviewed
prover-container expectation, a compiled copy of the static prover lifetime
wrapper, an executor plan, a resource amendment and a hash-bound runner.

Two admission profiles exist. ``live`` (default) registers the exact item 1.2
live labels: action ``sp05-loan-execute-once`` (SP05.3, implement,
finalized-financial-settlement) on campaign ``sp05-loan-live-01`` (stage i2,
owner MC02, status admitted-i2-source-only) with fresh distinct-vendor source
reviews, live acceptance and the digest-bound SP01 prerequisite bridge.
``sp01-verify`` keeps the historical design verify action for legacy checks.
No live store, wallet, service or network is used.
"""
import copy
import datetime
import difflib
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time

TESTS = Path(__file__).resolve().parents[1]
SCRIPTS = TESTS.parent / "scripts"
PLUGIN_ROOT = TESTS.parent
REPO_ROOT = PLUGIN_ROOT.parents[1]
for entry in (str(TESTS), str(SCRIPTS), str(Path(__file__).resolve().parent)):
    if entry not in sys.path:
        sys.path.insert(0, entry)

import test_records as tr  # noqa: E402
import fake_services  # noqa: E402
from moriarty_dev import accounting, records, store  # noqa: E402
from moriarty_dev.store import get_db_path, init_db  # noqa: E402

FOREMAN_LAUNCHER = "/home/charl/foreman/skills/foreman/runtime/dist/foreman-launch.js"
WRAPPER_SOURCE = SCRIPTS / "moriarty_dev" / "prover_lifetime.c"
FINANCIAL_FIXTURE = REPO_ROOT / "deliverables/sp05-financial-integration-2026-09-09/preview-loan-exit-01/public-plan-draft.json"
CONTAINER_ID = "c" * 64
IMAGE_DIGEST = "sha256:801bbc0340e9e96f16735f77b523f23c7459e3359842f7c79c2c53f4e994d531"
MEMORY = 4 * 1024 * 1024 * 1024
PROVER_ARGS = ["/control/prover-control", "--", "/usr/local/bin/midnight-proof-server", "--port", "6300"]
TOKEN = "store-token-fixture-0001"
PREFLIGHT_CHARGE = "preflight-charge-01"
PREFLIGHT_RESERVATION = "preflight-reservation-01"
RUNTIME_CHARGE = "runtime-charge-01"
GRANT_SECONDS = 4000
GRANT_DISPATCHES = 2
RUNTIME_SECONDS = 1835
ENVELOPE = "mc02-envelope-01"
UNIT = "moriarty-loan-fixture.service"
TIMER = "moriarty-loan-fixture-cleanup"
LIVE_CAMPAIGN = "sp05-loan-live-01"
LIVE_ACTION = "sp05-loan-execute-once"
IMM = "deliverables/loan-fixture/immutable"
_WRAPPER_CACHE = {}


def sha256_bytes(data):
    return hashlib.sha256(data).hexdigest()


def sha256_file(path):
    return sha256_bytes(Path(path).read_bytes())


def write_json(path, payload):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    return sha256_file(path)


def utc_now():
    return datetime.datetime.now(datetime.timezone.utc)


def iso(stamp):
    return stamp.isoformat()


def compiled_wrapper():
    """Compile the actual static wrapper once per test session (gcc required)."""
    digest = sha256_file(WRAPPER_SOURCE)
    cached = _WRAPPER_CACHE.get(digest)
    if cached and Path(cached).is_file():
        return Path(cached)
    build_dir = Path(tempfile.mkdtemp(prefix="moriarty-wrapper-build-"))
    target = build_dir / "prover_lifetime"
    gcc = shutil.which("gcc")
    if gcc is None:
        return None
    result = subprocess.run([gcc, "-std=c11", "-Wall", "-Wextra", "-Werror", "-O2", "-static", "-o", str(target),
                             str(WRAPPER_SOURCE)], capture_output=True, text=True)
    if result.returncode != 0:
        return None
    _WRAPPER_CACHE[digest] = str(target)
    return target


def _review_receipt(agent, candidate_sha, scope):
    return {"agent": agent, "candidateSha256": candidate_sha, "recordedAt": iso(utc_now()),
            "reviewer": agent + " fresh independent", "scope": scope, "verdict": "APPROVED", "findings": []}


class LoanFixture(object):
    def __init__(self, root=None, with_store_events=True):
        self.temp = tempfile.TemporaryDirectory(prefix="moriarty-loan-executor-") if root is None else None
        self.root = Path(self.temp.name if root is None else root).resolve()
        self.external = Path(tempfile.mkdtemp(prefix="moriarty-loan-external-"))
        self.with_store_events = with_store_events
        self.services = None

    def cleanup(self):
        if self.temp is not None:
            self.temp.cleanup()
        shutil.rmtree(self.external, ignore_errors=True)

    # --- build ------------------------------------------------------------------
    def build(self, profile="live", python=None):
        root = self.root
        self.profile = profile
        subprocess.run(["git", "init", "-q"], cwd=str(root), check=True, capture_output=True)
        self.db_path = get_db_path(root)
        init_db(self.db_path).close()
        tr.copy_genuine_registers(root)
        campaigns = tr.load_json(root / "evidence/moriarty-completion-program-2026-09-07/report-reconciliation/campaign-admission.json")
        tr.write_current_accounting(root)
        self.python = python or str(Path(sys.executable).resolve())
        if profile == "live":
            self.campaign_id = LIVE_CAMPAIGN
            self.action = {"id": LIVE_ACTION, "requirement": "SP05.3", "capability": "live-executor",
                           "kind": "implement", "candidate": None, "admissionRef": "campaign:" + LIVE_CAMPAIGN,
                           "commandRef": "commands.json#driver", "evidenceProfile": records.LIVE_PROFILE}
        else:
            self.campaign_id = "sp01-loan-swap-grok-01"
            self.action = tr.loan_action("verify", "commands.json#driver")
            tr.attach_current_accounting(root, campaigns, self.campaign_id)
            campaigns = tr.load_json(root / "evidence/moriarty-completion-program-2026-09-07/report-reconciliation/campaign-admission.json")
        self.charge_id = RUNTIME_CHARGE
        self.state_dir = root / "fake-state"
        self.services = fake_services.FakeServices(self.state_dir)
        self.service_table = fake_services.write_fake_executables(root / "fake-bin", self.state_dir)
        self.service_table["/usr/bin/timeout"] = "/usr/bin/timeout"

        evidence_root = root / "deliverables/loan-fixture"
        (evidence_root / "immutable").mkdir(parents=True)
        self.evidence_dir = evidence_root / "evidence"
        self.evidence_dir.mkdir()
        self.control_dir = evidence_root / "control"
        self.control_dir.mkdir()
        (root / "deliverables/loan-fixture/mutable").mkdir()

        # History receipts: retained outcomes whose contents state their class.
        receipt_a = write_json(evidence_root / "immutable/loan-attempt-1.json",
                               {"attempt": 1, "outcome": "failed", "defectClass": "loan-attempt"})
        receipt_b = write_json(evidence_root / "immutable/loan-attempt-2.json",
                               {"attempt": 2, "outcome": "unknown", "defectClass": "loan-attempt"})
        disposition = write_json(evidence_root / "immutable/pending-use-disposition.json",
                                 {"schema": "fixture.pending-use/1", "outcome": "verified-success",
                                  "defectClass": "pending-use", "resolves": [], "resolved": True})
        self.disposition_sha = disposition
        # Canonical store: adopted token and receipt-bound history rows.
        store.adopt_identity_token(self.db_path, TOKEN)
        self.event_ids = []
        if self.with_store_events:
            for idx, digest in enumerate((receipt_a, receipt_b)):
                store.record_event(self.db_path, str(root), self.action["requirement"], self.action["capability"],
                                   "fixture", "historical-loan-%d" % idx, "defect_failure",
                                   {"defectId": "LOAN-ATTEMPT-%d" % idx, "receiptSha256": digest,
                                    "details": "retained historical attempt"})
            import sqlite3
            from contextlib import closing
            with closing(sqlite3.connect(self.db_path)) as conn:
                self.event_ids = [row[0] for row in conn.execute(
                    "SELECT id FROM events WHERE event_kind='defect_failure' ORDER BY id").fetchall()]
        # The retained rows count as two same-lineage failures for the policy
        # reader; the fixture resolves them through an approved review event so
        # the guarded run stays admissible without erasing them.
        store.record_event(self.db_path, str(root), self.action["requirement"], self.action["capability"],
                           "fixture", "fixture-review", "result_review",
                           {"verdict": "APPROVED", "reviewer": "gpt-6-astra", "scope": "fixture",
                            "receiptSha256": disposition, "resolvedFindings": ["LOAN-ATTEMPT-0", "LOAN-ATTEMPT-1"]})
        history = {
            "schema": accounting.HISTORY_SCHEMA, "repositoryKeys": [str(root)],
            "requirement": self.action["requirement"], "capability": self.action["capability"],
            "receipts": [{"path": IMM + "/loan-attempt-1.json", "sha256": receipt_a},
                         {"path": IMM + "/loan-attempt-2.json", "sha256": receipt_b},
                         {"path": IMM + "/pending-use-disposition.json", "sha256": disposition}],
            "outcomes": [{"receiptSha256": receipt_a, "defectClass": "loan-attempt", "outcome": "failed", "resolvedBy": []},
                         {"receiptSha256": receipt_b, "defectClass": "loan-attempt", "outcome": "unknown", "resolvedBy": []}],
            "eventBindings": [{"repository": str(root), "requirement": self.action["requirement"],
                               "capability": self.action["capability"], "eventIds": self.event_ids or [1]}],
        }
        self.history_sha = write_json(evidence_root / "immutable/history.json", history)
        inventory = write_json(evidence_root / "immutable/canonical-store-before-source.json", {"events": 0, "token": None})
        observer_entry = evidence_root / "immutable/observe-preview-funding.mjs"
        observer_entry.write_text("// pinned observer entry fixture\n")
        self.observer_entry_sha = sha256_file(observer_entry)
        sdk_closure = write_json(evidence_root / "immutable/sdk-closure.json", {"files": {}})
        write_json(evidence_root / "immutable/finality.json", {"block": 1, "hash": "f" * 64})
        self.lock_path = self.external / "wallet-use.lock"
        self.receipt_path = "deliverables/loan-fixture/mutable/observation.json"
        self.parent_evidence_rel = "deliverables/loan-fixture/mutable/parent.json"
        self.parent_evidence_path = root / self.parent_evidence_rel
        self.correspondence_path = "deliverables/loan-fixture/mutable/correspondence.json"
        self.expected = {
            "allocationId": "sp05-preview-loan-exit-20260910-01",
            "network": {"networkId": "preview", "genesisHash": "a" * 64, "protocolVersion": 1000000},
            "roleIdentity": {"publicKey": "b" * 64,
                             "address": "mn_addr_preview18adpl4a5kmjm5ffq7wy6wtq5sjy5pl67g4js8ldpvzdqrnpdgthqvlhhye"},
            "selectedToken": {"id": "USD_TEST_ASSET", "unit": "units"},
            "walletStateDirectory": "/private/wallet/state", "seedPath": "/private/wallet/seed",
            "pendingUseDisposition": {"path": IMM + "/pending-use-disposition.json", "sha256": disposition},
        }
        node = str(Path(shutil.which("node")).resolve())
        self.observer_execution = {
            "interpreter": {"path": node, "sha256": sha256_file(node)},
            "entry": {"path": IMM + "/observe-preview-funding.mjs", "sha256": self.observer_entry_sha},
            "files": {}, "sdkClosure": {"path": IMM + "/sdk-closure.json", "sha256": sdk_closure},
            "launcher": {"path": FOREMAN_LAUNCHER, "sha256": sha256_file(FOREMAN_LAUNCHER)},
            "argv": [node, FOREMAN_LAUNCHER, "--timeout", "90", "--grace", "5", "--require-containment", "strong",
                     "--", node, IMM + "/observe-preview-funding.mjs"],
            "environment": {}, "parentEvidencePath": self.parent_evidence_rel,
        }
        missing = []
        common = records._repository_identity(root, missing)
        context = {
            "schema": accounting.CONTEXT_SCHEMA, "repository": str(root), "gitCommonDir": common,
            "store": {"path": str(self.db_path), "identityToken": TOKEN,
                      "inventory": {"path": IMM + "/canonical-store-before-source.json", "sha256": inventory}},
            "history": {"path": IMM + "/history.json", "sha256": self.history_sha},
            "observer": {"entryPath": IMM + "/observe-preview-funding.mjs", "entrySha256": self.observer_entry_sha,
                         "receiptPath": self.receipt_path, "maxAgeSeconds": 120, "preflightSeconds": 125,
                         "preflightAttempts": 1, "lockPath": str(self.lock_path)},
            "correspondencePath": self.correspondence_path,
            "expected": self.expected,
            "observerExecution": self.observer_execution,
        }
        self.context_sha = write_json(evidence_root / "immutable/execution-context.json", context)

        # Retained predecessor master and the projection derived from it.
        self.master_path = self.external / "master.json"
        predecessor = {
            "schema": "moriarty.supervised-accounting/1", "package": "MC01", "master_limit_seconds": 30000,
            "planning_charge_seconds": 2323.0473305040214, "planning_overhead_reserved_seconds": 300,
            "package_limit_seconds": 18000, "worker_dispatches": 1, "worker_dispatch_limit": 8,
            "charges": [{"id": "prior", "seconds": 600}], "reserved": [], "externalPackageCharges": [],
            "externalWorkerDispatches": 31, "master_worker_dispatch_limit": 83,
        }
        self.predecessor = predecessor
        self.predecessor_sha = write_json(evidence_root / "immutable/predecessor-master.json", predecessor)
        self.closure_seconds = 26777   # ceil(30000 - 3223.0473305040214)
        self.closure_adjustment = 1    # ceil(3223.0473... + 26777) - 30000
        projection = {
            "schema": accounting.PROJECTION_SCHEMA,
            "predecessor": {"path": str(self.master_path), "sha256": self.predecessor_sha},
            "ledgerOwner": "MC01", "selectedOwner": "MC02", "envelopeId": ENVELOPE,
            "grantSeconds": GRANT_SECONDS, "grantDispatches": GRANT_DISPATCHES,
            "closureSeconds": self.closure_seconds, "closureAdjustmentSeconds": self.closure_adjustment,
            "campaignId": self.campaign_id, "actionId": self.action["id"],
            "historicalDispositions": [{"path": IMM + "/pending-use-disposition.json", "sha256": disposition},
                                       {"path": IMM + "/predecessor-master.json", "sha256": self.predecessor_sha}],
            "executionContextSha256": self.context_sha,
        }
        self.projection_sha = write_json(evidence_root / "immutable/projection.json", projection)
        votes = []
        for model in ("claude-opus-5", "gpt-6-astra"):
            digest = write_json(evidence_root / ("immutable/vote-%s.json" % model), {
                "schema": accounting.VOTE_SCHEMA, "reviewer": model + " fresh", "model": model,
                "role": accounting.VOTE_ROLE, "scope": accounting.VOTE_SCOPE,
                "candidateSha256": self.projection_sha, "verdict": "APPROVED", "findings": ["fixture"]})
            votes.append({"path": IMM + "/vote-%s.json" % model, "sha256": digest})
        # Reviewed prover container expectation: the exact admitted configuration.
        self.wrapper_rel = IMM + "/prover_lifetime"
        wrapper = compiled_wrapper()
        wrapper_dest = root / self.wrapper_rel
        if wrapper is not None:
            shutil.copy2(wrapper, wrapper_dest)
            self.wrapper_compiled = True
        else:
            wrapper_dest.write_text("#!/bin/sh\nexit 64\n")
            self.wrapper_compiled = False
        wrapper_dest.chmod(0o755)
        self.wrapper_sha = sha256_file(wrapper_dest)
        self.prover_expectation = {
            "schema": accounting.PROVER_CONTAINER_SCHEMA, "containerId": CONTAINER_ID, "imageDigest": IMAGE_DIGEST,
            "memoryLimitBytes": MEMORY, "entrypoint": "/prover_lifetime", "args": list(PROVER_ARGS),
            "mounts": [{"source": str(wrapper_dest.resolve()), "destination": "/prover_lifetime", "readOnly": True},
                       {"source": str(self.control_dir.resolve()), "destination": "/control", "readOnly": True}],
            "restartPolicy": "no", "pidMode": "", "privileged": False, "capAdd": [],
        }
        self.prover_expectation_sha = write_json(evidence_root / "immutable/prover-container.json", self.prover_expectation)
        self.block_deadline = utc_now() + datetime.timedelta(hours=4)
        amendment = {
            "schema": "moriarty.delegated-resource-amendment/1",
            "authority": "raw/assignments/moriarty-grok-high-gpt6-execution-2026-09-07.md",
            "additionalSeconds": GRANT_SECONDS, "additionalWorkerDispatches": GRANT_DISPATCHES,
            "reason": "isolated loan executor fixture", "at": iso(utc_now()),
            "resources": {"projection": {"path": IMM + "/projection.json", "sha256": self.projection_sha},
                          "votes": votes,
                          "executionContext": {"path": IMM + "/execution-context.json", "sha256": self.context_sha},
                          "proverContainer": {"path": IMM + "/prover-container.json", "sha256": self.prover_expectation_sha},
                          "blockDeadlineUtc": iso(self.block_deadline)},
        }
        self.admission_rel = IMM + "/resource-amendment.json"
        self.admission_sha = write_json(root / self.admission_rel, amendment)
        self.campaigns = campaigns

        # Mutable observation (with parent evidence), correspondence and ownership.
        self.write_observation()
        self.ownership_path = Path(str(self.lock_path) + ".ownership.json")
        write_json(self.ownership_path, {
            "schema": accounting.OWNERSHIP_SCHEMA, "allocationId": self.expected["allocationId"],
            "executionContextSha256": self.context_sha, "projectionSha256": self.projection_sha,
            "storeIdentity": TOKEN, "preflightReservationId": PREFLIGHT_RESERVATION,
            "preflightChargeId": PREFLIGHT_CHARGE, "runtimeChargeId": None, "runtimeReservationId": None,
            "ownerPid": 0, "ownerStartTicks": 0, "state": accounting.OWNERSHIP_STATE_PREFLIGHT})

        # Executor plan inputs.
        financial = json.loads(FINANCIAL_FIXTURE.read_text())
        financial["limits"]["deadlineMs"] = 0
        self.financial_rel = IMM + "/financial-plan.json"
        self.financial_sha = write_json(root / self.financial_rel, financial)
        financial_script = root / "fake-financial.py"
        financial_script.write_text("raise SystemExit(0)\n")
        self.plan_rel = IMM + "/executor-plan.json"
        self.plan = {
            "schema": "moriarty.loan-executor-plan/1", "allocationId": self.expected["allocationId"],
            "actionId": self.action["id"], "candidateHash": None,
            "financialPlan": {"path": self.financial_rel, "sha256": self.financial_sha},
            "command": [self.python, "fake-financial.py", "{runtimeFinancialPlan}"],
            "closure": {self.python: sha256_file(self.python), "fake-financial.py": sha256_file(financial_script)},
            "authority": {"path": self.admission_rel, "sha256": self.admission_sha},
            "unit": UNIT, "timerUnit": TIMER,
            "prover": {"containerId": CONTAINER_ID, "imageDigest": IMAGE_DIGEST, "memoryLimitBytes": MEMORY,
                       "lifetime": {"wrapper": {"path": self.wrapper_rel, "sha256": self.wrapper_sha},
                                    "controlDirectory": "deliverables/loan-fixture/control",
                                    "controlName": "prover-control", "containerControlPath": "/control/prover-control"}},
            "evidenceDirectory": "deliverables/loan-fixture/evidence",
            "resultPath": "deliverables/loan-fixture/evidence/loan-process-result.json",
            "limits": {"outerSeconds": 1800, "graceSeconds": 5, "setupSeconds": 120, "operationDeadlineSeconds": 1600,
                       "bootstrapExitSeconds": 1606, "collectionDeadlineSeconds": 1618, "timerSeconds": 1620,
                       "cleanupSeconds": 1740, "showSeconds": 5, "stopSeconds": 25},
        }
        self.services.seed_container(fake_services.container_config(
            CONTAINER_ID, IMAGE_DIGEST, MEMORY, str(wrapper_dest.resolve()), str(self.control_dir.resolve()),
            "/prover_lifetime", "/control", "/control/prover-control", args=PROVER_ARGS))
        # Driver script: production main and production transports routed to the fakes.
        self.driver = root / "loan-executor-driver.py"
        self.driver.write_text(
            "import sys\nsys.path.insert(0, %r)\nfrom pathlib import Path\nfrom moriarty_dev import loan_executor\n"
            "deps = loan_executor.production_dependencies(Path.cwd(), service_table=%r)\n"
            "raise SystemExit(loan_executor.main(sys.argv[1:], dependencies=deps))\n"
            % (str(SCRIPTS), self.service_table))
        if profile == "live":
            self._build_live_records()
        else:
            self.action["candidate"] = tr.LOAN_CANDIDATE
        self.plan["candidateHash"] = self.action["candidate"]
        self.write_plan()
        actions = [self.action] if profile == "live" else [self.action, tr.loan_action("report")]
        tr.write_actions(root, actions)
        self.write_runner(self.campaigns)
        return self

    # --- live profile ------------------------------------------------------------
    def _build_live_records(self):
        """Exact item 1.2 live labels on an isolated campaign with the SP01 bridge."""
        root = self.root
        # Current SP01 document bytes (the edited live input) replace the fixture original.
        current_doc = REPO_ROOT / records.BRIDGE_PATH
        (root / records.BRIDGE_PATH).write_bytes(current_doc.read_bytes())
        original = (root / records.BRIDGE_ORIGINAL_PATH)
        original.parent.mkdir(parents=True, exist_ok=True)
        original.write_bytes((REPO_ROOT / records.BRIDGE_ORIGINAL_PATH).read_bytes())
        owned = {"loan-executor-driver.py": sha256_file(self.driver), "fake-financial.py": sha256_file(root / "fake-financial.py")}
        candidate = sha256_bytes(json.dumps(owned, separators=(",", ":"), sort_keys=True).encode("utf-8"))
        self.action["candidate"] = candidate
        diff = "".join(difflib.unified_diff(
            original.read_text().splitlines(keepends=True), (root / records.BRIDGE_PATH).read_text().splitlines(keepends=True),
            fromfile=records.BRIDGE_ORIGINAL_PATH, tofile=records.BRIDGE_PATH, n=3, lineterm="\n")).encode("utf-8")
        bridge_core = {"originalSha256": records.BRIDGE_ORIGINAL_SHA256, "currentSha256": sha256_file(root / records.BRIDGE_PATH),
                       "diffSha256": sha256_bytes(diff)}
        bridge_candidate = sha256_bytes(json.dumps(bridge_core, sort_keys=True, separators=(",", ":")).encode("utf-8"))
        self.live = {
            "manifest": {"ownedFiles": owned, "inputs": {}, "digestAlgorithm": records.LIVE_DIGEST_ALGO,
                         "candidateSha256": candidate},
            "review": _review_receipt("claude-fable-5-1", candidate, records.LIVE_SCOPE),
            "sourceReview": _review_receipt("gpt-6", candidate, records.LIVE_SCOPE),
            "acceptance": {"candidateHash": candidate, "derivation": "direct owned/input equality",
                           "publishedControlPath": "loan-executor-driver.py",
                           "publishedControlSha256": owned["loan-executor-driver.py"],
                           "review": IMM + "/live-review.json", "sourceCandidate": IMM + "/live-candidate.json",
                           "status": records.LIVE_ACCEPTANCE_STATUS},
            "bridge": {"schema": records.BRIDGE_SCHEMA, "path": records.BRIDGE_PATH,
                       "originalPath": records.BRIDGE_ORIGINAL_PATH, **bridge_core,
                       "diffPath": IMM + "/sp01-prerequisite.diff",
                       "opusDisposition": {"path": IMM + "/bridge-disposition-a.json", "sha256": None},
                       "astraDisposition": {"path": IMM + "/bridge-disposition-b.json", "sha256": None}},
            "dispositionA": _review_receipt("claude-fable-5-1", bridge_candidate, records.BRIDGE_SCOPE),
            "dispositionB": _review_receipt("gpt-6-high", bridge_candidate, records.BRIDGE_SCOPE),
            "binding": {"schema": records.BINDING_SCHEMA, "status": records.LIVE_BINDING_STATUS,
                        "authority": "raw/assignments/moriarty-grok-high-gpt6-execution-2026-09-07.md",
                        "scope": records.LIVE_SCOPE, "stage": records.LIVE_STAGE,
                        "resources": {"workerSeconds": [RUNTIME_SECONDS]},
                        "inputs": {}, "ownedFiles": list(owned), "runners": {}},
            "diff": diff,
        }
        self.live_review = self.live["review"]
        self.live_source_review = self.live["sourceReview"]
        self.live_acceptance = self.live["acceptance"]
        self.live_bridge = self.live["bridge"]
        self.live_original = copy.deepcopy(self.live)
        self.binding_rel = IMM + "/live-binding.json"
        program = tr.load_json(root / "openspec/moriarty-completion-program.json")
        sprints = tr.load_json(root / "openspec/sprints/sprints.json")
        stage = tr.stage_by_id(program, "i2")
        stage["campaignRecordId"] = LIVE_CAMPAIGN
        stage["candidateHash"] = candidate
        stage["acceptedProfile"] = None
        tr.use_supported_i2_prereq(program, sprints)
        tr.dump_json(root / "openspec/moriarty-completion-program.json", program)
        tr.dump_json(root / "openspec/sprints/sprints.json", sprints)
        self.campaigns["campaigns"][LIVE_CAMPAIGN] = {
            "stage": records.LIVE_STAGE, "owner": records.LIVE_OWNER, "status": records.LIVE_BINDING_STATUS,
            "scope": records.LIVE_SCOPE, "binding": self.binding_rel, "bindingSha256": None,
            "resourceAmendment": self.admission_rel, "candidateHash": candidate,
            "candidateManifest": IMM + "/live-candidate.json", "review": IMM + "/live-review.json",
            "sourceReview": IMM + "/live-source-review.json", "acceptance": IMM + "/live-acceptance.json",
            "reconciliation": IMM + "/sp01-prerequisite-disposition.json",
            "currentAccounting": {"schema": "moriarty.supervised-accounting/1", "path": tr.CURRENT_ACCOUNTING_REL},
        }
        self.write_live_records()

    def write_live_records(self):
        """Write every live record and recompute the hash-bound references."""
        root = self.root
        live = self.live
        (root / live["bridge"]["diffPath"]).write_bytes(live["diff"])
        digest_a = write_json(root / live["bridge"]["opusDisposition"]["path"], live["dispositionA"])
        digest_b = write_json(root / live["bridge"]["astraDisposition"]["path"], live["dispositionB"])
        live["bridge"]["opusDisposition"]["sha256"] = digest_a
        live["bridge"]["astraDisposition"]["sha256"] = digest_b
        manifest = dict(live["manifest"])
        manifest["candidateSha256"] = sha256_bytes(json.dumps(manifest["ownedFiles"], separators=(",", ":"),
                                                              sort_keys=True).encode("utf-8"))
        paths = {"review": IMM + "/live-review.json", "sourceReview": IMM + "/live-source-review.json",
                 "acceptance": IMM + "/live-acceptance.json", "bridge": IMM + "/sp01-prerequisite-disposition.json",
                 "manifest": IMM + "/live-candidate.json"}
        write_json(root / paths["manifest"], manifest)
        inputs = {}
        for key in ("review", "sourceReview", "acceptance", "bridge"):
            inputs[paths[key]] = write_json(root / paths[key], live[key])
        binding = dict(live["binding"])
        binding["inputs"] = inputs
        self.binding_sha = write_json(root / self.binding_rel, binding)
        record = self.campaigns["campaigns"][LIVE_CAMPAIGN]
        record["bindingSha256"] = self.binding_sha
        tr.dump_json(root / "evidence/moriarty-completion-program-2026-09-07/report-reconciliation/campaign-admission.json",
                     self.campaigns)

    def mutate_live(self, mutate):
        """Apply a near-miss mutation to the live records and rewrite them consistently."""
        mutate(self.live["binding"], self.live["manifest"])
        self.write_live_records()
        if hasattr(self, "runner"):
            self.write_runner(self.campaigns)

    def restore_live(self):
        self.live = copy.deepcopy(self.live_original)
        self.live_review = self.live["review"]
        self.live_source_review = self.live["sourceReview"]
        self.live_acceptance = self.live["acceptance"]
        self.live_bridge = self.live["bridge"]
        self.write_live_records()
        if hasattr(self, "runner"):
            self.write_runner(self.campaigns)

    # --- mutable producer-shaped records ---------------------------------------------
    def write_plan(self, plan=None):
        plan = plan or self.plan
        self.plan_sha = write_json(self.root / self.plan_rel, plan)
        return self.plan_sha

    def write_observation(self, age_seconds=0, **patch):
        stamp = iso(utc_now() - datetime.timedelta(seconds=age_seconds))
        observation = {
            "schema": accounting.OBSERVATION_SCHEMA, "allocationId": self.expected["allocationId"],
            "preflightChargeId": PREFLIGHT_CHARGE, "sourceSha256": self.observer_entry_sha,
            "observedAt": stamp, "syncTime": stamp,
            "network": self.expected["network"], "roleIdentity": self.expected["roleIdentity"],
            "amounts": {"USD_TEST_ASSET": "20000000000", "DUST": "5000000000000000"},
            "unitNames": {"USD_TEST_ASSET": "units", "DUST": "speck"},
            "finalityRef": {"path": IMM + "/finality.json", "sha256": sha256_file(self.root / IMM / "finality.json")},
            "pendingUseDispositionSha256": self.disposition_sha,
            "canonicalInputsUnchanged": True, "childrenStopped": True,
        }
        observation.update(patch)
        self.observation_sha = write_json(self.root / self.receipt_path, observation)
        self.write_parent_evidence(observation)
        return observation

    def write_parent_evidence(self, observation, **patch):
        started = utc_now() - datetime.timedelta(seconds=95)
        inputs = {"seedSha256": "5" * 64, "snapshotSha256": "6" * 64}
        parent = {
            "schema": accounting.PARENT_EVIDENCE_SCHEMA, "allocationId": self.expected["allocationId"],
            "preflightChargeId": PREFLIGHT_CHARGE, "contextSha256": self.context_sha,
            "launchSha256": store.canonical_digest(self.observer_execution),
            "sourceSha256": self.observer_entry_sha, "outputSha256": self.observation_sha, "processExit": 0,
            "containment": {"capability": "posix_pidns_userns_strong", "childrenStopped": True},
            "canonicalInputsBefore": dict(inputs), "canonicalInputsAfter": dict(inputs),
            "startedAt": iso(started), "endedAt": iso(started + datetime.timedelta(seconds=60)),
        }
        parent.update(patch)
        write_json(self.parent_evidence_path, parent)
        return parent

    def current_master(self, runner_digest):
        """Current master derived from the retained predecessor: M + D + G, reserve transfer, one MC02 row."""
        master = json.loads(json.dumps(self.predecessor))
        master["master_limit_seconds"] = self.predecessor["master_limit_seconds"] + self.closure_adjustment + GRANT_SECONDS
        master["reserved"] = list(self.predecessor["reserved"]) + [
            {"id": "mc01-closure", "seconds": self.closure_seconds, "basis": accounting.CLOSURE_BASIS,
             "sourceSha256": "2" * 64, "amendmentSha256": self.admission_sha},
            {"id": "mc02-reserve", "seconds": GRANT_SECONDS - RUNTIME_SECONDS, "basis": accounting.RESERVE_BASIS,
             "sourceSha256": "2" * 64, "amendmentSha256": self.admission_sha}]
        master["externalPackageCharges"] = list(self.predecessor["externalPackageCharges"]) + [{
            "id": RUNTIME_CHARGE, "seconds": RUNTIME_SECONDS, "actionId": self.action["id"],
            "candidateHash": self.action["candidate"], "runnerDigest": runner_digest, "package": "MC02",
            "envelope": ENVELOPE, "storeIdentity": TOKEN, "reservationId": PREFLIGHT_RESERVATION,
            "launchClaim": "unclaimed", "purpose": "financial-runtime"}]
        master["externalWorkerDispatches"] = self.predecessor["externalWorkerDispatches"] + 1
        master["master_worker_dispatch_limit"] = self.predecessor["master_worker_dispatch_limit"] + GRANT_DISPATCHES
        master["successorEnvelopes"] = {ENVELOPE: {
            "owner": "MC02", "projectionSha256": self.projection_sha, "amendmentSha256": self.admission_sha,
            "remainingReservedSeconds": GRANT_SECONDS - RUNTIME_SECONDS, "workerDispatches": 1,
            "workerDispatchLimit": GRANT_DISPATCHES}}
        return master

    def write_master(self, runner_digest, **patch):
        master = self.current_master(runner_digest)
        master.update(patch)
        self.master = master
        self.master_sha = write_json(self.master_path, master)
        self.write_correspondence()
        return master

    def write_correspondence(self, reservation_id=None, **patch):
        correspondence = {
            "schema": accounting.CORRESPONDENCE_SCHEMA, "allocationId": self.expected["allocationId"],
            "executionContextSha256": self.context_sha, "masterSha256": sha256_file(self.master_path),
            "projectionSha256": self.projection_sha, "observationSha256": self.observation_sha,
            "preflightChargeId": PREFLIGHT_CHARGE, "runtimeChargeId": RUNTIME_CHARGE, "reservationId": reservation_id,
        }
        correspondence.update(patch)
        self.correspondence_sha = write_json(self.root / self.correspondence_path, correspondence)
        return correspondence

    def write_runner(self, campaigns=None, launcher=None, argv=None, timeout=1800, grace=5):
        """Hash-bound runner with the executor launch entry, debited in the contained projection."""
        root = self.root
        campaigns = campaigns or self.campaigns
        node = str(Path(shutil.which("node")).resolve())
        launcher = launcher or [node, FOREMAN_LAUNCHER]
        argv = argv or [self.python, "loan-executor-driver.py", "--plan", self.plan_rel, "--sha256", self.plan_sha,
                        "--admission", self.admission_rel]
        files = {self.python: sha256_file(self.python), self.plan_rel: self.plan_sha,
                 self.admission_rel: self.admission_sha}
        for operand in argv[1:]:
            if not operand.startswith("-") and (root / operand).is_file() and operand not in files:
                files[operand] = sha256_file(root / operand)
        launcher_files = {launcher[0]: sha256_file(launcher[0]), launcher[1]: sha256_file(launcher[1])}
        if launcher[1] != FOREMAN_LAUNCHER:
            launcher_files[FOREMAN_LAUNCHER] = sha256_file(FOREMAN_LAUNCHER)
        self.runner = {
            "schema": "moriarty.bound-runner/1", "action": self.action, "argv": argv, "files": files,
            "launcher": launcher, "launcherFiles": launcher_files, "timeoutSeconds": timeout, "graceSeconds": grace,
            "outputLimitBytes": 65536, "chargeId": RUNTIME_CHARGE, "chargedSeconds": RUNTIME_SECONDS,
            "accountingPath": tr.CURRENT_ACCOUNTING_REL,
            "executor": {"schema": "moriarty.loan-executor-launch/1", "plan": self.plan_rel, "sha256": self.plan_sha,
                         "admission": self.admission_rel},
        }
        tr.dump_json(root / "commands.json", {"schema": "moriarty-dev.commands/1",
                                              "commands": {"driver": {"argv": argv}}})
        self.runner_digest = hashlib.sha256(json.dumps(self.runner, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        campaign = campaigns["campaigns"][self.campaign_id]
        if self.profile == "live":
            binding = tr.load_json(root / self.binding_rel)
            binding["runners"] = {self.action["id"]: self.runner}
            self.live["binding"]["runners"] = {self.action["id"]: self.runner}
            tr.dump_json(root / self.binding_rel, binding)
            campaign["bindingSha256"] = sha256_file(root / self.binding_rel)
            self.binding_sha = campaign["bindingSha256"]
        else:
            binding = tr.load_json(root / campaign["binding"]) if not campaign["binding"].startswith("current") \
                else tr.load_json(root / "current-binding.json")
            binding["runners"] = {self.action["id"]: self.runner}
            tr.dump_json(root / "current-binding.json", binding)
            campaign["binding"] = "current-binding.json"
            campaign["bindingSha256"] = sha256_file(root / "current-binding.json")
        tr.dump_json(root / "evidence/moriarty-completion-program-2026-09-07/report-reconciliation/campaign-admission.json",
                     campaigns)
        accounting_path = root / tr.CURRENT_ACCOUNTING_REL
        budget = tr.load_json(accounting_path)
        budget["charges"] = [row for row in budget["charges"] if row.get("id") != RUNTIME_CHARGE]
        if self.profile == "live":
            # The contained projection mirrors the MC02 envelope and its rows.
            projection_master = self.current_master(self.runner_digest)
            budget["externalPackageCharges"] = projection_master["externalPackageCharges"]
            budget["successorEnvelopes"] = projection_master["successorEnvelopes"]
            budget["externalWorkerDispatches"] = projection_master["externalWorkerDispatches"]
            budget["master_worker_dispatch_limit"] = projection_master["master_worker_dispatch_limit"]
        else:
            budget["charges"].append({"id": RUNTIME_CHARGE, "seconds": RUNTIME_SECONDS,
                                      "candidateHash": self.action["candidate"], "actionId": self.action["id"],
                                      "runnerDigest": self.runner_digest})
        tr.dump_json(accounting_path, budget)
        self.write_master(self.runner_digest)
        return self.runner

    # --- helpers for in-process executor tests -------------------------------------
    def reserve_and_handover(self, owner_pid=None, owner_ticks=None):
        """Existing store.reserve plus the runtime handover tail; returns reservation id."""
        from moriarty_dev.records import load_snapshot
        from moriarty_dev.store import make_history_reader
        snap = load_snapshot(str(self.root), self.action["id"], history_reader=make_history_reader(self.db_path))
        self.reservation_id = store.reserve(self.db_path, str(self.root), self.action, snap, charge_id=RUNTIME_CHARGE)
        from moriarty_dev import loan_executor
        plan = loan_executor.load_plan(self.root, self.plan_rel, self.plan_sha)
        admission = loan_executor.load_admission(self.root, self.admission_rel, plan)
        state = accounting.runtime_handover(self.root, plan, admission, RUNTIME_CHARGE, self.reservation_id,
                                            self.runner_digest, owner_pid if owner_pid is not None else os.getpid(),
                                            owner_ticks if owner_ticks is not None else 7)
        assert not isinstance(state, accounting.Refusal), state
        self.state = state
        self.loaded_plan = plan
        self.loaded_admission = admission
        return self.reservation_id

    def invocation_payload(self, nonce, outer_start_ns=None, **patch):
        start = time.monotonic_ns() if outer_start_ns is None else outer_start_ns
        payload = {
            "schema": accounting.INVOCATION_SCHEMA, "allocationId": self.plan["allocationId"],
            "actionId": self.action["id"], "candidateHash": self.action["candidate"],
            "runnerDigest": self.runner_digest, "chargeId": RUNTIME_CHARGE, "reservationId": self.reservation_id,
            "storeIdentity": TOKEN, "executionContextSha256": self.context_sha, "projectionSha256": self.projection_sha,
            "correspondenceSha256": self.state["correspondenceSha256"], "authoritySha256": self.admission_sha,
            "bootId": Path("/proc/sys/kernel/random/boot_id").read_text().strip(),
            "outerStartMonotonic": start, "outerDeadlineMonotonic": start + 1800 * 1_000_000_000,
            "blockDeadlineUtc": iso(self.block_deadline), "parentPid": os.getpid(), "parentStartTicks": 7,
            "launcherPid": 1, "launcherStartTicks": 0, "nonceSha256": hashlib.sha256(nonce).hexdigest(),
        }
        payload.update(patch)
        return payload

    def record_invocation(self, payload):
        return store.record_invocation(self.db_path, str(self.root), self.action, payload)


# --- in-process execution helpers ------------------------------------------------
class FakeChannel(object):
    """Injected exchange_invocation result: an already authenticated payload."""

    def __init__(self, payload, digest, alive=True, lose_after=None):
        self.invocation = payload
        self.invocation_sha256 = digest
        self.sent = []
        self.alive = alive
        self.lose_after = lose_after  # phase name after which the parent disappears
        self.closed = False

    def parent_alive(self):
        return self.alive

    def send(self, phase, detail=None):
        self.sent.append((phase, detail))
        if self.lose_after is not None and phase == self.lose_after:
            self.alive = False
        return True

    def close(self):
        self.closed = True


class FaultyWriter(object):
    """write_exclusive with injectable failures per artifact basename."""

    def __init__(self, fail=None, timeout=None):
        self.fail = set(fail or [])
        self.timeout = set(timeout or [])
        self.calls = []

    def __call__(self, path, data, mode=0o600, timeout_seconds=25):
        from moriarty_dev import loan_executor
        name = Path(path).name
        self.calls.append((name, mode, timeout_seconds))
        if name in self.fail:
            return loan_executor.DurableWriteResult(False, path, error_class="OSError", detail="injected write failure")
        if name in self.timeout:
            return self._stalled(path, data, mode, min(float(timeout_seconds), 0.3))
        return loan_executor.production_write_exclusive(path, data, mode, timeout_seconds)

    @staticmethod
    def _stalled(path, data, mode, bound):
        # A real bounded worker whose fsync stalls: killed at its bound.
        from moriarty_dev.loan_exit_retention.exit_retention import run_bounded
        from moriarty_dev import loan_executor
        outcome = run_bounded(lambda: time.sleep(5), bound)
        assert outcome.status == "timeout"
        return loan_executor.DurableWriteResult(False, path, error_class="PersistenceTimeout",
                                                detail="PERSISTENCE_TIMEOUT")


class ExecutorRun(object):
    """Drive execute_once in-process with fake services and one injected virtual clock."""

    def __init__(self, fixture, script=None, writer=None, entry_offset_ns=0, wall_offset_ms=0,
                 channel=None, block_deadline=None, outer_deadline_ns=None, jump_after_launch_ns=0,
                 setup_delay_seconds=0):
        self.fx = fixture
        self.jump_after_launch_ns = jump_after_launch_ns
        self.setup_delay_seconds = setup_delay_seconds
        if script is not None:
            fixture.services._save("script.json", script)
        self.writer = writer or FaultyWriter()
        self.fx.reserve_and_handover()
        import secrets
        self.nonce = secrets.token_bytes(32)
        self.start_ns = time.monotonic_ns() - entry_offset_ns
        patch = {}
        if outer_deadline_ns is not None:
            patch["outerDeadlineMonotonic"] = outer_deadline_ns
        self.payload = self.fx.invocation_payload(self.nonce, outer_start_ns=self.start_ns, **patch)
        self.event_id, self.digest = self.fx.record_invocation(self.payload)
        self.channel = channel or FakeChannel(self.payload, self.digest)
        self.wall_offset_ms = wall_offset_ms
        self.slept = []
        self.clock_offset_s = 0.0
        self._jumped = False
        self._delayed = False

    def _virtual_offset(self):
        rows = self.fx.services.calls()
        if self.jump_after_launch_ns and not self._jumped:
            if any("show" in row and UNIT in row and "Result" in row[-1] for row in rows):
                self._jumped = True
                self.clock_offset_s += self.jump_after_launch_ns / 1e9
        if self.setup_delay_seconds and not self._delayed:
            # Setup work consumed time after validation: the first occupancy check.
            if any("show" in row and UNIT in row and row[-1].endswith("InvocationID") for row in rows):
                self._delayed = True
                self.clock_offset_s += self.setup_delay_seconds
        self.fx.services._save("clock-offset.json", {"seconds": self.clock_offset_s})
        return self.clock_offset_s

    def dependencies(self):
        from moriarty_dev import loan_executor, loan_cleanup
        deps = loan_executor.production_dependencies(self.fx.root)
        run_command = fake_services.run_command_for(self.fx.services)
        deps["exchange_invocation"] = lambda admission: self.channel
        deps["run_command"] = run_command
        deps["write_exclusive"] = self.writer

        def monotonic():
            return time.monotonic() + self._virtual_offset()
        deps["monotonic"] = monotonic
        deps["wall_time_ms"] = lambda: int(time.time() * 1000) + self.wall_offset_ms + int(self._virtual_offset() * 1000)

        def sleep(seconds):
            # Injected sleep advances the injected clock instead of blocking.
            self.slept.append(seconds)
            self.clock_offset_s += float(seconds)
            self.fx.services._save("clock-offset.json", {"seconds": self.clock_offset_s})
        deps["sleep"] = sleep
        deps["observe_containment"] = lambda ownership, deadline_ns, purpose="safety-cleanup": loan_cleanup.contain_owned(
            ownership["intent"], ownership.get("proverReceipt"), ownership.get("unitReceipt"), run_command, deadline_ns,
            purpose, unit_started=ownership.get("unitStarted"), prover_started=ownership.get("proverStarted"),
            monotonic_ns=lambda: int(monotonic() * 1_000_000_000))
        return deps

    def run(self):
        from moriarty_dev import loan_executor
        self.outcome = loan_executor.execute_once(self.fx.loaded_plan, self.fx.loaded_admission, self.dependencies())
        self.result = self.outcome["result"]
        return self.outcome

    def calls(self, verb=None):
        rows = self.fx.services.calls()
        if verb is None:
            return rows
        return [row for row in rows if verb in row]

    def evidence_kinds(self):
        return [item["kind"] for item in self.result["evidence"]]
