---
name: develop
description: Moriarty development workflow enforcing stop rules, eligible next-action selection, and bounded guarded dispatch.
---

# Moriarty Development Workflow

Use this skill to guide and guard development on the Moriarty financial language for Midnight.
It enforces the repository stop rules, prevents orchestration displacement, and directs execution toward concrete, demonstrable product capabilities.

## Operating Principles

1. **User Steering**: User requests, intent, or interruptions always take precedence over automatic action selection.
2. **Current Routing**:
   - Primary implementation: AGY Gemini 3.8 Flash high.
   - First substantive review: Grok 4.6 high.
   - Independent second review: fresh GPT-6 Astra (after Grok passes the identical candidate bytes).
   - Consequential design decisions retain majority rule (two substantive agreeing votes). Routine repairs within approved scope proceed without repeated design votes.
3. **Guarded Dispatch**: Never invoke raw commands or arbitrary shell scripts. All execution goes through the plugin CLI `run` command, which validates authority, candidate commitments, and stop rules in an atomic reservation.
4. **Transaction Transparency**: When Midnight public transaction notifications are present, always post pending transaction IDs in the conversation before providing a network status summary.
5. **No Synthetic Progress**: Tests and packet approvals cannot establish missing behavior. Mandatory Preview financial settlement and PCD correspondence remain open until proven by real ledger evidence.

## Standard Workflow Steps

### Step 1: Inspect Status
Check the current capability, last demonstrated result, active stop rules, and pending transactions:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status
```
If pending transaction IDs are returned, post them immediately to the conversation.

### Step 2: Determine Next Eligible Action
Query the policy for the next authorized action:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . next
```
If an implementation attempt is blocked by repeated failures (two cycles of the same defect class), the policy will require a reproducer (`reproduce`) before any retry.

### Step 3: Execute Action Guarded by Policy
Run the action through the guarded CLI:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . run --action <ACTION_ID>
```
Exit codes:
- `0`: Success.
- `2`: Policy denial (e.g. repeated failure, administrative displacement, or resource exhaustion).
- `3`: Unavailable or stale inputs.
- `4`: Child command failure.

### Step 4: Ingest Review Receipts
When independent review completes, ingest the receipt to update the operational store:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . review --receipt <PATH_TO_RECEIPT_JSON>
```

### Step 5: Report Progress Honestly
Generate an all-sprint status report:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . report
```
