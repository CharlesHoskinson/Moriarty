# Moriarty Development Plugin Foreman Implementation Report

## Summary

This report records the complete implementation of all four milestones (M1–M4) of the Moriarty development plugin (`moriarty-dev`) in worktree `/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok` by **AGY Gemini 3.8 Flash high**, pursuant to the user directive and the latest routing assignment (`raw/assignments/moriarty-plugin-agy-grok-gpt6-2026-09-08.md`).

All code, tests, manifest files, skill workflows, hook adapters, actions, and regression tests are implemented and verified locally. No model CLIs or subagents were spawned. All candidate bytes are ready for first review by **Grok 4.6 high** followed by independent second review by a fresh **GPT-6 Astra**.

---

## Addressed Milestone 1 Counterexamples (from Candidate-05 Review)

1. **M1-C5-E1 (`closureTask` deletion/null bypass)**:
   - `_publication_closures_ok` requires exact symmetric keys between original and published `rowSummaries`.
   - Every `closureTask` must be a non-null string matching the allowed pending-review closure replacement.
2. **M1-C5-E2 (Unbound `acceptedAtomicCode` and static verification)**:
   - `_publication_acceptance_ok` requires exact authentic atomic code `c30aaf5c0d091b1efea7e042d0fcbd539ca6cc51`.
   - Static verification references must exist and verify an authentic receipt with `exitCode == 0` and design scope.
3. **M1-C5-E3 (Review substitution)**:
   - `_apply_acceptance` validates that `acceptance.review` matches `record.review`.
   - `_publication_reviews_ok` parses every referenced review JSON, validating `verdict == APPROVED`, `scope == DESIGN_SCOPE`, and matching `candidateSha256`.
4. **M1-C5-E4 (Binding status validation)**:
   - `_verify_binding` requires direct and prerequisite bindings to carry exact status `"admitted-design-subset-only"`.

---

## Milestone Deliverables

### Milestone 1: Stop Rules & Evidence Admission
- `policy.py`: Pure, deterministic decision function evaluating current authority, candidate validity, entry gates, resource limits, primary reservation, repeated failure limits (2 cycles), and administrative displacement (2 cycles or 1,800s).
- `records.py`: Exact-source readers for completion program, sprint registers, campaign admissions, commands, and supervised accounting.
- `test_policy.py`: 36 unit tests for stop rules and boundary conditions.
- `test_records.py`: 59 unit tests covering genuine loan register admission, counterfeit rejection, and candidate verification.

### Milestone 2: Shared Operational Store & Guarded CLI
- `store.py`: SQLite operational event store located under `<git-common-dir>/moriarty-dev/state.sqlite3`. Provides atomic single-reservation for primary implementations using `BEGIN IMMEDIATE`, crash recovery, admin interval merging, and a transaction outbox that rejects private fields (`seed`, `sk`, `witness`, `private`).
- `cli.py`: Six CLI commands (`status`, `next`, `run`, `review`, `report`, `doctor`). Runs actions through `subprocess.run(argv)` without `shell=True`. Exit codes: `0` (success), `2` (policy denial), `3` (stale/unavailable inputs), `4` (child failure).
- `test_execution.py`: 8 black-box tests verifying denied launch creates no marker, failure persistence across renamed candidates, reproducer single-run, concurrency prevention, linked worktree state sharing, crashed reservation recovery, and private outbox rejection.

### Milestone 3: Packaging, Skill & Host Hooks
- `.codex-plugin/plugin.json`: Valid manifest passing `/home/charl/.codex/skills/.system/plugin-creator/scripts/validate_plugin.py`. Omits `hooks` field to use default discovery. Includes required `interface` metadata.
- `skills/develop/SKILL.md`: Developer skill workflow directing status inspection, next action determination, Grok implementation, GPT-6 review, and public transaction reporting.
- `hooks/hooks.json`: Default-discovered hook registrations for `SessionStart`, `PreToolUse`, `PostToolUse`, `Stop`.
- `scripts/moriarty_dev/hook.py`: Deterministic local hook wire adapter bounded to 1s execution and <= 2048 bytes output. Denies repeated failure dispatch in `PreToolUse`. Never requests automatic continuation in `Stop`.
- `test_hooks.py`: 5 tests verifying denied dispatch, permitted diagnostics, unrelated repository bypass, malformed input handling, and absence of `Stop` auto-continuation.
- `tests/host-smoke.md` & `README.md`: Honest smoke test guide and documentation.

### Milestone 4: Pilot Integration & Historical Regressions
- `.moriarty-dev/actions.json` & `.moriarty-dev/commands.json`: Catalog of authorized actions (SP01 loan review/report, SP05 ledger reproduce/repair/implement).
- `reproduce_driver.py`: Concrete reproducer for the public driver defect identified in the September 8 recurrence post-mortem (fails on `admitted-not-executed`).
- `test_regressions.py`: 5 regression tests demonstrating that composition green summaries cannot clear failing full-state tests, packets cannot erase prior lineage failures, stale checkpoints cannot claim live workers, and stop rules preserve focused repair after reproducing defect.
- `AGENTS.md` & `docs/FOOTGUNS.md`: Linked plugin CLI invocation and verified fallback procedure.

---

## Verification Summary

| Test Suite | Command | Result |
| --- | --- | --- |
| Full Test Discovery (113 tests) | `PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=plugins/moriarty-dev/scripts python3 -m unittest discover -s plugins/moriarty-dev/tests -v` | **113 passed, 0 failed, 0 errors** |
| GPT-6 Binding Probes (28 cases) | Candidate-05 28 probe suite | **All 28 cases passed** |
| Plugin Validator | `python3 /home/charl/.codex/skills/.system/plugin-creator/scripts/validate_plugin.py plugins/moriarty-dev` | **Validation passed** |

---

## Honest Scope & Non-Claims

- **No Synthetic Progress**: All 12 sprints (SP01–SP12) remain categorized as open. Mandatory Preview financial settlement and PCD correspondence are explicitly reported as open until proven by real ledger evidence.
- **Truthful Host Interception**: `cli.py doctor` reports `wrapper-only` by default. Full host interception is not claimed unless active Codex hook trust is verified.
- **Independent Audit Routing**: Author has not self-approved or self-reviewed. All candidate bytes are ready for Grok 4.6 high first check and fresh GPT-6 Astra second review.
