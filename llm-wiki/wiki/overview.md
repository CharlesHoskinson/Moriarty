---
title: "Overview"
type: synthesis
tags: []
sources: [agents-charter, general-purpose-definition, c6-outcome-ledger9-review, design-2-7-chip-leaf, runlength-consensus, runlength-consensus-audit, final-design-decisions, land-3x-merge, c1-dense-memory-experiment, lookup-cache-consensus, lookup-cache-audit, sherlock-architecture, arch-audit-astra, arch-audit-response, rule-maxima-experiment, council-findings, council-revisions, cost-lane-fable, cost-lane-astra, cost-lane-gemini, n1-native-circuits, elastic-chip-brief, elastic-chip-gpu-fable, n1b-native-proving-time]
last_updated: 2026-11-12
---

# Overview

Moriarty: Sherlock VM, general-purpose recursion and the Doctor Watson factory: findings, decisions and open owner questions.

1. The goal is to finish the Sherlock recursion processor ([[agents-charter]]).
2. The target is general-purpose recursion: all of GP-1 to GP-4, never narrowed without the owner ([[GeneralPurposeRecursion]], [[OwnerRules]]).
3. C6 outcome-only publication is decided and adopted into the spec: the public statement is `[prog_d, input_d, outcome_d, n_steps]` ([[C6OutcomeOnlyPublication]]).
4. The chip leaf is proposed, not adopted. Its final spike key is 6,511 leaf rows, 823,466 node rows and 929,166 decider rows, all `cost_model` figures with no proof ([[ChipLeaf]]).
5. 2^64 is the range of the SRC-004 step counter, not a circuit size; no circuit has 2^64 rows ([[RunLengthCeiling]], [[runlength-consensus-audit]]).
6. The run-length study is done and audited (adopt with changes). Lead decisions pending owner review: dense tail-buffered memory replaces D3 ([[DenseTailMemory]]; its backend is measured in experiment C1, [[c1-dense-memory-experiment]]), continuations by tier 1 and retained memory ([[Continuations]]), the audit's GP-1 text ([[final-design-decisions]]).
7. The lookup cache study is done and audited (lead L-1 to L-10: block cache of 8-record lines, scalar cold read kept, one arena, LogUp benchmark-only; [[BlockCache]]). The final-design architecture (revision B2) merges six drafts into 23 module kits with two seams and a build order; the Astra audit's 20 findings are fixed as design text; nothing is built for it. On branch `pipeline-anim` the council revisions P-1 to P-10 add a LEDGER CONTRACT kit (24 kits), one seal path and program admission ([[ModularArchitecture]], [[sherlock-architecture]], [[council-revisions]]).
8. Owner questions now: OD-1 TOP >= 63 (backed by M-2), OD-2 no claim counter, OD-17 input admission, OD-18 one-arena SRC-021, OD-20 GP-2 scope, OD-21 semantic reference, OD-23 `rules_d` consumer, plus L-2, L-9 and the older D-1, D-2, D-6 and exhaust kind 7 items ([[OpenOwnerQuestions]]).
9. Cost against native Compact: `spend` is k = 15, 24,025 rows and proves in 1.755 s (measured, [[n1b-native-proving-time]]); Sherlock is about 10^4 times that in rows today, dominated by the node (ESTIMATE, three unmerged lanes). The elastic chip round (12 lanes) is merging; its GPU lane finds seeded proofs reproducible only at a fixed thread count. The land-3x dispatch partition is on `main` (`e39bc7af8`), no task ticked ([[n1-native-circuits]], [[cost-lane-fable]], [[elastic-chip-gpu-fable]], [[land-3x-merge]]).
10. The design runs on Midnight ledger 9 with changes; no blocker in the ledger ([[MidnightLedger9]]).

Current state: [[WorkstreamStatus]].
