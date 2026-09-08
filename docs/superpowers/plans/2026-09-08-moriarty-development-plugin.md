# Moriarty Development Plugin Implementation Plan

> **For agentic workers:** Use `superpowers:executing-plans` for this plan. Preserve Grok 4.6 high implementation and fresh independent GPT-6 Astra result review. Steps use checkboxes. Do not add recursive planning or review rounds.

**Goal:** Block repeated ineffective Moriarty work and direct execution to a demonstrable, authorized product capability.

**Architecture:** One development skill calls a deterministic Python policy and guarded launch command. A small SQLite store preserves operational history across sessions and worktrees. Optional host hooks use the same policy, while existing registers, bounded runners and evidence retain their authority.

**Tech stack:** Python 3.11+, standard library, SQLite, unittest, Git, existing Foreman/Grok execution and Codex plugin packaging.

**Status:** User approved execution. Four Milestone 1 candidates failed independent review. Candidate04 still permits stage/profile substitution and unsupported publication/status claims. A scoped evidence correction is underway; resource admission remains separately open. No milestone is accepted. Installation and host trust remain open. See [candidate04 evidence](../../../evidence/moriarty-completion-program-2026-09-07/MC08/development-plugin-m1-result-04/README.md).

**Design:** [Moriarty development plugin](../specs/2026-09-08-moriarty-development-plugin-design.md).

## Global constraints

- Preserve every SP01–SP12 requirement, mandatory PCD, bounded execution and Preview financial acceptance.
- Use Grok 4.6 high for implementation and a fresh independent GPT-6 Astra for result review.
- Use existing task entry gates, not whole-sprint completion, to determine eligibility.
- Keep program/campaign acceptance and resource records authoritative. The plugin cannot grant admission.
- Use one primary implementation and at most one independent review, subject to stricter existing limits.
- Trigger redirection after two failed cycles of the same unresolved defect class.
- Trigger redirection after two process-only cycles or 1,800 observed administrative seconds.
- Preserve history across candidate names, budgets, sessions and linked worktrees.
- Use Python 3.11 or later and the standard library. Add no daemon, dashboard, MCP server or model API client.
- Make hooks local, bounded to one second and 2 KiB output, without model calls, test runs or automatic continuation.
- Do not collect private reasoning, credentials, seeds, wallet state or witness data.
- Finish four milestones. Prove usefulness on a Moriarty defect before adding plugin features.

The [user execution instruction](../../../raw/assignments/moriarty-development-plugin-execution-2026-09-08.md) records approval of this plan.
Execution must use an existing applicable budget or a recorded bounded allocation under the standing authority.
This plan neither creates that allocation nor requires a new planning campaign to obtain it.

## File structure

All plugin paths below are relative to `plugins/moriarty-dev/` unless explicitly marked repository-relative.

```text
.codex-plugin/plugin.json             Valid installable manifest
README.md                            Invocation, guarantees and coverage limitations
skills/develop/SKILL.md               Single product-oriented development workflow
scripts/moriarty_dev/__init__.py      Package marker
scripts/moriarty_dev/policy.py        Pure assessment and next-action selection
scripts/moriarty_dev/records.py       Exact-source and evidence adapters
scripts/moriarty_dev/store.py         Shared operational events and reservations
scripts/moriarty_dev/cli.py           Six public commands and guarded execution
scripts/moriarty_dev/hook.py          Host wire-format adapter
hooks/hooks.json                     Default-discovered host hooks
tests/test_policy.py                 Independent stop-rule and admission tests
tests/test_records.py                Evidence identity and authority tests
tests/test_execution.py              Launch, crash and concurrency tests
tests/test_hooks.py                  Hook behavior and contract tests
tests/host-smoke.md                  Actual-host verification procedure
```

Repository-relative files:

- `.moriarty-dev/actions.json`: references to current actions and existing admissions, not acceptance state.
- `AGENTS.md`: one plugin invocation link and the verified fallback procedure.
- `docs/FOOTGUNS.md`: point to the implementation only after it exists.

