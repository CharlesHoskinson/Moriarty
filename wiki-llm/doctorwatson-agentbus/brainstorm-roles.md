# AgentBus in the DoctorWatson factory: coordination and roles (brainstorm)

Date: 2026-10-24. Angle: factory coordination and roles. Status: brainstorm only; no code, identities, services or messages were touched.

## Premises

Facts this note relies on (read from the sources, not from a live bus):

- The factory loop is `scripts/sherlock-next`, then `doctorwatson check`, `heavy`, `stop-rule` and `holdout verify` (`models/doctorwatson/factory/CONTRACT.md`). `status.json` is rewritten in full on every update and is the only state the driver reads. `heavy` serializes on `fcntl.flock(~/.cache/doctorwatson/heavy.lock)` and runs preflight. `stoprule.evaluate()` returns `stop`, `reasons`, `trace`.
- AgentBus gives: topics `agentbus/v1/<run>/{to/<agent>, all, ch/<chan>, presence/<agent>, claims/*, kv/<key>}`; envelope kinds `msg|ask|reply|claim|...` with `corr`, `ttl_s`, `refs`; bodies <= 16 KiB, secret/email-linted. Claims are run-local, FIFO, leased (30 min default, 4 h max), cooperative, on free-form names like `slot:cargo`. `kv` is retained. `agentbus watch --exit-on-first` exits 2 to wake a session. Exit 5 = bus unavailable, 75 = `wait` found the broker down. Broker sessions are in memory, so a restart loses queued traffic.
- The old adapter (`models/doctorwatson/agentbus_adapter.py`) is advisory-only. Notices carry a stable `event_id`, are classified `stale|duplicate|invalid|advisory`, and can only say "inspect canonical state". `publish_notice(None, ...)` returns `disabled`. That fail-open posture is the template for everything below.

Design laws:

1. **Files are truth, the bus is a doorbell.** `status.json`, `runs.jsonl`, `qa/holdout/` and git stay authoritative; a message only gets someone to look sooner.
2. **Fail open.** Every call goes through one wrapper (`factory/bus.py`) with a hard timeout (default 1 s) that catches everything and returns `None`. Bus down, no identity, no binary or `DW_BUS=off` gives today's behavior and exit codes. Nothing in `check`, `heavy`, `next` or the hook blocks on the bus.
3. **Nothing waits on a reviewer.** The only timed wait is the T3 one-hour rule.
4. **The bus never gates a predicate.** F-set predicates check wiring and health, not message content.
5. **Messages are untrusted data.** Reuse the adapter's `classify_notice` pattern (schema, `event_id`, head context, `stale|duplicate|invalid`).

