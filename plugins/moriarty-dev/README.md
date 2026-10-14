# Moriarty Development Plugin (`moriarty-dev`)

Repository-scoped development guidance for Moriarty's permissionless financial language on Midnight. Existing CLI controls guard registered maintainer execution; the skill selects current work and qualifies evidence.

## Current execution focus

Load the [develop skill](skills/develop/SKILL.md), read checkout `AGENTS.md`, and run CLI `status --json` before following a recovered queue. The user's current request controls task selection. Historical campaign status remains historical operational truth; it does not select today's task or establish consolidated roadmap acceptance.

Current nonsecurity work uses `gpt-6.1-sol` authoring and fresh independent `muse-wonder-1.0` audits, both at high effort. All security work belongs to Opus 5.5 under the [current assignment](../../docs/audits/2026-10-14/SECURITY-ROUTING.md). Later explicit user steering controls its scope. Preserve historical receipts and frozen contracts with their original rosters. Consequential design/resource decisions retain two substantive agreeing votes; that rule is separate from the current audit roster. The CLI does not enforce that roster or make receipt ingestion full acceptance.

The [roadmap](../../ROADMAP.md), [audit plan](../../docs/audits/2026-10-14/AUDIT-PLAN.md), [mid-roadmap review](../../docs/audits/2026-10-14/MID-ROADMAP-REVIEW.md), [OpenSpec/EARS package](../../openspec/changes/consolidated-language-kernel/proposal.md) and [wiki index](../../wiki/index.md) supply current requirements and evidence. The [execution reference](skills/develop/references/execution-focus.md) covers profiles, independent financial/work expectations, unchanged U/M/MC/SP/G exits and the Opus-owned zero-native boundary. Existing disjoint source ownership can continue with one integration owner; no scheduler or new campaign is added.

Report implementation, evidence and acceptance separately. Local/static child capabilities, historical proofs and Preview receipts retain their scopes. Fenrir remains separately qualified with unknown external facts left unknown. This plugin update does not complete a product exit or authorize native execution.

## Guarantees

1. **Stop Rule Enforcement**:
   - Denies repeated implementation dispatches after 2 failed cycles of the same defect class (`REPRODUCE_BEFORE_RETRY`).
   - Requires an admitted reproducer before permitting any focused repair.
   - Blocks administrative displacement after 2 administrative cycles or 1,800 observed seconds of administrative overhead (`PROCESS_DISPLACEMENT` / `ADMIN_LIMIT`).
2. **Atomic Single-Reservation**:
   - Enforces one primary reservation for registered guarded execution across all linked git worktrees.
   - Ordinary authorized source edits do not acquire this reservation. Native dispatch retains every existing gate.
   - Shared operational failure history stored in `<git-common-dir>/moriarty-dev/state.sqlite3`.
3. **Review Validation and Ingestion**:
   - Strictly validates review receipts against registered candidate hash, campaign scope, and independent non-author reviewer identity before mutating operational state.
   - Rejects receipts for unrelated candidate or scope with exit code 3.
   - Only authentic independent reviews (`verdict == "APPROVED"`) can clear active blockers; author `defect_resolved` entries cannot erase unresolved findings.
4. **History Integrity and Admin Accounting**:
   - Corrupt or unverified empty operational history remains explicitly unknown (`None`), preventing crashes and preventing false assertion of clean operational zeros.
   - Admin intervals are union-merged, excluding `test`, `testing`, `afk`, and `idle` durations.
5. **Transaction Outbox**:
   - Lists selected public Preview observations for posting in the conversation. `deliver` verifies an emitted assistant message in the current Codex session; it does not send a message itself.
   - Accepts only typed public fields. No arbitrary receipt, witness object or free-form status note is persisted. Transaction IDs and addresses must be deliberately selected public identifiers.
6. **No Synthetic Progress**:
   - Reports the retained 12-sprint campaign records without marking open gates as complete. Current U/M/MC/SP/G interpretation comes from the roadmap and evidence, not rewritten historical runtime records.

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

## Host adapter

The wire adapter uses `hook_event_name`, `tool_name` and `tool_input` (with legacy aliases), and emits `additionalContext` for session/post-tool reminders. Output stays valid JSON within 2 KiB. Stop hooks never request continuation. Hook failures produce a diagnostic; the CLI is the independent launch gate. File edits are not interpreted as shell dispatches.

The [official Codex hooks reference](https://developers.openai.com/codex/hooks) documents the inspected wire fields and compatibility `CLAUDE_PLUGIN_ROOT` variable. This source uses that supported variable without changing host internals. `doctor` reports source/installation observations and unverified runtime coverage. Actual interception must be checked through the host after installation and trust; directly calling the Python adapter is only a protocol test.

## Validation and installation

Run the read-only guidance contracts from the checkout root:

```bash
python3 -m unittest discover -s plugins/moriarty-dev/tests -p test_guidance.py -v
```

These checks cover explicit task precedence, current nonsecurity routing, profile/evidence qualification, links and manifest metadata. They are not agent-compliance trials or product acceptance. Existing admission/security/notification-auth tests remain under Opus ownership; inspect side effects and obtain their applicable execution authority before selecting any additional suite. No native checks are implied.

The canonical maintained package is `plugins/moriarty-dev/` in the reviewed repository checkout. Installed caches are snapshots: never patch them directly. Inspect `codex plugin marketplace list --json` and `codex plugin list --marketplace personal --json` to locate the configured source and installed version. For the inspected local personal marketplace, the catalog is `/home/charl/.agents/plugins/marketplace.json` and its mutable source is `/home/charl/plugins/moriarty-dev`; it is separate from the repository.

After the reviewed change is integrated, synchronize that canonical package into the configured mutable local source and verify package file digests against the reviewed candidate. Then use the host installer:

```bash
codex plugin add moriarty-dev@personal --json
codex plugin list --marketplace personal --json
```

The inspected CLI uses `plugin add`, not `plugin install`. `plugin marketplace upgrade` refreshes Git marketplace snapshots and does not publish a changed local source. Reopen a fresh context, load the refreshed skill, and compare the installed manifest/skill bytes with the reviewed source. If re-add does not refresh this host's installed version, report its exact result before choosing a host-supported recovery; do not silently edit cache contents or claim success. Installation does not establish hook trust: keep `doctor` coverage unverified until actual host interception has been observed. Repository links in a cache resolve through the active checkout paths described in the execution reference.
