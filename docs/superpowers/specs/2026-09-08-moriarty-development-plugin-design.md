# Moriarty development plugin design

Date: 2026-09-08. Status: proposed design, not implemented or installed.
User request: use Superpowers to plan a development plugin that prevents orchestration displacement and advances Moriarty's goals.

## Purpose and success

Build `moriarty-dev`, a repository-scoped development plugin that interrupts repeated ineffective work and selects a useful next action.
Its first demonstration must reject the actual repeated-failure pattern and permit a focused repair of the current ledger path.
The plugin succeeds through its effect on Moriarty development, not by accumulating its own receipts or features.

The [recurrence post-mortem](../../postmortems/2026-09-08-orchestration-recurrence.md) supplies the failure cases.
The [footguns](../../FOOTGUNS.md#orchestration-stop-rules) supply the stop rules.
The [sprint register](../../../openspec/sprints/sprints.json) supplies task identities.
The [program register](../../../openspec/moriarty-completion-program.json) and its referenced campaign records remain acceptance and admission authorities.

Planning this plugin is explicitly authorized by the latest request.
That authorization permits this focused design despite the earlier instruction against unsolicited orchestration infrastructure.
It does not authorize implementation, installation, resource expansion or changes to product acceptance through this planning document.

### Superpowers planning checklist

- [x] Inspect the existing product goals, failure evidence and plugin interfaces.
- [x] Recover constraints from the user's existing instructions.
- [x] Compare three approaches and recommend one.
- [x] Write the proposed design and a concrete implementation plan.
- [x] Self-review scope, interfaces, failure behavior and requirement coverage.
- [ ] Obtain review of this proposed design before implementation.

No visual companion is needed for this command-line workflow.
Working assumptions: Codex is the host, Grok implements, fresh GPT-6 reviews, and existing authority covers routine repairs.
All twelve sprints remain required. These assumptions come from the current session.

## Alternatives

| Approach | Benefit | Limitation | Decision |
| --- | --- | --- | --- |
| Skills and stronger prose only | Smallest installation and maintenance burden | Repeats the first remedy, which depended entirely on agent compliance | Reject as the sole control |
| One skill, deterministic checks, guarded execution and host hooks | Blocks known ineffective actions and gives a concrete replacement action | Enforcement covers verified integration paths, not every possible agent action | Recommend |
| New scheduler, model council service and autonomous supervisor | Could centralize more execution | Creates a large competing product and repeats the failure mechanism | Exclude from this project |

## User workflow

Invoke the `moriarty-dev:develop` skill with a goal or resume request.
The skill reads one short status response and continues the next eligible action.
It preserves the existing Grok 4.6 high implementation and fresh GPT-6 Astra result-review routing.
Consequential decisions retain the existing majority rule. Routine repairs do not acquire another design vote.

The status response has five fields:

1. Requested capability and owning sprint/task.
2. Last demonstrated result, evidence scope and candidate identity.
3. Missing acceptance conditions.
4. Next eligible action and its existing admission reference.
5. Active stop rule or missing evidence, if any.

Example after repeated ledger failures:

```text
Capability: SP05 fixed financial execution and receipt decoding
Last result: local custody accepted; financial settlement remains open
Blocked action: another broad ledger correction
Reason: two failures of required execution behavior
Next: run the driver-call reproducer against the candidate, then give Grok that failing test
Network: no financial transaction evidence recorded
```

The plugin never asks a question merely because a routine action needs selecting.
User interruption or steering takes precedence over a recovered task.
A requested plan, post-mortem or status report is a valid deliverable and does not force an unrelated coding action.

## Components and boundaries

Source lives at `plugins/moriarty-dev/` in Moriarty for Git review and recovery.
Installation uses the existing personal marketplace, not a new team marketplace.
Version one has no MCP server, dashboard, daemon, model API client, new scheduler or wallet implementation.

| Component | Responsibility |
| --- | --- |
| `skills/develop/SKILL.md` | Apply the workflow and select existing Superpowers skills when needed |
| `scripts/moriarty_dev/policy.py` | Pure decision function for next actions, stop rules and evidence requirements |
| `scripts/moriarty_dev/records.py` | Read exact repository/campaign references and normalize selected evidence fields |
| `scripts/moriarty_dev/store.py` | Persist operational events and reservations in one local SQLite database |
| `scripts/moriarty_dev/cli.py` | Status, action execution, review ingestion, reporting and diagnostics |
| `scripts/moriarty_dev/hook.py` | Translate supported host events into calls to the same policy |
| `tests/` | Independent regressions, launch-boundary tests and an actual-host smoke recipe |

Use Python 3.11 or later and the standard library. Use `unittest`, `sqlite3`, `subprocess` and `hashlib`.
The database is operational history, not a second acceptance register or resource ledger.
Store it under the Git common directory as `moriarty-dev/state.sqlite3` so linked worktrees share failure history.
Keep immutable product evidence in existing evidence locations.
Keep installed plugin caches and marketplace registration outside the repository source tree.

### Interface contract

The proposed invocation is:

```text
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo ROOT status --json
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo ROOT next --json
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo ROOT run --action ACTION
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo ROOT review --receipt PATH
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo ROOT report --json
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo ROOT doctor --json
```

`ACTION` selects an exact action from `.moriarty-dev/actions.json`.
That file maps existing task, requirement, candidate, command and admission references. It grants no permission itself.
The CLI accepts no free-form command string, no `--force` and no agent-writable success flag.
Tests use disposable repositories and fake child processes, not live provider calls.

`assess(snapshot, action) -> Decision` is pure.
`Decision` contains `allow`, `reasonCode`, `nextActionId` and `missingEvidence`.
`load_snapshot(repo, action_id) -> Snapshot` verifies current referenced bytes and resolves that action's stage/campaign state.
`reserve(db, action, snapshot) -> reservation_id` atomically claims one launch for the candidate/action identity.
`finish(db, reservation_id, receipt)` records observed process completion without accepting a product claim.
`render_report(snapshot, events) -> dict` separates verified results, author reports and missing evidence.

## Rules the program must execute

### Product identity and eligible work

Every action references a sprint task and an existing acceptance requirement, or an explicit current user deliverable.
Its stable lineage key is `(repository, requirement, capability)`, independent of candidate number, branch, session or budget amendment.
Only one primary implementation reservation may be active across the repository and all linked worktrees.
One independent review may run alongside it.
Existing stricter heavy-process and resource limits continue to apply.

Resolve task entry through `entryGates` and authoritative stage records, not whole-sprint completion.
An F0 blocker must not make an otherwise admitted SP05 task ineligible.
Unknown, conflicting or stale admission produces `ADMISSION_UNRESOLVED`, naming the exact reference.
The plugin must not guess that a historical `dispatchEnabled` field or a newer-looking file overrides later authority.
If an external live amendment cannot be resolved through an explicit reference, block only the dependent action.

### Repeated failure

Two failed result cycles for the same requirement and unresolved defect class block another broad implementation dispatch.
Use defect identities from independent review findings, with a retained mapping to the requirement.
Missing classification remains an unresolved failure. Renaming the finding must not clear it.

The replacement is an already admitted reproducer, focused repair, required result review or independent eligible task, in that order.
A focused repair requires the failing behavioral command, affected source paths and a concrete changed approach.
A passing reproducer closes only its covered finding after the required independent review.
Other unresolved findings persist. Budget renewal and packet approval clear none.

### Process displacement

Two completed administrative cycles without relevant behavioral evidence block another administrative cycle.
At 1,800 accumulated seconds of observed administrative activity, apply the same rule.
Measure registered activity intervals, not elapsed AFK time.
Count overlapping intervals once. Exclude actual build/test/proof execution and user waiting.
Unknown intervals are reported as unknown, never as zero measured overhead.

Packet publication, source hashing, report formatting and resource renewal are administrative activity.
Documentation explicitly requested by the user is a product deliverable for that task.
A technical blocker is resolved only by a distinguishing experiment or an enabled operation, with evidence.
An agent-written explanation alone does not reset the counter.

The hook records supported activity but does not try to infer all behavior from shell text or prose.
An unregistered action receives no progress credit.
No production-test allowance can be earned by running the plugin's own tests.

### Demonstration and review

An execution requirement needs a registered behavioral command through the callable production path.
The command must test input-dependent behavior and a relevant independent rejection case.
Missing commands remain missing. The plugin must not invent a runner for planned K or network functionality.

The root runner records exit status, exact argv, input/output hashes, duration and candidate source hashes.
An author report cannot supply these observations on the runner's behalf.
The evidence profile distinguishes syntax, local runtime, K execution, theorem discharge, proven build and finalized financial settlement.
Evidence does not automatically move upward between those profiles.

GPT-6 review must bind the same candidate bytes and required scope and originate from a separate reviewer task.
Requested and observed model identities remain separate. Unknown serving identity must stay unknown.
A valid review envelope establishes provenance and verdict, not the truth of every semantic claim.
Semantic correctness still needs the required tests and proofs.

### Guarded launch and recovery

`run` checks policy, source identity and existing admission inside a single reservation transaction before launching.
It then invokes the exact existing bounded runner, which remains responsible for resource charging and process containment.
The plugin must not charge a second budget or refund the existing one.
No shell evaluation or arbitrary command construction is permitted.
Changed source or exhausted authority between selection and launch rejects the action.

A crash leaves an unfinished reservation visible.
Recovery checks the actual runner/process and receipt before classifying it as active, failed or interrupted.
It never starts a duplicate launch or assumes a timed-out blockchain request was unsubmitted.
Read-only status and the concrete repair path remain available when other actions are denied.

## Host integration and honest enforcement claims

Local observation: installed Codex CLI is `0.153.4`.
The installed plugin validator rejects a manifest `hooks` field, while its sample reference includes that field.
Use the conventional `hooks/hooks.json` location and omit the manifest field.
The official plugin documentation describes that default discovery path. [Plugin packaging](https://developers.openai.com/plugins/build/plugins)

Official hooks can deny supported local tool calls, but some paths are excluded.
Plugin hooks require trust before running. Stop-hook blocking creates a continuation.
These facts require runtime coverage tests and prohibit claims of universal enforcement. [Hooks](https://learn.chatgpt.com/docs/hooks)

Proposed adapters:

- `SessionStart`: inject the five-field status after startup, resume or compaction.
- `PreToolUse`: deny identified repeated dispatch or unsupported acceptance mutations and return the next permitted action.
- `PostToolUse`: record selected safe observations. Do not infer successful completion from output text alone.
- `Stop`: add a concise evidence/status correction when needed. Never request automatic continuation in version one.

All hook code is local, deterministic and bounded to one second and 2 KiB output.
Hooks make no model calls, run no tests, submit no transactions and never request approval.
Malformed hook output or host failure cannot be assumed to block execution.
The launch command independently fails closed on policy/store errors.

`doctor` reports `wrapper-only`, `host-verified` or `degraded`, with the observed host/tool coverage.
It cannot declare host coverage by calling the hook script directly.
The host smoke test must actually attempt a harmless denied dispatch and prove its child marker was never created.
Test ordinary shell, unified exec, `apply_patch`, native delegation and the current `functions.exec` path separately.
Report unsupported paths rather than implementing a general JavaScript or shell parser.

A plugin cannot guarantee mathematical breakthroughs, force a model to reason correctly, or stop all commands outside its integration path.
An unrestricted agent can also edit local controls. This design is not a security boundary against that actor.
It enforces concrete dispatch and reporting predicates on the verified workflow and exposes remaining gaps.
Do not describe a wrapper-only installation as universal enforcement.

## Connection to all twelve sprints

The plugin reads this mapping from existing records. It does not duplicate or rewrite their acceptance status.

| Sprint | Demonstration category the report must preserve |
| --- | --- |
| SP01 | Reviewed financial semantics and exact task admission |
| SP02 | Complete successor grammar/lexical rules and parser agreement |
| SP03 | Executable Moriarty K and separately discharged scoped claims |
| SP04 | Actual native interface feasibility and meaningful rejection controls |
| SP05 | Loan and swap financial settlement and independent readback on Preview |
| SP06 | Real recursive financial history proof |
| SP07 | ACTUS obligation and lifecycle behavior |
| SP08 | DeFi actions and intent lifecycle behavior |
| SP09 | Mandatory PCD and ledger correspondence |
| SP10 | Private handoff and bounded composition |
| SP11 | Complete required financial and formal conformance |
| SP12 | Reproducible developer release and evidence |

For Midnight, consume public transaction notifications from the existing driver.
Emit each ID immediately to the current tool output, persist it, and retain failure or unknown-finality status.
The skill must post those IDs in the conversation. A local outbox records which still need presentation.
Do not mark an ID delivered merely because it exists in a file.
Finalized status requires the existing canonical block and financial readback predicate.
The plugin never reads seeds, private witness data, wallet state or provider reasoning logs.

## Acceptance and limit on this plugin project

The implementation plan has four milestones. Each adds working behavior and a corresponding regression.
Do not add a fifth milestone for a framework, dashboard, generic scheduler or another review service.
Any new feature must be necessary for a failing acceptance case in this design.
After each milestone, continue directly to the next listed acceptance case under existing authority.
Do not add a new design packet when the design and resource envelope remain unchanged.

Before expanding the plugin, demonstrate one actual Moriarty defect repaired and independently reviewed through the guarded workflow.
The initial target is the SP05 ledger driver/decoder gap preserved by the post-mortem.
That repair uses its own existing or properly amended implementation admission. Plugin installation grants no network or proving budget.

Release requires these observations:

- Repeated process and defect cases are denied before the child process starts.
- A concrete reproducer and bounded repair remain possible under valid authority.
- New sessions, worktrees, packet names and resource amendments do not erase history.
- Stale tests, counterfeit success flags and mismatched reviews cannot produce accepted status.
- The ledger stubs fail their behavioral requirement despite a supplied green test summary.
- Invalid admission blocks only dependent work, with a concrete next eligible action when one exists.
- Actual-host coverage is reported truthfully, and hooks never create an autonomous retry loop.
- All original sprint and financial/proof requirements remain visible and unchanged.

The implementation follows Grok high authorship, root verification and fresh independent GPT-6 review.
Self-review of this proposed plan is not implementation acceptance.
