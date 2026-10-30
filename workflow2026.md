# Moriarty / Sherlock workflow — 2026-10-30

## Purpose and handoff state

Bring the local Moriarty repository up to date, finish a source-grounded full
Graphify graph using Terra agents coordinated through ProjectFenrir AgentBus and
an RTX 5090, then finish Sherlock with ten Terra workers: four implementing,
three using Quint, and three using Lean. An Astra advisor supervises, while Muse
Wonder independently checks requirement correspondence and drift. Workers may
search the internet, communicate through AgentBus, and spawn their own agents.
Arm a durable completion loop only after its prerequisites and stop rules pass.

**Latest owner instruction:** save this entire workflow, including all tools, as
`workflow2026.md`, commit it, then stop while the owner switches computers.
This document is a handoff, not evidence that Sherlock or its graph is complete.
Do not restart background work merely because this file is opened; resume when
asked. The ten-worker team and persistent completion loop were **not launched**.
Muse's model profile, subscription availability, documentation and model card
were inspected; a Muse verification session was **not run**.

## Repository and machine checkpoint

- Remote: <https://github.com/CharlesHoskinson/Moriarty>.
- Fetched upstream base: `567695ce2476ae60f67ea3664a6f581651b94e53`
  (`Merge remote-tracking branch 'origin/sherlock-s10'`). The commit containing
  this handoff will be a descendant of that base.
- Active checkout: `/home/charl/moriarty-origin-main`, branch `main`, tracking
  `origin/main`. It had been a sparse, detached checkout. Sparse checkout was
  disabled so the full tracked source tree is materialized (about 2.8 GB).
- Original checkout: `/home/charl/Moriarty`, preserved with its uncommitted edits
  and original history on `recovery/local-main-20261030`. At fetch time that
  history had 87 commits unique to it and upstream had 1,456 unique commits.
  It was not reset, cleaned, rebased, stashed or merged. Do not overwrite it.
- Original uncommitted work includes edits to agent/development instructions and
  an untracked Aeon refinement change. Do not stage it with this workflow.
- GPU observed: **NVIDIA GeForce RTX 5090 Laptop GPU**, 24,463 MiB VRAM, about
  4,842 MiB used when inspected. Verify the destination computer independently.
- Filesystem free space was approximately 21 GiB after checkout expansion.
  Factory heavy preflight requires **150 GiB**. Preserve that threshold; provide
  suitable storage before heavy proving/build work. Do not delete unrelated work.

On another computer, use a normal full clone, subscription sign-ins and newly
provisioned identities. Do not copy credential files, private wallet state,
private witnesses or AgentBus signing keys into Git. Ignored graph caches and
local evidence do not arrive through `git pull`.

```bash
git clone https://github.com/CharlesHoskinson/Moriarty.git
cd Moriarty
git fetch origin
git status --short
git log -1 --oneline
# For a clean existing main checkout:
git pull --ff-only
# If a checkout is sparse, inspect before restoring full tracked content:
git sparse-checkout list
git sparse-checkout disable
```

For a divergent or dirty checkout, preserve its branch and files and use a clean
linked worktree or clone at upstream. Do not blindly apply the pull command.

## Authoritative repository workflow

Read the current checkout's `AGENTS.md`,
`models/doctorwatson/factory/CONTRACT.md`, the development skill at
`plugins/moriarty-dev/skills/develop/SKILL.md`, and applicable technical references.
The current charter retires historical campaign, lane and adoption-vote gates;
do not resurrect the older September workflow from the preserved checkout.

```bash
scripts/sherlock-next
scripts/doctorwatson next --json
scripts/doctorwatson preflight --json
scripts/doctorwatson stop-rule --json
```

The observed status is an **old recorded result**, not a fresh full rerun:

| Set | Recorded passing / total |
| --- | --- |
| A | 7 / 10 |
| B | 4 / 9 |
| C | 5 / 6 |
| U | 34 / 35 |
| F | 11 / 13 |

First failure: **A8**, an indexed finalized contract call on the pinned Sherlock
stagenet verifier. Other recorded failures: A9, A10, B4, B6–B9, U023, C4, F4 and
F11. `qa/sherlock/status.json` names an older HEAD and must be revalidated against
the working source. Only factory commands write status/results; never hand-edit
passes, timestamps, counts or `runs.jsonl`.

