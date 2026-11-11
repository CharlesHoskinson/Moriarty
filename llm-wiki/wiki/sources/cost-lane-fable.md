---
title: "Cost round, Fable lane: making Sherlock competitive with native proving"
type: source
tags: [lane-report, cost, compact, gpu, multicore, ledger9]
sources: [cost-lane-fable]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-cost-optimization-20261111/proposals/fable.md
source_url: local
---

# Cost round, Fable lane

## Summary

The Claude Fable 5.2 answer to [[cost-round-brief]]. Nothing was built, proved or measured in the lane. With the measured native sizes ([[n1-native-circuits]]) and the unmeasured step estimate, the Sherlock to native ratio for `zerocash` `spend` is 10,400 to 31,300 times in rows and 11,500 to 50,000 times in prover time (ESTIMATE), so "1,000 to 10,000 times" was low. The best universal design reaches 4 to 8 times for one call, and only if ledger 9 verifies a key with k > 14. Within 2x is not shown reachable under one key set. A proposal, not merged.

## Key Claims

- Fixed floor: every run pays one leaf, one node step (61 to 67 s, k = 20) and the decider (70 s, k = 20), about 135 s and 1.7 to 2.0 million rows; that alone is 70 to 85 times native `spend` in rows and 65 to 140 times in time (ESTIMATE on measured parts).
- The node term is 13 to 62 times the leaf term per step, not a fixed 60; in every case more than 90 percent of the rows per step (ESTIMATE).
- Native cost is almost all SHA-256: about 1,860 rows per block upstream and Poseidon 20 rows per permutation; Sherlock's Poseidon is 70 rows (DESIGN) and the design has no SHA-256 chip.
- 78 percent of the measured node is two non-native MSMs (`collapse_self` 239,577 and `collapse_final` 267,514 of 652,437 rows, measured).
- Proofs on this host are memory and bus bound: a k = 20 proof uses 16.3 to 17.6 GB and 9.77 of 20 cores; three at once slow each other by 1.73 (measured).
- Central `spend` decomposition: steps per instruction about 33 x node over leaf about 56 x leaf rows per step over native rows per instruction about 10.8 = about 20,000 (ESTIMATE). The step factor rests on 20 to 45 steps per ZKIR instruction (ESTIMATE; S1 decides).
- Ranked levers (all ESTIMATE): L1 large leaf S = 1,024 to 2,048 as a second leaf kind, 40 to 65 times per step; L5 straight-line mode with a cheap tape, 11 to 44 times on steps; L3b direct leaf acceptance, floor from 135 s to 4 to 8 s; L6 Poseidon at 20 rows and a SHA-256 chip in the large leaf; GPU kernels 2 to 4 times on every proof; L2b cheaper non-native multiplication up to 2 times on the node; L7 native proof through GP-3 at 10 to 20 times native per call in a batch; merge tree for wall time across hosts; L2c folding (very high risk: no curve cycle for BLS12-381, no decider construction known, no gain for GP-3). L8 native key per short entry is 1x and a finding, not a design.
- Design rule: every column and lookup added to the leaf is paid again in the node (7,298 node rows per advice column, about 22,600 per lookup, from the design fit). A naive SHA-256 chip adds about 190,000 node rows and breaks the 950,000 gate (ESTIMATE).
- Multicore: cores as rows is lever L1; cores as columns add about 88,000 node rows at C = 2 and break the gate at C = 2 or 3 (ESTIMATE), so not affordable with the present node. GPU cores feed prover kernels (MSM, NTT, quotient), not machine steps.
- Wall time for `spend` (ESTIMATE): today 10.2 h CPU; L1 + L5 + L6 + L3b about 8 s CPU, 2 to 4 s with the GPU. 10^9 steps is a cluster job under every lever in the file.
- Ledger 9 finding (section 7): ledger 9 embeds verifier parameters of a 2^14 setup (`VERIFIER_MAX_DEGREE = 14`), and a comment says no verifiable circuit exceeds 2^14 rows. The ledger wrapper compares no k with the parameters; whether `midnight_zk_stdlib::verify` succeeds for a k > 14 key was not traced. The question stays open; experiment C7 must include a k > 14 key on ledger 9.
- Thirteen experiments C1 to C13 plus S1 and N1b; recommended order: S1, N1b and the ledger 9 k question first, then the large-leaf census and the straight-line rule family, then the owner decision on L3b, then chips, one re-key, the GPU track in parallel, folding last.
- Papers: all UNREAD, several ids from memory.

## Key Quotes

- "The owner's challenge is answered: '1,000 to 10,000 times' was low for `spend`."
- "the structural floor of a universal machine against a specialized circuit for a hash-dominated call is about 2 to 4 times, and reaching the low end takes all the levers."

## Connections

[[cost-round-brief]], [[n1-native-circuits]], [[cost-lane-astra]], [[cost-lane-gemini]], [[IvcNode]], [[RunDecider]], [[MidnightLedger9]], [[ChipLeaf]], [[elastic-chip-brief]].

## Contradictions

- Steps for `spend`: 6,000 to 12,000 here; [[cost-lane-gemini]] uses 3,288 (12 steps per instruction). Instruction count: 273 here and in [[cost-lane-astra]]; 274 in [[cost-lane-gemini]].
- Native `spend` prove time: 1 to 2 s here (ESTIMATE); 4.8 s in [[cost-lane-gemini]] (ESTIMATE); measured later at 1.755 s ([[n1b-native-proving-time]]).
- Column-core node cost: about 88,000 rows at C = 2 here; 279,080 in [[cost-lane-gemini]]; 241,000 to 254,000 per column core in elastic area A ([[k21-recalibration-brief]]).
- The k = 19 node: here "the k = 19 route was closed as not reachable by trims" (citing the consensus design); [[cost-lane-gemini]] claims a single-collapse node at k = 19.
- The ledger 9 2^14 question is open here; [[elastic-chip-gpu-fable]] confirms from sources that a k = 20 decider verifies.
- L3a and L3b (leaf or decider-direct acceptance) conflict with the one-seal-path and one-contract revisions P-3 and P-8 ([[council-revisions]]), which the brief did not point to.
