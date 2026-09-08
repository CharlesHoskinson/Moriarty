---
name: develop
description: Moriarty development workflow enforcing stop rules, eligible next-action selection, and bounded guarded dispatch.
---

# Moriarty Development Workflow

Use this skill to guide and guard development on the Moriarty financial language for Midnight.
It enforces repository stop rules, prevents orchestration displacement, and directs execution toward concrete, demonstrable product capabilities.

## Operating Principles

1. **User Steering**: User requests, intent, or interruptions always take precedence over automatic action selection.
2. **Product Routing**:
   - Product implementation: Grok 4.6 at high reasoning effort.
   - Independent result review: fresh independent GPT-6 Astra.
   - Temporary plugin author exception: AGY Gemini 3.8 Flash high was explicitly selected solely for implementing the `moriarty-dev` plugin itself.
   - Consequential design decisions retain majority rule (two substantive agreeing votes). Routine repairs within approved scope proceed without repeated design votes.
3. **Guarded Dispatch**: Never invoke raw commands or arbitrary shell scripts. All execution goes through the plugin CLI `run` command or verified host hooks, validating authority, candidate commitments, and stop rules in an atomic reservation.
4. **Host Interception and Fallback**: Direct CLI execution (`cli.py run`) is the verified fallback procedure whenever host hook trust is unverified, degraded, or wrapper-only.
5. **Transaction Transparency**: When Midnight public transaction notifications are present, always post pending transaction IDs in the conversation before providing a network status summary.
6. **No Synthetic Progress**: Tests and packet approvals cannot establish missing behavior. Mandatory Preview financial settlement and PCD correspondence remain open until proven by real ledger evidence.

## Standard Product Development Workflow

The required sequence for all Moriarty tasks:
```text
User Intent -> status -> next -> Grok Implementation -> Actual Checks -> Fresh GPT-6 Review
```

### 1. Inspect Status (`status`)
Check current capability, last demonstrated result, active stop rules, and pending transactions:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status
```
If pending transaction IDs are returned, post them immediately to the conversation.

### 2. Determine Next Eligible Action (`next`)
Query policy for the next authorized action:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . next
```
If an implementation attempt is blocked by repeated failures (two cycles of the same defect class), the policy enforces reproduction (`reproduce`) before any retry.

### 3. Implement Focused Repair (Grok 4.6 High)
Implement the required repair within approved scope and limits without widening the design or erasing prior failures.

### 4. Execute and Check Action (`run`)
Run the action through the guarded CLI:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . run --action <ACTION_ID>
```
Exit codes:
- `0`: Success.
- `2`: Policy denial (repeated failure, administrative displacement, or resource exhaustion).
- `3`: Unavailable or stale inputs.
- `4`: Child command failure.

### 5. Ingest Independent Review (`review`)
When fresh GPT-6 Astra review completes on the exact candidate bytes, ingest the receipt:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . review --receipt <PATH_TO_RECEIPT_JSON>
```
Author self-review, mismatched candidate hashes, and empty scopes are rejected.

### 6. Report Progress Honestly (`report`)
Generate an all-sprint status report derived from actual program records:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . report
```

### 7. Inspect Environment Health (`doctor`)
Check repository store state, hook registration, and host trust:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . doctor
```
Reports `host-verified` (full hook interception active), `degraded` (partial hook configuration), or `wrapper-only` (CLI fallback active).
