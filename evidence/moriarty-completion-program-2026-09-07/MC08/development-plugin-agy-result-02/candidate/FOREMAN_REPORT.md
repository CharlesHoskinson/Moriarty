# Moriarty Development Plugin Foreman Implementation Report

## Summary

This report records the correction and verification of all four milestones (M1–M4) of the Moriarty development plugin (`moriarty-dev`) in worktree `/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok` by **AGY Gemini 3.8 Flash high**, following the terminal independent code review by **Grok 4.6 high** (`grok-result-review.json` / `grok-result-review.md`).

All 12 Grok findings have been addressed in the production paths. All unit and regression tests (114 tests) pass cleanly. Both the plugin creator validator and the skill creator validator pass. No git index mutations (`git add` or `git commit`), network calls, wallet interactions, or autonomous self-reviews were performed. All candidate bytes are ready for independent re-review by **Grok 4.6 high** on identical candidate bytes, followed by fresh independent **GPT-6 Astra** review only after Grok passes.

---

## Dispositions of Independent Grok Code Review Findings

1. **`M1-E2-STATIC-UNBOUND` (Static verification receipt binding)**:
   - *Fix*: In `records.py:_publication_acceptance_ok`, bound `staticVerification` to the authentic receipt path `"evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/verify3-receipt.json"` and SHA-256 digest `"eeef0e21216de0348ac96250508f38c2fc89d657c63176a822a7f6c8e0f3c8f2"`. Required `kind == "verify3"`, `exitCode == 0`, exact design scope, `unauthorizedGitActivity is False`, and `storageStop is False`.
   - *Verification*: Added unit test `test_substitute_success_shaped_static_verification_receipt_rejected` verifying that substitute receipts with success exit codes are rejected.

2. **`M1-RESOURCE-IDENTITY-UNREAD` (Resource authority and pass scope linkage)**:
   - *Fix*: In `records.py:_remaining_from_current`, enforced `authority_sha256 == "f6b7f17d1bda437d5f83abc656ccf4a0d9c7a57a83995668387c29bd0328f492"`. Read and validated `pass_binding` fields: worktree matches workspace root, parentCandidate matches record candidateHash, and scope matches record scope. Explicitly rejected attaching a plugin-focused pass binding to loan campaigns (`resource-scope-mismatch`).
   - *Verification*: Removed artificial `master_limit_seconds` inflation from `test_records.py:test_actual_master_family_admits_review`, asserting that attaching a plugin pass to a loan campaign is rejected.

3. **`M2-HISTORY-ZERO-ON-MISSING` (Unresolved history on missing operational store)**:
   - *Fix*: Removed unconditional `init_db` from `cli.py:main`. In `store.py:get_history`, missing SQLite file or missing events table returns `None`, reporting `operational-history` in `missingEvidence` rather than verified zeros. Added explicit `bootstrap` CLI command (`cli.py bootstrap`) and unit tests for missing store behavior. Handled `None` history values gracefully in `policy.py` when `_history_gap` is present.

4. **`M2-LAUNCH-NOT-REVALIDATED-NOT-CHARGED` (Post-reservation revalidation and bounded runner)**:
   - *Fix*: In `cli.py:cmd_run`, immediately revalidated snapshot and policy after `reserve` before child process launch. Wrapped child execution in a bounded runner with a 120-second timeout, repo-confined working directory, and 2048-byte stdout/stderr limits. Restricted `reproducer_verified` event recording to actual runner-observed behavioral assertions (`proc.returncode != 0` with `"Defect reproduced"` / `"admitted-not-executed"` in stderr without unhandled tracebacks/ImportError, or clean test pass `proc.returncode == 0`).

5. **`M2-REVIEW-AND-FINDING-IDENTITY` (Review ingest validation and finding lineage)**:
   - *Fix*: In `store.py:record_review`, strictly rejected author self-review (`author in reviewer`), enforced minimum 16-character candidateHash, and required non-empty scope. In `store.py:get_history`, tracked unresolved finding IDs by `(repository, requirement, capability)` lineage; generic `defect_resolved` counts cannot clear unresolved findings without matching finding IDs or defect classes. In `cli.py`, recorded admin intervals for `status`, `review`, and `report`, excluding `testing`/`afk`/`idle`.

6. **`M2-OUTBOX-DELIVERED-ON-STATUS` (Transaction outbox delivery persistence)**:
   - *Fix*: Removed automatic delivery marking from `cli.py:cmd_status`. Outbox transactions remain pending until explicitly confirmed delivered via `cli.py deliver --tx-id <ID>`. In `store.py:update_tx_status`, implemented state machine transitions (`submitted` -> `unknown-finality` -> `failed` / `confirmed` / `delivered`).

