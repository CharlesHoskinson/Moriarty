# Dense memory Quint verdict (tasks 1.3 and 2.4)

Lead lane kit-membackend, 2026-11-15. Run: `qa/gemini-quint/dense-mem/rerun.sh <outdir>`, 49 commands; results in `rerun-results.txt`.

## Models

- `models/sherlock/quint/denseMemory.qnt` and `denseMemory_test.qnt`: the C1 dense-memory model, moved from `qa/gemini-quint/c1-dense/dense.qnt` (task 1.3). The only edit is the test import `from "./denseMemory"`. Commands 1-30 re-run the C1 verdict: `dense_sound` holds `invRefine` (4,000 samples, 3 steps, seed 0x20261111), the two reachability probes are violated as required, the test file has 34 passing tests, and all 23 mutant modules are caught.
- `models/sherlock/quint/denseStark.qnt` and `denseStark_test.qnt`: the STARK kit as built (`crates/sherlock-leaf/src/heap.rs`, openvm branch `kit-membackend` 1abcfa0). It models `HeapReadAir` (capacity-tagged record and node digests, boolean bits, limb accumulator, root bound to `pre.root`) and `SegAir` plus `FlushAir` (tail open on every leaf, PAD mask, `f` iff `r_pre + appends >= SEGC`, old walk from `EMPTY_SEG` to the pre root, new walk on the same siblings). Parameters: SEGC 2, TOP 2, W 1. The hash is keyed mod 2^31 - 1. The pre state may hold an extra occupant above `q` (`Dense.extra` of the kit), so the occupied-slot attacks are reachable.

## Results (`denseStark`, commands 31-49, 3,000 samples, 3 steps, seed 0x20261115)

| Property or mutant | Result |
|---|---|
| `inv` = `invLeaf` and `invColdRead` on `denseStark_sound` | holds |
| `reachFlush`, `reachNoAppendLeaf`, `reachColdRead` (non-vacuity) | violated, as required |
| `cold_value_forged`, `wrong_kind_cold`, `cold_read_root_unbound` | caught |
| `cold_route_forged` (CORE's `addr < base` route dropped) | caught |
| `forged_tail_across_leaf_boundary`, `tail_open_skipped_when_no_append`, `non_canonical_tail` | caught |
| `flush_skipped`, `flush_spurious` | caught |
| `address_reuse_occupied_slot`, `empty_seg_forgery`, `walk_sibling_differs_old_new` | caught |
| `cold_read_uses_running_root` | GUARD HOLDS (no violation, expected exit 0) |
| `denseStark_test` | 8 passing |

## Findings

1. **The cold read needs CORE's route.** On a pre state with an occupant above `q`, the backend alone accepts a read of that occupant: the walk reaches `pre.root`. CORE's route `addr < base` (kit CORE, the `ReadDriver` stand-in does not check it) is what refuses it. The model states the route as an upstream constraint, and `cold_route_forged` shows that it is load-bearing. Batch 3 must keep the route on CORE's read port.
2. **`cold_read_uses_running_root` is a second guard.** With the route in place, a read below `base` against the post root opens the same record, because a flush writes only slot `q_pre`. The kit mutant (read of address 35 at `len_pre` 32) is caught by the STARK gate because the `ReadDriver` stand-in has no route. The `pre.root` binding (MBK spec) stays as defence in depth. Lean O16 `walk_root_determines` states the same: a walk proves the leaf of whichever tree it reaches.
3. **Not modelled:** `walk_bit_nonboolean` and `domain_tag_swapped_node_height`. With a collision-free hash and an honest root there is no semantic counterexample. Their soundness is Lean O16 (`MemContract.walk_sound`: the bits of the address, height-tagged nodes). The STARK gate proves both mutants against an attacker-chosen root.

## Lean

`clean/fork/Clean/Sherlock/MemContract.lean`, first committed in 21d25b519 (swept in by the VALUE DIGEST commit): O14 `partition_independent_window`, O15 `ones_fit_window`, O16 `walk_sound` and `cold_read_sound`. Axioms: propext, Quot.sound and Classical.choice only (`MemContractAxioms.lean`).
