---
id: moriarty.midnight.readiness-lessons
title: Local Midnight readiness lessons
type: runtime
status: active
created: 2026-09-10
updated: 2026-09-10
updated_at: 2026-09-10T06:23:18Z
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


**Validate storage inputs before submission.** The later [local deployment](../deliverables/sp05-financial-integration-2026-09-09/local-execution-04/RESULT.md) was indexed successfully, but no financial stage completed. Offline validation of the retained hexadecimal password failed the installed SDK character-class policy. Inspected provider ordering and a synthetic reproduction explain why metadata can exist without private value or signing key; the sanitized original exception does not independently prove that causal explanation. The launcher now calls the pinned installed validator before wallet work. Successful deployment followed by a local persistence failure must preserve its address, native bytes and charge.

**Exercise the provider mechanism during recovery review.** A fresh GPT-6 reviewer [reproduced a LevelDB lock collision](../deliverables/sp05-financial-integration-2026-09-09/deployment-recovery-01/source-review-gpt6-level-lock-reproduction-01.json): simultaneous private-state and signing-key reads opened separate handles on one database. Sequential reads succeeded. This is a bounded synthetic experiment against the pinned actual provider, not inspection of the original database. The [corrected source](../experiments/moriarty-midnight-financial/ledger/recover-deployment.mjs) serializes both readbacks before the driver; the original rejected candidate and review remain available.

**Recover identity before continuing financial behavior.** [Recovery design02](../deliverables/sp05-financial-integration-2026-09-09/deployment-recovery-01/recovery-design-02.md) requires native and canonical-finality checks, exact historical/current state, original constructor/key identity, metadata-only inspection of a copy, strict synchronized wallet inputs, and isolated private restore/readback. The complete trace must contain the original deployment plus initialize/accrue/settle, with new calls charged separately. Constructor reconstruction and 281 passing source tests do not establish live recovery. Current recovery source has GPT-6 approval; Grok and subsequent resource/runtime gates remain pending as of this note.

These are provisional implementation lessons linked to observed failures and inspected code. No source or claim ledger assessment is promoted. See [[wiki/index|the research index]], [the roadmap](../ROADMAP.md) and [orchestration stop rules](../docs/FOOTGUNS.md).
