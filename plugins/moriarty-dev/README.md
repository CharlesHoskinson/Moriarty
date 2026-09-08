# Moriarty Development Plugin (`moriarty-dev`)

Repository-scoped development plugin that enforces stop rules, prevents orchestration displacement, and directs execution toward concrete, demonstrable financial capabilities for Moriarty on Midnight.

## Guarantees

1. **Stop Rule Enforcement**:
   - Denies repeated implementation dispatches after 2 failed cycles of the same defect class (`REPRODUCE_BEFORE_RETRY`).
   - Requires an admitted reproducer before permitting any focused repair.
   - Blocks administrative displacement after 2 administrative cycles or 1,800 observed seconds of administrative overhead (`PROCESS_DISPLACEMENT` / `ADMIN_LIMIT`).
2. **Atomic Single-Reservation**:
   - Enforces single primary implementation reservation across repository and all linked git worktrees.
   - Shared operational failure history stored in `<git-common-dir>/moriarty-dev/state.sqlite3`.
3. **Review Validation and Ingestion**:
   - Strictly validates review receipts against registered candidate hash, campaign scope, and independent non-author reviewer identity before mutating operational state.
   - Rejects receipts for unrelated candidate or scope with exit code 3.
   - Only authentic independent reviews (`verdict == "APPROVED"`) can clear active blockers; author `defect_resolved` entries cannot erase unresolved findings.
4. **History Integrity and Admin Accounting**:
   - Corrupt or unverified empty operational history remains explicitly unknown (`None`), preventing crashes and preventing false assertion of clean operational zeros.
   - Admin intervals are union-merged, excluding `test`, `testing`, `afk`, and `idle` durations.
5. **Transaction Outbox**:
   - Forwards public transaction IDs to conversation output.
   - Rejects private witness data, private keys, seeds, or spending keys from persistence or outbox.
6. **No Synthetic Progress**:
   - Reports all 12 sprints honestly without marking open gates as complete.

## Limitations

- **Runner admission**: Execution requires a `moriarty.bound-runner/1` entry under `runners[ACTION_ID]` in the campaign's hash-verified binding. It binds the complete action, argv, executable and input hashes, Foreman launcher hashes, timeout, cleanup grace, output limit and existing debit identity. A command catalog entry alone cannot authorize a launch. Current production catalog entries without this binding remain unavailable.
- **Charging**: The existing budget runner records the full debit before invoking the CLI. The debit must bind `actionId`, `candidateHash` and the canonical runner-plan SHA-256 (`runnerDigest`). The plugin consumes that charge once in the shared operational store and never charges or refunds the budget. An already charged verification does not require a second allocation. Existing worker-dispatch caps still apply.
- **Behavioral evidence**: An admitted reproducer declares a nonempty assertion ID, a distinct defect exit code and expected stdout digest. Its reviewed source must emit that result only after evaluating the specified behavior. The parent records observed output and process status; child JSON cannot supply charge or completion authority. This contract is not a formal proof or a product acceptance verdict.
- **Recovery**: Unresolved supervisor completion leaves the reservation and charge claimed. Parent death does not establish child termination. Output is bounded during collection; overflow fails the action. Automatic reconciliation of ambiguous runs is not implemented.
- **Coverage**: Direct invocation via the CLI wrapper is guaranteed. Host hook interception depends on host trust and active hook execution; wrapper-only status is reported unless host interception is independently verified.
- **Security Boundary**: The plugin does not protect against an adversarial agent rewriting local files or bypassing the CLI. It guards the standard agent workflow against accidental loop recurrence.

## CLI Usage

```bash
# Status (five fields + pending tx IDs)
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json

# Next eligible action
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . next --json

# Guarded execution
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . run --action <ACTION_ID>

# Review receipt ingestion
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . review --receipt <RECEIPT_PATH>

# Sprint status report
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . report --json

# Diagnostic doctor
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . doctor --json
```
