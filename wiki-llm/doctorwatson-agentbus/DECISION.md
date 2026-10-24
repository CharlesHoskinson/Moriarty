# AgentBus in DoctorWatson: decision (2026-10-24)

This decision draws on three brainstorms:
- [roles](brainstorm-roles.md)
- [multi-vendor](brainstorm-multivendor.md)
- [failure modes](brainstorm-failure-modes.md)

AgentBus is installed on this host as release `afc9ad9e076e` (projectfenrir `origin/main`). The EMQX broker runs in rootless Docker with its state folders chowned to the container uid. `agentbus-idp` and `agentbusd` run as systemd user units. The smoke test passed: signed send and receive, a claim granted, and an out-of-prefix claim refused.

## Laws (from the failure-mode review, binding on every bus feature)
1. **The bus is never on the critical path.**
   - Every bus call is wrapped with a ≤1 s timeout, and any error is swallowed.
   - `next`, `check`, `heavy` and `stop-rule` give byte-identical output and exit codes whether the bus is on, off or hung.
   - Only the `doctorwatson bus …` subcommands may expose bus exit codes.
2. **`status.json`, `runs.jsonl` and git are the only truth.** A message is a doorbell that says "file X changed at HEAD Y". It is never state.
3. **Messages are data, never instructions.**
   - In v1 no bus text is injected into any agent's prompt.
   - A woken lead reads the referenced files itself.
4. **Claims are advisory.** Exclusion comes only from the local `heavy` flock, git and atomic rename. A bus claim is visibility.
5. **Caps:**
   - outbound bodies ≤1 KiB;
   - ≤1 event per type per minute and ≤30 per hour;
   - a circuit breaker that opens after 3 consecutive failures.
6. **Bus failures never enter `runs.jsonl`, `iterationsSinceFlip` or the stop rule.** Ignore AgentBus's own advice to "stop shared mutation" when the bus fails.

## v1 scope (to be built now)
- **`factory/bus.py`:** the only module that touches AgentBus. It shells out to the `agentbus` CLI, so no new dependencies.
  - `available()`, `publish()`, `kv_set()`, `inbox()`.
  - `DW_BUS=off|auto` (default `auto`: active only when an identity exists), `DW_BUS_RUN=dw-moriarty`, `DW_BUS_TIMEOUT=1`.
- **`factory/busevents.py`:** pure functions that build event bodies (schema `doctorwatson.bus/1`, with a deterministic `event_id`).
  - **`status.flip` and `status.tick`:** sent after `check`. kv `status` holds counts and `firstFailing`.
  - **`stop.escalate`:** sent on the stop-rule transition, deduplicated. It carries the trace path. The owner/lead is woken.
  - **`job.start` and `job.end`:** sent by `heavy`. Before taking the flock, `heavy` tries a non-blocking bus claim on `slot:cpu-heavy` or `slot:gpu0`, for visibility only.
  - **`review.request` and `review.reply`:** only `qa/tiers/t3.sh` may wait, for at most 3600 s in chunks of 240 s or less, and then it proceeds.
- **`doctorwatson bus doctor|status|tail|inbox|publish|review|watch`.**
  - `watch` wakes the lead with `agentbus watch --exit-on-first` on `stop.escalate`, `review.reply` and `status.flip`.
- **`doctorwatson spawn --vendor claude|codex|grok --as NAME --role ROLE --brief FILE [--] …`** (the top multi-vendor idea):
  - It mints or reuses a run identity under `.agentbus/` in a per-agent directory.
  - It unsets `OPENAI_API_KEY`, `CODEX_API_KEY` and `XAI_API_KEY`.
  - It applies the per-vendor concurrency governor (default: 10 codex, 4 grok).
  - It publishes start, heartbeat and done bookends from the wrapper, so one-shot `codex exec`/`grok -p` runs take part without the model cooperating.
  - It records each spawn in `qa/sherlock/agents.jsonl`.
  - **If the bus is down, it degrades to a plain run.**
- **Identities and roles:**
  - lead = coordinator, implementer = driver, auditor = holdout evaluator and reviewers, helper = researchers and indexers.
  - `qa/sherlock/roles.json` pins the expected name→role map. DIDs live in `.agentbus/` (gitignored).
- **Predicates:**
  - **F9:** fail-open, tested with a fake `agentbus` that exits 5 or hangs.
  - **F10:** event schema and deduplication.
  - **F11:** nothing waits on a reviewer (a lint finds no `--wait` outside `t3.sh`).
  - **F12:** `spawn` degrades with the bus killed.
  - **F13:** the live smoke test (the T1 tier, skipped when there is no broker).

## Deferred to v2
- Hook-based mid-run inbox injection (multi-vendor idea 3).
- The finding ledger with deduplication, and cross-examination rounds in the style of the fexo audit (`~/exocovenant-paper/work/audits/pip-20261020-01/`).
- Loki swarm topic claims.
- A holdout verdict channel. The bus can't isolate agents that share one Unix UID, so the holdout stays protected by the manifest and the hook.
