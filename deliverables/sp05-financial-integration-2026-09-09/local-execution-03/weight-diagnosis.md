# Local execution 03: block weight diagnosis

Read-only source assessment, 2026-09-10. No service start, transaction, proof, compile, runtime change, chain reset, or source acquisition was performed for this assessment.

## Established observations

The native deployment transaction `7f567d8759d8289016040278b39f92a1912396f56416e469e5ae10a548081d01` was validated for the node mempool and remained pending when the bounded run stopped. Finality remains unknown and the reservation remains retained. The resource containment stopped the run after 419.245 seconds.

`transaction-dispatch-weight.json` gives ledger cost 419,720,000,001 plus configured dispatch overhead 20,000,000,000 = **439,720,000,001 refTime**. This is below the observed normal maximum extrinsic weight 1,299,891,843,000 and normal total 1,500,000,000,000. The earlier oversized-transaction hypothesis is rejected by these measurements. Native payload length is 11,199 bytes.

`block-weight-usage.json` records finalized block 20286, hash `0xb578703e182d95497ca14cd9ae99ac5b8c73501acb73d7b6bc1c77375b4f8051`, with mandatory refTime 2,021,961,378,686 against maximum 2,000,000,000,000, and proofSize `0xfffffffffe1ca42e` (u64::MAX minus 31,677,393). Normal and operational consumption are zero. Four inherent success events sum to only 857,720,325 refTime. `node-block-weight-observation.json` independently records repeated `HitBlockWeightLimit` with four extrinsics and renewed mempool validation.

The final mandatory counter alone does not prove pre-extrinsic exhaustion: FRAME also registers idle-hook weight in that class. The retained artifact contains only ExtrinsicSuccess events, so absence of session events in it is not evidence that no rotation occurred.

## Concrete pre-extrinsic mechanism found in local release source

Repository `/home/charl/midnight/midnight-node`, existing public `node-1.0.0` commit **f4a4cd543e6fbdfa34e11671737598f5b40c19fd** (paths and line numbers below refer to `git show` at that commit):

- `partner-chains/substrate-extensions/partner-chains-session/src/lib.rs:371–379`: session **on_initialize** calls `rotate_session()` when `ShouldEndSession` is true and returns **the entire `T::BlockWeights::get().max_block`**, otherwise zero. This is an initialization reservation, not idle consumption or measured execution time.
- `runtime/src/lib.rs:526–534,934–936`: the live-shaped runtime Session pallet is this partner-chains session implementation, configured with `ValidatorManagementSessionManager`.
- `runtime/src/session_manager.rs:91–96`: rotation condition is `current_epoch_number > current_committee_storage().epoch && next_committee().is_some()`.
- `partner-chains/toolkit/committee-selection/pallet/src/lib.rs:217–235,413–424`: each required committee inherent selects for current committee epoch + 1; rotation takes NextCommittee and promotes it to CurrentCommittee. A committee lag can therefore require one rotation per produced block until epochs catch up, while each rotation consumes the entire block budget.
- `partner-chains/toolkit/sidechain/pallet/src/lib.rs:172–181,207–222`: current epoch derives from current slot and configured slots per epoch. It can advance across service downtime independently of produced blocks.
- `runtime/src/lib.rs:308–321`: maximum block Weight is `(2 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX)`, with 75 percent normal ratio. This explains the full refTime and near-u64::MAX proof-size signature if the rotation hook ran.
- `pallets/midnight/src/lib.rs:315–328,616–620`: Midnight initializes using ConfigurableOnInitializeWeight; transaction dispatch weight separately equals ledger cost plus ConfigurableTransactionSizeWeight. Neither should be conflated with the session reservation.

**Supported diagnosis:** `session-rotation-log.json`, captured from the stopped container, supplies the missing direct evidence. At 04:41:54 the node rotated to committee epoch 993885 and started session 137 in sidechain epoch 993897; at 04:42:00 it rotated to 993886/session 138; at 04:42:06 it rotated to 993887/session 139, still in epoch 993897, then scheduled committee 993888. The node was catching up one committee epoch every six-second block. These live session records, together with the release hook, support pre-extrinsic session reservation as the cause. This conclusion does not rely solely on final BlockWeight.

At the last observed rotation the backlog was **993897 - 993887 = 10 epochs**. If sidechain epoch stayed 993897 and block cadence stayed six seconds, ten more rotation blocks would reach the current epoch after approximately 60 seconds; the following non-rotation block could admit normal work, approximately 66 seconds after 04:42:06. This is a conditional estimate, not an observed inclusion or a promise after a later restart. New elapsed epochs, missed slots, or committee availability can alter it. The early stop was conservative but likely preceded natural recovery by about a minute. The evidence supports finite catch-up, not a permanent runtime defect.