7. **`M3-DOCTOR-AND-HOST-UNVERIFIED` (Dynamic host trust inspection and pure-Python git traversal)**:
   - *Fix*: Replaced all `git` subprocess invocations in `records.py`, `store.py`, and `hook.py` with pure-Python `.git` traversal. In `cli.py:cmd_doctor`, dynamically inspected `~/.codex/hooks.json` and `~/.codex/plugins/moriarty-dev`, reporting `host-verified`, `degraded`, or `wrapper-only`. Updated `host-smoke.md` to distinguish CLI fallback, hook wire tests, and host interception.

8. **`M3-SKILL-ROUTING` (Product workflow alignment in develop skill)**:
   - *Fix*: Updated `skills/develop/SKILL.md` to document the approved product workflow: `User Intent -> status -> next -> Grok Implementation -> Actual Checks -> Fresh GPT-6 Review`. Clarified that AGY authoring was a temporary exception scoped solely to the plugin itself. Documented all 6 CLI verbs (`status`, `next`, `run`, `review`, `report`, `doctor`) and direct CLI execution as verified fallback.

9. **`M4-HARDCODED-REPRODUCER` (Controlled transport driver invocation)**:
   - *Fix*: Replaced in-process hardcoded stub in `reproduce_driver.py` with actual invocation of `run-local.mjs` via `node` through controlled inert transport (no wallets, no private keys, no network endpoints). The reproducer catches the unexecuted defect, writes `"Defect reproduced: driver returned 'admitted-not-executed'..."` to stderr, and exits with code 1. Updated `test_regressions.py:test_reproducer_demonstrates_defect` to assert this observed defect.

10. **`M4-PRINT-ONLY-AND-BORROWED-CATALOG` (Unborrowed actions catalog and real command runners)**:
    - *Fix*: In `.moriarty-dev/actions.json`, pointed SP05 actions to `campaign:sp05-ledger-driver-01` (honestly reporting unavailable/missing admission until admitted). In `.moriarty-dev/commands.json`, replaced dummy print commands with real runner commands (`node --test ...`).

11. **`M4-REPORT-NOT-FROM-RECORDS` (Dynamic 12-sprint status report from program records)**:
    - *Fix*: In `cli.py`, implemented `_derive_last_result` to inspect completion program stages and `_derive_sprint_reports` to read `sprints.json` and `moriarty-completion-program.json`, deriving distinct open predicates for all 12 sprints (e.g. BNF for SP02, executable K semantics for SP03, Preview for SP05).

12. **`M4-NO-PILOT-OR-HOST-INSTALL` (Honest boundaries and non-claims)**:
    - *Fix*: Maintained honest boundaries: no false claims of host trust or product pilot in worktree; `doctor` truthfully reports `wrapper-only` with CLI fallback; all 12 product sprints remain categorized as open with unestablished predicates.

---

## Verification Summary

| Verification Suite | Command | Result |
| --- | --- | --- |
| Full Test Discovery (114 tests) | `PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=plugins/moriarty-dev/scripts python3 -m unittest discover -s plugins/moriarty-dev/tests -v` | **114 passed, 0 failed, 0 errors** |
| Plugin Packaging Validator | `python3 /home/charl/.codex/skills/.system/plugin-creator/scripts/validate_plugin.py plugins/moriarty-dev` | **Plugin validation passed** |
| Skill Creator Validator | `python3 /home/charl/.codex/skills/.system/skill-creator/scripts/quick_validate.py plugins/moriarty-dev/skills/develop` | **Skill is valid!** |
| CLI Doctor Inspection | `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . doctor --json` | **Honest `wrapper-only` output** |
| CLI Dynamic Sprint Report | `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . report --json` | **12 distinct open sprint predicates** |
| Controlled Reproducer | `python3 plugins/moriarty-dev/tests/reproduce_driver.py` | **Exit 1, defect demonstrated on driver** |

---

## Honest Scope & Non-Claims

- **No Synthetic Progress**: All 12 sprints (SP01–SP12) remain open. Mandatory Preview financial settlement and PCD correspondence are reported as open until proven by real ledger evidence.
- **Truthful Host Interception**: `cli.py doctor` dynamically inspects `~/.codex/hooks.json` and `~/.codex/plugins/moriarty-dev`, reporting `wrapper-only` by default. Host hook interception is not claimed active without verified trust.
- **Independent Audit Routing**: Author has not self-approved or self-reviewed. All candidate bytes are ready for independent Grok 4.6 high code review on identical candidate bytes, followed by fresh GPT-6 Astra review only after Grok passes.
