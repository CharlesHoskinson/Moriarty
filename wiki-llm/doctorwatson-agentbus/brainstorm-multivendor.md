# AgentBus inside DoctorWatson: multi-vendor orchestration brainstorm

Date: 2026-10-24. Angle: Claude Code as coordinator, Codex GPT-6.1 Sol and Grok 4.7 as headless workers. This is a brainstorm; nothing here has been run. Items marked VERIFY need a smoke test before anyone relies on them.

## What I read, and what constrains the design

- **AgentBus:** MQTT 5, signed `did:key` envelopes, SQLite inbox per agent, run-local advisory claims (`file:`, `crate:`, `slot:`), durable log. Body at most 16 KiB, subject 120 characters, 32 refs. Kinds: `msg ask reply claim renew release grant deny status event`.
- **Entry points:** `agentbus-codex` validates an identity and injects `PostToolUse` and `UserPromptSubmit` hooks via `-c hooks.*=`. It also passes `--dangerously-bypass-hook-trust`, a different flag from the banned sandbox bypass but easy to confuse, so the owner should see it once. `agentbus-hook` prints at most 10 messages / 1,500 characters and nothing when the inbox is empty, so an idle bus costs zero tokens. `fenrir-spawn new --existing-worktree` mints a binding without a branch.
- **Waking the lead:** `agentbus watch --exit-on-first` plus `hook/lead-watch.hooks.json` (`asyncRewake`). Its regex already knows `SOL-DONE`, `READY`, `IDLE`, `FREEZE` and every ask; it needs `FENRIR_LEAD_RUN` and `FENRIR_RUN_DIR`.
- **Authority:** bus messages are advisory and untrusted and cannot complete, adopt or trigger anything. A broker restart can lose queued traffic. DIDs attribute a key, not a vendor, and all agents share one UID.
- **Factory:** `runs.jsonl` and `status.json` are written only by `heavy` and `check`. The bus must never be a prerequisite for a predicate. Subscriptions only; the three API-key variables stay unset.
- **Fexo audit location:** `formal/exocovenant/fexo/audit/` does not exist in `projectfenrir-main`. The matching run is `/home/charles/exocovenant-paper/work/audits/pip-20261020-01/`: 20 role-matched reviews (Astra, AGY Gemini, Muse, Grok, five each), then 3 consolidators, moved entirely by files and JSON ledgers. Patterns to steal: reviewers finish independently before seeing peers; requested and served model are recorded separately; parent operators extract findings from raw captures; 241 raw records became 43 issue targets plus 27 counterevidence atoms in one ledger; per-lane reading coverage; four failed lanes were stopped, not papered over.
- **Loki today:** 10 Sol runs write `wiki-llm/loki/nodes/loki-NN-*.md`. Node 01 (prover profile) feeds 02, 03, 06 and 08, and cannot tell them its numbers are final.

## Principles

