import Scheduling.Spec

/-!
# Step-level pins and reachable witnesses of Scheduling

Expected values are written from the requirement sentences and the T16 records
(`RESULTS-CDE.md` of Moriarty `t16-elastic` a7a7b8867), not copied from `#eval`. Each pin is
checked by `decide +kernel` and by `#guard`. Memory in MB (10^6 bytes), time in ms.

`hug0` is huginn with its MEASURED calibration (`RESULTS-CDE.md:45` to `:50`: shape 0 CPU
5.604 s, 1.747 GB host; shape 0 GPU 2.135 s, 1.001 GB host, 1.026 GB device; shape 1 CPU 5.355 s,
1.747 GB; shape 1 GPU 2.094 s, 0.997 GB host, 1.026 GB device) and VRAM 16,303 MiB total,
15,533 MiB free (`profile.rs:656`). Assumed (not recorded in the result file): host budget
30,000 MB, 24 cores, S = 2 for shape 0; S = 4 for shape 1 follows from the recorded cut 4, 1 of the
5-step slice run.
-/

namespace Scheduling

def hug0 : St :=
  { memAvail := 30000, cgroupHead := 0, hasGpu := true, gpuReportsMem := true,
    hwGpuTotal := 17095, hwGpuFree := 16287, cores := 24, proverKeyD := 1,
    caps := [2, 4], calT := [2135, 5604, 2094, 5355], calHost := [1001, 1747, 997, 1747],
    calDev := [1026, 0, 1026, 0],
    profWritten := false, profFormatOk := false, profFieldsOk := false, profKeyD := 0,
    memBudget := 0, unified := false, gpuCount := 0, gpuTotal := 0, gpuFree := 0, profCores := 0,
    calMask := 0, setupStops := 0, memUnit := 1026, limMemOn := false, limMem := 0,
    gpusOff := false, limGpuMemOn := false, limGpuMem := 0, n := 5, prog := 11, input := 22,
    rP := 33, salt := 44, box := 1, scheduled := false, calibrated := false, schShape := 0,
    schDevice := 0, schConc := 0, schEst := 0, cut := [], forced := false, policy := 0,
    peakStark := 8800, peakWrap := 13300, peakCoset := 8050, wrapFree := 23235, starkExtra := 9751,
    margin := 805, earlyWraps := 0, runs := 0, recBox := 0, recShape := 0, recDevice := 0,
    recLeaves := 0, recCalibrated := false, recKeys := 0, recClaim := 0, nodes := 0 }

/-- setup, then a run from the profile (`h-gpu`). -/
def hugRun : St := run hug0 [0, 16]
def hugProved : St := apply hugRun (6 * 16)

-- STM-009, STM-017 (`h-gpu`, RESULTS-CDE.md:77, 3.1): shape 1, GPU, cut 4, 1, 2 at once, 1 wave,
-- estimate 2,094 ms; the profile is of this key set (STM-008).
theorem pin_hgpu : hugRun.scheduled = true ∧ hugRun.schShape = 1 ∧ hugRun.schDevice = 0 ∧
    hugRun.cut = [4, 1] ∧ hugRun.schConc = 2 ∧ hugRun.schEst = 2094 ∧ hugRun.calibrated = true := by
  decide +kernel
#guard hugRun.cut == [4, 1] && hugRun.schEst == 2094 && hugRun.schDevice == 0

-- STM-010, ELA-004 (`h-nogpu`): SHERLOCK_GPUS = 0 moves the pick to the CPU, shape 1, 2 at once,
-- estimate 5,355 ms.
def hugNoGpu : St := run hug0 [0, 3 * 16, 16]
theorem pin_hnogpu : hugNoGpu.schDevice = 1 ∧ hugNoGpu.schShape = 1 ∧ hugNoGpu.schConc = 2 ∧
    hugNoGpu.schEst = 5355 := by decide +kernel
#guard hugNoGpu.schDevice == 1 && hugNoGpu.schEst == 5355

-- STM-010, ELA-003 (`h-cap`): SHERLOCK_GPU_MEM = one leaf's device bytes: shape 1, GPU, 1 at
-- once, 2 waves, 4,188 ms.
def hugCap : St := run hug0 [0, 4 * 16 + 1, 16]
theorem pin_hcap : hugCap.schDevice = 0 ∧ hugCap.schShape = 1 ∧ hugCap.schConc = 1 ∧
    hugCap.schEst = 4188 := by decide +kernel