Runtime-only state: `<git-common-dir>/moriarty-dev/state.sqlite3`.
Local personal installation: `~/plugins/moriarty-dev`, registered through the existing plugin-creator helpers.
No plugin cache or database is committed.

## Data contracts shared by all milestones

Use closed dictionaries at file boundaries. Reject unknown keys and wrong types before deriving a policy snapshot.
Internal normalized values use these exact names:

```python
Action = {
    "id": str,
    "requirement": str,
    "capability": str,
    "kind": str,  # implement, reproduce, repair, verify, review, admin, report
    "candidate": str,
    "admissionRef": str,
    "commandRef": str,
    "evidenceProfile": str,
}
Snapshot = {
    "authorityCurrent": bool,
    "entryEligible": bool,
    "candidateCurrent": bool,
    "resourceAdmitted": bool,
    "sameDefectFailures": int,
    "adminCycles": int,
    "adminSeconds": int,
    "primaryActive": bool,
    "reproducerVerified": bool,
    "approachChanged": bool,
    "nextActionId": str | None,
    "missingEvidence": list[str],
}
Decision = {
    "allow": bool,
    "reasonCode": str,
    "nextActionId": str | None,
    "missingEvidence": list[str],
}
```

The booleans above are derived internally by `records.py` and the event store.
The CLI must not accept a caller-supplied Snapshot as authority.
`approachChanged` needs a changed test/mechanism reference tied to the unresolved finding, not merely a new description.
`reproducerVerified` needs a runner-observed behavioral assertion failure on the affected candidate, not an import error or arbitrary nonzero exit.

Event columns: `id`, `repository`, `requirement`, `capability`, `candidate`, `action_id`, `event_kind`, `observed_at`, `payload_json`.
Keep action start/end, result review, changed approach and public transaction notification as distinct event kinds.
Use SQLite transactions and unique reservation identities. Preserve raw referenced review findings without rewriting them.

CLI output is JSON when requested. Exit codes are `0` success, `2` policy denial, `3` unavailable/stale inputs, and `4` child failure.
Never return exit `0` with a fabricated success receipt after a failed child.

## Milestone 1: Reject the recurrence and select useful work

**Deliverable:** A local checker rejects repeated broad correction and permits its concrete reproducer without provider calls.

**Files:** Create `policy.py`, `records.py`, `__init__.py`, `tests/test_policy.py`, `tests/test_records.py`.

**Consumes:** The design's stop rules, real sprint entry gates and retained review findings.
**Produces:** `assess(snapshot, action) -> Decision`, `load_snapshot(repo, action_id) -> Snapshot`, and normalized action/evidence readers.

- [ ] Write the first failing tests with independently chosen actions and boundary values.

```python
import unittest
from moriarty_dev.policy import assess

def baseline():
    return dict(authorityCurrent=True, entryEligible=True,
        candidateCurrent=True, resourceAdmitted=True,
        sameDefectFailures=0, adminCycles=0, adminSeconds=0,
        primaryActive=False, reproducerVerified=False,
        approachChanged=False, nextActionId="ledger-driver-reproduce",
        missingEvidence=[])

def action(kind):
    return dict(id="ledger-driver-" + kind, requirement="SP05",
        capability="fixed-financial-driver", kind=kind, candidate="candidate-a",
        admissionRef="admission.json", commandRef="commands.json#driver",
        evidenceProfile="local-runtime")

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
        self.assertEqual(assess(s, action("repair"))["reasonCode"],
                         "EVIDENCE_STALE")
```

- [ ] Run `PYTHONPATH=plugins/moriarty-dev/scripts python3 -m unittest discover -s plugins/moriarty-dev/tests -p 'test_policy.py' -v`.
  First run must fail because the implementation is absent. Retain it as a development baseline, not a semantic reproducer.