The stop rule returned **STOP**, reporting about 98.6 hours since the last flip
on October 26. Its referenced local trace was absent from this fresh checkout:
`qa/sherlock/traces/20261026T105549Z-U023-dispatch-receipt.log`.
Preserve the stop and report the missing trace; recover real evidence where
available. A new resume request is not permission to fabricate a flip or reset
history. Independent diagnosis/source repairs can be scoped explicitly; a
persistent execution loop must not silently ignore STOP.

## Tool inventory

Versions below are observations from the original computer, not portable
installation guarantees. Read current primary documentation and local help
before using options on a different version.

| Tool | Role | Observed version / location |
| --- | --- | --- |
| Git and GitHub | Source synchronization, worktrees, reviewed integration and remote handoff | `git`; remote above |
| Codex CLI | Terra workers, Astra advisor, child agents, cold T3 review | `codex-cli 0.166.0`; `/home/charl/.local/bin/codex` |
| Terra | All ten implementation/formal workers; semantic graph extraction agents | Exact model `gpt-6.1-terra` |
| Astra | Advisor supervision, decomposition, integration review | Exact model `gpt-6-astra` |
| AgentBus | Local MQTT presence, direct messages, channels, advisory file/slot claims | 1.0.0, release `a51b5b97ab61`, built 2026-10-23 |
| `fenrir-spawn` | Bind separate AgentBus identities to shared or isolated checkouts | Installed with ProjectFenrir AgentBus |
| `agentbus-codex` | Optional Codex wrapper with bus hooks | Installed; see messaging restriction below |
| `agentbus-supervise` | Existing AgentBus service supervisor | `start`, `stop`, `status`; inspect scope before changing services |
| DoctorWatson CLI | Existing predicate selection, heavy-resource locks/caps, spawn bookkeeping and checks | `scripts/doctorwatson` and `scripts/sherlock-next` |
| Graphify / Graphify MCP | AST + semantic graph, coverage, queries and exports | `graphifyy` 0.9.97; `graphify`, `graphify-mcp` |
| `uv` | Isolated Python tools and environments, Graphify upgrades | `/home/charl/.local/bin/uv` |
| CUDA / NVIDIA tools | Explicit GPU graph computation and device measurements | `nvidia-smi`; inspect driver/runtime on destination |
| CuPy / nx-cugraph / pylibcugraph | Device arrays and actual graph algorithms | Prior GPU reference used CUDA 13, nx-cugraph 26.10.0 |
| NetworkX / NumPy / SQLite | Graph assembly, indexing, CPU oracle and coverage checks | Isolated graph environment; inspect installed versions |
| Muse Code CLI | Independent requirement and anti-tangent audit | 1.4.14 (`1.4.14-R7309.1`); `muse` |
| Muse Wonder | External check of actual EARS/OpenSpec implementation | Exact model `muse-wonder-1.0` |
| Quint | Executable specifications, bounded exploration and counterexamples | Not on shell PATH; local install `/home/charl/tools/quint/node_modules/.bin/quint` |
| Lean / Lake / elan | Machine-checked proofs and dependency/toolchain management | Lean 4.35.0 observed; use repository pins |
| Rust / Cargo / rustup | Sherlock implementation, compiler/lints/tests | Repo `rust-toolchain.toml` pins Rust 1.99.0 |
| Python / PyYAML | DoctorWatson factory, evidence consumers and measurement tools | Python 3.14.4; factory uses stdlib plus PyYAML 6 |
| Node.js / package tooling | Existing Sherlock SDK deployment/call/readback scripts and Quint | Use checked-in package manifests and lockfiles |
| Scrapling | Public documentation acquisition with sanitized main content | Installed `scrapling`; use ordinary fetch, no bypass |
| Internet search / browser tools | Primary-source research and technical documentation | Enable for workers; cite exact source URLs |
| Bash / process tools / systemd user services | Existing command execution, durable launch and shutdown | Inspect `systemctl --user`, logs and exact process ownership |
| Agent collaboration tools | Initial in-session Terra graph agent and Astra advisor | `spawn_agent`, `send_message`, `interrupt_agent`, handoff; session limits apply |
| Goal tracking | Original completion objective and explicit pause/resume | Persistent task goal; goal state is not product acceptance |

