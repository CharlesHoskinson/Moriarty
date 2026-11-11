---
title: "acceptRun contract"
type: entity
tags: [contract, on-chain, c6]
sources: [c6-outcome-ledger9-review, council-findings, council-revisions, cost-lane-gemini]
last_updated: 2026-11-11
---

# acceptRun contract

The Compact contract `tools/sherlock-sdk/sherlock-run.compact` whose `acceptRun` circuit accepts a run on Midnight ([[c6-outcome-ledger9-review]]).

- Before C6 it refused a replay by `stmt`, which hashed `fin_d`. Under D3 one run could have many `stmt` values, so the same run could be accepted again ([[c6-outcome-ledger9-review]]).
- Under SRC-007 (C6) it binds `prog_d`, `input_d`, `outcome_d` and `n_steps`, refuses a `claim_d` already in `runs`, stores `outcome_d` and `n_steps`, takes `input_d` as call data, and takes or stores no state digest ([[C6OutcomeOnlyPublication]]).
- The sealed `program_d` stays, so the contract still serves one program. One contract for every program (GP-1) needs `prog_d` as call data too; that is not part of the C6 decision ([[c6-outcome-ledger9-review]]).
- No `acceptRun` transaction has been validated by a 9.1.0.1 node; the stagenet deployment was on a pre-release ledger 9 and had zero calls ([[c6-outcome-ledger9-review]]).

## LEDGER CONTRACT kit (council revision P-8, branch `pipeline-anim`)

- C6: no kit owned the claim ledger contract, and the recommended N-11(a) wording used two contracts with separate ledgers, so one `claim_d` could be accepted once in each, which breaks "accepted once on-chain" ([[council-findings]]).
- P-8: a new kit LEDGER CONTRACT (Compact, outside the recursion crates; spec `sherlock-ledger-contract`, LDG-001 to LDG-005). One contract with shared state: `runs`, `foreign`, `verified_roots` and the registry root. Entry points: `acceptRun(prog_d, input_d, outcome_d, n_steps)` with `prog_d` as call data; `acceptCollection(reg_root, set_root, shape_d)`; `descend(root, k openings)` sized to the 256-operation cap; `openClaim(sub_root, head preimage)`, idempotent. A `claim_d` is in `runs` at most once, whichever entry inserted it. DEC-L2 moves here as LDG-L1. No circuit rows ([[council-findings]], [[council-revisions]]).
- SRC-007's sealed `program_d` question stays with the owner ([[council-findings]]).
- [[cost-lane-gemini]] proposes a second entry `acceptDirectLeaf` for one-leaf runs, which would need an OWNER exception to GP-001; it was written against the pre-council design.

See also [[RunDecider]], [[MidnightLedger9]].
