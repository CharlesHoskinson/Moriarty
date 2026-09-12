# Loan attempts and pending-spend lineage inventory

Read-only inventory,2026-09-11. No network, private snapshot/seed contents, wallet connection or mutation. Exact source hashes, stage IDs, finality anchors and current filesystem metadata are in `loan-attempt-and-pending-inventory.json`. This is factual preparation, not acceptance voting.

## Two identified real Preview loan cases

| Ordinal / allocation | Actual result / historical finality | Consumption and persistence |
| --- | --- | --- |
| 1: `sp05-preview-loan-20260910-01` | `FAILED`; four stages with historical finalized height807357 | attemptCONSUMED; admission hash matches;4reserved submissions/1200000000000004SPECK; active reservationnull; persistence event{'status': 'PERSISTED', 'children': 3}; cleanup walletStoppedtrue/pendingOperations0 |
| 2: `sp05-preview-loan-exit-20260910-01` | `FINANCIAL_COMPLETE`; four stages with historical finalized height808148 | attemptCONSUMED; admission hash matches;4reserved submissions/1200000000000004SPECK; active reservationnull; persistence event{'status': 'PERSISTED', 'children': 3}; cleanup walletStoppedtrue/pendingOperations0 |

The exact stage sequence for both is **deploy → initialize → accrue → settle**. There is no separate fund stage. Initialize mints the fixed test asset; settle consumes the borrower input and pays lender/change. The first actual command remains FAILED/INCOMPLETE despite reviewed financial comparisons. The second returns FINANCIAL_COMPLETE but strict raw process exit0 was not retained while loaded. Neither defect refunds its consumed case. A newly admitted successor is case ordinal3.

Both `execution-admission.json` files and their matching `attempt.json` CONSUMED records are retained under the corresponding `preview-loan-01`/`preview-loan-exit-01` directories. Their actual-run integration records bind native payload hashes, transaction identifier pairs, canonical block hashes/heights and finalized-head fields. Exact-stage comparison records plus the GPT-6 actual-result reviews and separate retained15-request historical RPC readbacks establish the recorded scoped observation lineage. This inventory does not rerun decoding, query finality or convert false acceptance flags to true.

## Shared-wallet persistence and pending checks

The four real public allocations use the retained Preview wallet in order loan01 → swap01 → loan-exit01 → swap-exit01. Later swap activity must be included in a funding observer precondition even though it is not an additional loan attempt. The latest reviewed swap-exit allocation has FINANCIAL_COMPLETE, recorded walletStoppedtrue/pendingOperations0, null active reservation, persistence event and separate outer-containment approval. Its partial cumulative count27 and reserved8100000000000027SPECK must not be called a global total.

Current inspection read only lstat metadata for the three snapshot files, pending-marker existence and four named backup directories; JSON records the results. Historical `actual-run/005.json` PERSISTED event and metadata observations support the producer-reported persistence lineage. They do not independently prove private snapshot contents or future restorability. Absence of the pending marker proves only that filesystem fact. SDK dust restore resets pendingDust toempty, so restored pendingCoins cannot replace reservation/submission reconciliation.

## Exclusions and remaining uncertainty

Collector author01–06 attempts, review timeouts, source tests, source candidate approvals and `sp05-loan-exit-retention-grok-2026-09-10/author06-user-stop.json` describe implementation processes. They are not additional public loan case invocations. The later `local-command-execution-proposal-01/loan/attempt.json` is a LOCAL loan and does not increment Preview loan count; its existence prevents treating older cumulative resource subsets as a complete global account. Compiler full-build loan-attempt is a compiler attempt, not a public loan.

Exactly two identified real Preview loan allocations are supported by this inspected lineage. No exhaustive host-wide or external-wallet transaction coverage is claimed. Before a funding observer/new public action, bind this lineage, both intervening swaps, any later account use, current reservation/persistence state and exclusive ownership to a fresh bounded read-only observation. Unknown/untracked account use or pending finality must remain UNKNOWN and block spendability claims. Do not increment the case count for source work, or infer a third actual public attempt solely from an intended successor name.