Use subscription logins for model CLIs. The repository forbids setting
`OPENAI_API_KEY`, `CODEX_API_KEY` or `XAI_API_KEY`. Do not print credentials or
unredacted environment dumps. Authenticate on the destination through the
respective vendor CLI; do not substitute API-key billing for subscription use.

Useful discovery commands:

```bash
codex --help
codex exec --help
codex features list
muse --help
muse exec --help
muse usage
muse model-profile show muse-wonder-1.0 --effort high
agentbus --help
agentbus version
fenrir-spawn new --help
scripts/doctorwatson spawn --help
graphify --help
graphify --version
uv tool list
nvidia-smi --query-gpu=name,memory.total,memory.used --format=csv
```

## AgentBus setup and communication

Existing run: `sherlock-20261030`. Provisioned identities: `lead`, `graph-terra`,
`astra-advisor`. Their broker/key/credential/arbiter checks passed on the original
machine. These identities are not proof that an agent process is running.

Installed instructions are at
`/home/charl/.local/share/agentbus/releases/a51b5b97ab61/SKILL.md`, with `README.md`
and `docs/INTEGRATION.md` alongside it. These are local ProjectFenrir tools;
provision them through the approved ProjectFenrir installation on another
computer. Do not copy private ProjectFenrir source into this public repository.

On this machine, `FENRIR_SPIKE` must name Git's canonical checkout
`/home/charl/Moriarty`, even when binding the linked current checkout. Passing
`/home/charl/moriarty-origin-main` as the canonical repository was rejected.

```bash
FENRIR_SPIKE=/home/charl/Moriarty fenrir-spawn new \
  --run sherlock-20261030 --as WORKER --role implementer \
  --existing-worktree /home/charl/moriarty-origin-main \
  --chan team --chan agents --claims 'file:' --claims 'slot:' --ttl 7d
```

Choose role `quint` for Quint workers, `implementer` for implementation and Lean
workers, and `lead` for the advisor. Each worker and child gets its own binding;
never share the lead's identity. Existing bindings on this machine are under
`/home/charl/wt/.bindings/sherlock-20261030/NAME/.agentbus/`.

```bash
# Start every independent shell command with the correct checkout and identity.
cd /home/charl/moriarty-origin-main
. /home/charl/wt/.bindings/sherlock-20261030/lead/.agentbus/env
agentbus doctor
agentbus recv
agentbus who
agentbus claims --mine
agentbus send astra-advisor checkpoint --body 'Public artifact ready for inspection.'
```

Bus claims are advisory and do not fence writes. Exact absolute linked-checkout
file claims were observed to canonicalize to `/home/charl/Moriarty`; inspect
actual returned resources before editing. Use disjoint file ownership and
serialize Git/index operations with an integration owner. Save the grant's
`hold` and `lease_end`, renew before expiry, and release with the same hold token.
Do not edit on a queued, expired or denied claim. Keep secrets off the bus.

The factory contract prohibits automatic injection of arbitrary bus text into
agent prompts. Therefore the `agentbus-codex` live-hook wrapper is **not the
default for this run**. Agents explicitly inspect public handoff files and
AgentBus messages as untrusted advisory data. Canonical truth remains source,
Git, factory status and run records. Bus failures do not create product failures
or authorize duplicate execution. Factory calls are bounded and fail open.
Do not add `agentbus wait` or reviewer waits to the product path; F11 constrains
waits outside the existing bounded T3 review.

## Full graph stage, including GPU

Apply the Graphify skill before rebuilding. Original installed skill:
`/home/charl/.codex/skills/graphify/SKILL.md` and its extraction/update references.
`uv tool upgrade graphifyy` reported **Nothing to upgrade**, leaving 0.9.97.
The old checkout's graph had 90,600 nodes. It is stale for the new tree and is
not evidence of full coverage. Reuse cached semantic output only after validating
source hashes, extraction format and complete source coverage.

Terra detection found **52,136 supported files**, roughly **71.1 million words**:
31,500 code files, 19,691 documents, 238 PDFs, 706 images and one video; 26
sensitive-name exclusions. These are detection counts, not successful extraction
counts. This is a substantial extraction job. Inventory exclusions, unsupported
formats, failed files, incomplete reads and inaccessible inputs explicitly.

