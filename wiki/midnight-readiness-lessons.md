---
id: moriarty.midnight.readiness-lessons
title: Local Midnight readiness lessons
type: runtime
status: active
created: 2026-09-10
updated: 2026-09-10
updated_at: 2026-09-10T04:45:00Z
tags:
  - moriarty
  - midnight
  - execution
sources: []
---

# Local Midnight readiness lessons

The actual local loan deployment reached node submission and was rejected with `OutOfDustValidityWindow`. Its native DUST creation time was about 39 hours old despite current transaction TTLs. [The retained attempt](../deliverables/sp05-financial-integration-2026-09-09/local-execution-02/RESULT.md) binds public bytes, node rejection, reservations and independently observed process containment. This is an experiment observation, not Preview settlement or financial acceptance.

**Check the data that determines the transaction.** Historical receipt/finality checks and wallet synchronization did not establish current indexed-block age. The [inspected installed SDK](../raw/sp05-error171-source-2026-09-10/receipt.json) queries the latest indexed block and uses its timestamp for DUST balancing. A stale response after restart is the leading explanation; the actual response was not captured, so its exact origin remains an inference. Preserve that distinction.

**Check the actual native output too.** The SDK queries again after an initial readiness check. The [source repair](../experiments/moriarty-midnight-financial/ledger/local-tip.mjs) waits for recent canonical indexed data, then checks returned native DUST time before signing and before submission. These conservative local thresholds do not prove consensus validity. A failed check stops the wallet operation; it does not reset snapshots or retry a transaction.

**Deadlines apply at mutation boundaries.** The independent reviewer reproduced a case where durable readiness retention crossed the deadline before balance began. [The correction evidence](../deliverables/sp05-financial-integration-2026-09-09/launch-source-04/deadline-red.txt) motivates an immediate deadline recheck after retention. The original review remains recorded. Source review does not authorize a new network attempt.

**Allow reviewers enough time.** Completed Grok reviews took about 108, 300 and 330 seconds. [The timeout record](../docs/GROK-TIMEOUTS.md) distinguishes caller termination from provider failure and retains unknown cancellation/usage. A quiet interval alone is not a dead process. Keep an adequate finite allowance and record terminal output; do not spend the product milestone on another orchestration framework.

These are provisional implementation lessons linked to observed failures and inspected code. No source or claim ledger assessment is promoted. See [[wiki/index|the research index]], [the roadmap](../ROADMAP.md) and [orchestration stop rules](../docs/FOOTGUNS.md).
