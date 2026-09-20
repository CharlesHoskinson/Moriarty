---
title: "Paper tutorial — conditional exchange with recovery"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: tutorial
tags: [moriarty, consolidation, recursion]
---

# Paper tutorial — conditional exchange with recovery

This is a design exercise, not a working program.

1. Sign an intention to trade 10 A for at least 20 B, with fees at most 1 A within an 11-A gross cap. Bind assets, domains, recipients, partial-fill rounding, evidence and recovery.
2. Fund escrow only under the applicable consent. Record the pending duty separately from delivery.
3. Require recipient acceptance and an authenticated document predicate. A digest alone establishes no document truth.
4. Commit an authorized partial fill and retain the remainder, cumulative fees, work and authority.
5. Race a late authenticated result with recovery. Timeout is not nonexecution; only the allowed terminal outcome consumes the entitlement.
6. Test a second permitted candidate and hostile changes to recipient, fees, predecessor, key, hidden reservation and residual debt.
7. Require native ZKIR correspondence and actual ledger effects before claiming completion. The optional federation is not necessary for direct authoring or submission.

[Design](explanation.md) · [Index](index.md).

