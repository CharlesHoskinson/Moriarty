# Stage s7: recursion and aggregation

> Sources: astra-s7-recursion wave 1 (gpt-6-astra high, Haiku helpers), 2026-10-26; Loki 10x studies, 2026-10-24
> Raw: [findings](../../clusters/s7-recursion/wave-1/FINDINGS.md); [experiments](../../clusters/s7-recursion/wave-1/EXPERIMENTS.md); [arithmetic](../../clusters/s7-recursion/wave-1/model-results.json); [Astra-2 prior](../../raw/s7-recursion/astra2-prior.md); [Loki-05 prior](../../raw/s7-recursion/loki05-prior.md)
> Updated: 2026-10-26

## Overview

The strongest route to a tenfold chain improvement is still a **direct fixed-pair terminal**. It proves the A102→A80 transitions and their link in one relation of about k15, instead of verifying two child proofs inside a k20 parent and then a decider. Wave 1 narrows that claim to explicit conditions and does not declare it achieved. Generic recursion has no demonstrated tenfold route. Every figure below is an ESTIMATE from recorded baselines, not a new prover measurement.

## Key estimates

- **Fused terminal.** The k20/k15 domain ratio is 32. With an unscaled share h of 0.02–0.06, the gain `1/(h+(1-h)/32)` is **11.19–19.75×** against the A3 interval. At h = 0.10 it drops to **7.80×**. k15 needs h below **7.10%**; k16 needs it below **4%**.
- **Leaf partition for 16 transactions.** One mono16 proof at k19 versus four k17 leaves gives **8.84–9.88×** against the historical recl9 chain under a leaf-calibrated proxy. That is close to 10× but not there.
- **Fusion thresholds by arity.** A binary fused terminal fits k20 with about 157k rows of glue headroom. Ternary needs 4.5% of its verifier removed; quaternary needs **23.63%** to keep the existing pairing gadget within k20.
- **Accelerating only the node is Amdahl-capped.** A 10× faster node gives 1.93× overall, and a free node 2.16×. With the decider unchanged, the ceiling is 2.76× even if the leaves and node are free.
- **Folding schemes.** NeutronNova/Vega, MicroNova and FLIP get no speedup credit until a ledger-9 wrapper is costed. A 586 s wrapper caps any folding gain below 2.76×.
- **Further KZG deferral** yields 1.00×: the baseline already accumulates and defers the pairing once.

## Must survive any redesign

Keep ordinary PLONK/KZG over BLS12-381, the Blake2b outer transcript, the official SRS and the stock ledger-9 reader. The binding of statement, communication and anchor must stay exact. Any new relation needs a new registered verifier key, which matches the planned redeploy. Fusion proves equivalent transition semantics; it must not claim to authenticate child proof bytes that were never produced.

## Next experiments

- **E1:** fit the fused fixed-pair terminal at k15/k16 with equivalence mutations, the stock reader and complete-chain timing. In parallel, synthesize the 4×4, 2×8 and mono16 partitions.
- **E2:** fflonk inner verifiers aimed at the 23.63% threshold.
- **E3:** folding only after costing the final verifier first.

## Wave 2: making C1 checkable

> Raw: [findings](../../clusters/s7-recursion/wave-2/FINDINGS.md); [experiments](../../clusters/s7-recursion/wave-2/EXPERIMENTS.md); [OpenSpec draft](../../clusters/s7-recursion/wave-2/OPENSPEC-DRAFT.md); [model](../../clusters/s7-recursion/wave-2/model.py)

- C1 constrains A102→A80 and their link directly. It needs **no in-circuit verifier, accumulator or pairing gadget**, so the 316,833-row gadget and its k19 floor do not apply to C1. The ledger still checks the new outer PLONK/KZG proof.
- s8 wave 1 rank 5 was a different fusion (terminal node with the decider). That fusion keeps the pairing floor and can double the domain.
- The fit is unproven. The provisional budget is 20,466 advice rows, with tables constrained separately. It is not a synthesis result, and exact A102/A80 row counts are absent from the records. Strict >10× needs h < 7.0968% at k15 or h < 4% at k16.
- Next: E1 synthesis-only (rows, columns, degree, table extent, fitted k) after C0 lands.

## See Also

- [s2 witness synthesis](s2-witness.md)
- [s3 commitments and MSM](s3-commit-msm.md)
- [Consensus tracker](../../CONSENSUS.md)
