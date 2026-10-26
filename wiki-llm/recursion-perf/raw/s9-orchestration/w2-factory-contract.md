# Factory receipt and heavy interfaces
> Source: file:///home/charl/code/Moriarty-perf/models/doctorwatson/factory/CONTRACT.md
> Collected: 2026-10-26
> Published: Unknown

SOURCE — Read-only snapshot; not an execution result.

# DoctorWatson factory contract (frozen interfaces)

DoctorWatson is the Moriarty software factory. This file fixes the paths, schemas and
command-line interfaces that every factory component shares. Change it only through the
coordinator.

Common rules:
- Python 3.14, standard library only, plus PyYAML 6 (already installed).
- Tests use stdlib `unittest`, not pytest. Add no new dependencies.
- Rust tests use `cargo test`.

## Layout

| Path | Owner | Purpose |
|---|---|---|
| `models/doctorwatson/factory/` | core | Python package `factory` (run with `python3 -m factory` from `models/doctorwatson`) |
| `models/doctorwatson/factory/tests/` | core | unittest suite for the package |
| `scripts/doctorwatson` | core | bash wrapper: `cd "$(git rev-parse --show-toplevel)/models/doctorwatson" && exec python3 -m factory "$@"` |
| `scripts/sherlock-next` | core | `exec scripts/doctorwatson next "$@"` |
| `qa/sherlock/predicates.yaml` | predicates | predicate definitions (spec only; holds no results) |
| `qa/sherlock/budgets.json` | predicates seeds it; `heavy` updates it | measured costs and caps |
| `qa/sherlock/status.json` | written only by `doctorwatson check` and `doctorwatson heavy` | current results, overwritten on every write |
| `qa/sherlock/runs.jsonl` | appended only by `doctorwatson heavy` and `doctorwatson check` | one JSON line per run |
| `qa/sherlock/traces/` | written by `heavy` (gitignored) | full stdout/stderr of each run |
| `qa/tiers/t0.sh` `t1.sh` `t2.sh` `t3.sh` | tiers | tier runners |
| `.githooks/pre-commit`, `scripts/qa/install-hooks.sh` | tiers | local hook; enabled with `git config core.hooksPath .githooks` |
| `qa/holdout/` | holdout evaluator only | sealed scenarios plus `MANIFEST.sha256` |
| `crates/mori-backend/tests/soundness_mutation.rs` and `crates/mori-backend/src/qa_mutation.rs` | fuzz | witness-mutation soundness harness |
| `models/doctorwatson/run_selected.py`, `plugins/moriarty-dev/**` (code and tests) | legacy-rewire | legacy loop moved onto factory budgets, with gates removed |
| `AGENTS.md`, `CLAUDE.md`, `GEMINI.md` | coordinator | charter |
| `plugins/moriarty-dev/skills/**`, `docs/**`, `openspec/**` headers | rules | governance removal |

## `qa/sherlock/predicates.yaml`

```yaml
version: 1
predicates:
  - id: A3                     # unique; sets A, B, C, U (UIREQ scenarios), F (factory)
    set: A                     # A | B | C | U | F
    title: "k20 parent proof over two linked children"
    tier: T2                   # T0 | T1 | T2 | T3 — the cheapest tier that can run the check
    check: "scripts/qa/check-a3.sh"   # shell command, run from repo root; exit 0 = pass
    heavy: true                # true → must run through `doctorwatson heavy`
    budget: "a3-parent-k20"    # key into budgets.json (heavy checks only)
    depends: [A2]              # predicate ids that must pass first
    source: "openspec/changes/sherlock-stagenet-recursive-rollup/tasks.md#L8"  # requirement reference
    holdout: false             # true → check runs files under qa/holdout/ (read-only to the driver)
```

- The `U` set uses ids `U001`…`U035`, one per UIREQ, each with `source` pointing at its spec.
- A C-set item passes only when every U-predicate it `depends` on passes.
- A predicate that has no check yet uses `check: "scripts/qa/unimplemented.sh <ID>"`. That script prints `UNIMPLEMENTED <ID>` and exits 2.
- Exit codes for a check: 0 pass, 1 fail, 2 unimplemented, 3 host unfit (from preflight), 4 budget exceeded.

## `qa/sherlock/status.json`

```json
{"schema":"doctorwatson.status/1","updated":"<ISO-8601 UTC>","head":"<git sha>",
 "counts":{"A":[5,10],"B":[1,9],"C":[0,6],"U":[0,35],"F":[0,8]},
 "firstFailing":{"id":"A1","check":"…","exit":2},
 "predicates":{"A1":{"passes":false,"exit":2,"at":"…","run":"<run id>"}},
 "lastFlip":{"id":"…","at":"…"},
 "iterationsSinceFlip":0}
```