- [ ] Implement the policy in this order: current authority/source checks, entry/resource gates, concurrency, repeated failures, administrative limits, allowed action.
  Read-only status/report remain available when dispatch is denied.
  A focused repair after repeated failure requires both `reproducerVerified` and `approachChanged`.
- [ ] Add table-driven cases for administrative cycles 1/2, one/two failures, unavailable authority, exhausted resources and active primary work.
  Unknown action kinds reject. A new action name with the same lineage retains its failures.
- [ ] Implement exact-reference readers for the program, sprint and campaign records.
  Use explicit field adapters for supported schemas. Missing or conflicting source returns named unresolved evidence.
  Resolve worktree paths through `git rev-parse --git-common-dir` and real paths.
- [ ] Test with disposable copies of existing stage records: F0 blocked plus valid atomic/RP01-MC02 prerequisites keeps an admitted SP05 action eligible.
  Full successor syntax remains ineligible without its actual RP01 prerequisite.
  A `dispatchEnabled` value alone cannot grant an action.
- [ ] Run both test modules. Expected: all stop-rule cases pass and no source/register mutation occurs.
- [ ] Commit the checker and tests together. Request one fresh GPT-6 result review of this behavior under the execution routing.

## Milestone 2: Enforce the decision at launch and preserve failures

**Deliverable:** Denied actions cannot start a child process, and a restart or parallel invocation cannot erase or duplicate a launch.

**Files:** Create `store.py`, `cli.py`, `tests/test_execution.py`. Extend `records.py` and its tests.

**Consumes:** Milestone 1 policy and exact admitted command references.
**Produces:** `reserve`, `finish`, `status`, `next`, `run`, `review`, `report` and `doctor` behavior defined in the design.

- [ ] Write failing black-box tests using a disposable repository and a child command that creates a marker file.
  The test fixture's commands and admission are local-only and cannot launch a provider or network call.

```python
def test_denied_launch_has_no_child_effect(self):
    # make_case creates a disposable Git repo with a real local fixture admission.
    case = self.make_case(failures=2, action_kind="implement")
    result = case.cli("run", "--action", "fixture-action")
    self.assertEqual(result.returncode, 2)
    self.assertFalse(case.child_marker.exists())

def test_restart_preserves_failure(self):
    case = self.make_case(failures=2, action_kind="implement")
    self.assertEqual(case.cli("run", "--action", "fixture-action").returncode, 2)
    case.rename_candidate("candidate-b")
    self.assertEqual(case.cli("run", "--action", "fixture-action").returncode, 2)

def test_allowed_reproducer_runs_once(self):
    case = self.make_case(failures=2, action_kind="reproduce")
    first = case.cli("run", "--action", "fixture-action")
    second = case.cli("run", "--action", "fixture-action")
    self.assertEqual(first.returncode, 0)
    self.assertNotEqual(second.returncode, 0)
    self.assertEqual(case.child_marker.read_text(), "one launch\n")
```

Implement `make_case` within the test module using `TemporaryDirectory`, `git init`, the closed action contract above and `sys.executable`.
Its child script writes the marker and exits. `case.cli` uses `subprocess.run` with an argv list and captured output.
`rename_candidate` changes only the candidate identity while keeping requirement/capability lineage fixed.
It must not modify recorded failures.

- [ ] Run `PYTHONPATH=plugins/moriarty-dev/scripts python3 -m unittest discover -s plugins/moriarty-dev/tests -p 'test_execution.py' -v` and retain the failing result.
- [ ] Implement SQLite events and reservations with `BEGIN IMMEDIATE` and one active primary implementation reservation across the repository.
  Reserve only after exact input validation. Write completion separately from product acceptance.
  Do not hold a database transaction open throughout the child process.
- [ ] Implement `run` without shell evaluation. Revalidate source and admission immediately before invoking the existing bounded runner.
  Missing containment/resource evidence rejects launch. Delegate budget charging to that runner exactly once.
  The plugin adds no automatic retry.