1. **File is truth, bus is doorbell.** Every worker writes its result to a file in its out directory. A bus message only says "look at file X (sha256 Y)". The collector can rebuild everything from files if the bus was down the whole time.
2. **The wrapper speaks for one-shots.** A headless run is a single burst and the model may never run `agentbus send`. So the `spawn` process (outside the model's sandbox) posts start, heartbeat and done under the worker's own identity. Model-authored messages are a bonus.
3. **Fail open, always.** Every bus call goes through one helper with a 3 s timeout that never raises. No bus means a log line `bus: off` in the manifest and the run proceeds unchanged.
4. **Advisory only.** No bus message can start a run, adopt a finding or flip a predicate. Spawning is always an explicit coordinator command.

## Cross-cutting: how a one-shot run takes part

| Phase | Codex Sol (`codex exec`) | Grok (`grok -p`) | Claude Sonnet (in-process Agent) |
|---|---|---|---|
| Before | Coordinator DMs the identity; the digest also goes into the prompt file (hooks are capped at 1,500 characters) | same | Digest in the Agent prompt |
| Start | `UserPromptSubmit` hook delivers the DM backlog once | Only `PostToolUse` can inject; the docs say an allowing `UserPromptSubmit` hook's stdout is discarded | No hook for it |
| Mid-run | `PostToolUse` hook via `agentbus-codex`-style `-c hooks.*` (VERIFY it fires under `exec`) | Global `~/.grok/hooks/agentbus.json` is trusted without a prompt and is a no-op when `AGENTBUS_IDENTITY_DIR` is unset (VERIFY the `additionalContext` reply shape) | Hook skips subagents by design. The brief gives the worker `AGENTBUS_IDENTITY_DIR=… agentbus …` prefixes, or the lead relays |
| End | Wrapper posts done, with exit code, wall time, served model and output sha256 | same | The lead posts for it |

Sandbox caveat (VERIFY): `-s read-only` or `--sandbox read-only` may block the model's own `agentbus send` (loopback network, SQLite write). The hooks run in the harness and probably work. Idea 4 covers the gap.

Quota, shared by all ideas: bus traffic costs no model tokens unless the hook injects (at most about 400 tokens per injection, and only when there is mail). Real costs are concurrency and context. Idea 9 is the governor.

Failure matrix, shared by all ideas: bus down at spawn means no identity, no hooks, a plain run. Bus drops mid-run means silent hooks and a silently failed final post. A message lost on broker restart is still found by the collector as a file. A duplicate delivery is harmless because `event_id` is a content hash.

## Ideas

### 1. `doctorwatson spawn`: one command, one vendor agent, one identity

- **Mechanism:** new subcommand in `factory/spawn.py`.
  - Usage: `doctorwatson spawn --vendor codex|grok --campaign loki --id sol-03 --brief PATH --out DIR [--bus auto|off|required] [--isolated] [--effort E]`.
  - It builds the exact CLI line from the ai-cli-refs README. Codex: `-m gpt-6.1-sol -c model_reasoning_effort='"xhigh"' -s workspace-write --ephemeral -C <out> --json -o last.txt - < prompt.md`. Grok: `-m grok-4.7 --effort xhigh --output-format streaming-json --prompt-file prompt.md --cwd <out> --no-auto-update`.
  - It scrubs `OPENAI_API_KEY`, `CODEX_API_KEY` and `XAI_API_KEY` from the child environment. It always redirects stdin from `/dev/null` for Codex.
- **Flow:**
  1. Check bus health (`agentbus doctor`, 3 s).
  2. If healthy, `fenrir-spawn new --run <campaign> --as <id> --role helper --existing-worktree <repo> --ttl 24h`.
  3. Render `prompt.md` as the brief, then `agentbus brief-snippet --role helper`, then the swarm snippet.
  4. Export `AGENTBUS_IDENTITY_DIR`, `AGENTBUS_RUN` and `AGENTBUS_AGENT`.
  5. Exec the vendor CLI through the codex hook wrapper or the grok hook environment.
  6. Run the sidecar (idea 2).
  7. Post done, then `fenrir-spawn reap` and write the manifest.
- **Identity:** `<campaign>/<vendor>-<nn>`, for example `loki/sol-03`, role `helper`. The manifest binds the DID to `{vendor, model_requested, model_served, effort, session_id}`. Served model is parsed from Codex stderr (`model: gpt-6.1-sol`) and from Grok's JSON usage (`grok-4.7-build`). This gives the vendor attribution a DID cannot, as in the fexo audit.
- **Cost:** none beyond one key mint per worker. It takes a quota slot (idea 9) before launching.
- **Fail open:** `--bus auto` (default) degrades to a plain run. `--bus required` exits 9 when the bus is down, for rare cases only. `--bus off` is deterministic and used by tests.

### 2. Wrapper-published bookends and an event-stream tap

- **Mechanism:** the `spawn` parent reads the child's `--json` (Codex) or `streaming-json` (Grok) stdout, so the child never needs to cooperate.
- **Flow:**
  - Publishes `status` `START <id>` to `#<campaign>` at launch.
  - Publishes `HEARTBEAT` every 5 minutes with turn count and last tool name, and only when the stream changed.
  - Publishes `SOL-DONE` (Codex) or `GROK-DONE` (Grok) at exit, with a body of exit code, wall seconds, served model and the sha256 of `last.txt` and of every file in `--out`.
  - Publishes `BLOCKED` if the stream shows an auth or rate-limit error.
- **Identity:** the worker's own, since the wrapper holds the key under the same UID.
- **Why:** liveness of all 10 Sol runs without reading files; a hung run is distinguishable from a finished one.
- **Cost:** a handful of small envelopes. **Fail open:** the tap swallows publish errors. Child exit code is propagated untouched.
- **Matching:** `SOL-DONE` is already in the lead's wake regex.

### 3. Mid-run inbox injection by hook

- **Mechanism:** reuse `agentbus-codex` as is for Sol; ship `~/.grok/hooks/agentbus.json` (owner action) for Grok.
- **Flow:** a sibling or the coordinator sends `msg` to `loki/sol-03`. At the next tool call the worker sees `- [msg] from loki/sol-01 subj "PROFILE FROZEN" …`, flagged as untrusted data.
- **Use it for:** cheap steering only (numbers frozen, topic taken, stop). Tasks stay in the prompt.
- **Cost:** zero tokens when idle; 3 s hook timeout. **Fail open:** the hook exits 0 silently on any error. A stretch with no tool calls sees nothing, which is acceptable.

### 4. File outbox for sandboxed workers

- **Mechanism:** the brief tells the worker to drop `outbox/<n>.json` (subject plus body) instead of calling `agentbus send`. The sidecar publishes each file under the worker's identity, applies the existing lint, and renames it `.sent` or `.refused`.
- **Why:** it works when the sandbox blocks loopback network or the SQLite write, and the worker never touches the bus.
- **Fail open:** if the bus is down the files stay in `outbox/` and the collector reads them as findings anyway.

### 5. Research swarm with topic claims and sibling broadcast (Loki)

- **Mechanism:** a campaign is one bus run (claims are run-local). Each worker holds a standing `slot:<campaign>/<topic>` claim for its topic, auto-renewed by its heartbeats (a 2 h lease renews on any envelope). The coordinator creates channel `#loki`.
- **Flow:**
  1. Node 01 finishes and posts `FINDING loki-01 profile-frozen` with a ref to its file and hash.
  2. Hooks inject it into the still-running 02, 03, 06 and 08, so they cite fresh numbers.
  3. A worker about to start a sub-topic tries `agentbus claim slot:loki/gpu-ntt-radix`. A queued reply means a sibling has it, so it takes another angle.
- **Identity:** one per worker, so claim holders are visible.
- **Fail open:** claims are advisory. A refused or unreachable claim means "proceed", and dedup then happens in the ledger (idea 6).

### 6. Finding schema and deduplicated ledger

- **Mechanism:** a new schema `doctorwatson.agentbus.finding/1` in `factory/swarm.py`, separate from the adapter's fixed advisory vocabulary. Fields:
  - `event_id`: sha256 of the canonical body without that field, the same trick as the adapter.
  - `campaign`, `topic`, `claim` (at most 300 characters), `evidence` (path, bytes, sha256), `kind` (proved, witnessed, insufficient, open-question, editorial, as in the fexo brief) and `dedup_key` (normalized topic plus claim tokens).
- **Flow:**
  - Workers write `findings.jsonl` in their out directory and announce through the wrapper or outbox.
  - `doctorwatson swarm collect <campaign>` reads the files, merges any bus announcements (`agentbus log --since`), groups by `dedup_key` then `event_id`, and writes `wiki-llm/<campaign>/LEDGER.jsonl`.
  - Each row has `sources: [worker ids]` and `agreement: n/N vendors`.
- **Why:** this reproduces the fexo "241 raw to 43 targets" step mechanically, and agreement across vendors becomes a visible signal.
- **Cost:** none (local). **Fail open:** the collector needs no bus at all.

### 7. Cross-examination rounds (the fexo-audit pattern)

- **Mechanism:** `doctorwatson swarm round`, a script around `spawn`.
- **Flow:**
  1. **Round 1, isolated:** spawn with `--isolated`, which installs no hooks and joins no channel. This preserves the independence rule from the fexo brief.
  2. **Digest:** the collector builds the deduped ledger.
  3. **Round 2, cross-vendor:** each finding goes to a reviewer of a different vendor than its author (Sol reviews Grok findings and the reverse). The verdict is accept, refute, narrow or insufficient, with a counterevidence ref.
  4. **Round 3:** the Claude coordinator adjudicates disagreements only, as the fexo consolidators did.
- **Identity:** `fexo/r2-sol-04-reviews-grok-02`, so each verdict binds to a pair of identities and served models.
- **Bus use:** `ask` messages carry the review request (body is a ref plus hash, never the finding text). Replies are `reply` envelopes that point to a verdict file.
- **Cost:** batch several findings per reviewer run to stay inside quota.
- **Fail open:** the round runs from files alone. The bus only adds liveness.

### 8. Dormant authors: resume on ask

- **Mechanism:** the manifest keeps `session_id` (`codex exec resume`, `grok -r`; `fenrir-spawn bind --session-id` stores one). `doctorwatson swarm answer` relaunches a finished author's session when an `ask` addressed to it arrives.
- **Flow:** a round-2 reviewer refutes a claim, the author session is resumed with the refutation, and it writes a rebuttal or concession file.
- **Cost:** a resume re-bills the prior context against the window, so limit it to disputed P0/P1 items, one resume per finding. **Fail open:** on failure the ledger marks the finding "unanswered" and the coordinator decides.

### 9. Vendor quota governor

- **Mechanism:** flock-based slots in `~/.cache/doctorwatson/slots/<vendor>.<n>`, modeled on `heavy.lock`. Defaults come from `DW_SLOTS_CODEX` and `DW_SLOTS_GROK`. I do not know the plan limits, so the defaults are conservative and the owner tunes them. Today's 10 concurrent Sol runs is the known-good ceiling.
- **Flow:** `spawn` waits for a slot (queued state posted as `status`). If the stream shows a rate-limit error it posts `BLOCKED`, backs off with jitter and retries once. A second failure ends with exit 10 and the lead decides.
- **Accounting:** a new append-only `qa/sherlock/agents.jsonl` (spawn is the only writer, since CONTRACT reserves `runs.jsonl` for `heavy` and `check`): `{run, campaign, id, vendor, model_requested, model_served, effort, exit, wallSeconds, bus}`. Codex quota is probably not exposed headlessly (VERIFY), so wall time is the proxy.
- **Fail open:** with no slot directory or no flock, the governor logs a warning and runs.

### 10. Waking the Claude Code coordinator

- **Mechanism:** install `hook/lead-watch.hooks.json` in the coordinator's Claude settings (owner action) with `FENRIR_LEAD_RUN=<campaign>`. Extend the regex with `SWARM-DONE|BLOCKED|ROUND-READY|PREDICATE-FLIP`.
- **Flow:** the wrapper posts `SOL-DONE`, the watcher appends to `<run-dir>/lead/INBOX.jsonl` and exits 2, and the session wakes with the one-line event. For simple fan-outs, `run_in_background` exit notification already works; the bus adds mid-run wake (BLOCKED, ask), aggregation (`SWARM-DONE` when the last of N posts) and a wake while the coordinator is idle. `doctorwatson swarm status <campaign>` prints a table from `agentbus who`, the log and manifests.
- **Cost:** each wake spends a coordinator turn, so wake on `SWARM-DONE` and `BLOCKED`, not per-worker chatter. **Fail open:** the watcher exits 75 or silently; polling `swarm status` always works.

### 11. Retro-bridge for file-only runs (the current Loki 10)

- **Mechanism:** `doctorwatson swarm bridge loki` polls `wiki-llm/loki/nodes/*.md` mtimes and frontmatter and posts `BRIDGED` status events as a separate identity `loki/bridge`, so nobody mistakes it for the worker.
- **Value:** no change to the running agents. Claude sees progress, and the collector can seed `LEDGER.jsonl` from front matter. One-way only: the 10 running Sol runs cannot gain hooks after launch.

### 12. Predicate-flip and heavy-lock announcements

- **Mechanism:** after `check` and `heavy` have written their canonical files, they call `busio.publish_status()`. A flip posts `PREDICATE-FLIP A3 pass` to `#gate`; heavy start and end post the lock holder.
- **Value:** workers stop re-running a passing predicate and avoid launching heavy work while the lock is held (the flock stays authoritative). **Fail open:** a try/except side effect after the canonical write, so no exit code can change.

### 13. T3 cold review as a bus `ask` with a one-hour TTL

- **Mechanism:** `spawn --role reviewer` runs the existing `codex exec -s read-only` review as `review/sol-01`; the coordinator posts an `ask --ttl 3600`. The verdict file is announced when ready and recorded as an input. If nothing arrives in an hour, T3 proceeds exactly as AGENTS.md says. The review never gates anything.

## Ranked top five

1. **Idea 1, `doctorwatson spawn`.** Everything else hangs off one command that adds identity, brief, slot and manifest with fail-open degrade.
2. **Idea 2, wrapper-published bookends.** It is the only part that works for a one-shot even when the model cooperates with nothing. It gives liveness for 10 Sol runs immediately.
3. **Idea 6, finding ledger with dedup (file is truth).** It turns N raw outputs into one reviewable ledger, and it needs no bus to be correct.
4. **Idea 10, coordinator wake.** The owner feels this directly: Claude Code learns that the swarm finished or blocked without polling.
5. **Idea 9, quota governor.** It prevents subscription lockouts as swarms grow, and it keeps the no-API-key rule enforced in one place.

Next: idea 7 (cross-examination) and idea 5 (topic claims), once v1 has run one real campaign.

## Minimal v1

**Scope:** ideas 1, 2, 6, 9 (single slot pool) and the wake regex from 10. No hooks installed on Grok, no resume, no bridge. Mid-run injection is on for Codex only, behind a flag, until the smoke test passes.

**Files (all new unless noted):**
- `models/doctorwatson/factory/busio.py`: `run(argv, timeout=3) -> (ok, out)`, never raises. Calls `agentbus doctor`, `send` and `log`.
- `models/doctorwatson/factory/spawn.py`: argument parsing, command builder, environment scrub, slot flock, sidecar thread (stream tap, outbox poller), manifest writer, `--dry-run`.
- `models/doctorwatson/factory/swarm.py`: finding schema, `event_id`, dedup, `collect`, `status`.
- `models/doctorwatson/factory/briefs/swarm-worker.md`: the brief template, with the `agentbus brief-snippet --role helper` text and the outbox convention.
- `models/doctorwatson/factory/tests/test_spawn.py` and `test_swarm.py`: stdlib unittest, with fake vendor binaries (`DW_CODEX_BIN`, `DW_GROK_BIN`) and `DW_BUS=off`.
- `scripts/qa/check-f9.sh` through `check-f12.sh`.
- `qa/sherlock/agents.jsonl` and the CONTRACT.md additions: the `spawn` and `swarm` rows, a new exit code 9 (bus required but down) and 10 (quota exhausted), and the `agents.jsonl` line schema. These need a coordinator edit.
- Owner actions outside the repo: lead-watch hook entry in Claude settings, with `FENRIR_LEAD_RUN`.

**Commands:**
```
scripts/doctorwatson spawn --vendor codex --campaign loki --id sol-11 --brief wiki-llm/loki/briefs/11.md --out wiki-llm/loki/nodes --dry-run
scripts/doctorwatson spawn --vendor grok  --campaign loki --id grok-01 --brief … --out … --bus auto
scripts/doctorwatson swarm collect loki      # writes wiki-llm/loki/LEDGER.jsonl
scripts/doctorwatson swarm status  loki
```

**Smoke test S0 (VERIFY before enabling hooks):** one throwaway identity in a throwaway run, then check four things:
- `codex exec` with the `-c hooks.*` flags delivers a DM at `UserPromptSubmit` and at `PostToolUse`;
- Grok's `PostToolUse` accepts `additionalContext`;
- `agentbus send` succeeds from inside a read-only sandbox;
- both CLIs print the served model.

**Predicates** (append to `qa/sherlock/predicates.yaml`; F9 to F12 are T0 and run with no bus and no network, and none is a dependency of any A, B, C or U predicate):

```yaml
  - id: F9
    set: F
    title: "doctorwatson spawn --dry-run prints the vendor command line for codex and grok with no API-key variables in the child environment"
    tier: T0
    check: "scripts/qa/check-f9.sh"
    heavy: false
    depends: [F2]
    source: "wiki-llm/doctorwatson-agentbus/brainstorm-multivendor.md#idea-1"
    holdout: false
  - id: F10
    set: F
    title: "spawn fails open: with the bus disabled and with a missing agentbus binary, a stub vendor run exits with the stub's own exit code and the manifest records bus off"
    tier: T0
    check: "python3 -m unittest factory.tests.test_spawn.FailOpen"
    heavy: false
    depends: [F9]
    source: "wiki-llm/doctorwatson-agentbus/brainstorm-multivendor.md#cross-cutting"
    holdout: false
  - id: F11
    set: F
    title: "swarm collect rebuilds a deduplicated ledger from files alone: duplicate findings collapse to one row with both sources, and a corrupted line is reported, not fatal"
    tier: T0
    check: "python3 -m unittest factory.tests.test_swarm"
    heavy: false
    depends: [F2]
    source: "wiki-llm/doctorwatson-agentbus/brainstorm-multivendor.md#idea-6"
    holdout: false
  - id: F12
    set: F
    title: "Quota governor serializes at most DW_SLOTS_<VENDOR> concurrent spawns and appends one agents.jsonl line per run without touching runs.jsonl or status.json"
    tier: T0
    check: "scripts/qa/check-f12.sh"
    heavy: false
    depends: [F9]
    source: "wiki-llm/doctorwatson-agentbus/brainstorm-multivendor.md#idea-9"
    holdout: false
The live smoke S0 (script `scripts/qa/check-f13.sh`, tier T3, exits 3 when no bus is installed) stays out of predicates.yaml until the bus is live, so it never blocks the inner loop. F9 to F12 come after F8, so `sherlock-next` still names an A, B, C or U predicate first. The `F` count in `status.json` goes from `[0,8]` to `[0,12]`; the coordinator updates CONTRACT.md.

**Owner decisions:** approve `--dangerously-bypass-hook-trust` for Codex and a global `~/.grok/hooks/agentbus.json`; default slot counts per vendor; whether `spawn` may provision identities itself (README treats provisioning as a lead action).
