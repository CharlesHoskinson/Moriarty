# doctorwatson<UTXO> local reader

Run `./scripts/doctorwatson-utxo.sh` from any directory for inventory integrity
and the focused synthetic regressions. Arguments pass directly to `check.py`;
any argument selects the reader only. Examples:

```bash
./scripts/doctorwatson-utxo.sh --json
./scripts/doctorwatson-utxo.sh --requirement EUC-001 --json
./scripts/doctorwatson-utxo.sh --gate EUC-001
./scripts/doctorwatson-utxo.sh --next
```

The reader discovers this checkout root. `--repo`, `--baseline` and `--evidence`
select another local inventory; baseline and evidence paths must be explicit
repository-relative regular files. All referenced files require complete
lowercase SHA-256 pins. Links, directory references, special files and traversal
are rejected. Command-like metadata is inert text.

Exit 0 means inventory integrity; an all-open inventory can pass. Exit 1 means
inconsistent membership, stale or missing evidence, or an open/unknown gate.
Exit 2 means malformed input or unsafe file selection. Requirement selectors
match an exact ID or complete stable key, never an ID prefix. The JSON report
keeps implementation, observations, declared acceptance and computed file
currentness separate; `nextActions` retains the declared actual consumers.
Requirement inspection includes the verbatim stored baseline clause and original
scenario text, along with recorded expectations, gaps, applicability and next
action. Declared `ready` and `ready_*` next actions precede blocked entries;
blockers and original order within each group are retained. This ordering does
not establish readiness or authorize execution.

A closed declaration additionally needs complete observed scenarios, observed
validation, complete implementation, applicable original profile and target,
current transitive dependencies and pinned evidence. Baseline `originalScope`,
applicability and acceptance must name the same profile and target. Every
observation and review reference must carry those fields. Independent review
declarations require distinct nonempty `author` and `reviewer`, `independent:
true`, and both `role: independent-source` and `role: actual-result` references.
Changed profile/target scopes require separately reviewed additive mappings;
this reader does not infer one. Each family layer is reported independently.
`dependsOn` rows are acceptance prerequisites and must have closed gates before
their parent can close. Inputs needed only for byte freshness use `sourcePins`.

The checker only reads public local files with Python's standard library. It
does not authenticate declarations or reviewers, interpret substantive
contracts, prove semantic or cryptographic correctness, run declared commands,
or establish compiler, proof, wallet, node or ledger acceptance. Synthetic
regressions exercise reporting only. No SDK, model, network or native operation
is part of this entrypoint, and no scheduler or hosted CI is installed.

The Cardano workflow status consumer also loads `cardano_graph_context.py`.
Its `load_graph_context(repo_root, graphify_out=None)` resolves an explicit
argument first, then `GRAPHIFY_OUT`, then that checkout's `graphify-out`.
Relative selections resolve against `repo_root`; it never falls back to the
parent checkout. For the existing external graph, select it explicitly:

```bash
export GRAPHIFY_OUT=/home/charl/moriarty-c2-opus55-restored-20261017/graph-refresh-20261019/graphify-out
./scripts/doctorwatson-utxo.sh --cardano-next --json
```

The helper reuses `scripts/graph-nav.py` freshness checks: graph/manifest/coverage
hashes, recorded root, snapshot cardinality, HEAD, every included body and primary
inventory membership. The report bounds displayed issues and lists the omitted
counts. Matching graph identity permits at most four exact-node/source-location
handoff references and one inert 1500-token query suggestion. The budget is
approximate. A stale graph's suggestion is explicitly `dated-discovery-only`;
changed source hashes, original snapshot/status and the partial semantic coverage
qualification remain visible. An unbound graph offers no guessed node/query.
Read the complete applicable current sources after lookup. No graph rebuild,
model, proof, SDK, wallet, network or ledger operation runs in this helper.

This reuses the existing Moriarty graph policy and additive fragments in
`docs/utxo/graphs/`. The inspected ProjectFenrir conventions are in
`/home/charl/projectfenrir/aitools/fenrir-graph/SKILL.md`, its
`references/context-packs.md`, and `aitools/ctx/{cmd_q.py,stamp.py,cmd_pack.py}`:
explicit external graph selection, bounded exact-ID navigation, pointer packs,
source reads and freshness qualification. ProjectFenrir's
`fenrir-workspace/SPRINT-WORKFLOW.md` supplies its sprint workflow;
`sequencer/quint/durable_handoff.qnt` and
`sequencer/quint/settlement/enforced_settlement.qnt` are protocol models, not a
DoctorWatson execution model. Its PPR implementation, measured recall and update
commands are not imported or claimed here. Moriarty's existing workflow model
remains `models/doctorwatson/doctorwatson.qnt`; this Python context handoff has no
new Quint correspondence result. Graph metadata establishes no ledger acceptance
and no live/rearmed loop.

Focused ordinary checks:

```bash
python3 -m unittest discover -s models/utxo/qa -p test_cardano_graph_context.py -v
```