- [ ] Add subprocess tests for simultaneous reservations, a source change between selection and launch, interruption and recovery of an unfinished reservation.
  A linked worktree must see the same unresolved failures.
  A different capability cannot acquire a second primary implementation reservation.
  Missing/corrupt state must not become a fresh clean history.
- [ ] Implement review ingestion with exact candidate/scope matching and separate author/reviewer identities.
  Test author-authored `APPROVED`, stale review hashes, missing review scope and renamed finding IDs.
  None may clear an unresolved independent finding.
- [ ] Implement administrative interval accounting with an injected clock.
  Test overlap, a 1,800-second boundary, AFK gaps, active test execution and unknown timing.
  Packet approval, renewed resources and plugin self-tests must not reset product progress.
- [ ] Implement safe public transaction events and an undelivered-ID outbox.
  Test submitted → unknown finality → failed and duplicate notification handling.
  Capture only selected public fields. Reject private payload keys at this boundary.
- [ ] Run the entire plugin suite. Demonstrate a blocked child marker and an allowed reproducer from a fresh process.
- [ ] Commit the guarded path with its tests and obtain the required independent review.

## Milestone 3: Package the workflow and verify host interception

**Deliverable:** Codex loads the development skill and reports verified hook coverage without claiming unsupported enforcement.

**Files:** Create the manifest, README, skill, `hook.py`, `hooks/hooks.json`, `tests/test_hooks.py` and `tests/host-smoke.md`.

**Consumes:** Milestone 2 CLI and policy.
**Produces:** An installable plugin with one skill, local hook adapters and a truthful `doctor` result.

- [ ] Write failing hook-contract tests for a denied known dispatch, a permitted diagnostic, an unrelated repository and malformed input.
  Assert no subprocess/model/network invocation from hook handlers.
  Test `Stop` twice and require no automatic continuation on either call.

The denied call uses this output contract:

```json
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Repeated unresolved failure. Run ledger-driver-reproduce."
  }
}
```

- [ ] Run the hook tests and retain the failing baseline.
- [ ] Implement hook translation into the existing policy, with no second set of rules.
  Use the event's selected public fields. Do not parse provider reasoning or depend on an unstable transcript schema.
  Unknown tool coverage cannot produce `host-verified`.
  A hook exception produces a diagnostic, while the independent launch boundary still refuses unsafe dispatch.
- [ ] Package hooks at the conventional path and omit the manifest `hooks` field for compatibility with the inspected local validator.
  Use these manifest fields with version `0.1.0`: `name=moriarty-dev`, real description, `author.name=Moriarty contributors`, `skills=./skills/` and required interface metadata.
  Do not add apps or MCP fields.
- [ ] Write the skill workflow: current user intent → status → next eligible action → Grok implementation → actual checks → fresh GPT-6 review → scoped report.
  Tell the agent to post pending transaction IDs before a network status summary.
  Design/planning requests follow Superpowers without becoming automatic implementation permission.
- [ ] Run the package validator:

```bash
python3 /home/charl/.codex/skills/.system/plugin-creator/scripts/validate_plugin.py plugins/moriarty-dev
```

- [ ] Prepare the personal install using the existing scaffold/marketplace helpers, preserving other entries.
  Stage the reviewed source under `~/plugins/moriarty-dev`, validate it, read the actual marketplace name, then use `codex plugin add`.
  Do not add a team marketplace or hand-edit unrelated Codex settings.
  Installation and hook trust happen only after the concrete plugin is reviewable.
- [ ] Run the actual-host smoke recipe in a disposable repository after hook trust and a new thread.
  Attempt the marker-producing command through ordinary shell, unified exec and current code-mode tool composition.
  Test file edits and native delegation separately. Record host version, path and observed denial/coverage.
  Calling `hook.py` directly is only a unit test, not host interception evidence.
- [ ] Test disable/re-enable and changed hook definitions. `doctor` must degrade when trust or coverage is absent.
  Do not repair Codex internals when coverage is absent. Retain the guarded CLI and report the limitation.
