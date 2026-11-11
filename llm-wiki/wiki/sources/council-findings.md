---
title: "Council synthesis: does the Sherlock design need improvement?"
type: source
tags: [council, review, architecture, animation]
sources: [council-findings]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty-wt/pipeline-anim/research/sherlock-pipeline-animation-20261111/council/COUNCIL.md
source_url: local
---

# Council synthesis: does the Sherlock design need improvement?

## Summary

The pipeline-animation council checked the final design (`ARCHITECTURE.md` at head `680d2b5af`) while the animation was built. Three members returned `changes_requested`; a non-author verifier checked each finding against the text and merged them into items C1 to C10. Verdict after verification: **changes_requested**, with one blocker (C5) and five major items (C1 to C4, C6). No finding narrows GP-1 to GP-4. The file is on branch `pipeline-anim` only. The fixes are in [[council-revisions]].

## Key Claims

| Id | Finding | Severity after verification | Verdict |
|---|---|---|---|
| C1 | PROG canonicality premise has no discharge point on the acceptance path; any caller can supply a non-canonical `prog_d` once `prog_d` is call data | major | CONFIRMED |
| C2 | By-digest input admission has no schedule for wide constructors and no read-only view | major | PARTIAL |
| C3 | Foreign-verifier phases have no authenticated operand store across units | major | CONFIRMED |
| C4 | Two seal paths: the M4 run decider skips the `deps_d` check, and the GP-path run entry has no specified head opening | major | CONFIRMED |
| C5 | `reg_root` is not tied across child slots; a FOREIGN child proved under an attacker's root can sit in one slot | blocker | CONFIRMED (node); PARTIAL (run entry, latent) |
| C6 | No kit owns the claim ledger contract; the recommended N-11(a) wording uses two contracts with separate ledgers, so one `claim_d` could be accepted once in each | major | CONFIRMED |
| C7 | `rules_d` has no consumer after the leaf | minor | CONFIRMED |
| C8 | The MERGE-WORK fallback allows a second encoding of the machine relation | minor | CONFIRMED |
| C9 | `shape_d` has no `(out_a, out_b)` case for an FV output; the FV payload duplicates `reg_root` | minor | CONFIRMED (main part) |
| C10 | DATA cell packing for `e = 1` needs limb range checks | none | REFUTED (restates open OD-11) |

- C1 revision: a new administrative node mode ADMIT-WORK walks the ROM tables once per program and emits an admission claim; SEAL checks it. Cost: one admission chain per distinct `prog_d`, reused by every run (unlabeled).
- C2 revision: a new mode VIEW-WORK builds a certified read-only input view `view_root`; a field read is one Merkle opening of about 20 to 64 hashes (ESTIMATE).
- C3 revision: a `work_root` record store in the foreign-verifier state.
- C4 revision: one seal path; the run entry accepts only a SET root sealed by NODE; on the M4 path one SEAL node runs before the decider, about 65 s more on the reference host (as cited from the storyboard).
- C5 revision: `reg_root` equal in every enabled NODE or FOREIGN child and the parent; two field equalities per node.
- C6 revision: a new kit LEDGER CONTRACT, one contract with shared state (`runs`, `foreign`, `verified_roots`) and entry points `acceptRun`, `acceptCollection`, `descend`, `openClaim`; no circuit rows.
- C7: owner choice. Give `rules_d` a consumer in the LEAF base case of `shape_d`, or amend SRC-001 to a reserved zero lane; removing it saves about 9 permutations per leaf at S = 16, about 630 rows (ESTIMATE).
- C8: build the inline kernel from the same kit `configure` and `assign` functions as the leaf; no separate emitter.
- Apply order: C5, C9, C4, C8, C1, C2, C3, C6, C5 part 2, C7. The proposed decision ids K-1 to K-6 are placeholders.

## Key Quotes

- C1: "This is not a 'Lean open' item. The chain has no step that checks the premise, so any caller can supply a non-canonical `prog_d`."
- C5: "the attack forges any foreign claim and the fix is small."
- "No finding was INADMISSIBLE in full. No finding narrows GP-1 to GP-4."

## Connections

[[council-revisions]], [[ModularArchitecture]], [[IvcNode]], [[RunDecider]], [[AcceptRunContract]], [[ValueDigest]], [[AnimationV2]], [[sherlock-architecture]].

## Contradictions

- The council found that [[sherlock-architecture]] assigned the at-most-once settlement property (DEC-L2) to the DECIDER circuit kit, which cannot enforce it; C6 moves it to the contract.
- The storyboard places SEAL in the decider scene; the council calls this presentation, not a design conflict (C4).
