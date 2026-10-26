# Stage s8: decider and settlement

> Sources: astra-s8-decider-settle wave 1 (gpt-6-astra high, Haiku helpers), 2026-10-26; Loki-10 settlement study, 2026-10-24
> Raw: [findings](../../clusters/s8-decider-settle/wave-1/FINDINGS.md); [experiments](../../clusters/s8-decider-settle/wave-1/EXPERIMENTS.md); [plan](../../clusters/s8-decider-settle/wave-1/PLAN.md); [sources](../../clusters/s8-decider-settle/wave-1/SOURCES.md); [recorded runs](../../raw/s8-decider-settle/recorded-runs.md); [DUST management](../../raw/s8-decider-settle/manage-dust.md); [DUST sponsorship](../../raw/s8-decider-settle/dust-sponsorship.md); [prior Loki-10](../../raw/s8-decider-settle/prior-loki10.md)
> Updated: 2026-10-26

## Overview

**No source supports a 10× single-batch improvement from Consumed to Finalized yet.** The decider already folds to a residue, so there is no protocol swap to make.

Two labels hide different workloads. A4's 324.739 s is a whole check invocation, not a `prove()` timer. B4's 125–153 s stock verification rebuilds the leaf, parent and decider keys. These figures cannot be added into one pipeline.

The best near-term wins remove repeated setup and keep the wallet synchronized. The structural target is a smaller decider, which must cut verifier and MSM work as well as pairing work.

## Ranked opportunities

1. **Persistent authenticated prover plus a synchronized wallet.** ESTIMATE: removing all 46.9 s of PK work from recl9 gives `634.8/587.9 = 1.080×`. The 435 s DUST registration on this host was mostly a 375 s wallet sync; a kept-warm wallet removes most of it (bound 7.24× on the registration interval).
2. **Verifier/MSM specialization plus a domain reduction.** By N log N, k19 costs 0.475 and k18 costs 0.225 of k20. With 15% of the work unchanged, that gives 1.81× or 2.93× decider latency.
3. **Fused foreign-field relations, especially paired lines.** At most 1.18× if a measured 30% of decider time is halved. A naive port can lose.
4. **The existing GPU work plus a smaller decider.** On Loki's assumptions, 2.98×. Do not multiply independent headline speedups.
5. **Fuse the terminal parent/node with the decider.** Optimistically 1.68× at unchanged k. If fusion doubles the domain, the result is 25.3% *slower*. Neither case predicts the fused domain.

## Notes

- Fees: model the fee per logical transaction as `F(n)/n`. With `F(n) = F0 + n·f`, 16-way amortization gains between 1× and 16×. Proof size (6,432 B) alone does not determine F(n).
- A single sponsor or fee coin can serialize the pipeline even with an idle GPU. Measure fee-input contention and transaction expiry before pipelining decider jobs.
- First experiments: E1 identifies the real decider and verifier critical path (it needs the pinned ledger verifier binary); E2 tests the domain cliff before any new pairing design; E3 separates wallet proof count, sync and per-batch economics.
- The verbatim recl9 and stand-in decider captures this wave used are private Fenrir files. They stay on the research host and are not published.

## See Also

- [s1 key setup and residency](s1-keys.md)
- [s7 recursion and aggregation](s7-recursion.md), whose fused fixed-pair terminal (C1) must answer idea 5's domain caveat
- [s9 scheduling, memory and hardware](s9-orchestration.md)
- [Consensus tracker](../../CONSENSUS.md)
