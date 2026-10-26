# Speedup consensus tracker

A speedup is **agreed** only when (a) at least two independent research sources rank it top-3 with consistent bounded-gain arithmetic, and (b) a cold cross-review by Muse Wonder Full and agy Gemini 4 Argon finds no refuting evidence. Agreed items go to the wiki, then an OpenSpec change in `openspec/changes/`, then a quint-llm-kit design check, then Sherlock roadmap Phase 5, then factory implementation.

| # | Speedup | Stages | Support so far | Gain bound (ESTIMATE) | Review | Status |
|---|---|---|---|---|---|---|
| C0 | **Measure the phase profile first** (L0: synthesis, MSM, lookups, quotient, multiopen, key load; one pinned midnight-proofs version) | all | **nine of nine** wave-1 clusters say independently that no 10× claim is checkable without it (s1, s6 and s8 each make a phase or critical-path measurement their experiment E1); Loki 10x plan step 1 | prerequisite, no gain claimed | queued (C0 meets the two-source test) | **candidate (strong)** |
| C1 | **Direct fixed-pair (fused) terminal**, about k15 | s7 | s7 wave 1 rank 1; Loki astra-2 (s7 built on Loki, so not fully independent) | 11.2–19.8× vs the A3 interval if the unscaled share is 2–6%; 7.8× at 10%. **Caveat (s8 rank 5):** fusing the terminal with the decider at a doubled domain is 25.3% slower, so the fused circuit's k must be measured, not assumed | pending | candidate |
| C2 | **Leaf partition** (mono16 or 2×8 with a fused binary terminal) for 16-tx batches | s7, s9 | s7 wave 1 rank 2 | 8.84–9.88× vs the historical recl9 chain (proxy) | pending | candidate |
| C3 | **Commitment-aware GPU bucket MSM service** (sppark first) | s3 | s3 wave 1 rank 1; Loki GPU-MSM study | 1.9–2.3× over Nocy GPU; 5.4–6.5× over CPU, at a 60% MSM share | pending | candidate |
| C4 | **Phase-aware witness schedule + frontier batch inversion** | s2 | s2 wave 1 ranks 1–2; Loki-09 lazy inversions | C ≈ 1.5–2.4× | pending | candidate |
| C5 | **Phase-aware scheduler with a bounded unfinished-batch window** and a reserved completion path | s9 | s9 wave 1 rank 2; Loki astra-3 throughput scheduler | throughput ceiling 9.6× (W=3) to 12.8× (W=4); latency unchanged | pending | candidate |
| C6 | **Structured table-ID permutation + parallel scans** for lookups | s4 | s4 wave 1 ranks 1 and 4 | 1.14–1.29× of the proof | pending | candidate |
| C7 | **Resident bounded-memory quotient evaluator with coset tiles** | s5 | s5 wave 1 ranks 1–2; Loki-03 | 2.11–2.38× overall at a 60% share | pending | candidate |
| C8 | **Backport only the BLST MSM dispatch correction to midnight-proofs 0.8.2** (cold VK build) | s1 | s1 wave 1 rank 2 | 1.33–4.71× of the 622.8 s cold VK build only; at most 2.85× of the keygen process; no proving gain | pending | candidate |
| C9 | **Resident exact-Midnight GPU multiopen**, transcript and PCS unchanged | s6 | s6 wave 1 rank 1; s3's GPU MSM service would carry its two MSMs | 1.03–1.37× overall | pending | candidate |
| C10 | **Persistent authenticated prover with resident prepared keys and a kept-synced wallet** | s1, s8 | s1 wave 1 rank 1 (prepared-key registry) and s8 wave 1 rank 1 (persistent prover), independent clusters, consistent arithmetic | 1.07–1.10× of proving today, growing as other stages speed up; up to 7.24× on the DUST-registration interval (sync removed) | queued (meets the two-source test) | **candidate (two sources)** |

**Cross-stage reading (2026-10-26, all nine stages):** only C1, a structural change, plausibly reaches about 10× alone, and s8 adds that its domain must be measured. C3, C4, C6, C7 and C9 are bounded additive accelerations of 1.03–2.4× each; C8 helps cold keygen only; C5 multiplies throughput but not latency; C10 is small today but removes a fixed cost that dominates once the rest is fast. A 10× practical system therefore looks like **C1 (fused terminal) + C5 (scheduler) + a subset of the stage accelerations**, all gated on C0 measurements.

Log of changes: see [wiki/log.md](wiki/log.md).
