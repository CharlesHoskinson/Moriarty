---
title: "Lessons: prover performance"
project: lessons
date: 2026-10-24
tags: [lessons, loki, performance, gpu, msm, ntt, blst]
---

# Prover performance

## Pin which parent and which host a baseline came from (2026-10-24)
- Scouts kept tripping over two "k20 parent" numbers: 882.6 s prove (and 867 s at 268% CPU, 15 GB) in the older recl9 logs, and 220 s
  for Moriarty's A3. They are different runs: the recl9 logs came from another host and user (`/home/charl`), using recl9's
  `sherlock_link.rs` parent on stock 0.8.2. A3 is Moriarty's ported `mori-recursion` parent on this host, at RAYON_NUM_THREADS=4.
  The parent's shape: k=20, 15 advice, 45 fixed, 5 lookups, 18 permutation columns, single phase.
- **Lesson:** every timing note must name the circuit (crate and commit), the prover (stock version or Loki commit and features),
  the thread counts and the host. Otherwise downstream agents compare unlike things.

## Accelerating MSM and NTT alone does not speed up the real workload (2026-10-24)
- **Measured:** the k20 Sherlock parent (A3, RAYON_NUM_THREADS=4) took 220.2 s prove+verify on stock 0.8.3, 252.6 s on Loki CPU and 220.8 s on
  Loki with GPU MSM+NTT, with a 16.0 GiB peak RSS. The proof was accepted each time.
- Yet the GPU MSM alone is 13–16× faster: 2^20 takes 759 → 58 ms warm, 2^22 takes 3,005 → 181 ms. The GPU NTT gives 1.41× on the k17 quotient.
- **Lesson:** the time is in the phases still on the CPU (synthesis, quotient evaluation, lookup preparation, multiopen). Profile
  the real circuit by phase before choosing what to accelerate. Synthetic k16/k17 fixtures don't show the k20 parent's mix.
  This matches the 10x verdict: caching gives about 1.07× and parent-only acceleration is capped.

## Cargo feature unification spreads `blst/no-threads` to every crate (2026-10-24)
- **What happened:** W3 made BLST single-threaded, scheduling its MSM chunks on the Rayon pool, so CPU use stays within one budget. Cargo
  unifies features, so this also disabled BLST threading for the stock crate in the same build and for every other BLST user.
  The k17 harness at 1 thread went from 3.6 s to 8.7 s for both Loki and stock, and A3 slowed by 15%.
- **Lesson:** a dependency feature in a library is a global decision for every downstream build. Make it opt-in
  (`loki-cpu-budget`, in W9) and benchmark at the thread counts the real checks use.

## Micro-gains don't add up to end-to-end gains (2026-10-24)
- W3 measured 1.83×/1.38× on rational conversion and 2.85× on sorting, yet the k16 end-to-end time went 1,767 → 1,781 ms.
- **Lesson:** report end-to-end medians next to kernel numbers, and treat any kernel speed-up as unproven until the full proof moves.

## Where the real limits are (from the scouts, 2026-10-24)
- The lookup table (about 1.03M rows in Fenrir's node and decider), not the advice rows, is what pins k20. Reaching k19 needs about half
  the table (Track B, new keys). Measure the Sherlock parent's per-table rows before designing this.
- Lookups force the quotient degree to be at least 4, so the extended domain is at least 4n.
- In quotient evaluation the cheapest win is in software: fuse the gate, permutation, lookup and trash passes and share common
  subexpressions (an estimated 1.2–1.8×). A GPU quotient needs batched intermediates; a naive layout needs about 26 GB at k22.
- fflonk (midnight-zk PR #565) makes the prover 1.29× slower; its benefit is VK size, not speed.
- See [speedups/](../loki/speedups/) for the ranked tables and sources.
