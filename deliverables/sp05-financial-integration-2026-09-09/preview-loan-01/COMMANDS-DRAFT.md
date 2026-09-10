# Preview loan: one proposed execution

Status: **PROPOSED, source reviews and execution admission pending.** This packet
creates no execution admission. The draft plan uses `deadlineMs: 0` so it fails
production validation. No private contents were read to prepare it.

The intended result is the compiled Moriarty loan's actual `deploy`, `initialize`,
`accrue`, and `settle` sequence on public Preview. Use
`ledger/preview-bootstrap.mjs` and its `launchPreviewFinancialCase(plan)` API.
The CLI takes exactly `--run --plan <absolute-path> --sha256 <plan-digest>`.
No local recovery field or source adapter belongs in the plan.

The public identity and promotion receipts identify the original Preview seed,
address and cache. `metadata-observation.json` records owner-only original
snapshot, role and password paths. It establishes existence and permissions;
current sync, DUST balance and private capability validity remain unobserved.
The second role is the existing payout recipient, not a new funded wallet.

## Finalize and admit before any operation

1. Bind the final combined source candidate, including the actual bootstrap,
   public writer, integration, imported production dependencies and installed
   runtime pins. The proposal's `sourceCandidate: null` must be replaced by the
   exact reviewed path and digest. Author source manifests in `publicProvenance`
   are observations, not approval votes.
2. Retain fresh GPT-6 Astra and Grok 4.6 source reviews and resource votes on the
   exact final bytes. Bind this packet's files in the proposal/admission. Preserve
   the actual reviewer receipts. Missing, stale or unavailable votes block launch.
3. Adapt the existing bounded execute-once procedure, keeping its exclusive
   admission/attempt reservation, fsynced exact argv, independent timer and
   remaining-window manager timeout checks. `execute-once.py` is the concrete narrow adaptation. It requires an exact
   `execution-admission.json` and both source/resource votes before operations;
   none exists yet.
4. Recheck the proposed unit, timer, allocation, base, output, new store and
   persistence backup are absent. Recheck no pending persistence marker exists.
   Verify container images, existing limits, loopback proof port, stopped state,
   disk/memory headroom, and original input metadata. Preserve all old ledgers.

## One-shot command after admission

Once the exact source and resource bytes are approved and root has created the
exclusive admission, the single dispatcher command is:

```sh
python3 deliverables/sp05-financial-integration-2026-09-09/preview-loan-01/execute-once.py
```

Do not run that command for checks. It reserves the attempt durably and cannot
replay an existing attempt. It leaves the independent stop timer armed and returns
after acknowledged activation, so final status and containment require separate
actual observations.

Use the current checkout root
`/home/charl/Moriarty/.worktrees/sp05-preview-owner`. The new private base is
`/home/charl/.local/state/moriarty/sp05-preview-loan-20260910-01`.
After admission only, create that base exclusively with mode `0700`. Do not create
`run` or `contract-state`: the reviewed bootstrap creates both exclusively after
its public checks. Copy no seed, role or password values into the plan or repo.

Arm `moriarty-sp05-preview-loan-stop-01.timer` before starting the existing
`moriarty-midnight-proof-server`. Derive the actual monotonic arming epoch from
the manager timer deadline, and retain it durably. Track timer launch attempted
separately from observed timer active. The timer begins shutdown at arming+2500s,
with at most 1s accuracy, and terminates only the loan launcher cgroup and existing
proof server. Local node and indexer must remain stopped throughout.

All readiness, plan materialization and acknowledged service activation share
arming+120s. The existing bounded TCP-listener check can test loopback port16300;
it is not proof readiness or financial acceptance. At most30 connect samples,
at least1s between samples, each at most2s and clipped to remaining setup time.

Materialize the public draft with one change: set `limits.deadlineMs` to the
wall-clock counterpart of **arming+2490s**, preserving the conservative monotonic
remaining window. Retain the plan as mode `0600`, fsync it and its directory,
and retain its digest. Never recompute a later deadline after delay or failure.
The operation deadline covers bootstrap preflight, restore, sync, all four proof
and submission stages, canonical finalized readbacks and comparisons together.
These phases have no separate additive allowance. At latest activation2370s
remain. Native cleanup/persistence can use at most5s beyond that deadline; the
bootstrap hard exit is at +6s and the independent stop begins at arming+2500s.

The resolved launcher argv has this form (placeholders deliberately non-runnable):

