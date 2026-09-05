# S02 Recovery Registry Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this task with independent review.

**Goal:** Make both mandated recovery subscenarios explicit in the closed S02 machine-readable requirements.

**Architecture:** Extend the existing specification-only registry, not the future trace validator. Preserve every current candidate, profile, property, witness, control, and package gate identifier.

**Tech Stack:** JSON, Python pytest.

## Global Constraints

- Adopted design: `docs/superpowers/specs/2026-09-05-moriarty-s02-common-design-decision.md`, recovery coverage disposition.
- Preserve Core `0.0.0-e00.2` and every immutable receipt. No Foreman work.
- Keep `status: specified-only`, `evidence: []`, and `selected_candidate: null`.
- This is requirements transcription, not evidence that a recovery trace exists.
- Shared worktree: modify only the two owned files below. Do not stage or commit; the controller will serialize commits after review.

## Task 1: Closed recovery subscenario requirements

**Files:** Modify `evidence/s02-model-comparison/requirements.json` and `tests/test_s02_contract.py` only.

**Interface:** Add `required_subscenarios`, a list of the exact two records below.
Bump `schema_version` from 1 to 2 because the closed top-level contract changes.
No other existing value changes.

```json
[
  {
    "id": "cancel-wins/recovery-before-any-fill",
    "witness_id": "cancel-wins",
    "workload": "two-installment-obligation",
    "candidates": ["A", "B", "C", "D"],
    "signing_profiles": ["SignAfterResolve", "SignBeforeResolve"],
    "package_gates": ["S02-05", "S02-09"],
    "required_events": ["parent-cancelled", "recovery-signed", "recovery-verified", "recovery-committed"],
    "terminal": {"escrow": 0, "paid_to_bob": 0, "refunded_to_alice": 10, "parent_cancelled": true, "used_slots": [], "parent_nonce": 0, "recovery_nonce": 1, "recovery_consumed": true, "second_slot_authorized": false}
  },
  {
    "id": "fill-wins/recovery-after-first-fill",
    "witness_id": "fill-wins",
    "workload": "two-installment-obligation",
    "candidates": ["A", "B", "C", "D"],
    "signing_profiles": ["SignAfterResolve", "SignBeforeResolve"],
    "package_gates": ["S02-05", "S02-09"],
    "required_events": ["first-fill-committed", "stale-cancel-rejected", "fresh-parent-cancelled", "recovery-signed", "recovery-verified", "recovery-committed"],
    "terminal": {"escrow": 0, "paid_to_bob": 5, "refunded_to_alice": 5, "parent_cancelled": true, "used_slots": [1], "parent_nonce": 0, "recovery_nonce": 1, "recovery_consumed": true, "second_slot_authorized": false}
  }
]
```

`required_events` is a membership requirement, not an additional total ordering
constraint: SignBeforeResolve may sign recovery earlier. Later path checking must
enforce the true causal lifecycle and observe these events in that path.

- [ ] Update the closed-key and schema-version assertions; add a test comparing
  `value["required_subscenarios"]` to the exact JSON above loaded as a Python
  literal. Add assertions that each record's `witness_id` is in the preserved
  witness list, candidates/profiles exactly match the registry, and final money
  sums to ten. Do not replace existing exact-vocabulary assertions.
- [ ] Run `/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_contract.py -q`.
  RED must report the missing required field/version, not an import or syntax error.
- [ ] Add exactly the field above and schema version 2 to the JSON registry.
- [ ] Run the focused test command again, then
  `/home/charl/Moriarty/.venv/bin/python scripts/validate_s01_intent_evidence.py`
  and `git diff --check`. Require the S01 gate to remain unchanged.
- [ ] Self-review exact preservation of the original identifier lists and
  specification-only status. Write the requested scratch report with RED/GREEN
  outputs and files changed. Do not claim a model trace, validator, or passed S02 gate.
- [ ] Controller obtains independent spec/quality review and commits the two files.
