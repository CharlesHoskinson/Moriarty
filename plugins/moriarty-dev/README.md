# Moriarty Development Plugin (`moriarty-dev`)

Repository-scoped development plugin that enforces stop rules, prevents orchestration displacement, and directs execution toward concrete, demonstrable financial capabilities for Moriarty on Midnight.

## Execution focus

The [develop skill](skills/develop/SKILL.md) adds the September 11 delivery recommendations: one acceptance predicate in progress, one short reconciliation in existing records, implementation through the actual caller, independent failure cases before authoring, and concurrent fresh audits of the complete candidate. The current target is uncertified I2/MC02 integration; it is not full SP05 or mandatory PCD acceptance. A separately bounded F0 verifier-compatibility decision may proceed with independent capacity and resource authority. Existing K failures call for a single-case reproduction before broad retries, with full final conformance retained.

The [execution reference](skills/develop/references/execution-focus.md) provides financial coverage checks, context and review practice, six manual metrics, and native/K investigation boundaries. These are agent workflow instructions, not new runtime enforcement or proof of increased speed. No scheduler, admission exception, or new evidence register is introduced. New profiles, broad finance expansion, backend migration and deferred productization/mandatory CI work remain outside this focus interval.

For this plugin update the user selected Astra authoring and fresh Grok 4.6 high plus Astra medium audits. The earlier general product routing remains Grok authoring with fresh Opus/Astra reviews unless later user steering changes it. The CLI's receipt validation does not itself enforce either complete reviewer roster.

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
   - Only authentic independent reviews (`verdict == "APPROVED"`) with explicit `resolvedFindings` IDs (legacy alias `resolved`) can clear those named blockers; missing or empty lists clear nothing and malformed lists are rejected; author `defect_resolved` entries cannot erase unresolved findings.
4. **History Integrity and Admin Accounting**:
   - Corrupt or unverified empty operational history remains explicitly unknown (`None`), preventing crashes and preventing false assertion of clean operational zeros.
   - Admin intervals are union-merged, excluding `test`, `testing`, `afk`, and `idle` durations.
5. **Transaction Outbox**:
   - Lists selected public Preview observations for posting in the conversation. `deliver` verifies an emitted assistant message in the current Codex session; it does not send a message itself.
   - Accepts only typed public fields. No arbitrary receipt, witness object or free-form status note is persisted. Transaction IDs and addresses must be deliberately selected public identifiers.
6. **No Synthetic Progress**:
   - Reports all 12 sprints honestly without marking open gates as complete.

## Limitations

- **Runner admission**: Execution requires a `moriarty.bound-runner/1` entry under `runners[ACTION_ID]` in the campaign's hash-verified binding. It binds the complete action, argv, executable and input hashes, Foreman launcher hashes, timeout, cleanup grace, output limit and existing debit identity. A command catalog entry alone cannot authorize a launch. Current production catalog entries without this binding remain unavailable.
- **Charging**: The existing budget runner records the full debit before invoking the CLI. The debit must bind `actionId`, `candidateHash` and the canonical runner-plan SHA-256 (`runnerDigest`). The plugin consumes that charge once in the shared operational store and never charges or refunds the budget. An already charged verification does not require a second allocation. Existing worker-dispatch caps still apply.
- **Behavioral evidence**: An admitted reproducer declares a nonempty assertion ID, a distinct defect exit code and expected stdout digest. Its reviewed source must emit that result only after evaluating the specified behavior. The parent records observed output and process status; child JSON cannot supply charge or completion authority. This contract is not a formal proof or a product acceptance verdict.
- **Recovery**: Unresolved supervisor completion leaves the reservation and charge claimed. Parent death does not establish child termination. Output is bounded during collection; overflow fails the action. Automatic reconciliation of ambiguous runs is not implemented.
- **Coverage**: Direct invocation via the CLI wrapper is guaranteed. Host hook interception depends on host trust and active hook execution; `doctor` reports unverified coverage and never infers interception from source or registration files. Supported hook adapters recognize shell action IDs and exact registered argv; arbitrary shell encodings, code-mode composition and native delegation remain outside that recognition guarantee.
- **Security Boundary**: The plugin does not protect against an adversarial agent rewriting local files or bypassing the CLI. It guards the standard agent workflow against accidental loop recurrence.
- **Historical campaign compatibility**: The reader validates the closed bounded-K observation family (`admission`, `result`, and nonnegative integer `consumed` compile/krun counters) for its three recorded failed/pending-review statuses. Provenance paths must be present and contained. These retained observations grant no dispatch authority or budget credit; current bindings, candidates, accounting and operational history still need their independent checks. Malformed observations and unknown fields remain fail-closed.

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

## Public transaction reporting

`notify --tx-id <ID> --status submitted` records a selected public Preview submission. Later observations use the same command with `unknown-finality`, `failed` or `confirmed`; a status update requires a known submission. Duplicate submissions preserve the latest chain status. These commands report observations and do not query the ledger, submit transactions or grant product acceptance.

`report --json` returns pending IDs and statuses. Post a dedicated conversation message containing only one or more lines of this form: `Midnight Preview transaction <ID>: <STATUS>.` Then run `deliver --tx-id <ID>`. Delivery reads only the current host session's metadata and bounded 2 MiB tail, selects a dedicated emitted assistant notification message after that status became pending, and persists message identity, timestamp and digest. User/tool/analysis messages, any message containing prose, Markdown, HTML or indentation, stale messages and caller JSON acknowledgements do not count. The accepted message has a closed format, so delivery does not need to interpret Markdown. No transcript text is persisted.

Delivery and chain status are separate: a later confirmation requires another notification. Unavailable host records leave delivery pending. The adapter supports the locally inspected Codex session format; it proves local host emission, not human receipt or network finality. Local same-user tampering remains outside this plugin's security boundary.

## Compatibility and upgrades

See [the compatibility contract](COMPATIBILITY.md) for the support matrix, package-only tests, installation diagnostics and cache-retention upgrade procedure. Active sessions keep their versioned cache paths; preserve those packages through upgrades. `doctor --expected-plugin-root PATH --json` checks a known session reference without changing it.

## Host adapter

Normal Stop returns `{}`. Permitted PreToolUse calls omit the permission decision; explicit denials remain intact. Malformed input receives bounded diagnostic JSON. Only supported CLI `run` invocations and exact registered raw argv are treated as dispatches, so searches and review commands may mention action IDs.

The wire adapter uses `hook_event_name`, `tool_name` and `tool_input` (with legacy aliases), and emits `additionalContext` for session/post-tool reminders. Output stays valid JSON within 2 KiB. Stop hooks never request continuation. Hook failures produce a diagnostic; the CLI is the independent launch gate. File edits are not interpreted as shell dispatches.

The [official Codex hooks reference](https://developers.openai.com/codex/hooks) documents the inspected wire fields and compatibility `CLAUDE_PLUGIN_ROOT` variable. This source uses that supported variable without changing host internals. `doctor` reports source/installation observations and unverified runtime coverage. Actual interception must be checked through the host after installation and trust; directly calling the Python adapter is only a protocol test.