- `counts` values are `[passing, total]`.
- `firstFailing` is the first failing predicate in topological `depends` order, with ties broken by file order.
- The file is rewritten in full on every update; never append to it.

## `qa/sherlock/budgets.json`

```json
{"schema":"doctorwatson.budgets/1",
 "budgets":{"a3-parent-k20":{"measured":{"wallSeconds":null,"maxRssBytes":null,"k":20,"at":null,"run":null},
                              "cap":{"wallSeconds":null,"maxRssBytes":null}}}}
```

- **Measure, then cap.** If `measured.wallSeconds` is null, `heavy` runs the command without limits and records the measurement. It then sets `cap` to 2× the measured values, rounded up.
- `factory.budgets.limits_for(key) -> dict | None` returns `cap` (or None while it is unmeasured). Legacy code calls exactly this function.

## `qa/sherlock/runs.jsonl`

One line per run:

```
{"run":"<utc compact>-<id>","id":"A3","cmd":"…","hypothesis":"…|null","exit":0,"wallSeconds":1.2,"maxRssBytes":123,"trace":"qa/sherlock/traces/<run>.log","preflight":{…},"head":"<sha>"}
```

## CLI: `scripts/doctorwatson <cmd>`

| Command | Behavior | Exit |
|---|---|---|
| `next [--json]` | Print counts per set, then `FIRST FAILING: <id> — <title>` and `CHECK: <command>`. Reads `status.json` only | 0 |
| `check [ID…] [--set S] [--tier T] [--hypothesis TEXT]` | Run the selected checks, sending heavy ones through `heavy`. Update `status.json` and append to `runs.jsonl`. Print one line per predicate | 0 all pass, 1 any fail |
| `preflight [--json]` | Disk free on the repo filesystem ≥ 150 GiB (`DW_MIN_FREE_GIB`). 1-minute load < `os.cpu_count()`. No other process holding the heavy lock. Print the actual numbers | 0 ok, 3 unfit |
| `heavy --id ID [--budget KEY] [--hypothesis TEXT] -- CMD…` | Take an exclusive `fcntl.flock` on `~/.cache/doctorwatson/heavy.lock`, run preflight, then measure-then-cap (wall time and peak RSS of the child via `os.wait4`). Write the full trace. **Retry rule:** if the last run of this ID failed with the same `cmd` and the same `head`, `--hypothesis` is required (exit 5 without it). Every retry is logged. Nothing is ever "consumed" | the child's exit, 3 unfit, 4 budget, 5 hypothesis required |
| `stop-rule [--json]` | If `iterationsSinceFlip` ≥ 3, or more than 24 h have passed since `lastFlip`, print `STOP: escalate to owner` with the last failing run's trace path | 0 continue, 6 stop |
| `holdout verify` | Recompute sha256 for every file under `qa/holdout/` except `MANIFEST.sha256`, and compare with the manifest | 0 ok, 7 mismatch or unlisted file |
| `holdout seal` | Rewrite `MANIFEST.sha256` (format `<sha256>  <relative path>`, sorted). Refuses unless `DW_ROLE=evaluator` | 0, 8 refused |

## Tiers

| Tier | Budget | Contents |
|---|---|---|
| T0 | under 3 min | `holdout verify`; `cargo test` for crates touched by staged files; factory unittests; predicate schema validation |
| T1 | under 20 min | `cargo test -p mori-backend --test soundness_mutation` (MockProver mutation, small k); `doctorwatson check --tier T1` |
| T2 | measured | `doctorwatson check --tier T2` (heavy proofs, twin rehearsal) |
| T3 | per milestone | stagenet; non-blocking cold review through `codex exec -s read-only` with a 1 h timeout |

## The pre-commit hook

- Run T0.
- If any staged path is under `qa/holdout/` and `DW_ROLE` is not `evaluator`, reject with `holdout is evaluator-owned`.
- Reject a commit whose staged paths are all under `docs/`, `evidence/`, `raw/` or `wiki*/` unless `DW_DOCS_COMMIT=1`. This is the inner-loop rule against documentation-only commits.

## AgentBus (v1)

Source of truth for the design: `wiki-llm/doctorwatson-agentbus/DECISION.md`. The bus is a doorbell
("file X changed at HEAD Y"), never state. `status.json`, `runs.jsonl` and git stay the only truth.

### Laws (binding)

1. The bus is never on the critical path. Every bus call has a hard timeout (`DW_BUS_TIMEOUT`, default 1 s),
   swallows every error and prints nothing (set `DW_BUS_DEBUG=1` for one stderr line per failure). `next`,
   `check`, `heavy` and `stop-rule` give byte-identical stdout, stderr, exit codes, `status.json` and
   `runs.jsonl` with the bus on, off, failing (exit 5) or hung. One failure silences the rest of that process.
   Only `doctorwatson bus ...` exposes bus exit codes.