#guard hugCap.schConc == 1 && hugCap.schEst == 4188

-- STM-010 (scenario "A limit above the profile"): a limit above the profile budget uses the
-- profile budget.
theorem pin_stm010 : hostBudget { (run hug0 [0]) with limMemOn := true, limMem := 80000 } = 30000 := by
  decide +kernel
#guard hostBudget { (run hug0 [0]) with limMemOn := true, limMem := 80000 } == 30000

-- STM-012 (scenario "A 1 GB budget", here 0): a budget that fits no leaf on any device is refused.
theorem pin_stm012 : (stepRefusals (run hug0 [0, 2 * 16]) 16).map Refusal.code = ["no-shape-fits"] := by
  decide +kernel
#guard (stepRefusals (run hug0 [0, 2 * 16]) 16).map Refusal.code == ["no-shape-fits"]

-- STM-003 (scenarios): a cgroup headroom of 60 GB below MemAvailable 100 GB gives 60 GB; no
-- cgroup limit gives 100 GB.
theorem pin_stm003 : budgetOf 100000 60000 = 60000 ∧ budgetOf 100000 0 = 100000 ∧
    (run { hug0 with memAvail := 100000, cgroupHead := 60000 } [0]).memBudget = 60000 := by
  decide +kernel
#guard budgetOf 100000 60000 == 60000 && budgetOf 100000 0 == 100000

-- STM-002 (scenario "muninn"): a GPU that reports no own memory is unified, `vram_total` null (0).
theorem pin_stm002 : (run { hug0 with gpuReportsMem := false } [0]).unified = true ∧
    (run { hug0 with gpuReportsMem := false } [0]).gpuTotal = 0 := by decide +kernel
#guard (run { hug0 with gpuReportsMem := false } [0]).unified

-- STM-001, STM-004: setup records one GPU and calibrates every shape on the CPU and the GPU
-- (entries 0..3); a box without a GPU only on the CPU (entries 1, 3).
theorem pin_stm004 : (run hug0 [0]).gpuCount = 1 ∧ (run hug0 [0]).calMask = 15 ∧
    (run { hug0 with hasGpu := false } [0]).calMask = 10 := by decide +kernel
#guard (run hug0 [0]).calMask == 15

-- STM-005: a refused calibration proof stops setup and writes no profile; a run then has none.
theorem pin_stm005 : (stepRefusals hug0 1).map Refusal.code = ["setup-stopped"] ∧
    (run hug0 [1]).profWritten = false ∧
    (stepRefusals (run hug0 [1]) 16).map Refusal.code = ["no-profile"] := by decide +kernel
#guard (stepRefusals hug0 1).map Refusal.code == ["setup-stopped"]

-- STM-007 (scenario "A zero leaf time"): a present entry with leaf time 0 refuses the profile;
-- another format refuses it too.
theorem pin_stm007 :
    (stepRefusals { (run hug0 [0]) with calT := [0, 5604, 2094, 5355] } 16).map Refusal.code =
      ["profile-refused"] ∧
    (stepRefusals { (run hug0 [0]) with profFormatOk := false } 16).map Refusal.code =
      ["profile-refused"] := by decide +kernel
#guard (stepRefusals { (run hug0 [0]) with calT := [0, 5604, 2094, 5355] } 16).map Refusal.code == ["profile-refused"]

-- STM-008 (scenario "The profile after the rekey"): a profile of the T10m keys with the T25 keys
-- gives a run recorded as not calibrated.
def afterRekey : St := apply { (run hug0 [0]) with proverKeyD := 2 } 16
theorem pin_stm008 : afterRekey.calibrated = false ∧ afterRekey.scheduled = true ∧
    (apply afterRekey (6 * 16)).recCalibrated = false := by decide +kernel
#guard !afterRekey.calibrated

