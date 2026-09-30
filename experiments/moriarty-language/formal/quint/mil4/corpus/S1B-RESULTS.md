# S1B finite Quint results

Quint 0.32.0, TypeScript backend, seed `0x5`: 83/83 finite witnesses passed
(59 existing witnesses and 24 Stage guard controls).
Six `.qnt` files typechecked. Exact argv, exit codes, stdout, stderr and
candidate hashes are retained in
[s1b-stage-guard-command-results.json](s1b-stage-guard-command-results.json).
That new receipt supersedes the prior S1B candidate for the local Quint result;
[s1b-command-results.json](s1b-command-results.json) remains unchanged.
The previous S1B receipt is preserved byte-for-byte as
[s1b-pre-audit-command-results.json](s1b-pre-audit-command-results.json).
Earlier receipts describe their recorded bytes.

| Corpus | Passed |
| --- | ---: |
| Original `s0_witnesses` | 15 |
| Ordering `s0_divergence_witnesses` | 14 |
| Common-state `s0_common_witnesses` | 3 |
| Diagnostic `s0_diagnostic_witnesses` | 3 |
| S1B `s0_s1b_witnesses` | 48 |
| Total | 83 |

The retained original witnesses now supply explicit successor IDs and typed
comparison requests. Original repayment fixtures use absent recipient and
fee-recipient fields. Original rejection witnesses pin exact diagnostic
codes. Original and diagnostic history imports explicitly stipulate heads
`0` and `1` for postcommit stale/replay checks. `committedCount` contributes
nothing to snapshot authentication. A separate S1B witness stipulates only
head `0`, commits head `1`, and rejects its next submission at Stage because
that current snapshot premise is unavailable.

| S1B witness | Complete first decision |
| --- | --- |
| H1 | Accepted, empty code, diagnostic work 0 |
| H2, H3, H3SelfPremise | History, `S0_HISTORY_SUCCESSOR`, diagnostic work 1 |
| H4 | Effect, `S0_EFFECT_MISMATCH`, diagnostic work 1 |
| H5 | History, `S0_HISTORY_STALE`, diagnostic work 1 |
| H6 | History, `S0_HISTORY_REPLAY`, diagnostic work 1 |
| F1, F2, F3 | Failure, `S0_FAILURE_UNSUPPORTED`, diagnostic work 1 |
| P1 | Stage, `S0_STAGE_PREMISE`, diagnostic work 1 |
| P2Intent, P2PreState, P2Round, P2Outcome | Stage, `S0_STAGE_PREMISE`, diagnostic work 1 |
| P3 | Stage, `S0_STAGE_UNSUPPORTED`, diagnostic work 1 |
| nonArithmeticSuccessor | Accepted stipulated `0 → 9`, empty code, diagnostic work 0 |
| failureAfterEffect, effectAndReplay, effectAndWrongSuccessor | Effect, `S0_EFFECT_MISMATCH`, diagnostic work 1 |
| staleAndReplay | History, `S0_HISTORY_STALE`, diagnostic work 1 |
| committedHeadPremiseUnavailable | Stage, `S0_STAGE_PREMISE`, diagnostic work 1 |
| repayRecipientAbsent, repayFeeRecipientAbsent | Intent, `S0_INTENT_SCOPE`, diagnostic work 1 |

Every S1B negative witness checks literal complete financial pre-state,
signed-intent map, round, commit count and published effect record, then
checks that submission is disabled. The postcommit unavailable-head witness
retains the previous accepted effects; all other rejection fixtures have an
empty record. `fail()` ends the witness trace; a
disabled action has no committed successor, so these are pre-observation
and blocked-submission checks rather than a sampled post-rejection state.
H1 and the separately stipulated `0 → 9` witness check literal complete
post-state and exact effects. The common H/F/P baseline has round 0, net floor
0, balances O100/R0/F0, owner allowance11/spent0 and work1/spent0. H4 submits
head1 with an effect head9; H5 has predecessor9, current head0 and submitted
head9; H6 has consumed replay and submitted head9. P2PreState changes only the
premise owner balance from 100 to 99. P3 omits the required owner balance cell
and marks the tuple unavailable. These are Quint observations on named
comparison fixtures; this file does not execute a K or TypeScript comparison.
The non-arithmetic, absent-endpoint and unavailable-head witnesses are
supplementary Quint controls.

The tuple binds stored intent, complete financial snapshot, round,
expected successor and requested outcome. A supplied intent substitution still reaches Intent when
the stipulated tuple correctly names the stored intent. Successor equality
is checked after stale predecessor and consumed replay; the effect vector
must first agree with the submitted successor. Requested phase and retained
vectors are typed. Stage checks equality of the submitted outcome and
premise outcome; Failure checks whether that matching outcome is supported.
F1–F3 bind their requested outcomes in the tuple; P2Outcome changes the tuple
outcome only. Combined controls pin stale before replay and effect mismatch
before replay or successor mismatch. H3SelfPremise rejects a self-successor
even when the tuple names that same successor.

These executions establish local behavior under supplied premises. They do
not authenticate signatures or snapshots, establish head extension, qualify
native proofs, execute a ledger, prove exhaustive invariants or implement an
executable K/Quint projection. W-D0–W-D4 and Sprint 1 remain open.

## Postreview Stage guard repair

Repository observation: distinct Transfer endpoints require exactly their three
balance keys, empty obligations and singleton payer remaining/spent allowance
maps. Repay requires exactly payer and stored creditor balance keys, the singleton
bound obligation and singleton payer allowance maps. Signed Transfer aliases
retain the existing Intent diagnosis; they cannot reach commit. State checks use
the stored intent so supplied intent substitutions retain their original ordering.
Both paths now require each work component to be UInt128 and their sum at most
`UINT128_MAX`; the owner allowance aggregate receives the same Stage bound.
These local failures precede premise comparison and return the complete decision
`{ accepted: false, judgment: Stage, code: "S0_STAGE_UNSUPPORTED", diagnosticWork: 1 }`.

Experiment observation: each path has ten isolated negative controls: an extra
balance, remaining-allowance key, spent-allowance key or obligation; negative or
above-bound remaining/spent work; work aggregate overflow; and owner allowance
aggregate overflow. Each binds a matching complete pre-state tuple, checks the
complete decision and all unchanged observable state, and confirms submission
is disabled. Two further controls combine malformed work with an unavailable
tuple to pin unsupported state before missing premise. Two positive controls
admit work `(1, UINT128_MAX - 1)` and commit it to `(0, UINT128_MAX)`.

The prior model failed all 22 new negative controls and passed both new boundary
controls; the exact commands, prior model hash and output are preserved in
[s1b-stage-guard-pre-repair-command-results.json](s1b-stage-guard-pre-repair-command-results.json).
The repaired model passes all 83 witnesses. Original and diagnostic receiving
fixtures now seed only balances, removing unrelated zero allowance rows.
The repayment ordering fixture removes unrelated account X; its stored payer
mismatch still rejects at Stage. The 59 existing witness names and intended first
judgments are retained. The prior Grok review covers its recorded candidate
bytes; this repair requires a fresh review of the new candidate.

External signatures, snapshots, successor extension, native qualification and
ledger atomicity remain stipulated. No exhaustive verification or executable
cross-layer projection was run. W-D0–W-D4 and Sprint 1 remain open.