The release already stamps the current committee epoch at block 1 (`committee-selection/pallet/src/lib.rs:198–203`), so a genesis-zero defect is not established. Newer local node HEAD `706fc087fb255cab1588ef05a9dd9e07ec41579a`, same file lines 230–252, also documents preventing rotation on every block by stamping current/queued epochs at genesis; that newer change is context, not proof of this image's defect.

## Accounting and idle qualification

Existing local bare Polkadot SDK repository `/home/charl/.cargo/git/db/polkadot-sdk-dee0edd6eefa0594`, commit **660acefe66599a3e54363797007befcb01bd610b**, provides corroborating FRAME accounting:

- `substrate/frame/executive/src/lib.rs:532–542,645–665`: initialization weights are registered as Mandatory before inherents.
- Same file `809–822`: idle receives only `max_block.saturating_sub(total)` and its returned weight is also registered Mandatory. An already full initialization reservation leaves zero refTime for idle.
- `substrate/frame/system/src/extensions/check_weight.rs:217–228,494–529`: total block weight participates in admission; an explicit test reserves full Mandatory weight during initialization and rejects Normal while allowing separately reserved Operational work.
- `substrate/frame/system/src/lib.rs:2467–2483`: post-dispatch refunds reduce the class counter. Saturating full proof-size reservation followed by refunds is consistent with the observed MAX-minus-31,677,393 signature. Exact refund reconstruction was not done and this signature is not proof of integer corruption.

This cached SDK commit is **not** the public release Cargo.lock pin, which is `2e4dd0bc22366a5af820492528869a493b5a5208` (not locally available). Thus its executive/refund details corroborate a mechanism, not exact image execution. The decisive session hook itself is present directly in the local node release source.

## Smallest evidence-based next step and remedy

The retained stopped-container session logs now confirm shrinking committee lag. The smallest remedy is to allow ordinary, bounded node catch-up before financial execution; no node runtime repair, chain reset, larger transaction allowance, or altered validation is supported by this evidence. A future readiness check should observe current committee epoch catching up and a subsequent non-rotation block before wallet balance/proof. Recheck after wall-clock gaps, because another epoch boundary can start a legitimate rotation.

For a separately admitted future read-only service window, exact public methods are `api.call.sessionValidatorManagementApi.getCurrentCommittee()`, `getNextCommittee()`, and `api.call.slotApi.slotConfig()` (release runtime lib.rs:1578–1600), plus `api.query.sidechain.epochNumber()`, timestamp and full system events at the same block. Confirm available methods against that runtime metadata and pin all reads to the same hash. Capture the current-slot epoch as well when predicting the next block, because a stopped chain's stored epoch can itself lag wall time.

For the already submitted transaction, only a separately reviewed continuation resource vote should authorize a future service window. Observe the retained hash and reconcile its finality/reservation first. Restart may have lost the in-memory pool; do not infer that the transaction remains pending, auto-resubmit it, or allocate a new financial attempt. Derive a fresh catch-up bound from then-current epoch lag, slot duration and epoch duration, and inspect native validity windows before considering any further transaction action. A node-only catch-up window with condition-based stop preserves consensus checks and existing chain state.

If the backlog cannot converge within an acceptable explicit bound, the diagnosis does **not** authorize an arbitrary epoch write, weight reduction, larger block limit, or chain reset. A reviewed state-preserving runtime correction would need to preserve authority-selection/session semantics. Increasing max_block alone will not solve a hook that reserves whichever max_block is configured.

A minimal Moriarty prevention change would reject execution readiness while repeated session-rotation/budget-exhaustion evidence persists, before wallet balance/proof. A final System.BlockWeight snapshot alone is insufficient for that gate; use identified initialization/session evidence. This gate avoids another doomed submission but does not repair the node.

## Source binding and result limit

Observed node image digest: `ede01da35e982b6a4b85461ad8492ae2753ef14246fba33c8039b782aa8e39fb`; image git label `c48905267920d5599e849e8e043f801c6baab94a`. That labeled commit is unavailable in existing local source (a prior bounded fetch failed). Public release commit f4a4cd543e6fbdfa34e11671737598f5b40c19fd must not be represented as the image's exact source. Runtime metadata and observed transaction cost bind the quantitative counterevidence; the live session evidence now strengthens the causal diagnosis, while exact image-source binding remains unresolved.

Confidence: high that per-transaction normal weight is not the observed blocker; high that release source contains a full-block initialization reservation on session rotation; strong source-and-log support that finite epoch catch-up caused this attempt's block starvation, subject to the image-source binding qualification. No product acceptance or transaction finality is claimed.