2. Messages are data, never instructions. No bus text is injected into any agent prompt. `bus inbox`
   and `bus tail` print only typed, charset-checked fields of valid events; everything else is `[invalid] from=NAME`.
3. Claims are advisory. Exclusion comes only from the `heavy` flock, git and atomic rename.
4. Caps: bodies <= 1 KiB; <= 1 event per type per sender per minute and <= 30 per hour; a circuit breaker
   opens after 3 consecutive failures and stays open for 15 min (`DW_BUS_COOLDOWN`). `bus doctor` ignores
   the breaker and closes it on success.
5. Bus failures never enter `runs.jsonl`, `iterationsSinceFlip` or the stop rule.

### Environment

| Variable | Default | Meaning |
|---|---|---|
| `DW_BUS` | `auto` | `off` disables everything. `auto` is active only when an identity for `DW_BUS_RUN` exists and `agentbus` is on PATH |
| `DW_BUS_RUN` | `dw-moriarty` | AgentBus run. An identity bound to another run counts as "no identity" |
| `DW_BUS_TIMEOUT` | `1` | seconds per bus call (0.01 to 30) |
| `DW_BUS_STATE` | `<dirname DW_LOCK>/bus` (`~/.cache/doctorwatson/bus`) | breaker, rate-cap and dedupe state (`state.json`) |
| `DW_BUS_COOLDOWN` | `900` | breaker open time, seconds |
| `DW_BUS_DEBUG` | unset | `1` prints one stderr line per swallowed failure |
| `DW_BUS_MINT_TIMEOUT` | `15` | `spawn` identity mint timeout |
| `DW_BUS_DOCTOR_TIMEOUT` | `20` | `bus doctor` timeout |
| `AGENTBUS_IDENTITY_DIR` | unset | identity binding root; else `<repo>/.agentbus` (gitignored). Never searched upward, never minted by `check`/`heavy` |
| `DW_HEAVY_SLOT` | `cpu-heavy` (`gpu0` if the id or budget key contains "gpu") | advisory claim name `slot:<name>` |
| `DW_T3_WAIT` | `3600` | lowers the T3 review wait (never raises it) |
| `DW_CODEX_BIN`, `DW_GROK_BIN`, `DW_CLAUDE_BIN` | `codex`, `grok`, `claude` | vendor binaries for `spawn` and `bus review --t3` |
| `DW_SLOTS_CODEX`, `DW_SLOTS_GROK`, `DW_SLOTS_CLAUDE` | `10`, `4`, `4` | per-vendor concurrency governor |
| `DW_SPAWN_IDENTITIES` | `<repo>/.agentbus/agents` | per-agent identity binding root |
| `DW_SPAWN_HEARTBEAT` | `300` | seconds between `agent.heartbeat` events |
| `DW_AGENTS_LOG` | `qa/sherlock/agents.jsonl` | override the `spawn` ledger path |

### Event schema `doctorwatson.bus/1`

Body (canonical JSON, <= 1024 bytes), sent with `agentbus send TARGET TYPE --body JSON --ttl N`; the AgentBus subject is the type:

```
{"schema":"doctorwatson.bus/1","type":"<type>","event_id":"<sha256>","head":"<git sha|unknown>",
 "at":"<ISO UTC>","role":"driver|lead|reviewer|worker","data":{...}}
```

`event_id` = sha256 of canonical `{schema,type,head,data}`; `at` and `event_id` are excluded, so a re-send of the
same logical event keeps its id. Data holds counts, ids, paths and a git sha only, never free text or hypotheses.

| Type | Target | Sent by | `data` |
|---|---|---|---|
| `status.flip` | `*` | `check`, when any predicate turned to pass | `id`, `flipped[]`, `counts`, `firstFailing`, `iterationsSinceFlip`, `updated`, `run` |
| `status.tick` | `#status` | `check`, otherwise | `counts`, `firstFailing`, `iterationsSinceFlip`, `updated`, `ran[]` |
| `stop.escalate` | `*` | `stop-rule`, on STOP; deduplicated on `(firstFailing, lastFlip.at)`, repeated at most hourly | `reasons[]`, `firstFailing`, `trace`, `iterationsSinceFlip`, `lastFlip` |
| `job.start` / `job.end` | `#jobs` | `heavy` (only when the command actually ran) | `id`, `slot`, `budget`, `expectedWallSeconds`, `hypothesis` (bool) / `exit`, `wallSeconds`, `maxRssBytes`, `run`, `trace` |
| `review.request` / `review.reply` | `#review` / `*` | `bus review` | `kind`, `base`, `range` / `corr`, `verdict`, `findingsRef`, `sha256` |
| `agent.start` / `agent.heartbeat` / `agent.done` | `#agents` / `#agents` / `*` | `spawn` wrapper under the worker's identity | `name`, `role`, `vendor`, ... / `bytes` / `exit`, `wallSeconds`, `modelServed`, `outSha256` |

