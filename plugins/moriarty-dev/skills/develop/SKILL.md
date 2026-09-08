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
   - Current plugin implementation: GPT-6, as selected by the user. Follow later user routing for subsequent product work.
   - Independent result review: a separate, fresh GPT-6 Astra reviewer checks the exact candidate bytes.
   - Consequential design decisions retain majority rule (two substantive agreeing votes). Routine repairs within approved scope proceed without repeated design votes.
3. **Guarded Dispatch**: Launch registered campaign actions through the plugin CLI `run`, which validates authority, candidate commitments, debit identity and stop rules in an atomic reservation. Routine authorized file inspection and source edits do not need a new runner, campaign or design vote.
4. **Host Interception and Fallback**: Direct CLI execution (`cli.py run`) is the verified fallback procedure whenever host hook trust is unverified, degraded, or wrapper-only.
5. **Transaction Transparency**: When Midnight public transaction notifications are present, always post pending transaction IDs in the conversation before providing a network status summary.
6. **No Synthetic Progress**: Tests and packet approvals cannot establish missing behavior. Mandatory Preview financial settlement and PCD correspondence remain open until proven by real ledger evidence.

## Standard Product Development Workflow

The required sequence for all Moriarty tasks:
```text
User Intent -> status -> next -> GPT-6 Implementation -> Actual Checks -> Separate GPT-6 Review
```

### 1. Inspect Status (`status`)
Check current capability, last demonstrated result, active stop rules, and pending transactions:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status
```
If pending transaction IDs are returned, obtain their current statuses from `report --json` and post a dedicated conversation message containing only the public notification lines (one or more), before a separate network status summary:

```text
Midnight Preview transaction <TX_ID>: <STATUS>.
```

Use `notify --tx-id <TX_ID> --status submitted` after selecting a real public submission observation, and the same command with `unknown-finality`, `failed` or `confirmed` for subsequent observed changes. These are reported observations; the command does not query Midnight or establish financial acceptance. Never use test IDs as real transaction evidence.

After the line has been emitted in this conversation, call `deliver --tx-id <TX_ID>`. The CLI checks the current Codex session's emitted assistant message and stores its identity and digest. Tool output and caller-written acknowledgement JSON do not establish delivery. If the host record is unavailable, leave the notification pending. A later chain status becomes pending again. Delivery means an emitted host message, not proof that the user read it.

### 2. Determine Next Eligible Action (`next`)
Query policy for the next authorized action:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . next
```
If an implementation attempt is blocked by repeated failures (two cycles of the same defect class), the policy enforces reproduction (`reproduce`) before any retry.

### 3. Implement Focused Repair (GPT-6)
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
Reports `unverified` or `installed-unverified`. Hook definition and cached installation paths do not establish runtime trust or interception. Use the guarded CLI and retain a separate actual-host smoke result for each tested tool path.

Do not expand orchestration infrastructure to follow this workflow. Name the next demonstrable capability, implement its smallest authorized repair, and check its behavior. Preserve unresolved safety stops and independent acceptance gates.
