# S0 full diagnostic decision witnesses

**Model:** repaired provisional `../s0.qnt`, Quint 0.32.0.  
**Corpus:** `s0_diagnostic_witnesses.qnt`.  
**Backend:** TypeScript; seed `0x5`; three fixed traces.

Rejected decisions report one abstract diagnostic unit. Accepted decisions
report zero. This observation is independent of the committed work budget:
rejection does not debit `workRemaining` or increase `workSpent`.

The import stipulates signature verification, authenticated snapshots, native
qualification and atomic ledger readiness. These runs do not establish those
premises. Exact commands, outputs, exit codes, timestamps and candidate SHA256
values are in [diagnostic-command-results.json](diagnostic-command-results.json).

From the checkout root:

```text
$ quint typecheck experiments/moriarty-language/formal/quint/mil4/corpus/s0_diagnostic_witnesses.qnt
(no output; exit 0)

$ quint test experiments/moriarty-language/formal/quint/mil4/corpus/s0_diagnostic_witnesses.qnt --main s0_diagnostic_witnesses --match '.*Test' --seed 0x5 --backend typescript

  s0_diagnostic_witnesses
    ok stageDiagnosticTest passed 1 test(s)
    ok effectDiagnosticTest passed 1 test(s)
    ok historyDiagnosticTest passed 1 test(s)

  3 passing (55ms)
(exit 0)
```

| Witness | Full rejected Decision checked |
| --- | --- |
| `stageDiagnosticTest` | `{ accepted: false, judgment: Stage, code: "S0_STAGE_UNSUPPORTED", diagnosticWork: 1 }` for an absent signed intent |
| `effectDiagnosticTest` | `{ accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH", diagnosticWork: 1 }` for an omitted fee credit |
| `historyDiagnosticTest` | `{ accepted: false, judgment: History, code: "S0_HISTORY_STALE", diagnosticWork: 1 }` after the first transfer advances the head |

The History witness first checks the complete accepted Decision with
`diagnosticWork=0`, then commits one transfer and observes rejection of the
old signed pre-head. Each witness checks the committed counters and effect
record before invoking a blocked submit with `fail()`. The Effect and History
witnesses also check all transfer balances and allowance cells. `fail()` ends
the trace; no successor after the failed action is sampled.

## Retained evidence and regression results

All three tests failed against the preceding ordering-repaired model when its
rejection diagnostic unit was zero (exit 1). The exact failed executions and
preceding candidate digests are retained in
[diagnostic-before-command-results.json](diagnostic-before-command-results.json).
After the one-unit repair, all three pass (exit 0). The unchanged original
corpus passes 15/15 and the ordering corpus passes 14/14 against the same
candidate; all four Quint files typecheck (exit 0).

Earlier ordering-only results remain in
[RESULTS-AFTER-ORDERING-REPAIR.md](RESULTS-AFTER-ORDERING-REPAIR.md) and
[DIVERGENCE-AFTER-ORDERING-REPAIR.md](DIVERGENCE-AFTER-ORDERING-REPAIR.md).

These are finite fixed executions. They do not establish a general Core cost
schedule, exhaustive proof, Source/6 admission, K/Quint correspondence theorem,
native qualification, or ledger admission. No `quint verify` was run. UInt128
literals exceed the default Rust evaluator's i64 range, so the TypeScript
backend was selected explicitly.