- [ ] Commit the plugin source and actual coverage report, with required independent review.

## Milestone 4: Demonstrate progress on Moriarty and stop plugin expansion

**Deliverable:** The plugin permits a focused, independently checked repair of a real Moriarty defect while denying the prior ineffective sequence.

**Files:** Create repository-relative `.moriarty-dev/actions.json`. Modify `AGENTS.md` and `docs/FOOTGUNS.md` only to link the working workflow.
Extend plugin tests with historical regressions. Keep product repairs in their separately scoped existing worktree.

**Consumes:** Reviewed plugin, existing SP05 ledger candidate, current budget/admission and retained failure evidence.
**Produces:** A guarded Moriarty correction demonstration and a scoped all-sprint status report.

- [ ] Register the current ledger reproducer, repair and review references without inventing a new budget or acceptance record.
  If a referenced runner does not implement verification, register an actual bounded verification command under existing authority before invoking it.
- [ ] Reproduce the public driver defect through controlled transport.
  Assert that admitted input invokes deploy, initialize and the case-specific calls, and returns observed results.
  The current unconditional `admitted-not-executed` response must fail this test.
  Supply no wallets, credentials or network endpoints to this test.
- [ ] Demonstrate that another broad correction is denied after the historical failure threshold, while this reproducer and a focused repair remain eligible.
  Give Grok the failing test and the exact approved implementation scope.
  Run the corrected production-path test and obtain fresh GPT-6 result review on the same candidate bytes.
  This is the first product-progress acceptance condition for the plugin pilot.
- [ ] Preserve the decoder's unconditional real-byte failure as a separate open requirement until a valid production implementation and sufficient fixture exist.
  A mock return or synthetic receipt cannot close finalized-byte or network acceptance.
- [ ] Add independent historical regressions: composition's supplied green summary cannot clear a failing full-state test, a new packet cannot erase prior failures, and stale checkpoint activity cannot count as a live worker.
- [ ] Generate status for all twelve sprint identities from existing source records.
  It must show full BNF, executable Moriarty K and financial Preview evidence as missing wherever their actual predicates remain unestablished.
  Plugin tests and the offline pilot must not close any full sprint.
- [ ] Test public notification forwarding with a synthetic transport event clearly marked test-only.
  Real transaction reporting is verified only on a separately admitted actual submission. Do not submit a transaction to qualify the plugin alone.
- [ ] Compare the pilot with the previous pattern: child dispatches, administrative intervals, repeated findings, concrete repaired behavior and review result.
  Do not invent a percentage improvement or promise a completion date.
- [ ] Commit the integration references and pilot evidence after review. Publish under the existing repository authority.
  Return to Moriarty development. Add no plugin feature unless a concrete accepted-workflow failure requires it.

## Self-review and acceptance coverage

| Requirement | Milestone |
| --- | --- |
| Repeated failure and process-only stop rules | 1, 2 |
| Task entry gates and independent eligible work | 1 |
| Persistence across recovery, budgets and worktrees | 2 |
| Atomic launch checks and no duplicate child | 2 |
| Candidate-bound evidence and independent review | 2 |
| Public transaction outbox and honest status | 2, 4 |
| Skills, packaging, trust and actual-host coverage | 3 |
| No stop-hook retry loop or new orchestration service | 3 |
| Real Moriarty production-path repair | 4 |
| All-sprint coverage and no false promotion | 4 |

Run the plugin tests only for implementation changes to the plugin.
Run `python3 openspec/sprints/verify.py` when integrating task mappings.
That command checks planning structure, not product acceptance.
Use `git diff --check` and validate documentation links before publication.

No task requires a model council merely to approve a routine correction within its existing scope and limits.
Two failures of the same plugin requirement trigger the same stop rule as Moriarty work.
The plugin project cannot exempt itself from the behavior it is intended to prevent.
