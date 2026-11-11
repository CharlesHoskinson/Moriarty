---
title: "acceptRun contract"
type: entity
tags: [contract, on-chain, c6]
sources: [c6-outcome-ledger9-review]
last_updated: 2026-11-10
---

# acceptRun contract

The Compact contract `tools/sherlock-sdk/sherlock-run.compact` whose `acceptRun` circuit accepts a run on Midnight ([[c6-outcome-ledger9-review]]).

- Before C6 it refused a replay by `stmt`, which hashed `fin_d`. Under D3 one run could have many `stmt` values, so the same run could be accepted again ([[c6-outcome-ledger9-review]]).
- Under SRC-007 (C6) it binds `prog_d`, `input_d`, `outcome_d` and `n_steps`, refuses a `claim_d` already in `runs`, stores `outcome_d` and `n_steps`, takes `input_d` as call data, and takes or stores no state digest ([[C6OutcomeOnlyPublication]]).
- The sealed `program_d` stays, so the contract still serves one program. One contract for every program (GP-1) needs `prog_d` as call data too; that is not part of the C6 decision ([[c6-outcome-ledger9-review]]).
- No `acceptRun` transaction has been validated by a 9.1.0.1 node; the stagenet deployment was on a pre-release ledger 9 and had zero calls ([[c6-outcome-ledger9-review]]).

See also [[RunDecider]], [[MidnightLedger9]].
