# DoctorWatson delivery: plan, execute once, inspect, reconcile

Superseded 2026-10-24: review routing, adoption votes, consumed-attempt and no-retry rules, permanent lanes and campaign queues in this file are retired. The process is the factory loop in [AGENTS.md](../AGENTS.md); technical descriptions below stand as history of the legacy tooling.

This page documents the ordinary DoctorWatson delivery surface in
`models/doctorwatson/` (`delivery.py`, `run_selected.py`,
`settle_observation.py`, `reconciliation.py`) and its shell routes in
`scripts/doctorwatson.sh`. It grants no native, proof, wallet or target
authority. Moriarty stays permissionless: these records are internal
project delivery evidence, never a public developer compile/prove/deploy
permission.

## Tutorial: inspect the blocked templates

```bash
./scripts/doctorwatson.sh --inspect \
  --plan models/doctorwatson/examples/delivery-plan.template.json \
  --results models/doctorwatson/examples/delivery-history.template.json
```

Expected result: exit 0 (records are inspectable) with every task
`blocked-input` and `ready_for_root_selection: []`. The templates use
exact contract schemas with `REPLACE-WITH-...` paths and zero pins, so
no real prerequisite is fabricated and nothing is selectable. Exit 0
from `--inspect` never means a task succeeded.

## How-to: execute one selected operation

Prerequisites, all unresolved until Root selects them for the current
source cut: a genuine plan whose pinned source/tool/config files exist
unchanged, and a selection file (`doctorwatson.execution-selection/2` for SDK build or `doctorwatson.handkey-selection/1` for the fixed verifier)
pinning the existing guard and config bytes, the consumer fingerprint,
the fixed outer sequence and the exact raw output paths. The stopped
C13 amendment stays stopped and whole-wrapper CPU `c` stays unknown;
no template, plan or selection in this tree resolves either.

```bash
./scripts/doctorwatson.sh --execute TASK \
  --plan PLAN.json --selection SELECTION.json --attempt NEW_DIR \
  --history HISTORY.json [--reconciliations RECONCILIATIONS.json]
```

Behavior: prelaunch readiness is the one common path
`reconciliation.load_readiness` — each raw
plan/history/reconciliations input is read once through the pinning
reader and the pinned bytes themselves are strictly parsed
(`delivery.parse_plan_bytes` / `parse_history_bytes` /
`parse_reconciliations_bytes`), with a post-read pin recheck —
folded around `delivery.assess`, which is also the shell `--inspect`
route, so inspection and launch cannot disagree on reconciled
readiness for the same file identities. This pins readiness inputs
only: at dispatch the guard is still executed and the config still
read by pathname after their pin verification and actual-use
recheck, so no atomic pathname-dispatch claim is made. Execution requires an explicit complete `--history` file; an initial empty
history must be an actual versioned history file. Omitting it refuses before
reservation or child launch. For the handkey path a durable sibling reservation
journal retains each task/attempt and its original Result. Changing an output
directory or omitting a journaled original from later supplied history does not
restore eligibility. An incomplete reservation stays blocked. Completeness
outside that local journal remains an external premise; this is not a global
append-only or authenticated history service. The bridge requires
that path to report the task as a current selection candidate (active
disposition, dependencies met, no unresolved unknown or cleanup debt,
current attempt not itself unresolved), re-verifies guard/config pins,
writes and fsyncs `PREEXEC.json` (SDK `doctorwatson.preexec/4`, carrying
the pinned `readinessIdentity`), re-runs the same readiness and
selection checks at actual use immediately before dispatch (any drift
is blocked with zero dispatch), dispatches the fixed argv at most once,
preserves the SDK raw receipt (`doctorwatson.execution-result/4`),
and lets the fixed collector produce the importable consumer Result in
`consumer-result.json` with a digest-pinned `collector.json` sidecar.
An existing attempt directory is consumed forever: no retry. The shared capability table distinguishes dependency-ready candidates from
`ready_for_dispatch`: both collector and executor must exist. This is dependency/capability readiness;
`dispatch_preflight: not-observed` records that selection, resource and durable
reservation checks still remain at actual execution. The fixed SDK
build adapter remains supported; the local handkey verifier adapter now has a
separate strict selection. Settlement has a collector but no executor in this
surface. Other unavailable executors report concrete blockers. Assessment caches
last for one assessment only; actual dispatch performs a separate fresh check.

### Fixed local verifier

`doctorwatson.handkey-selection/1` binds task, adapter, attempt, fingerprint and
five `{path,bytes,sha256}` pins: `tool`, `key`, `proof`, `statement`, `outputRoot`.
The first four must be declared inputs; the tool must also be an identity tool.
The outputRoot config pin identifies exact JSON
`{"schema":"doctorwatson.handkey-output-root/1","path":"/absolute/real/directory"}`.
The new attempt directory must be an immediate child of that directory. The
only command is `TOOL verify-pis KEY PROOF STATEMENT`; requests cannot provide
argv, environment, shell commands or limits.