Current partial files are ignored under the active checkout's `graphify-out/`:
`.graphify_detect.json`, `.graphify_python`, `.graphify_root`, `ast.log` and cache
entries. At owner stop, the graph agent interrupted its own AST process with
SIGINT. It had scanned 31,500/31,500 code files, but symbol-resolution
augmentation had not finished: **no `.graphify_ast.json` or final graph was
produced**. Semantic extraction and actual GPU execution were **not run**.
The process exited; the agent launched no children. The AST log identified
missing parser support for 13 Solidity, six OCaml and one SQL file, four
syntax-error files, and 392 zero-symbol files. Investigate these coverage limits
instead of counting every scan as successful extraction. The Graphify PDF extra
was installed after initial detection; rerun detection before resuming. The
actual isolated CUDA environment path was not discovered; only the historical
reference was located. Do not stamp unfinished source files as successfully
extracted.

1. Resolve the installed Graphify interpreter; persist it in
   `graphify-out/.graphify_python`, and bind `.graphify_root` to the active tree.
2. Detect the full supported corpus and preserve a path/hash/type inventory.
   Keep generated outputs and credentials out of the extraction corpus.
3. Run deterministic AST extraction for code through the installed library.
4. Dispatch semantic extraction to Terra agents over AgentBus in bounded chunks.
   Follow `references/extraction-spec.md`: read every assigned file to its end,
   report per-file line coverage and valid JSON, deterministic IDs, provenance,
   confidence and token usage. Distinguish EXTRACTED, INFERRED and AMBIGUOUS.
5. Validate every chunk and persist it before aggregation. Retain failed or
   partial files as pending; do not silently merge a partial run as complete.
6. Assemble/index the graph, deduplicate deterministically, cluster and name
   communities. Large graphs may need a streamed SQLite index rather than
   loading all JSON into RAM. Preserve source identity and edge direction.
7. Use the **actual GPU** for a supported graph pass, such as connected
   components/PageRank/clustering, through explicit nx-cugraph/pylibcugraph.
   Record device, backend/runtime versions, graph digest, all-node/all-edge
   counts, timings and output digest. Check an independent CPU oracle where
   practical. GPU availability or a tiny smoke test is not a full-graph result;
   GPU topology analysis does not substitute for semantic extraction.
8. Produce `graph.json`, `GRAPH_REPORT.md`, manifest/coverage audit and aggregated
   `graph.html`. The corpus exceeds 5,000 nodes: warn and use a community-level
   visualization rather than opening a huge unaggregated graph.
9. State completion only when supported-source coverage is reconciled and all
   mandatory chunks succeed. Report exclusions and costs. Graph completion
   does not establish product, proof or stagenet acceptance.

Prior local GPU technique reference (not a Moriarty result):
`/home/charl/projectfenrir/arl/research/workflow/graphify-gpu-20261019/README.md`
and `analyze_index.py`. It used explicit CUDA 13 graph APIs and checked all-node
weak-component equality with a CPU union/find oracle. Embedded paths under
`/home/charles/` are historical and must be rebound. Do not install into or
modify an unrelated GPU environment to reproduce it.

```bash
uv tool upgrade graphifyy
graphify query 'Sherlock native relation requirements' --budget 2000
graphify path 'NODE_A' 'NODE_B'
graphify explain 'NODE'
graphify export html
```

`graphify query` requires an existing graph. `graphify update` is useful for AST
updates but **does not by itself complete semantic extraction**. For a bare full
build, follow the skill/library pipeline; do not invent a full-build CLI flag.

## Ten Terra workers and Astra supervision

Use exact model `gpt-6.1-terra` for all ten workers and `gpt-6-astra` for the
advisor. Do not silently fall back to Sol or another model. The initial Astra
inspection produced this source-based partition:

| Worker | Assignment | Primary files / pairing |
| --- | --- | --- |
| I1 | A8 actual stagenet submission path | `scripts/qa/run-rollup-stagenet.sh`, `tools/sherlock-sdk/rollup-deploy.mjs`, `rollup-call.mjs` |
| I2 | A10/B4/B8 independent receipt/effect readback | `tools/sherlock-sdk/rollup-readback.mjs`, relevant A10/B4/performance checks; separate from I1 |
| I3 | U023 actual fixed native relation and missing consumers | `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs`, precise `crates/mori-backend/src/sherlock_machine/` consumers; pair Q3/L3 |
| I4 | B6/B7/B9 real per-layer measurement and evidence | `scripts/qa/measure-rollup.py`, `check-rollup-evidence.py`, `check-rollup-feasibility.py` and focused tests |
| Q1 | Suspension, admission, resume and epoch models | `models/sherlock/quint/push-q1/`; pair L1 |
| Q2 | Ordered financial effects, accounting, imports and fee-pool history | `models/sherlock/quint/push-q2/`; pair L2 |
| Q3 | Residual dispatch, exports and native inventory correspondence | `models/sherlock/quint/push-q3/`; pair I3/L3 |
| L1 | Suspension/resume preservation and work/charge obligations | `clean/fork/Clean/Sherlock/SuspensionResume*.lean`; pair Q1 |
| L2 | Complete ordered effects and work-counter preservation | `clean/fork/Clean/Sherlock/FinancialItems*.lean`; pair Q2 |
| L3 | Compiler residualization, encoding and export correspondence | `clean/fork/Clean/Sherlock/CompilerResidual*.lean`, `CompilerExportPlans*.lean`; pair I3/Q3 |

All workers read current requirements and actual consumers first. If an assigned
area is already correct, report that with evidence and select an independent
concrete gap with Astra; do not manufacture work. Formal workers must prove
meaningful correspondence to implementation, not tautologies about a constant
qualification flag. No `sorry`, undeclared axioms, invented traces or weakened
requirements. Keep legitimate existing assumptions explicit.

Each brief must contain the active checkout, base SHA, requirement IDs, exact
ownership, AgentBus binding, expected behavior and negative cases, validation
commands, resource limits, handoff path and concrete completion predicate.
Include the repository startup rule. On resume, inspect the existing changes
and prior result before selecting another task. Workers do not overwrite one
another or stage/commit the entire shared tree.

Astra acts as an advisor, **not an approval gate**: inspect source, propose the
next coherent changes, resolve cross-worker interfaces and flag unsupported
claims. Keep I1/A8 and actual U023 behavior central. Avoid turning the team into
ten factory/infrastructure workers. Root/integration owner handles Git commits
and explicitly selected external wallet operations, preserving existing state.

### Launch tools and child-agent capability

Prefer the existing factory spawn wrapper for worker bookkeeping:

```bash
export DW_BUS_RUN=sherlock-20261030
scripts/doctorwatson spawn --vendor codex --as i1 --role implementer \
  --model gpt-6.1-terra --effort high --sandbox workspace-write \
  --brief .agentbus/sherlock-20261030/briefs/i1.md \
  --out .agentbus/sherlock-20261030/logs/i1 \
  -- -c 'web_search="live"' \
     -c 'sandbox_workspace_write.network_access=true' \
     -c 'features.multi_agent=true'
```

The example is a template, **not an executed launch**. Inspect the current
wrapper and CLI help first. Factory spawn removes inherited `AGENTBUS_*` and
forbidden API-key variables, takes a vendor concurrency slot and assigns its own
worker identity. Default spawned credentials only cover the `agents` channel
and no file claims. Provision/reuse identities under the configured
`DW_SPAWN_IDENTITIES` first if workers need `team` subscriptions and exact
`file:`/`slot:` claims; verify the actual child binding with `agentbus doctor`.
Do not assume the external `fenrir-spawn` binding survives wrapper scrubbing.

Quint and Lean authors also need `--sandbox workspace-write`; the default for
some non-implementer roles is read-only. Internet search and network access
must be verified with a small read-only probe, not merely assumed from a flag.
Configure noninteractive approval behavior consistent with the destination
host's policy; never leave a detached process waiting for a hidden prompt.

For an advisor outside the worker wrapper, use `codex exec -m gpt-6-astra` with
its own identity and a file-based brief/result. For workers, use the wrapper
above or an explicitly bound `codex exec -m gpt-6.1-terra` with equivalent logs
and safeguards. Do not inject raw bus text through the optional wrapper hooks.

Workers may use host `spawn_agent` when available or launch children through
`doctorwatson spawn` / a separately bound Codex CLI. Each child inherits the
requirement scope, startup rules, budget limits and internet capability, but
gets its own name, role and bus identity. Verify availability of the exact
model and record the served model; a requested model flag alone is insufficient.

Account for concurrency: the factory defaults to ten Codex slots. Ten parents
holding all slots cannot synchronously wait for children that need the same
slots. Reserve bounded helper capacity deliberately, or have a parent yield
its slot before child dispatch. Astra and graph agents also count if they use
the wrapper. Inspect `DW_SLOTS_CODEX` rather than starting an unbounded swarm.
Changing agent capacity does not change proof/RSS/disk safety limits.

