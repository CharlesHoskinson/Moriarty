---
title: "Final council S9 on the co-processor design"
type: source
tags: [council, freeze, co-processor, owner-items]
sources: [council-final-s9]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-processor-final-20261111/COUNCIL-FINAL.md
source_url: local
---

# Final council S9 on the co-processor design

## Summary

Loop step S9 asked three blinded lanes (OpenAI Astra, Google Gemini Argon, Anthropic Opus) whether the design at `de4449b3a` on branch `pipeline-anim` can freeze its interfaces: bus tuple, kit interfaces, public statement, `claim_d`, key set. All three answered `changes_requested`. One revision followed (commits `0746e280e` and `d9c642088`). The S9 gate is not met: several admissible findings are open freeze prerequisites FP-1 to FP-8, and the round goes to the owner. The Opus lane shares the author family, so strictly only two non-author verdicts exist. Council output is advisory.

## Key Claims

| Item | State |
|---|---|
| Bus tuple over `Fq` while the leaves are BabyBear STARKs (all lanes) | FP-1, open; section 1.2 marked prototype, BUS-008 |
| Node statement envelope: deferred log `(d, n)`, forwarded `key_root` | FP-2, open |
| Public statement codec, SET-head layer, `L` public-value index table, public form of `prog_d` | FP-3, open; N-11 and the SET-head layer are owner items |
| Two digest definitions (`sherlock-value/1` against `/2`) | fixed in part (VDG-007, HSH-005, INIT-007); the one digest table is FP-4 |
| Seal read of `view_root` | FP-5, open (AD-17 owner) |
| Public `n_steps` exact or padded | FP-6, owner |
| Chunk claim address and E-1 window offset | FP-7, open |
| Universal key set | FP-8: after G1 to G5 and owner confirmation of OD-1 to OD-4, OD-17, OD-18 |
| Foreign proofs folded into a core accumulator the STARK node lacks | fixed: in-circuit check under any SRS (decision C-3) |
| Plain `prog_d` in `acceptRun` | fixed in part: never call data (ZK-002); form is N-11 |
| Minor items (AW and VW in SHP-005, 950,000-row gate, AD-6, kit count, labels, hard-rule sentences) | fixed |
| GPU wrap evidence (added at revision) | MEASURED under load: GPU 34.8 / 95.0 / 155.2 s against CPU 118.8 / 243.3 / 441.7 s at k = 19 proxy, 20, 21; path 2 and 3 wrap times ESTIMATE conditional on the fused quotient kernel and MSM rewrite |

## Links

[[OpenOwnerQuestions]], [[WorkstreamStatus]], [[final-design-decisions]], [[council-revisions]].
