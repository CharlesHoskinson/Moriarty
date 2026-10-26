---
title: "Lessons: Sherlock recursion chain and deployment"
project: lessons
date: 2026-10-24
tags: [lessons, sherlock, recursion, stagenet, verifying-key]
---

# Sherlock recursion chain and deployment


## Ledger verification needs its proof-verifying feature, or bad proofs pass (2026-10-24)
- **What happened:** A6's negative case, a reused proof bound to the wrong statement, was accepted by the local ledger-9 check. In the pinned `ledger-wasm`, `default = []`, and ledger has `default-features = false`. Without the `proof-verifying` feature, `ProofMarker::proof_verify` returns `Ok` without checking anything.
- **Lesson:** a validation harness must prove it can reject (mutation tests) before its accepts count. Check that the dependency features actually turn verification on.

## Redeploying is fast once the key lineage is recorded (2026-10-24)
- A4 passed on a new stagenet contract, `ac93b80e…` (block 1013319). Its key was regenerated from scratch, matched `3f3b7e13…ff0083` exactly, and was read back through the indexer and a Rust ledger reader. The full lineage (child VKs → parent `801467c8…` → template → decider → contract) is in the record. The old contract was `80a0291c…`.
- B3's own cohort (one 16-tx/50-header leaf → single-child parent → decider) is **not** the A-chain cohort (A102/A80 → A3). Its parent fits k19. Don't splice timings from different cohorts.

## A verifying key fingerprints the whole upstream chain (2026-10-24)
- **What happened:** A1–A3 passed. A2 gives children A102 at k15 and A80 at k12, each a 3,440-byte proof. A3 gives a k20 parent of 6,432 bytes. A4 rebuilt the
  terminal decider key, `cdf36971…`, and it did not match the key installed on contract `80a0291c…` (`bd9e9d89…`).
- **Why:** `sherlock_decider_relation(parent_vk, s_g2, template)` bakes in the parent's VK. Porting the parent into a new crate changed
  its VK (`801467c8…`), and that changes the decider's key.
- **Lesson:** any change to a lower layer forces a new terminal key and a redeploy. Record the full key lineage (child VKs → parent VK
  → template → decider VK → contract) with every deployment, so a mismatch can be traced in minutes instead of by searching history.
- **Owner direction:** while the design is unfrozen, redeploy rather than reproduce old keys.

## The fast inner loop works when retries are cheap and honest (2026-10-24)
- The previous approach (five days without converging) used review gates, consumed attempts, and failure traces that nobody opened. The factory has
  no review gate in the inner loop, a measured budget, a retry that needs a hypothesis, and traces read before retrying. With it, A2 and A3 passed within hours.
- Stock and Loki both pass A2/A3 with the `[patch.crates-io]` drop-in: same proof sizes, accepted.

## Pinned external sources must exist on the host (2026-10-24)
- B1 failed first because its pinned design sources (`~/research/fenrir-rollup-design-2026-10-19`) were missing, and projectfenrir lives at
  `~/repos/projectfenrir`. Checks that depend on external corpora should fail with a clear "missing source" error, not a test failure.

## See also

- [[doctorwatson-workflow-review-2026-10-25]]: 2026-10-25 retrospective (orphaned locks, idle agents, DW121–DW130)
- [[doctorwatson-workflow-review-2026-10-22]] and [[doctorwatson-improvement-log]]
- Lessons index: [[lessons/README|lessons]]