The Linux verifier capture runs inside a dedicated child subreaper with closed
stdin, a small environment and fixed 30-second wall, 25-second per-process CPU,
2 GiB per-process address-space, 64 MiB file and 4 MiB per-stream limits. These
are local capture limits, **not aggregate native resource containment or an
admission for a native proof experiment**. It records terminal state, resource
observations and owned-descendant cleanup, requiring reaping through ECHILD and
process-group absence before cleanup is complete. Incomplete cleanup remains
unknown. The verifier uses `doctorwatson.handkey-preexec/1` and
`doctorwatson.handkey-execution-result/1`. Durable PREEXEC and raw streams feed
the existing strict collector;
exit zero without the exact positive typed verifier verdict cannot succeed.

Fake-child tests establish this ordinary protocol only. No genuine handkey,
SDK, proof or wallet operation was dispatched by the workflow audit. Existing
native source/resource conditions and independent proof/target evidence remain
separate. Runtime SDK process evidence producers are explicitly unselected;
their absence yields unknown/debt states, never a synthesized success.

## How-to: reconcile an unknown or cleanup-debt attempt

Append a record to a `doctorwatson.reconciliations/1` file. Each
`doctorwatson.reconciliation/1` record binds exactly one original
`task`/`attempt`/`fingerprint`/`adapter` plus the canonical digest of
that original Result, with fixed evidence only: a
`doctorwatson.process-reconciliation-evidence/2` file (the canonical
original-result digest over the whole original Result record,
observed `not-started|exited` outcome, cleanup/restoration complete)
or, for settlement, the raw harness `source-result` decoded
by the fixed decoder plus post-state and cleanup receipt. The
evidence's `preexec_sha256` field is only an operator assertion: it
is format-checked but bound to no PREEXEC FileRef and no PREEXEC
file is opened, so it establishes nothing beyond the existing
`producer_authenticity: not-established` basis; the binding that
validates is the whole-Result digest. Process evidence must also be
consistent with the original's own recorded
terminal/exit_code/signal/submission: `not-started` cannot discharge
an original whose record proves it ran (known terminal or recorded
exit/signal), and exited evidence must equal any recorded
exit_code/signal exactly. Settlement `tx-included` decodes against
the current plan task's settle inputs, so reconcile against a plan
whose settle inputs still match the original dispatch. There is no
clear-debt flag. Then:

```bash
./scripts/doctorwatson.sh --inspect \
  --plan PLAN.json --results HISTORY.json \
  --reconciliations RECONCILIATIONS.json [--next]
```

History rows are preserved verbatim. Only the reconciled original
attempt's unknown/debt block lifts: each attempt's obligations are
derived by the consumer's own `delivery.attempt_obligations`
(record cleanup string and the actual resource-receipt artifact's
validity, identity and membership), the same derivation
`delivery.assess` uses, so one attempt's discharge never erases a
sibling attempt's receipt-artifact cleanup debt, and such
artifact-debt originals are reconcilable by a record naming that
attempt. `process-not-started` proves the
original never ran, so a separately selected newer attempt may reuse
the same fingerprint; any consumed outcome (`process-exited`,
`cleanup-complete` of an exited process, `tx-included`) suppresses a
same-fingerprint retry, and only a changed-fingerprint newer attempt
may become a candidate after that genuine discharge. A changed
fingerprint is not by itself proof of changed operation inputs: the
fingerprint recipe also covers owner, eligible reviewers and
integrator, so Root's separate selection decision remains the
substantive gate. A record naming
any other attempt clears nothing, an unknown settlement outcome is
never established by local process exit alone (it requires the
actual transaction readback through the fixed settle decoder), and a missing executor
receipt stays `unknown-reconcile`. The same reconciliations file is
then passed to `--execute` (with `--history`); inspection readiness
and prelaunch readiness are the same function over the same pinned
bytes. Deleting a consumed attempt directory does not restore
eligibility while the raw history retains the failed or unknown
original record.

## Reference

| Route | Consumer | Effect |
|---|---|---|
| `./scripts/doctorwatson.sh` | Sherlock QA + `models/doctorwatson` suites | Local ordinary checks; dispatches nothing |
| `--test-selected [MODULE...]` | DoctorWatson Python suites only | Focused ordinary tests |
| `--inspect --plan P --results H [--reconciliations R] [--next]` | `reconciliation.py` (`load_readiness`) | Read-only common readiness report with `readiness_identity` |
| `--execute TASK ... --history H [--reconciliations R]` | `run_selected.py` → same `load_readiness` | One dispatch at most; identity pinned in PREEXEC and rechecked at use; see exits |
| `--design` / `--design-flow` / `--host` | Existing Quint models / host wrapper | Explicit local checks |
| `--design-readiness` / `--design-cleanup` | Focused additive Quint slices | Typecheck and directed tests only |
| `--cleanup --request Q --report O [--apply]` | `cleanup.py` | Read-only plan by default; explicit cycle finalization on apply |

