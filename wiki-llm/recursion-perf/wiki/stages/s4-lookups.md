# Stage s4: lookups and grand products

> Sources: astra-s4-lookups wave 1 (gpt-6-astra high, Haiku helpers), 2026-10-26; Loki-06 prior study
> Raw: [findings](../../clusters/s4-lookups/wave-1/FINDINGS.md); [experiments](../../clusters/s4-lookups/wave-1/EXPERIMENTS.md); [model results](../../clusters/s4-lookups/wave-1/model-results.json); [Loki-06 prior](../../raw/s4-lookups/prior-loki06.md)
> Updated: 2026-10-26

## Overview

Prioritize **structured table-index permutation construction**, then **parallel inversion and product scans**. Both keep the current lookup equations, keys, transcript and official SRS. A large speedup of lookup *preparation* is credible enough to test. 10× end-to-end is not supported. Shrinking tables alone cannot lower the recorded k20 domain while advice stays above k19.

## Ranked ideas (ESTIMATE)

1. **Structured table-ID grouping:** linear counting or scatter in place of a comparison sort. Test range 2–10× preparation, giving 1.14–1.29× of the full proof if preparation is 25% of it.
2. **Resident GPU grand products:** a 5–20× arithmetic subphase gives 1.25–1.31× at a 25% share, or 1.67–1.91× at 50%.
3. **Share table work across identical lanes:** at most 4× on that component, about 1.08× of the stage.
4. **Parallel CPU scan baseline:** 2–8× on the arithmetic component, the control to beat.
5. **Global range-tag planning** (cap tags at 17): 1× domain gain while advice stays above k19.
6. **LogUp/GKR batching:** published 1.5–4× at small column lengths; no transferred prediction.

See also: [consensus tracker](../../CONSENSUS.md).