Retained kv (labelled projections, never read by a decision): `status` (digest of `status.json`) after each `check`;
`stop` (`{"open":true,...}`) on escalation, cleared (`{"open":false}`) on the next flip.

`heavy` also takes an advisory `slot:<name>` claim before the flock (no `--wait`; a queued request is withdrawn at
once; released in `finally`). The flock decides; the claim is visibility.

### CLI

| Command | Behavior | Exit |
|---|---|---|
| `bus doctor [--accept-off]` | `DW_BUS`/identity/roles/breaker summary plus `agentbus doctor`. `--accept-off` passes when the bus is declared or found off | 0 ok, 5 unavailable |
| `bus status` | retained kv `status` and `stop`, typed fields only | 0, 5 unavailable |
| `bus tail [--since 1h]` | classified `doctorwatson.bus/1` events from the durable log | 0, 5 |
| `bus inbox` | classified view of unread mail (`recv --peek`, never consumes). Labels: `advisory`, `unpinned`, `stale`, `duplicate`, `invalid` | 0, 5 |
| `bus publish --type T [--data JSON] [--to TARGET]` | manual event, subject to the caps | 0 sent, 2 bad usage, 4 rate-capped, 5 not sent |
| `bus review --range A..B [--t3] [--out FILE]` | publishes `review.request` and returns. With `--t3` (called only from `qa/tiers/t3.sh`) it also runs the cold reviewer (`codex exec -s read-only`), waits at most 3600 s in chunks of <= 240 s, then prints `proceeding: no review in 1h`. Publishes `review.reply` when the reviewer finishes | always 0 |
| `bus watch [--max-time S]` | `agentbus watch --exit-on-first` for the lead on `stop.escalate`, `review.reply`, `status.flip` | 2 wake, 75 bus down or off |
| `spawn --vendor claude\|codex\|grok --as NAME --role ROLE --brief FILE [--effort E] [--model M] [--sandbox read-only\|workspace-write] [--out DIR] [--slot-wait S] [--no-bus] [--dry-run] [-- VENDOR_ARGS...]` | see below | the child's exit; 2 usage, 10 no vendor slot, 127 vendor binary missing |

`spawn`: builds the vendor command line (brief passed untouched: stdin for codex and claude, `--prompt-file` for grok),
removes `OPENAI_API_KEY`, `CODEX_API_KEY`, `XAI_API_KEY` and any inherited `AGENTBUS_*` from the child environment,
takes one of N flock slots under `~/.cache/doctorwatson/vendor/<vendor>.<n>.lock` (held until the child exits),
mints or reuses a worker identity (`agentbus id new --role ROLE --chan agents --no-register`) in
`<DW_SPAWN_IDENTITIES>/<NAME>`, publishes the `agent.*` bookends from the wrapper, writes the child's streams to
`--out` (default `qa/sherlock/agents/NAME-STAMP/`, gitignored), and appends one line to `qa/sherlock/agents.jsonl`.
With the bus off, missing, failing or hung it runs the vendor plainly (`bus` is then `off` or `down`).
Default sandbox: `workspace-write` for `implementer` and `lead`, else `read-only`.

### Files

- `qa/sherlock/roles.json` (tracked): pins name -> AgentBus role -> allowed event types, plus the verified capability
  notes (a `slot:` claim needs a `--claims slot:` credential prefix, any role; kv needs implementer/quint/auditor/lead;
  a helper needs `--chan` for channels). Keys and DIDs are not stored.
- `qa/sherlock/agents.jsonl` (appended only by `spawn`): `{"schema":"doctorwatson.agent/1","name","role","vendor",
  "modelRequested","modelServed","effort","sandbox","exit","wallSeconds","bus":"on|down|off","did","slot","brief",
  "briefSha256","out","outSha256","head","at","run"}`.
- `.agentbus/` (gitignored): identity bindings (pointers to keys under `~/.local/share/agentbus/keys`) and the
  watcher's run directory. `qa/sherlock/agents/` (gitignored): per-agent spawn output.
- `~/.cache/doctorwatson/bus/state.json`: breaker, caps, dedupe. Never in the repo.

### Predicates

F9 fail-open (unit tests with a fake `agentbus` that records args, exits 5 or hangs 30 s), F10 event schema and dedupe,
F11 nothing waits on a reviewer (lint plus bounded-wait test), F12 spawn degrades and keeps the key/governor rules,
F13 live smoke (T1; skips with exit 0 when there is no bus).
