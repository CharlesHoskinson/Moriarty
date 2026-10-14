---
name: develop
description: Use when developing, reviewing, researching, planning, or reporting status in the Moriarty repository, including recovery and guarded campaign dispatch.
---

# Moriarty Development Workflow

Use this skill to guide and guard development on the Moriarty financial language for Midnight.
The CLI enforces registered dispatch controls; this skill supplies workflow guidance toward demonstrable product capabilities. Focus, context selection, model routing and acceptance reconciliation are agent responsibilities, not new mechanical gates.

## Operating Principles

1. **User Steering**: User requests, intent, or interruptions always take precedence over automatic action selection.
2. **Product Routing**:
   - The September 11 plugin-update request selects **GPT-6 Astra authoring**, with separate fresh **Grok 4.6 high** and **GPT-6 Astra medium** audits. This task-specific choice supersedes older defaults for this update; it does not change subsequent product routing.
   - The September 10 general product instruction selects **Grok 4.6 authoring** with fresh **Claude Opus and GPT-6 Astra medium** reviews. Apply any later user instruction to its stated scope; stale recovered routing must not override it.
   - Use explicit model identities (`grok-4.6`, `gpt-6-astra` where applicable), record requested effort and returned identity, and preserve historical verdicts. Both selected auditors review the full exact candidate independently and concurrently. Missing auditors do not approve or trigger silent substitution.
   - Consequential design decisions retain majority rule (two substantive agreeing votes). Routine repairs within approved scope proceed without repeated design votes.
3. **Guarded Dispatch**: Launch registered campaign actions through the plugin CLI `run`, which validates authority, candidate commitments, debit identity and stop rules in an atomic reservation. Routine authorized file inspection and source edits do not need a new runner, campaign or design vote.
4. **Host Interception and Fallback**: Direct CLI execution (`cli.py run`) is the verified fallback procedure whenever host hook trust is unverified, degraded, or wrapper-only.
5. **Transaction Transparency**: When Midnight public transaction notifications are present, always post pending transaction IDs in the conversation before providing a network status summary.
6. **No Synthetic Progress**: Tests and packet approvals cannot establish missing behavior. Mandatory Preview financial settlement and PCD correspondence remain open until proven by real ledger evidence.

## Standard Product Development Workflow

For execution work, use:
```text
User Intent -> status -> next -> Focused Implementation -> Actual Checks -> Both Fresh Audits -> Scoped Result
```
Read-only diagnosis, reviews and authorized edits need `status` but do not require a new campaign or executable `next` action. An admission failure blocks its dependent dispatch, not independent source repair.

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

Name one missing acceptance predicate and keep **one delivery implementation item in progress** through result review. Idle capacity helps with its independent expected results, fault cases or blockers. Allow one bounded native F0 feasibility decision alongside it only with independent capacity and existing resource authority; this is not a second broad proving campaign. Do not add a scheduler or orchestration framework.

For the current financial bottleneck, target **I2/MC02 complete-effect loan/swap integration**. I2 remains uncertified until MC05 and does not require native PCD; full SP05 still depends on SP01. Reconcile existing MC02 evidence once, then change or execute the actual missing caller path. Defer new profiles, broad ACTUS/DeFi expansion, backend migrations, productization and mandatory new CI work unless the user changes scope.

Read [execution focus](references/execution-focus.md) when selecting I2 work, preparing author/review context, investigating native feasibility or K failures, or measuring this workflow. It contains the exact financial gap checks, lifecycle failure matrix, F0 discriminator and reviewer pressure scenarios.

### 3. Implement Focused Repair (selected author)
Include the actual consumer of a new helper in the same demonstrable behavior. Before authoring, have an independent reviewer define expected outcomes and failure cases; run the existing callable integration tests before freezing bytes. Return all current failures together with focused patches and sufficient current context; retain complete history on disk. Continue the author through repairs and use a fresh author at the next independent capability boundary. Preserve approved scope, resources and recorded failures.

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
Obtain both selected fresh audits on the same frozen full candidate and applicable contract, not diff-only or reused source approval. Ingest supported receipts individually:
```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . review --receipt <PATH_TO_RECEIPT_JSON>
```
Author self-review, mismatched candidate hashes, and empty scopes are rejected.
Receipt ingestion validates its schema and identity; it does not mechanically enforce this skill's two-provider routing. Retain both actual audit outputs in existing records and verify both required scopes before reporting approval. Changed candidate bytes require fresh current audits; source approval never replaces actual-result review or live admission.

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