```text
/usr/bin/systemd-run --user --unit=moriarty-sp05-preview-loan-01
  --property=Type=exec
  --property=TimeoutStartSec=<floor(activationRemainingSeconds)-1>s
  --property=RuntimeMaxSec=<floor(operationRemainingSeconds)+6>s
  --property=MemoryMax=4294967296 --property=MemorySwapMax=0
  --property=CPUQuota=200% --property=TimeoutStopSec=1
  --property=KillMode=control-group --property=FinalKillSignal=SIGKILL
  --property=SendSIGKILL=yes --property=UMask=0077
  --property=StandardOutput=append:/home/charl/.local/state/moriarty/sp05-preview-loan-20260910-01/sdk.stdout
  --property=StandardError=append:/home/charl/.local/state/moriarty/sp05-preview-loan-20260910-01/sdk.stderr
  --working-directory=/home/charl/Moriarty/.worktrees/sp05-preview-owner
  /usr/local/bin/node
  /home/charl/Moriarty/.worktrees/sp05-preview-owner/experiments/moriarty-midnight-financial/ledger/preview-bootstrap.mjs
  --run --plan /home/charl/.local/state/moriarty/sp05-preview-loan-20260910-01/plan.json
  --sha256 <exact-admitted-plan-sha256>
```

Require the start allowance to be at least1s. Fsync the final resolved argv, then
require remaining setup time to be at least that allowance before dispatch.
Pass the remaining setup time to the caller's launch timeout. Confirm actual
manager `ActiveEnterTimestampMonotonic` is inside the setup bound. A successful
`systemd-run` return alone does not establish timely activation. Never launch
anything after shutdown begins.

The independent shutdown retains the existing bounded sequence: at most5s to
kill every process in `moriarty-sp05-preview-loan-01.service`, at most20s for
`docker stop -t 5 moriarty-midnight-proof-server`, and at most5s for the Docker
kill fallback. Include the fallback in the timer service's unconditional stop
path. Allow1s timer accuracy: planned containment is no later than arming+2531s
inside the2550s envelope. Do not cancel the timer until actual unit terminal
state, cgroup absence and proof-server exit are recorded; verify local node and
indexer are still stopped. Partial containment remains UNKNOWN.

## Evidence and stopping conditions

The public result writer retains native payloads before submission, then closed
stage files and the final integration result. Post every actual public candidate
transaction ID in the conversation, including unknown or failed status. SDK IDs
can have64 or66 hex characters; transaction hashes have64. Missing final output
does not license reconstruction or replay. Retain all partial files and charges.

On a deadline, cap, failed comparison, uncertain submission/finality, failed
wallet persistence or ambiguous cleanup: stop, preserve the allocation and
original evidence, and report UNKNOWN where the actual status is unknown.
Do not reset wallet state, reuse a reservation, regenerate capability secrets,
retry a stage or extend the deadline. SDK internal retries consume this same
allocation. The bootstrap's durable private backup and pending marker must remain
available after interruption; no automatic marker removal or promotion repair.

Four matching actual Preview stages, full native asset effects and canonical
readbacks are the evidence target. Independent result audits determine its exact
scope. Keep the producer's false acceptance flags and any FAILED/INCOMPLETE
cleanup result unchanged; outer containment is separate evidence. This loan
cannot close the swap, mandatory PCD, SP09, SP11 or SP12 gates.

Prior local reservations remain11 submissions and3,300,000,000,000,011 SPECK;
their prior admitted ceiling remains4,000,000,000,000,010. The new proposal adds
at most4 submissions and2,000,000,000,000,000 SPECK. Reserved amounts are not
measured paid fees. Earlier Preview, review, build, proof and failed-probe charges
remain consumed even where this packet has no complete numeric total.

## Safe draft checks

Parse the JSON, verify public provenance hashes and arithmetic, and call only
the pure `validatePreviewLaunchPlan` boundary with an in-memory future-deadline
copy. Keep the stored zero deadline and all other plan bytes unchanged. Never
import or execute `preview-bootstrap.mjs` or an execute-once script for these
checks. Controlled validation is neither admission nor Preview execution.

Run the non-operative checker from the checkout root:

```sh
python3 deliverables/sp05-financial-integration-2026-09-09/preview-loan-01/check-static.py
```

The executor exclusively creates both raw SDK diagnostic files with mode0600,
fsyncs them and their owner0700 directory, then directs the manager to append.
These files stay private. Publish only reviewed hashes or sanitized excerpts.
The shared time/CPU limits bound the process; no numeric log-byte ceiling is
claimed. The free-disk check provides headroom, not a quota. A global file-size
limit would also affect native payload, wallet and LevelDB persistence, so this
packet does not apply one or truncate logs automatically.

Four logical financial stages and four expected wallet finalizations describe
the intended sequence. The SDK proof provider has no separate aggregate callback
counter; internal proof requests consume the same wall/CPU/memory allocation.
The actual provider submission, cumulative DUST and gross-asset caps remain
enforced at4 submissions,2×10¹⁵ SPECK and20,000,000,000 USD test units.