-- STM-013 (scenario "A forced cut of five leaves") and ELA-001 (scenario "Five leaves against
-- one"): the cut 1,1,1,1,1 passes check_free and gives the claim of the profile cut.
def hugC5 : St := run hugRun [5 * 16 + 5, 6 * 16]
theorem pin_hc5 : hugC5.recLeaves = 5 ∧ hugC5.runs = 1 ∧ hugC5.recClaim = hugProved.recClaim := by
  decide +kernel
#guard hugC5.recLeaves == 5 && hugC5.recClaim == hugProved.recClaim

-- STM-013: an empty cut, a leaf above S (one leaf of 5 > 4), a leaf of 0 steps, a cut that
-- does not cover the run.
theorem pin_stm013 :
    (stepRefusals (apply hugRun (5 * 16 + 0)) (6 * 16)).map Refusal.code = ["empty-cut", "cut-not-covering"] ∧
    (stepRefusals (apply hugRun (5 * 16 + 1)) (6 * 16)).map Refusal.code = ["leaf-out-of-range"] ∧
    (stepRefusals (apply hugRun (5 * 16 + 6)) (6 * 16)).map Refusal.code = ["leaf-out-of-range"] ∧
    (stepRefusals { hugRun with cut := [1, 1] } (6 * 16)).map Refusal.code = ["cut-not-covering"] ∧
    (stepRefusals { hugRun with schShape := 2 } (6 * 16)).map Refusal.code =
      ["shape-no-key", "leaf-out-of-range"] := by decide +kernel
#guard (stepRefusals { hugRun with cut := [1, 1] } (6 * 16)).map Refusal.code == ["cut-not-covering"]

-- STM-017 (scenario "A tie with reversed entries"): equal estimated time in both orders gives
-- one pick (fewer leaves first, then the lower shape, then the GPU).
def tieA : Cand := { shape := 0, dev := 1, m := 2, conc := 2, est := 3000 }
def tieB : Cand := { shape := 1, dev := 0, m := 2, conc := 2, est := 3000 }
def tieC : Cand := { shape := 1, dev := 1, m := 2, conc := 2, est := 3000 }
theorem pin_stm017 : pickList [tieA, tieB] = some tieA ∧ pickList [tieB, tieA] = some tieA ∧
    pickList [tieC, tieB] = some tieB ∧ pickList [tieB, tieC] = some tieB := by decide +kernel
#guard pickList [tieB, tieA] == some tieA && pickList [tieC, tieB] == some tieB

-- STM-018 (scenario "A sequence of budgets"; the lane witness of `cx_f9`, profile.rs:632):
-- CPU only, shape 0 (4 GiB, 2.0 s) and shape 1 (1 GiB, 5.0 s), 13 steps, 16 cores; budgets of
-- 8 down to 1 GiB never give a faster pick (the pick moves from shape 0 to shape 1 at 7 GiB).
-- Expected by hand: 4 leaves each; shape 0 runs b/4 at once (2 waves at 8 GiB: 4,000 ms; 4 waves
-- below: 8,000 ms; none below 4 GiB); shape 1 runs min(b, 4) at once (5,000 ms down to 4 GiB, then
-- 2 waves at 3 and 2 GiB, 4 waves at 1 GiB).
def cx9 : St :=
  { hug0 with
    calMask := 10
    profCores := 16
    caps := [4, 4]
    n := 13
    calT := [0, 2000, 0, 5000]
    calHost := [0, 4, 0, 1]
    calDev := [0, 0, 0, 0]
    gpuCount := 0 }
def cx9Est (b : Nat) : Nat := match pickAt cx9 b with | some c => c.est | none => 0
theorem pin_stm018 : (List.range 8).map (fun i => cx9Est (8 - i)) = [4000, 5000, 5000, 5000, 5000, 10000, 10000, 20000] := by
  decide +kernel
#guard ((List.range 8).map (fun i => cx9Est (8 - i))) == [4000, 5000, 5000, 5000, 5000, 10000, 10000, 20000]

-- STM-015 (scenarios): a large GPU gives full overlap, a 16 GB one eviction, a small one none;
-- no GPU, no overlap.
theorem pin_stm015 : hugRun.policy = 1 ∧ (run { hug0 with hwGpuTotal := 34191 } [0, 16]).policy = 0 ∧
    (run { hug0 with hwGpuTotal := 8590 } [0, 16]).policy = 2 ∧ hugNoGpu.policy = 2 := by
  decide +kernel
#guard hugRun.policy == 1

-- STM-019 (scenario "One byte short"): free bytes one below STARK extra + tail + margin refuse
-- the early wrap start; at the sum it is admitted.
theorem pin_stm019 :
    (stepRefusals { hugRun with wrapFree := 9751 + 13300 + 805 - 1 } (10 * 16)).map Refusal.code =
      ["early-wrap-refused"] ∧
    stepRefusals { hugRun with wrapFree := 9751 + 13300 + 805 } (10 * 16) = [] := by decide +kernel
#guard (stepRefusals { hugRun with wrapFree := 9751 + 13300 + 805 - 1 } (10 * 16)).map Refusal.code == ["early-wrap-refused"]

-- ELA-001 (scenario "Two fresh runs"): a new preparation gives another claim.
theorem pin_ela001_fresh : (run hugRun [7 * 16, 16, 6 * 16]).recClaim ≠ hugProved.recClaim := by
  decide +kernel
#guard (run hugRun [7 * 16, 16, 6 * 16]).recClaim != hugProved.recClaim

-- ELA-005, ELA-002: the prepared run moved to odin (RTX 5090, 34,191 MB total and 30,407 MB free
-- per `profile.rs:695`; MEASURED calibration `RESULTS-CDE.md:47` to `:50`; host 93,079 MB), proved
-- with odin's own profile (`o-gpu`: shape 1, GPU, 2 at once, est. 3.158 s, full overlap), gives the
-- same claim under the same keys.
def odinRun : St :=
  run { hugProved with hwGpuTotal := 34191, hwGpuFree := 30407, memAvail := 93079,
                       calT := [3204, 8451, 3158, 8371], calDev := [1109, 0, 1109, 0],
                       calHost := [986, 1745, 987, 1745], cores := 20 }
      [8 * 16 + 2, 0, 16, 6 * 16]
theorem pin_ela005 : odinRun.recBox = 2 ∧ odinRun.schEst = 3158 ∧ odinRun.policy = 0 ∧
    odinRun.recClaim = hugProved.recClaim ∧ odinRun.recKeys = hugProved.recKeys := by decide +kernel
#guard odinRun.recClaim == hugProved.recClaim && odinRun.schEst == 3158

-- ELA-004, STM-010 (`o-nogpu`, RESULTS-CDE.md:101): odin with SHERLOCK_GPUS = 0: CPU, shape 1,
-- 2 at once, estimate 8,371 ms, no overlap.
def odinNoGpu : St := run odinRun [3 * 16, 16]
theorem pin_onogpu : odinNoGpu.schDevice = 1 ∧ odinNoGpu.schShape = 1 ∧ odinNoGpu.schConc = 2 ∧
    odinNoGpu.schEst = 8371 ∧ odinNoGpu.policy = 2 := by decide +kernel
#guard odinNoGpu.schEst == 8371

-- ELA-006 (scenario "A corrupted leaf from one box"): the join stops before N1.
theorem pin_ela006 : (stepRefusals hugProved (9 * 16 + 1)).map Refusal.code = ["leaf-not-verified"] ∧
    (apply hugProved (9 * 16)).nodes = 1 := by decide +kernel
#guard (stepRefusals hugProved (9 * 16 + 1)).map Refusal.code == ["leaf-not-verified"]

-- ELA-007: the record names the box, the shape and the device, the shard count, the key set and
-- the claim.
theorem pin_ela007 : hugProved.recBox = 1 ∧ hugProved.recShape = 1 ∧ hugProved.recDevice = 0 ∧
    hugProved.recLeaves = 2 ∧ hugProved.recKeys = 1 ∧
    hugProved.recClaim = claimOf 11 22 33 44 := by decide +kernel
#guard hugProved.recLeaves == 2

-- outside the capability
theorem pin_bad : (stepRefusals hug0 (11 * 16)).map Refusal.code = ["outside-capability"] ∧
    (stepRefusals { hug0 with caps := [1] } 0).map Refusal.code = ["outside-capability"] := by
  decide +kernel
#guard (stepRefusals hug0 (15 * 16)).map Refusal.code == ["outside-capability"]

end Scheduling