Shared envelope convention (the body JSON, inside AgentBus's envelope), `schema = "doctorwatson.bus/1"`:

```json
{"schema":"doctorwatson.bus/1","type":"<noun.verb>","event_id":"<sha256 of canonical body minus event_id>",
 "head":"<git sha>","at":"<ISO UTC>","role":"driver|evaluator|reviewer|indexer|swarm|lead|owner",
 "data":{...}}
```

`corr` carries the originating request id. `head` is how receivers decide `stale`. Run name: `dw-moriarty` (one run for the factory, so claims are visible to all roles; claims do not cross runs).

---

## Ideas

### 1. Predicate-flip and status broadcast (`status.flip`, `status.tick`)

- **What:** After `check` rewrites `status.json`, it publishes a one-line digest so the lead, reviewers, swarms and indexer learn of progress without polling. A flip is the high-value event; a tick (check ran, no flip) keeps `iterationsSinceFlip` visible.
- **Topics:** `all` for flips, `ch/status` for ticks, retained `kv/status` with the latest digest for late joiners.
- **Schema:** `{"type":"status.flip","data":{"id":"A6","from":"fail","to":"pass","counts":{"A":[6,10],"F":[9,12]},"firstFailing":{"id":"A7","exit":2},"iterationsSinceFlip":0,"run":"<run id>"}}`. Ticks carry `{id, exit, iterationsSinceFlip, hypothesis}`.
- **Emit/consume:** emitted at the end of `cli.cmd_check`, after `status.json` and `runs.jsonl` are written. Consumers: `doctorwatson bus tail`, the lead watcher, reviewers choosing a commit, the indexer.
- **Bus down:** `publish()` returns `None`; `check` prints nothing extra. `kv/status` goes stale, so consumers re-read `status.json` (the digest carries `head`).
- **Value/cost:** high; about 40 lines; enables ideas 5, 7, 8. Ticks go to a channel nobody subscribes to by default.

### 2. Heavy-job queue and slots over the flock (`job.*`)

- **What:** Today `heavy` takes one global flock and exits 3 if it is held. The bus adds visibility and polite ordering, not exclusion: agents see who holds the machine, the expected wall time (from `budgets.json`) and who is queued.
- **Resources:** `slot:heavy-cpu` (the flock's meaning), `slot:heavy-gpu0..N` (when the Loki GPU prover lands), optionally `slot:cargo`. Lease = `cap.wallSeconds` clipped to 4 h, renewed by `heavy`'s existing poll loop.
- **Messages:** `agentbus claim slot:heavy-cpu --wait 0`, then on `ch/jobs`: `job.start {id, budget, expectedWallSeconds, slot, holder, hypothesis}` and `job.end {exit, wallSeconds, maxRssBytes, trace}`.
- **Relation to the flock:** order is (a) best-effort claim, never waiting; (b) flock; (c) preflight; (d) run. A queued claim does not delay `heavy`: the flock decides. For GPU slots the flock becomes per-slot (`heavy-gpu0.lock`) and the claim picks which to try first. The flock always wins.
- **Emit/consume:** `heavy.main` emits; `preflight --json` can add the queue.
- **Bus down:** skip (a) and announcements; flock-only, as now. A dead claim holder's lease expires (<= 4 h); the kernel frees the flock on process death.
- **Value/cost:** medium-high once driver, evaluator and fuzz contend for the host. The trap is a second lock that disagrees with the flock.

### 3. Holdout evaluator isolation: verdict-only channel (`holdout.verdict`)

- **What:** The evaluator alone owns `qa/holdout/`; the driver sees only a bounded verdict. The bus carries it as a signed, narrow interface.
- **Topics:** evaluator publishes to `to/driver` and retained `kv/holdout`. Driver sends `holdout.request {head, predicates}`.
- **Schema:** `{"type":"holdout.verdict","data":{"head":"<sha>","pass":false,"failedBucket":"1-3","classes":["ordering"],"manifestOk":true}}`. `classes` is an allowlisted coarse vocabulary, never scenario names, inputs or expected values; counts are bucketed to blunt differential probing.
- **Emit/consume:** the evaluator runs `holdout verify` plus its runner, then writes `qa/sherlock/holdout-verdict.json` (evaluator-only, outside `qa/holdout/`) and publishes. `doctorwatson next` prints a `HOLDOUT:` line only if the verdict's `head` matches and its sender DID equals the pinned evaluator DID.
- **Honest limit:** DIDs are attribution, not isolation; agents share one Unix UID. Real isolation remains the hook plus `holdout verify`, optionally a separate Unix user. The bus adds the narrow interface, signed provenance and unforgeability of the verdict by a driver.
- **Bus down:** the verdict file is the truth; no verdict means "not evaluated", never "pass".
- **Value/cost:** medium. The value is the interface design, not the transport. Bucketing needs the owner's blessing.

### 4. Non-blocking review requests and replies with a 1 h timeout (`review.request/reply`)

- **What:** Codex GPT-6.1 Sol and Grok 4.7 cold reviewers pull a commit range from a channel and reply whenever. The requester records the request and moves on. The 1 h bound applies only to T3 stagenet promotion, where silence is recorded as "proceeded, unreviewed".
- **Topics:** `ch/review` out; replies on `to/<requester>`. Findings live in a file (`qa/reviews/<head>-<reviewer>.md`, untracked, so the docs-only-commit rule is not tripped); the reply carries a pointer.
- **Schema:** request (`agentbus ask`, `ttl_s: 3600`): `{"scope":{"base":"<sha>","head":"<sha>","paths":[..]},"kind":"cold-diff|t3-promotion","askFor":["soundness","scope-creep"]}`. Reply: `{"verdict":"none|concerns","severity":"info|warn","findingsRef":"/abs/qa/reviews/..md","sha256":".."}`. No severity blocks anything.
- **Emit/consume:** `doctorwatson bus review --range A..B` publishes and exits 0 at once. `next` appends `REVIEWS: 2 pending, 1 in`. Only `qa/tiers/t3.sh` may call `bus review --t3`, which waits at most 3600 s in `agentbus wait` chunks of <= 240 s (the CLI refuses longer), then prints `proceeding: no review in 1h` and exits 0. A reviewer wrapper wakes on `ch/review`, runs `codex exec -s read-only`, writes the file and replies.
- **Bus down:** `bus review` runs `codex exec -s read-only` in the background directly (today's T3 path) and writes the same file.
- **Value/cost:** high; it formalizes "reviews are inputs" and lets vendors review in parallel. Moderate cost (reviewer wrapper). Anti-deadlock guard: predicate F11 forbids any `--wait` outside t3.sh.

### 5. Stop-rule escalation to the owner (`stop.escalate`)

- **What:** When `doctorwatson stop-rule` returns STOP (exit 6), post the trace and best hypotheses (exactly what the charter says to bring) and wake the lead or owner.
- **Topics:** `ch/owner` plus `to/lead`; retained `kv/stop` holds the open escalation (cleared on the next flip) so a returning owner sees it.
- **Schema:** `{"type":"stop.escalate","data":{"reasons":["3 iterations since the last flip (limit 3)"],"firstFailing":"A7","trace":"qa/sherlock/traces/..log","lastHypotheses":["..",".."],"iterationsSinceFlip":3,"lastFlip":{"id":"A6","at":".."}}}`. Hypotheses come from `runs.jsonl`; no new data collection.
- **Emit/consume:** `stoprule.main` publishes on the CONTINUE-to-STOP transition (dedupe on `(firstFailing, lastFlip.at)`), then at most hourly while STOP persists. The lead's watcher (idea 7) wakes; the owner's free-text `reply` reaches the driver via `bus inbox`.
- **Bus down:** the `STOP: escalate to owner` line and exit 6 stay primary. Optionally write untracked `qa/sherlock/ESCALATION.json`, which `next` prints on top while present.
- **Value/cost:** high and very cheap; `evaluate()` already returns everything. Only issue is spam control.

### 6. Research-swarm coordination, Loki cross-linking (`swarm.*`)

- **What:** The 10 Codex Sol agents (`wiki-llm/loki/nodes/loki-NN-*.md`) work in isolation. On the bus they announce findings, ask a neighbour, and claim their own node file. Example: loki-02 (GPU MSM) and loki-03 (GPU NTT) share field-arithmetic assumptions; loki-08 (tree scheduling) needs loki-06's circuit costs.
- **Topics:** `ch/loki` for traffic, `to/loki-NN` for asks, retained `kv/loki.NN.status` (`researching|drafting|done`) as a free progress board. Resource `file:/abs/wiki-llm/loki/nodes/loki-NN-*.md` claimed by its owner so a second editor queues.
- **Schema:** `swarm.finding {node, claim, confidence: measured|derived|cited, ref, sha256}`; `swarm.ask {to, question, needs}` via `agentbus ask`; `swarm.link {from, to, relation: depends-on|contradicts|extends, why}`.
- **Emit/consume:** swarm agents use `agentbus send` directly, with `agentbus brief-snippet` in their briefs. Factory side: `doctorwatson bus swarm-board` renders `kv ls` plus recent findings; the indexer harvests `swarm.link` for the atlas.
- **Bus down:** agents write to node files as now; links are lost, research is not. The indexer can rebuild links from files.
- **Value/cost:** high right now (ten agents likely duplicate searches and cost models), low cost. Findings are claims to verify, not facts; `confidence` and `ref` are mandatory.

### 7. Wake the lead (`lead.wake`)

- **What:** The long-lived lead session hears about flips, STOP, heavy-job end, review replies, host-unfit and swarm completion through the existing `agentbus watch --exit-on-first` asyncRewake hook (`hook/lead-watch.hooks.json`) instead of polling.
- **Topics:** `to/lead`, `ch/gate`, `ch/owner`, filtered with `--match "stop.escalate|status.flip|job.end|review.reply|swarm.done|host.unfit"`. Ticks and `job.start` never wake.
- **Emit/consume:** `doctorwatson bus watch` wraps `agentbus watch --run dw-moriarty --as lead ... --exit-on-first`. `preflight` emits `host.unfit` on a fit-to-unfit transition (disk).
- **Bus down:** the watcher exits 75 and wakes the lead at most once an hour (existing behavior); polling `sherlock-next` and `stop-rule` via `/loop` needs no bus.
- **Value/cost:** high, nearly free. Risk: cache-busting wake storms; use a strict allowlist and a 5 minute re-wake floor per type.

### 8. Driver self-inbox (`doctorwatson bus inbox`)

- **What:** At the start of each iteration the driver gets two lines of what arrived: review findings, holdout verdict, owner steer, swarm findings relevant to the failing predicate. Bounded, `recv --peek`, never blocking.
- **Topics:** `to/driver`, `all`, review replies.
- **Emit/consume:** `scripts/sherlock-next` appends the brief; or the existing fail-open `hook-drain` injects it so `next` never calls the bus.
- **Bus down:** `hook-drain` prints nothing; `next` lists `qa/reviews/` and the verdict file from disk.
- **Value/cost:** medium-high; an input nobody reads is dead. Low cost.

### 9. Role registry and pinned DIDs (`qa/sherlock/roles.json`)

- **What:** One identity per role (`driver`, `evaluator`, `reviewer-codex`, `reviewer-grok`, `indexer`, `loki-01..10`, `lead`). A tracked `roles.json` maps role to expected `did:key` and permitted event types: `{"roles":{"evaluator":{"did":"did:key:z...","may_publish":["holdout.verdict"]}}}`. `bus.classify()` marks a message from the wrong DID `invalid`. Broker-side topic ACLs would be stronger (open question).
- **Emit/consume:** `doctorwatson bus who` joins `presence/*` with `roles.json`.
- **Bus down:** `who` prints `bus unavailable`; role checks only filter bus hints.
- **Value/cost:** medium; prerequisite for ideas 3 and 5 meaning anything. Identity provisioning is an operator step.

### 10. Smaller ideas (v2 candidates)

- **Indexer wake (`index.request`).** `status.flip` or a batch of `swarm.finding`s wakes the indexer on `ch/index` (60 s debounce); it replies `index.done {ref, sha256}`. Bus down: indexer runs on its schedule; the hook still rejects docs-only inner-loop commits. Value medium.
- **Host-unfit alarm (`host.unfit`).** `preflight` publishes on a fit-to-unfit transition (disk, load), which is on the lead's wake list, so a full disk is heard before the next `heavy` fails. Bus down: preflight prints as today. Value medium, cost trivial.

---

## Failure semantics, summarized

| Situation | Behavior |
|---|---|
| `agentbus` not on PATH, no identity, `DW_BUS=off` | `bus.available()` false, every publish is a no-op, no output change |
| Broker down (exit 5/75) | publish returns `None` in under 2 s; one stderr line at most once per process (`bus: unavailable, continuing`) |
| Arbiter stale / claim `queued` | `heavy` proceeds to the flock; never waits on a claim |
| Broker restart, queued messages lost | consumers re-read `status.json`/files on every wake; digests carry `head` for staleness |
| Duplicate / stale / replayed message | `event_id` + `head` + sender DID check (reuse adapter's classify) |
| Bus up, reviewer silent | `review.request` expires (`ttl_s`), T3 proceeds after 1 h, recorded as "no review" |

## Ranked top 5

1. **Stop-rule escalation to the owner (idea 5).** Smallest change for the largest failure-mode reduction: it turns the factory's key safety valve from a line on a terminal into a wake-up. Everything needed is already in `stoprule.evaluate()`.
2. **Lead wake via `watch --exit-on-first` (idea 7), fed by flip broadcast (idea 1).** Existing hook plumbing; the lead stops polling and hears about flips, STOP and job ends. Together with idea 5 this forms the core of v1.
3. **Non-blocking review requests/replies with the 1 h T3 rule (idea 4).** Realizes "reviews are inputs", lets Codex and Grok work in parallel, and removes the old deadlock class by construction (nothing may call `--wait` except the bounded T3 step).
4. **Research-swarm board and cross-links (idea 6).** Immediate payoff, because ten Sol agents are running now: duplicate-search avoidance, `swarm.ask` between neighbours, file claims on node files, and structured links for the indexer.
5. **Heavy-job visibility and slots over the flock (idea 2).** Valuable as soon as the driver, evaluator and fuzzer contend for the machine and essential for GPU slots; ranked fifth because the flock already provides correctness and the benefit is mostly ordering and visibility.

Sixth: the holdout verdict channel (idea 3). The hook and files do the real work, and the bus cannot enforce isolation under a shared UID.

## Minimal v1 integration

Scope: ideas 1, 5, 7 plus a visibility-only slice of 2 and a skeleton of 4. About 300 lines, stdlib only (it shells out to the `agentbus` CLI, matching the "no new dependencies" rule and avoiding importing `paho`).

### Modules (in `models/doctorwatson/factory/`)

- `bus.py` (the only module that touches AgentBus):
  - `available() -> bool`: `DW_BUS != off`, `shutil.which("agentbus")`, identity discoverable (`AGENTBUS_IDENTITY_DIR` or `.agentbus/identity.json`). No network call.
  - `publish(to, subj, body: dict, *, kind="msg", ttl_s=None, timeout=1.0) -> str|None`: builds the `doctorwatson.bus/1` body with `event_id`/`head`, runs `agentbus send` with `subprocess.run(timeout=...)`, returns the message id or `None`. Catches `OSError`, `TimeoutExpired`, any non-zero exit (5, 6, 75 are all "unavailable").
  - `kv_set(key, dict)`, `classify(envelope, head, roles) -> advisory|stale|duplicate|invalid` (port of `classify_notice`, same rule that a message may only prompt a file read).
  - `inbox(limit) -> list` using `agentbus recv --peek`.
- `busevents.py`: pure functions that build bodies from existing data (`flip_event(status_before, status_after)`, `stop_event(evaluate_result, runs)`, `job_event(rec)`). Pure means they are unit-testable with no bus.
- Hook points (each wrapped as `bus.try_(...)` so an exception never escapes): end of `cli.cmd_check` (status.flip/tick), `stoprule.main` (stop.escalate on transition), `heavy.main` (job.start/end, bus claim attempt with `--wait 0` before the flock).
- Tests (`factory/tests/test_bus.py`, `unittest`): a fake `agentbus` script on `PATH` that records args or exits 5; assert (a) bus off gives byte-identical stdout and exit codes for `next`, `check`, `heavy`, `stop-rule`; (b) a hung fake (sleep 30) costs <= 2 s; (c) event schema and `event_id` determinism; (d) stop dedupe.

### CLI additions (`scripts/doctorwatson bus ...`)

| Command | Behavior | Exit |
|---|---|---|
| `bus doctor` | `agentbus doctor`, plus a `DW_BUS` and identity summary | 0 ok; 5 unavailable (informational) |
| `bus status` / `bus tail` | Retained `kv/status`, `kv/stop`; durable log filtered to `doctorwatson.bus/1` | 0 |
| `bus inbox [--brief]` | Non-blocking `recv --peek`, classified | 0 |
| `bus publish --type T --data JSON` | Manual emit (owner/lead steer, tests) | 0 sent; 5 unavailable |
| `bus review --range A..B [--t3]` | Publish `review.request`; `--t3` waits at most 3600 s in chunks <= 240 s, then prints `proceeding: no review in 1h` and exits 0 | 0 always |
| `bus watch` | `agentbus watch` for the lead with the wake allowlist (idea 7) | 2 on wake, 75 bus down |

Environment: `DW_BUS=off|auto` (default `auto`), `DW_BUS_RUN=dw-moriarty`, `DW_BUS_TIMEOUT=1`. Bus exit codes are never propagated as `doctorwatson` exit codes except for `bus *` subcommands where explicitly listed.

### Predicates F9+ (append to `qa/sherlock/predicates.yaml`; the F set total becomes 12)

All T0 and light, none `heavy`, none depends on the bus being up, so they cannot wedge the loop when the bus is off.

```yaml
  - id: F9
    set: F
    title: "Bus wrapper fails open (bus-off and hung-broker leave next/check/stop-rule output and exit codes unchanged)"
    tier: T0
    check: "python3 -m unittest factory.tests.test_bus"   # run from models/doctorwatson
    heavy: false
    depends: [F2]
    source: "wiki-llm/doctorwatson-agentbus/brainstorm-roles.md#design-laws"
    holdout: false

  - id: F10
    set: F
    title: "Bus events validate (doctorwatson.bus/1 schema, event_id determinism, stop-rule dedupe)"
    tier: T0
    check: "scripts/qa/check-f10.sh"
    heavy: false
    depends: [F9]
    source: "wiki-llm/doctorwatson-agentbus/brainstorm-roles.md#ideas"
    holdout: false

  - id: F11
    set: F
    title: "Nothing waits on a reviewer (no --wait outside qa/tiers/t3.sh; wait is bounded to 3600 s)"
    tier: T0
    check: "scripts/qa/check-f11.sh"     # lint plus a bounded-wait unit test
    heavy: false
    depends: [F9]
    source: "AGENTS.md#the-loop"
    holdout: false

  - id: F12
    set: F
    title: "Bus installed and healthy (informational: passes if agentbus doctor exits 0 or DW_BUS=off is declared)"
    tier: T0
    check: "scripts/doctorwatson bus doctor --accept-off"
    heavy: false
    depends: [F9]
    source: "wiki-llm/doctorwatson-agentbus/brainstorm-roles.md#failure-semantics-summarized"
    holdout: false
```

F12 is deliberately weak (`DW_BUS=off` also passes) so a missing bus can never stop the factory. F11 is the machine-checked form of "nothing waits on a reviewer" and the highest-leverage F predicate.

### Rollout order

1. `bus.py`, `busevents.py`, tests (F9 to F11), landed with the bus effectively off (`auto` is a no-op when no identity exists).
2. After the operator provisions identities (not done here): `driver`, `lead`, `evaluator`; enable flip broadcast, stop escalation and the lead watcher.
3. Reviewer wrapper, `bus review`, and the swarm convention in the Loki agents' briefs.
4. Heavy `job.*` announcements and slot claims; roles.json and the holdout verdict channel after the owner decides on bucketing.

## Open questions

- Can EMQX topic ACLs be set per credential so only the evaluator DID can publish `holdout.verdict`? Otherwise role enforcement is receive-side only.
- One run for the factory versus separate swarm runs: claims are run-local, so Loki file claims only work if the swarm shares the run.
- Broker restarts lose queued traffic, which is acceptable only because every message here is a doorbell. Guaranteed delivery needs a file plus a doorbell.
