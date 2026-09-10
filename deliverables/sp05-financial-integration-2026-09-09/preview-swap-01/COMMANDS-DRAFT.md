# First Preview swap: proposed single execution

**Not admitted.** No service, wallet, proof or transaction was started to prepare
this packet. The stored public plan has deadline0 and fails production validation.
Root must bind the final source, both independent source/resource votes and the
prior loan result reviews before creating an exclusive execution admission.

Use the existing `preview-bootstrap.mjs --run --plan <absolute-path> --sha256
<digest>` entry point. This packet adapts the reviewed loan executor only for a
fresh swap allocation, private directory, build and native asset allowance. It
does not change the frozen bootstrap or either previous loan candidate/result.

The intended sequence is `deploy → initialize → swap → close`, using reviewed
swap receipt `3789da217a36cecd5f7603cbbaead32671418da36ecc5c2d20b1709b9577f362`
and source manifest `a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6`.
No compilation is needed. The fixed wrapper mints1,000,000 A and2,000,000 B to
the contract, plus100,000 A to the original trader. The swap spends10,000 A for
19,743 B, then close transfers1,010,000 A and1,980,257 B to the existing provider.

The native wallet input is100,000 A, with90,000 A change. Gross debit therefore
requires **ASSET_A100000, ASSET_B0**. It cannot use a net10,000-A allowance. After
initialization the production path waits for the exact native mint to appear as
a spendable original-wallet output before swap. No second wallet or pre-funded
external A/B asset is introduced; the new contract determines the asset colors.

Use the original Preview seed, address, latest three saved child wallets, role
file and password paths recorded in `public-plan-draft.json`. Metadata includes
the owner0700 parent `/home/charl/.local/state/moriarty`, both wallet-cache parents,
all current snapshots and the original loan backup. Do not restore older backup
bytes. Keep the loan backup intact; the swap has a distinct backup/allocation.
No private content was inspected to establish these metadata facts.

## Exact bounds and one-shot command

The new base `/home/charl/.local/state/moriarty/sp05-preview-swap-20260910-01`
must be absent. The executor creates it exclusively as0700 after admission and
timer activation; bootstrap creates its fresh `run` and `contract-state` children
after public checks. The executor creates private `sdk.stdout` and `sdk.stderr`
exclusively as0600 and fsyncs them before systemd launch. Raw diagnostics stay
private; publish only reviewed hashes or sanitized excerpts.

The cap is4 submissions,2,000,000,000,000,000 SPECK and the gross assets above.
Four logical stages/finalizations are expected; the SDK has no independent
aggregate proof-callback count. Internal retries consume the same wall/CPU/memory
allocation and financial caps. No application retry or second attempt is allowed.

Reuse the reviewed2550s envelope: activation/setup by arming+120s; all bootstrap
preflight, restore/sync, proof, submission, finalized readback and comparison work
shares arming+2490s. Latest activation leaves2370s. Bootstrap hard exit is at most
6s beyond its deadline. Independent shutdown starts at arming+2500s, with at most
31s cleanup including timer accuracy, inside2550s. The loan's four-stage run is
a supporting time observation, not a guarantee about this swap's runtime.

Only `moriarty-midnight-proof-server` may start. Preserve its existing image,
loopback16300 port,5GiB memory/10GiB total memory+swap,4CPU and no-restart limits.
Local node/indexer must remain stopped. Launcher limits remain4GiB RAM, no swap,
2CPU, control-group kill and SIGKILL fallback. Free-disk4GiB and host memory9GiB
are preconditions, not aggregate quotas. There is no numeric log-byte cap: a
global file-size limit would also affect wallet/native/LevelDB persistence. On
disk failure stop and preserve logs, partial state and reservations.

Once the exact packet is reviewed and an admission exists, the single command
from `/home/charl/Moriarty/.worktrees/sp05-preview-owner` is:

```sh
python3 deliverables/sp05-financial-integration-2026-09-09/preview-swap-01/execute-once.py
```

Do not run it for checks. It verifies every admitted source/resource pin and two
actual GPT-6/Grok source and resource receipts. Its existing vote loop also
requires `admission.priorLoanResultReviews`, two faithful projections with
`candidateSha256: 07921dddc628268569e963afae35cfd45be5a7b0c6d814b7b5995eace5d36974`,
an accepted scoped verdict and a bound `actualReview`. Missing reviews cannot
be inferred from four stage PASS records or an author statement.

The executor durably consumes the allocation before arming
`moriarty-sp05-preview-swap-stop-01.timer`; only then can it start the prover.
The manager's observed timer epoch fixes the operation deadline. It fsyncs the
resolved launch argv, uses `floor(remainingSetup)-1` for `TimeoutStartSec`, and
rechecks the recording reserve before dispatch. Observed activation must fit the
setup window; no late or deferred launch is allowed.

The independent timer kills all processes in
`moriarty-sp05-preview-swap-01.service` within a5s command allowance, stops only
the proof server within20s, and retains an unconditional5s Docker kill fallback.
It is never canceled by the executor. Root must observe unit terminal state,
cgroup absence, proof-server exit and local node/indexer still stopped before
canceling it. Timer launch attempted is separate from observed timer active.

## Evidence and stop conditions

Retain original native candidate payloads before submission and post every
actual public transaction ID and observed status in the conversation. Require
canonical node/indexer finalized agreement and protocol1000000, full decoded
fields, participant deltas, native balances and the independent fixed comparator
at all four stages. IDs may have64 or66 hex characters; hashes have64.

On failed comparison, unknown submission/finality, missing exact initialized
wallet mint, stale source, persistence marker, resource/deadline exhaustion or
uncertain cleanup: stop, preserve original evidence, and report UNKNOWN where
the status is unknown. No automatic retry, rollback, new wallet, deadline
extension, reservation reuse, log truncation or missing-result reconstruction.
The actual loan wrapper FAILED/INCOMPLETE and separate outer containment remain
unchanged; the same SDK disposal limitation may affect this swap's wrapper.
Only independent actual result audits can establish the scoped financial slice.

Prior local reservations remain11 submissions/3,300,000,000,000,011 SPECK. The
Preview loan added4/1,200,000,000,000,004: together15/4,500,000,000,000,015. This
proposal adds at most4/2,000,000,000,000,000, yielding at most19 reserved
submissions and6,500,000,000,000,015 SPECK for these listed allocations. Their
additive admitted ceiling is8,000,000,000,000,010 SPECK. Reservations are not paid
fees. All earlier Preview, review, build, proof, runtime and failed-preflight
charges remain consumed; no global total or refund is implied.

## Safe source checks

```sh
python3 deliverables/sp05-financial-integration-2026-09-09/preview-swap-01/check-static.py
```

The checker parses the executor AST and runs only extracted guards with inert
dependencies. Pure plan validation can use an in-memory future-deadline copy;
leave the stored deadline0 unchanged. Never import or execute the operative
executor or bootstrap for these checks. No test or packet vote closes mandatory
PCD, SP09, SP11 or SP12 acceptance.
