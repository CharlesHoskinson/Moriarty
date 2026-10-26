---
title: "Lessons: correctness of a faster prover"
project: lessons
date: 2026-10-24
tags: [lessons, loki, lean, quint, fuzzing, midnight-proofs]
---

# Correctness of a faster prover

## Use several methods: each caught things the others could not (2026-10-24)
- **W2 harness:** Loki and the unmodified crates.io 0.8.3 verifier must accept each other's proofs, and the VK bytes must match. This is the merge gate.
  It also showed that lookup proofs differ from stock byte for byte (first difference at byte 192) while both verify.
- **Lean spec** (`loki-lean`, all six modules accepted by an Opus auditor): `loki_proof_eq_stock` holds for any backend that meets the component
  contracts and emits the stock transcript order. The audit loop mattered: it rejected a quotient-kernel contract that no real kernel
  could meet (round 6 of 7).
- **Quint** (quint-llm-kit): models of the transcript, GPU dispatch, PK cache and scheduler, plus model-based replay of real Rust traces
  (24 traces, 1,022 operations). It found that the GPU NTT had no CPU fallback.
- **Fuzzing** (W7): differential against stock and naive references, plus negative cases.

## Stock midnight-proofs 0.8.3 pitfalls found by the spec work (2026-10-24)
- **Multi-circuit committed instances:** the prover absorbs per circuit (`prover.rs:392-455`) but the verifier absorbs all committed
  instances first (`verifier.rs:61-76`). They agree only for one circuit or no committed instances. Never "fix" one side alone.
- **Not byte-reproducible:** leftover lookup-table rows are placed in HashMap order (`lookup/prover.rs:399-430`), and the vanishing
  randomness is split by `current_num_threads()` (`vanishing/prover.rs:49-71`). Byte-equality tests must pin the map order and the thread count;
  "verifier-accepted" is the property that always holds. Loki's BTreeMap made proofs reproducible.
- **Limb-wise equality:** `Fq` `==`, `Hash` and `is_zero` compare Montgomery limbs. A GPU output that is correct but not fully reduced breaks the `[1]`
  shortcut, zero filtering and hashing. Outputs must be fully reduced.
- **The prover never checks `Z_H | numerator`:** a bad witness silently gives a garbage quotient that gets truncated.
- **`msm_specific` zips silently** and truncates mismatched lengths; `from_raw_bytes` does no `< r` check.
- **GPU NTT:** a wrong root of unity or coset generator still passes round-trip tests (`roundtrip_does_not_pin_root`). Pin the domain ω and
  ZETA, and apply 1/n exactly once (sppark's inverse already scales).
- **Batch inversion must skip zeros**, as ff's `batch_invert` does.
- **Signed-digit Pippenger** needs one window more than ⌈255/c⌉, or it silently drops points.

## See also

- [[doctorwatson-workflow-review-2026-10-25]]: 2026-10-25 retrospective (orphaned locks, idle agents, DW121–DW130)
- [[doctorwatson-workflow-review-2026-10-22]] and [[doctorwatson-improvement-log]]
- Lessons index: [[lessons/README|lessons]]
