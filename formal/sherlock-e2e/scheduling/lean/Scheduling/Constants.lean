import Scheduling.Spec

/-!
# CONSTANT-CHECKED theorems of Scheduling

`decide` over recorded values (no `native_decide`). Every value cites its source; read by
Formal-Lean-B on 2026-11-15 with `git show`. Labels: CODE (a constant or a test fixture at the
cited commit), MEASURED (a run record), DERIVED (computed from MEASURED values by a stated rule).
Sources: openvm `t16-elastic` 1baf605 (`crates/sherlock-leaf/src/profile.rs`, abbreviated
`profile.rs`; `crates/sherlock-wrap-export/examples/common/t16.rs`); Moriarty `t16-elastic`
a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md` (abbreviated `RESULTS-CDE.md`).
-/

namespace Scheduling

/-- STM-006: CODE `profile.rs:79` to `:89` (`to_json`) writes exactly the 11 fields. -/
theorem stm006_eleven_fields : profileFields.length = 11 := by decide

/-- STM-001 (scenario "huginn": one `gpus` entry with 7 fields): CODE `profile.rs:82`. -/
theorem stm001_seven_gpu_fields : gpuFields.length = 7 := by decide

/-- STM-011: CODE `profile.rs:227` `CPU_THREADS_PER_LEAF = 4`. STM-015: CODE `profile.rs:407`
    `PIPE_HEADROOM_DIV = 16`. STM-004: CODE `t16.rs:677` `T16_CAL_REPS` default 3. -/
theorem code_constants : cpuThreadsPerLeaf = 4 ∧ pipeHeadroomDiv = 16 ∧ defaultCalReps = 3 := by
  decide

def GIB : Nat := 2 ^ 30
def MIB : Nat := 2 ^ 20
/-- CODE `profile.rs:366` `PEAKS` (MEASURED per its comment, huginn h102 and odin h51):
    STARK 8.2 GiB, wrap 12.4 GiB, cosets 7.5 GiB. -/
def peakStark0 : Nat := 82 * GIB / 10
def peakWrap0 : Nat := 124 * GIB / 10
def peakCoset0 : Nat := 75 * GIB / 10

/-- STM-015 on the three boxes (CODE test `pipeline_from_the_three_boxes`, `profile.rs:662`;
    DERIVED row `RESULTS-CDE.md:76`): odin RTX 5090 (32,607 MiB) full overlap; huginn RTX 5080
    (16,303 MiB) eviction with overlap after the quotient; muninn unified under the 16,949,428,224 B
    budget, eviction with overlap after the quotient; odin capped to 16 GiB, eviction; an 8 GiB GPU,
    no overlap; odin `o-cap` (`SHERLOCK_GPU_MEM` = 1,109,393,408 B, MEASURED `RESULTS-CDE.md:102`),
    no overlap. -/
theorem stm015_three_boxes :
    policyOf (32607 * MIB) peakStark0 peakWrap0 peakCoset0 = 0 ∧
    policyOf (16303 * MIB) peakStark0 peakWrap0 peakCoset0 = 1 ∧
    policyOf 16949428224 peakStark0 peakWrap0 peakCoset0 = 1 ∧
    policyOf (16 * GIB) peakStark0 peakWrap0 peakCoset0 = 1 ∧
    policyOf (8 * GIB) peakStark0 peakWrap0 peakCoset0 = 2 ∧
    policyOf 1109393408 peakStark0 peakWrap0 peakCoset0 = 2 := by decide

/-- STM-019 on odin h66/h70 (MEASURED per the comment of CODE test `cx_odin_h70_wrap_tail_counts_
    resident_cosets`, `profile.rs:684` to `:705`): 22,159 MiB free, STARK extra 9,300 MiB, margin
    768 MiB. The constant tail 12,300 MiB is refused 209 MiB short; the tail of the profile (wrap peak
    less the resident cosets, 5,017 MiB) is admitted. -/
theorem stm019_odin_h70 :
    22159 * MIB < 9300 * MIB + 12300 * MIB + 768 * MIB ∧
    9300 * MIB + 12300 * MIB + 768 * MIB - 22159 * MIB = 209 * MIB ∧
    (peakWrap0 - peakCoset0) / MIB = 5017 ∧
    9300 * MIB + (peakWrap0 - peakCoset0) + 768 * MIB ≤ 22159 * MIB := by decide

/-- STM-011 per-GPU leaf capacity (DERIVED row `RESULTS-CDE.md:75`: 15, 12, 27): huginn VRAM free
    15,533 MiB (CODE fixture `profile.rs:656`, its `nvidia-smi` line) over 1.026 GB device bytes;
    muninn unified budget 16,949,428,224 B over 0.375 + 1.012 GB (device + host, `RESULTS-CDE.md:74`);
    odin VRAM free 30,406,606,848 B (CODE fixture `profile.rs:695`) over 1.109 GB, host budget
    93,079,199,744 B over 0.987 GB. -/
theorem stm011_leaf_capacity :
    divAtLeast1 (15533 * MIB) 1026000000 = 15 ∧
    divAtLeast1 16949428224 (375000000 + 1012000000) = 12 ∧
    min (divAtLeast1 30406606848 1109000000) (divAtLeast1 93079199744 987000000) = 27 := by decide

/-- STM-009, STM-011 (the estimate is waves times the measured leaf time): the DERIVED schedules of
    `RESULTS-CDE.md` section 3.2 (400 and 4,000 steps, 100 and 1,000 leaves) and the MEASURED caps
    `h-cap`, `o-cap` (`RESULTS-CDE.md:100`, `:102`: 2 leaves, 1 at once, 2 waves, 4.188 s and
    6.316 s). Leaf times 2.094 s (huginn), 23.729 s (muninn), 3.158 s (odin) in ms; the recorded
    estimates use unrounded leaf times, so each matches within one ms per wave. -/
def whatif : List (Nat × Nat × Nat × Nat × Nat) :=
  -- (leaves, concurrency, leaf ms, recorded waves, recorded estimate ms)
  [(100, 15, 2094, 7, 14657), (100, 12, 23729, 9, 213563), (100, 27, 3158, 4, 12631),
   (1000, 15, 2094, 67, 140293), (1000, 12, 23729, 84, 1993259), (1000, 27, 3158, 38, 119994),
   (2, 1, 2094, 2, 4188), (2, 1, 3158, 2, 6316)]

theorem stm011_estimates_match_records :
    whatif.all (fun (m, c, t, w, e) =>
      ceilDiv m c == w && (if ceilDiv m c * t ≥ e then ceilDiv m c * t - e else e - ceilDiv m c * t) ≤ w)
      = true := by decide

end Scheduling
