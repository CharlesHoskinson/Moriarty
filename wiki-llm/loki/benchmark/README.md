---
title: "Loki vs reference prover: head-to-head benchmark"
project: loki
date: 2026-10-25
tags: [loki, benchmark, performance, gpu, closing-condition]
---

# Loki vs the reference Midnight prover

**Owner request (2026-10-25):** produce a table of Loki's performance against the reference prover, stock `midnight-proofs` 0.8.3. Five Haiku and five GPT-6.1 Sol agents debate the suite over AgentBus (`#lokibench`). The suite runs once the full GPU and CPU paths are ready. The table is a closing condition of the Loki work.

## Files
- [SUITE.md](SUITE.md): the agreed suite. Workloads, configurations, metrics, statistics, readiness gate, run procedure, table template and Dissent. sol-1 writes it.
- [DEBATE.md](DEBATE.md): who argued what, the decisions, and the dissent.
- `RESULTS.md`: the head-to-head table, written after the run.

## Closing condition
Loki work is not done until all four of these hold:
1. The readiness gate in SUITE.md holds.
2. The full suite has run on this host through `dw-gpu-runner` under heavy.lock.
3. Every Loki proof in it was accepted by the unmodified stock 0.8.3 verifier.
4. `RESULTS.md` holds the table with provenance on every row, and slowdowns are reported alongside speedups.

The table is then linked from the loki-proofs README and `docs/GPU.md`.

## Lessons that apply
See [prover-performance](../../lessons/prover-performance.md):
- Benchmark on real scalar distributions.
- Pin the circuit, prover, threads and host on every row.
- Build stock and Loki separately, because Cargo feature unification spreads `blst/no-threads` to every crate in a build.
- Report end-to-end numbers next to kernel numbers.