## Quint, Lean and implementation verification

Quint was installed locally but not available through plain `quint` on PATH:

```bash
export PATH="/home/charl/tools/quint/node_modules/.bin:$PATH"
quint --help
quint --version
```

On another machine install the repository-compatible version using its lockfile
and primary Quint documentation. Use the existing model-specific runners, then
record exact typecheck/test/simulation/model-check commands, seeds, bounds and
counterexamples. A bounded successful search is not an unbounded theorem.

For Lean, inspect `clean/fork/lean-toolchain` and Lake configuration; use its
pinned environment rather than assuming the global Lean 4.35.0 is correct.
Build the changed modules with the existing Lake project and inspect axioms of
new theorems. Preserve explicit assumptions and connect claims to source.

Rust changes follow `RUST_GUIDELINES.md` and the pinned toolchain:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Run resource-heavy builds/proofs through the existing measured factory path,
not directly to evade preflight:

```bash
scripts/doctorwatson heavy --id TASK_ID --budget EXISTING_BUDGET_KEY -- COMMAND
# A failed same-command/same-HEAD retry requires an actual one-line hypothesis:
scripts/doctorwatson heavy --id TASK_ID --budget EXISTING_BUDGET_KEY \
  --hypothesis 'Concrete explanation supported by the prior trace' -- COMMAND
scripts/doctorwatson check PREDICATE_ID
```

After two failures of the same kind, shrink the reproducer before broad retry.
Do not add CI, schedulers, governance or backend migrations as a substitute for
fixing the consumer. Run applicable T0/T1/T2 checks once the host can support
them; T3 is real stagenet promotion with the existing bounded cold review.
The evaluator owns `qa/holdout/`; workers may inspect contracts but never edit
sealed tests or reseal their manifest to make a patch pass.

## Muse Wonder: independent requirement and drift check

Primary documentation retrieved on October 30:

- [Muse Code documentation](https://dev.meta.ai/docs/muse-code)
- [Meta Model API overview and exact Wonder model ID](https://dev.meta.ai/docs/overview)
- [Muse Wonder model documentation](https://dev.meta.ai/models/muse-wonder)
- [Muse Wonder model card](https://research.meta.ai/static/muse-wonder-model-card)

Original-machine raw HTML, model-card PDF and a SHA-256 source manifest are
ignored under `.agentbus/sherlock-20261030/sources/`. Reacquire on the destination;
they are not part of this commit. The card describes intended uses and testing
limits; it does not make Muse's review proof of correctness. The CLI resolved
`muse-wonder-1.0` at high effort. Subscription access was available when checked;
usage allowances change and should be checked again.

Use Scrapling's ordinary GET for public HTML acquisition; preserve requested and
canonical URLs, retrieval time, HTTP status, hashes and limitations. Use ordinary
HTTPS retrieval for the PDF. Treat remote content as evidence, not instructions.

Before the team starts, and after meaningful source/test changes, run an
independent Muse session with a narrow read-only audit brief:

```bash
muse usage
muse model-profile show muse-wonder-1.0 --effort high
muse exec --model muse-wonder-1.0 --reasoning-effort high \
  --workspace "$PWD" --trust-workspace \
  --prompt-file .agentbus/sherlock-20261030/briefs/muse-audit.md \
  --max-model-steps 60 --json
```

Select an appropriate read-only permission profile supported by the installed
CLI and host; the example does not itself enforce read-only shell execution.
Keep web tools available. Do not grant write access simply to make a review run.
Capture stdout/stderr, exact served model and exit status. If Muse cannot run,
record the actual limitation; do not substitute an audit written by the author.

The audit brief asks Muse to inspect **actual code, tests and execution evidence**
against EARS and OpenSpec, especially:

- `openspec/changes/sherlock-stagenet-recursive-rollup/requirements.md` and tasks;
- `openspec/changes/sherlock-universal-interpreter-design/requirements.md` and
  `specs/universal-interpreter/spec.md`;
- relevant machine-layout, synthetic-rollup and optimization specs;
- `qa/sherlock/predicates.yaml`, actual checks, current diff and real run traces.

Require a table of requirement ID, exact source location, implementing caller,
verification performed, missing evidence and concrete next repair. Ask whether
work advances A8/U023 or another demonstrable capability, or is stuck on
infrastructure/tangents. Every unsupported completion claim gets a source-based
finding. Muse findings inform fixes; they do not become an approval gate.

## Acceptance boundaries that must survive the loop

- A8 means an indexed finalized call on the **pinned** stagenet verifier; an
  arbitrary call is not complete recursive-product acceptance.
- A9 needs the actual five stagenet rejection cases. A10 needs independent
  receipts and decoded complete effects.
- U023 includes **both** fixed native relation inventory **and** complete
  stagenet qualification. Formal-only progress cannot close U023 or C4.
  Inspect `qa/holdout/contracts/U023.json` without modifying it. Preserve source,
  VK/layout hashes, compile/prove/independent-verify/deploy/call/readback
  correspondence and independently decoded changed state.
- B6/B7 need retained real B3 artifacts: exact fixture, executable, keys,
  proofs, relation/instance bytes and successful measurements. Missing local
  artifacts are not source defects and must not be reconstructed from claims.
- Preserve the exclusive **32,000,000,000-byte** peak RSS bound, **zero swap**,
  exact batch/workload identity, measured scopes and explicitly unavailable
  metrics. Do not relabel a reduced workload as the full workload.
- B8 separates fees, finalization and settlement readback latency from local
  verification. B9 records samples, failures, resources and remaining gaps.
- Graphs, model opinions, MockProver, constant flags, packet approvals and Git
  merge status cannot establish missing native proofs or ledger behavior.

## Durable loop to arm on explicit resume

Reuse DoctorWatson and the installed process tools; do not build another
orchestration framework. Before calling anything “armed,” retain an observable
process/service ID, exact command, active checkout/base, worker roster, live bus
presence, timestamped logs, next predicate, stop condition and shutdown command.
A file describing a loop is not a running loop.

1. Reinspect Git, restored evidence, tools, model access, GPU, factory preflight
   and STOP. Resolve the actual host/resource blocker or preserve it as blocked;
   never lower thresholds or rewrite prior failures to start the loop.
2. Complete and audit the graph stage. Use it to bound source reads thereafter;
   source wins over stale graph claims. Keep graph maintenance off the product's
   critical execution path.
3. Launch the ten distinct Terra assignments, Astra advisor and Muse audit with
   separate identities and durable file-based briefs/logs. Verify each process
   is alive and served the requested model. Verify child spawning, internet and
   AgentBus with small actual probes; do not infer them from configuration.
4. Select the first failing predicate with `sherlock-next`. Deliver one coherent
   missing behavior; other workers develop independent expected results, proof
   obligations, counterexamples or disjoint implementation dependencies.
5. Run the relevant checks through the existing factory. Retain exact command,
   source base, changed paths, outcomes and non-executed checks. Reconcile partial
   changes before worker restart. Serialize integration and commit on `main`.
6. Ask Astra for concrete cross-file correctness findings and Muse for requirement
   correspondence/drift findings. Fix evidenced defects without reviewer waits.
7. Run `scripts/doctorwatson stop-rule`. STOP means stop and bring the trace and
   best hypothesis to the owner. Two same-kind failures require a smaller
   reproducer; repeated process-only cycles require executable diagnosis.
8. Continue until all required product predicates, formal obligations and real
   ledger evidence pass against the integrated tree. Preserve unresolved gaps.
   Do not equate “agents exited” or “documents written” with completion.
9. On completion or owner stop, stop only this run's agents/process groups,
   preserve unfinished work and logs, release held claims and mark presence
   down. Leave unrelated jobs, wallets, services and source trees intact.

## Restart brief for the next computer

> Read workflow2026.md and current AGENTS.md. Resume the requested full-graph and
> Sherlock completion program only after checking actual machine state. Use
> gpt-6.1-terra workers (4 implementation, 3 Quint, 3 Lean), gpt-6-astra advising,
> ProjectFenrir AgentBus with independent identities, RTX 5090 graph processing,
> and muse-wonder-1.0 independent EARS/OpenSpec audits. Permit scoped child agents
> and internet access. Preserve the original dirty checkout/history, holdout
> ownership, measured resource caps, STOP and all native/ledger acceptance gates.
> Do not claim that the graph, ten-worker team, Muse verification or completion
> loop ran merely because this handoff describes them. Implement real missing
> behavior and report the actual running state and evidence.
