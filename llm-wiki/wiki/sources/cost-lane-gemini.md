---
title: "Cost round, Gemini lane: ratio decomposition, levers and GPU multicore scaling"
type: source
tags: [lane-report, cost, compact, gpu, multicore, folding]
sources: [cost-lane-gemini]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-cost-optimization-20261111/proposals/gemini.md
source_url: local
---

# Cost round, Gemini lane

## Summary

The Gemini 4 Carbon high answer to [[cost-round-brief]]. No build, simulation or proof was run. The lane splits the step count into two regimes: Regime U (no crypto builtins, software SHA-256 and Poseidon over 128-bit ALU operations) and Regime B (hashes as single primitive calls, 12 steps per ZKIR operation). In Regime B the `spend` ratio is 8,318 times in rows and 2,946 times in CPU time (ESTIMATE); in Regime U it is about 6 x 10^6 times. It names three "cliffs": missing crypto primitives, the node per step, and the unbatched decider floor. It claims stacks that reach within 10x and within 2x, the 2x ones resting on UNREAD folding papers or on a seal change. A proposal, not merged.

## Key Claims

- Regime U: SHA-256 about 45,000 machine steps per block and `transient_hash` about 60,000 steps (ESTIMATE); `spend` about 2.46 million steps.
- Regime B: `set` 252 steps, `vote$commit` 1,584, `spend` 3,288 (12 steps per ZKIR op, ESTIMATE); node rows 92.4 to 97.9 percent of Sherlock rows.
- Native rows track SHA-256 blocks at about 1,944 rows per block (derived from measured rows over counted blocks: 2, 8 and 12 blocks).
- Native CPU prove time ESTIMATE: 2.5 s (k = 13), 3.3 s (k = 14), 4.8 s (k = 15), scaled from a measured 2.55 s k = 13 ZKIR leaf.
- Decider floor: one unbatched decider is 38.7 to 41.6 times native `spend` rows; the lane cites 929,166 decider rows as a measured spike.
- Levers (ESTIMATE unless marked): L1b lookup-based spread SHA-256 on the shared L-6 table at about 240 rows per block, 8.1 times fewer than the upstream chip, +12,008 node rows; L2b straight-line `R_SL_*` rules with a fused Merkle step, `spend` 3,288 to 104 steps; L3 S = 128 to 512, `spend` in one k = 16 leaf; L4a single-collapse accumulation, removing one collapse (-239,577 rows) and taking the stand-in node to k = 19, plus a 4-leaf node; L4b ProtoGalaxy folding of homogeneous leaves (verifier counts UNREAD); L4c a zero-node one-leaf seal inside the DECIDER; L7a batching under one decider.
- Hasse's bound: no curve over the BLS12-381 scalar field has the base-field order, so Nova or CycleFold 2-cycle folding does not exist natively over BLS12-381.
- T-10 stack (L1b, L2b, L3, L4a with L4c, batch of 8): 209,924 rows per `spend`, 8.74 times native; unbatched 8.1 times in CPU wall time (ESTIMATE).
- T-2A (batch of 64 with L4b folding): 36,327 rows per `spend`, 1.51 times native (ESTIMATE, UNREAD verifier constants); without L4b 3.76 times. T-2B (unbatched): 42,880 rows, 1.78 times, needs either a future ledger native accumulator check or a second universal entry `acceptDirectLeaf`, which needs an OWNER exception to GP-001 proof-size uniformity.
- Native proof through GP-3 rejected for single calls (57.6 to 79.2 times native); kept for composing already-proved foreign proofs. Per-entry native keys are a finding, not a design.
- Multicore leaf: cores stacked vertically over the existing 12 advice columns (C = 8, `S_core` = 32, S = 256, k = 16); horizontal column cores add +279,080 node rows at C = 2. C is one fixed `LeafShape` constant.
- GPU (ESTIMATE): with quotient and lookup kernels a k = 20 node from 65 s to about 6 s; solo `spend` 4.75 s on this host with every lever; 10^6 steps 2.88 h; 10^9 steps 120 days on one host, 13.4 h on 64 GPUs.
- Wins over native: unbounded loops, pay-for-taken-branch, composition, zero key ceremony, and SHA-heavy batches with L1b.
- Papers: ten listed, all UNREAD.

## Key Quotes

- "Reaching within 10x or within 2x of native proving is mathematically impossible unless the decider is either amortized across a batch of calls or bypassed/shrunk."

## Connections

[[cost-round-brief]], [[n1-native-circuits]], [[cost-lane-fable]], [[cost-lane-astra]], [[IvcNode]], [[RunDecider]], [[MidnightLedger9]], [[GeneralPurposeRecursion]].

## Contradictions

- Native prove time: the lead later measured `spend` at 1.755 s, `vote$commit` at 1.022 s and `tiny` `set` at 0.552 s ([[n1b-native-proving-time]]), below this lane's 4.8 s, 3.3 s and 2.5 s estimates.
- Steps (3,288 against 6,000 to 12,000), instruction count (274 against 273), native time (4.8 s against 1 to 2 s), column-core node cost (279,080 against about 88,000) and the k = 19 node: see [[cost-lane-fable]].
- 929,166 decider rows is called a measured spike here; [[RunDecider]] records it as a `cost_model` figure on an unsafe SRS with no proof.
- L4c and `acceptDirectLeaf` conflict with the one seal path and one contract of [[council-revisions]] (P-3, P-8), which this lane did not read.
- The 2x claims stand against [[cost-lane-astra]] (universal 2x is research) and [[cost-lane-fable]] (2x not shown reachable under one key set).