Bridge exits: `0` dispatched and collector succeeded; `1` dispatched
and failed; `2` bad input before any attempt write; `3` blocked before
launch with zero child calls; `4` outcome unknown-reconcile. Plan,
history, result, observation, cleanup, review and adoption schemas are
the exact-field contracts in `delivery.py`
(`doctorwatson.delivery-plan/1`, `doctorwatson.delivery-history/1`);
task identity is predispatch only — observed post-state bytes are never
a plan input. Settlement `source-result` must be the actual harness
`sherlock-settle-result/1` + `sherlock-chain-observations/1` JSON:
`settle_observation.py` recomputes the derived baseline from the actual
pre observation and binds batch/root/height/membership plus the real
`assertReadbackAction` (`{address, tx_hash, unshielded_balances}`) and
`assertSuccessfulSubmitReceipt` returns. Supplied observation booleans
alone, non-JSON bytes, or the producer test's injected serialization
stub cannot pass.

Formal-model dependency (explicit, not part of this runtime packet):
the source03 Quint extension bytes are known-defective (five Root-run
counterexample witnesses reproduce) and are excluded here; no model
file is changed or claimed correct by this document. A separately
owned, later-reviewed model overlay must land before any model
correspondence claim, and Root runs its exact bounded Quint commands
then. No model result is claimed by any route above.

## Explanation: scope of the evidence

Mock/pure Python tests prove only the ordinary producer-to-consumer
dataflow at fixture scope: one predispatch plan and fingerprint through
a mock existing outer guard, producer-shaped raw files, fixed
collectors, and consumer readiness, including false-verifier,
wrong-root, tampered-output and unknown-outcome branches, plus the
true CLI/executor reconciliation roundtrip (unknown history blocks;
correct original terminal and cleanup evidence enables a separately
selected qualifying newer attempt; wrong token, old digest, missing
cleanup, wrong fingerprint, cancelled task and tampered
history/reconciliations all leave zero dispatch). They prove
no native compilation, proof, ledger acceptance or settlement.
`fixture_support.py` is test support only; no product module imports
it and no execution premise may depend on it. Producer authenticity
remains `not-established`; review and Root adoption stay explicit
external premises. There is no queue, scheduler, daemon, hidden retry
or hosted CI in this surface.


## How-to: finalize a cycle and clean redundant local branches

Refresh only the affected graph context with the existing Graphify worker first.
Write a `doctorwatson.graph-refresh/1` receipt binding `cycle_id`, exact
`affected_sources` (`path`, `sha256`), matching `coverage` paths, and nonempty
`artifacts` (`path`, `sha256`) for the actual graph outputs. The helper verifies
those bytes; it cannot attest the graph's semantic completeness or generate it
from a receipt. Keep private source and runtime data out of public graph exports.

Prepare `doctorwatson.cleanup-request/1` with that receipt's path/digest, an
existing `wiki_path` under `wiki-llm`, outcome/lessons, exact retained root refs
and tips, explicit active branches/worktrees, and explicitly selected inactive
candidates. Supply a fresh inventory of the common Git directory, checkout HEAD,
all open PR heads/bases and active worktree leases. API/owner completeness is a
caller premise, explicitly reported as such. Inventory expires after 900 seconds;
refresh it instead of changing its timestamp without re-observation.

```sh
./scripts/doctorwatson.sh --cleanup --request REQUEST.json --report REPORT.json
./scripts/doctorwatson.sh --cleanup --request REQUEST.json --report REPORT.json --apply
```

Default inspection validates the graph and plans the wiki append/retirements. It
writes the requested report but does not modify graph, wiki, branches or worktrees.
Apply locks the shared cleanup operation, appends exactly one cycle entry while
preserving the wiki prefix, then rechecks roots, tips, PRs, leases and full status
before any deletion. A protected, dirty or unique candidate is retained with its
reason and next action. A moved tip/root, missing inventory or failed mutation
leaves cleanup incomplete. A removed worktree followed by ref-deletion failure
is a partial outcome, not a success. Records retain branch configuration rather
than silently pruning unrelated config.

The helper supports local refs only. Remote cleanup remains an explicit Git/GitHub
operation with a fresh same-repository merged/reachable-head proof and expected
remote-head deletion lease. Neither age nor an offline agent proves redundancy.
No force removal, object GC, archive-branch creation or hidden retry is performed.
The shared advisory lock coordinates cooperating DoctorWatson instances; arbitrary
Git commands and concurrent filesystem edits remain outside that lock. Confirm no
other cleanup writer before applying to live worktrees.


### Retirement history and repeat inspection

The report retains `doctorwatson.local-retirement/1` receipts minted only after
an actual successful expected-tip ref transaction. `operation_sha256` binds the
cycle/source/graph/wiki/candidates/retained roots and actual Git common directory;
it excludes the live inventory, which is always freshly checked. The raw
`request_sha256` remains separate. Prior retirement receipts survive intervening
inspection and incomplete reports. An exact prior retirement can be recognized
only with its matching operation receipt and fresh absence of the ref, worktree
registration and path; recreated or moved refs do not inherit that outcome.
The no-concurrent-writer window includes inspectors sharing the same report
path: apply holds the common-dir lock, while inspect does not serialize report
replacement. Use distinct report paths for concurrent inspections.
