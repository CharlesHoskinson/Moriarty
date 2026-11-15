/-!
# Scheduling: setup mode, the box profile, the per-run schedule, the elastic property

Lean oracle of the Scheduling bounded context (slice B, `bus/ddd-B.md` section 1).
Specs: `sherlock-e2e-setup-mode` (STM-001 to STM-019), `sherlock-e2e-elastic-proving`
(ELA-001 to ELA-007); reused SPL-001 to SPL-012 (processor-final), LPC-006 (completion).

Abstraction:
* memory in bytes, time in milliseconds, digests as natural numbers;
* two leaf shapes (`caps`, steps per leaf `S` of each shape of the universal key set) and
  two devices (0 = GPU, 1 = CPU); calibration entry `e = shape * 2 + device`
  (`calT`, `calHost`, `calDev`: leaf time, host bytes, device bytes per leaf; `calMask` bit `e`
  set = the profile has entry `e`);
* one GPU per box (the multi-GPU calibration of STM-004 is OPEN in the spec);
* `claim_pub` of a prepared run is `claimOf prog input rP salt` (an abstract injective digest
  over the four values that fix a prepared run; ELA-001).

An operation is one natural number `op = code * 16 + arg` (`arg < 16`):

| code | operation | arg |
|---|---|---|
| 0 | `setup ok`: setup mode runs; every calibration proof verifies (ok = arg 0) or one is refused (arg 1) | 0..1 |
| 1 | `startRun`: a run starts with the box profile; the scheduler derives the schedule | - |
| 2 | `memLimit a`: `SHERLOCK_MEM_BUDGET` := `a * memUnit` | 0..15 |
| 3 | `gpus g`: `SHERLOCK_GPUS` := g (0 = no GPU) | 0..1 |
| 4 | `gpuMemLimit a`: `SHERLOCK_GPU_MEM` := `a * memUnit` | 0..15 |
| 5 | `forceCut k`: `T16_CUT` := the balanced cut of the run into `k` leaves | 0..15 |
| 6 | `prove`: the run is proved with the schedule | - |
| 7 | `newRun`: the same program and input are prepared again (fresh salt) | - |
| 8 | `moveTo b`: the prepared run moves to box `b` (that box's profile is needed) | 0..2 |
| 9 | `join v`: leaves of two boxes join under one N1; v = 1: one leaf fails the stock verifier | 0..1 |
| 10 | `earlyWrap`: the device guard is asked for an early wrap start | - |
-/

namespace Scheduling

/-- Observable state. Field names are the `--step` JSON keys. -/
structure St where
  -- box hardware as setup reads it (STM-001, STM-002, STM-003)
  memAvail : Nat          -- `MemAvailable` (bytes)
  cgroupHead : Nat        -- cgroup headroom of the prover process; 0 = no cgroup limit
  hasGpu : Bool           -- the box has a usable GPU
  gpuReportsMem : Bool    -- the GPU reports own memory (false: unified, STM-002)
  hwGpuTotal : Nat        -- VRAM total (bytes)
  hwGpuFree : Nat         -- VRAM free (bytes)
  cores : Nat             -- usable cores
  proverKeyD : Nat        -- the universal key set of the prover (`key_root`, `keys_d`)
  -- calibration of the universal key set's leaf shapes on this box (STM-004)
  caps : List Nat         -- S of shape 0 and shape 1
  calT : List Nat         -- leaf time (ms) of entry e = shape * 2 + device
  calHost : List Nat      -- host bytes of one leaf, per entry
  calDev : List Nat       -- device bytes of one leaf, per entry
  -- the profile `sherlock-profile/1` (STM-006, STM-007, STM-008)
  profWritten : Bool
  profFormatOk : Bool     -- `format` = `sherlock-profile/1`
  profFieldsOk : Bool     -- every one of the 11 fields present
  profKeyD : Nat          -- `key_root`/`keys_d` the profile was made with
  memBudget : Nat         -- `memory.mem_budget` (STM-003)
  unified : Bool          -- the GPU is unified (STM-002)
  gpuCount : Nat          -- 0 or 1
  gpuTotal : Nat          -- `vram_total` (0 = null for a unified GPU)
  gpuFree : Nat           -- `vram_free`
  profCores : Nat
  calMask : Nat           -- calibration entries in the profile (bits 0..3)
  setupStops : Nat        -- setup runs stopped by a refused calibration proof (STM-005)
  -- run-time limits (STM-010)
  memUnit : Nat
  limMemOn : Bool
  limMem : Nat
  gpusOff : Bool
  limGpuMemOn : Bool
  limGpuMem : Nat
  -- the prepared run (ELA-001)
  n : Nat                 -- run length in steps
  prog : Nat
  input : Nat
  rP : Nat
  salt : Nat
  box : Nat               -- 0 muninn, 1 huginn, 2 odin
  -- the schedule (STM-009 to STM-017); never enters a proof (SPL-011)
  scheduled : Bool
  calibrated : Bool       -- the profile is of this key set (STM-008)
  schShape : Nat
  schDevice : Nat         -- 0 GPU, 1 CPU
  schConc : Nat
  schEst : Nat            -- estimated leaf-stage wall time (ms)
  cut : List Nat
  forced : Bool           -- the cut is an operator override (`T16_CUT`)
  policy : Nat            -- wrap pipeline: 0 full overlap, 1 evict and overlap after the quotient, 2 no overlap
  -- the wrap pipeline inputs (STM-015, STM-019)
  peakStark : Nat
  peakWrap : Nat
  peakCoset : Nat
  wrapFree : Nat          -- free device bytes at the wrap start
  starkExtra : Nat
  margin : Nat
  earlyWraps : Nat
  -- run record (ELA-002, ELA-007)
  runs : Nat
  recBox : Nat
  recShape : Nat
  recDevice : Nat
  recLeaves : Nat
  recCalibrated : Bool
  recKeys : Nat
  recClaim : Nat
  nodes : Nat             -- N1 joins started (ELA-006)
  deriving DecidableEq, Repr

/-! ## Constants of the specs and the code -/

/-- STM-006: the 11 profile fields. -/
def profileFields : List String :=
  ["format", "host", "created", "key_root", "keys_d", "gpus", "cpu", "memory", "disk",
   "calibration", "setup_s"]
/-- STM-001: the fields of one `gpus` entry (index, name, compute capability, VRAM total,
    VRAM free, unified, maximum SM clock). -/
def gpuFields : List String :=
  ["index", "name", "compute_cap", "vram_total", "vram_free", "unified", "sm_clock_max_mhz"]
/-- STM-011: CPU threads per leaf (CPU concurrency at most max(1, cores / 4)). -/
def cpuThreadsPerLeaf : Nat := 4
/-- STM-015: headroom 1/16 of the device memory. -/
def pipeHeadroomDiv : Nat := 16
/-- STM-004: default `T16_CAL_REPS`. -/
def defaultCalReps : Nat := 3
def shapes : Nat := 2
def boxes : Nat := 3

/-! ## Arithmetic named as the code means it -/

/-- `a / d.max(1)` of `profile.rs` (division by a byte count of at least 1). -/
def divAtLeast1 (a d : Nat) : Nat := a / max d 1
/-- `n.div_ceil(c)` -/
def ceilDiv (n c : Nat) : Nat := (n + c - 1) / c

/-- STM-003: the memory budget is the lower of `MemAvailable` and the cgroup headroom. -/
def budgetOf (memAvail cgroupHead : Nat) : Nat :=
  if cgroupHead = 0 then memAvail else min memAvail cgroupHead

/-- `(x / 2^e) % 2 = 1`: entry `e` is in the profile. -/
def present (mask e : Nat) : Bool := (mask / 2 ^ e) % 2 == 1

def claimOf (prog input rP salt : Nat) : Nat :=
  ((prog * 1000003 + input) * 1000003 + rP) * 1000003 + salt

/-- STM-010: the host budget under the limit. -/
def hostBudget (s : St) : Nat := if s.limMemOn then min s.limMem s.memBudget else s.memBudget
/-- STM-010: `SHERLOCK_GPUS` only lowers the GPU count. -/
def gpusUsable (s : St) : Nat := if s.gpusOff then 0 else s.gpuCount

/-! ## The scheduler (STM-009, STM-011, STM-017) -/

/-- A candidate pick: one shape on one device. -/
structure Cand where
  shape : Nat
  dev : Nat
  m : Nat       -- leaves (fewest, `ceil(n / S)`)
  conc : Nat    -- leaf concurrency
  est : Nat     -- estimated leaf-stage time = waves * leaf time
  deriving DecidableEq, Repr

/-- STM-011: the leaf concurrency that the measured bytes allow, before the run's leaf count. -/
def concOf (s : St) (e host : Nat) : Nat :=
  let hb := s.calHost.getD e 0
  let db := s.calDev.getD e 0
  if e % 2 = 1 then
    -- CPU: host bytes under the budget, at most max(1, cores / 4)
    min (divAtLeast1 host hb) (max 1 (s.profCores / cpuThreadsPerLeaf))
  else if gpusUsable s = 0 then 0
  else if s.unified then
    -- one pool: device and host bytes of a leaf come from the same memory
    divAtLeast1 (if s.limGpuMemOn then min s.limGpuMem host else host) (db + hb)
  else
    -- discrete: device bytes in the free VRAM, host bytes under the host budget
    min (divAtLeast1 (if s.limGpuMemOn then min s.limGpuMem s.gpuFree else s.gpuFree) db)
        (divAtLeast1 host hb)

/-- Entry `e` is calibrated with a positive time on a shape with a key and the run is not empty. -/
def entryOk (s : St) (e : Nat) : Prop :=
  present s.calMask e = true ∧ 0 < s.calT.getD e 0 ∧ 0 < s.caps.getD (e / 2) 0 ∧ 0 < s.n

instance (s : St) (e : Nat) : Decidable (entryOk s e) := by unfold entryOk; infer_instance

def leavesOf (s : St) (e : Nat) : Nat := ceilDiv s.n (s.caps.getD (e / 2) 0)

def cand (s : St) (host e : Nat) : Option Cand :=
  let m := leavesOf s e
  let c := min (concOf s e host) m
  if entryOk s e ∧ 0 < c then
    some { shape := e / 2, dev := e % 2, m := m, conc := c, est := ceilDiv m c * s.calT.getD e 0 }
  else none

/-- STM-017: the strict order of the picks: least estimated time, then fewer leaves, then the
    lower shape index, then the GPU (device 0) before the CPU (device 1). -/
def Better (x y : Cand) : Prop :=
  x.est < y.est ∨ (x.est = y.est ∧ (x.m < y.m ∨ (x.m = y.m ∧
    (x.shape < y.shape ∨ (x.shape = y.shape ∧ x.dev < y.dev)))))

instance (x y : Cand) : Decidable (Better x y) := by unfold Better; infer_instance

def pickList : List Cand → Option Cand
  | [] => none
  | x :: xs =>
    match pickList xs with
    | none => some x
    | some y => if Better x y then some x else some y

def entries : List Nat := [0, 1, 2, 3]

def candsAt (s : St) (host : Nat) : List Cand := entries.filterMap (cand s host)

/-- The pick of the scheduler at a host budget. -/
def pickAt (s : St) (host : Nat) : Option Cand := pickList (candsAt s host)

def pick (s : St) : Option Cand := pickAt s (hostBudget s)

/-- The greedy cut of `profile.rs`/`sched.rs`: `k` leaves, each `min(cap, left - rest)`. -/
def cutAux (cap : Nat) : Nat → Nat → List Nat
  | _, 0 => []
  | left, k + 1 => let take := min cap (left - k); take :: cutAux cap (left - take) k

/-- The forced cut `T16_CUT` of `k` leaves (balanced). -/
def balCut (n k : Nat) : List Nat := (List.range k).map (fun i => n / k + if i < n % k then 1 else 0)

/-- STM-015: the wrap pipeline policy from the largest GPU memory less 1/16. -/
def policyOf (mem stark wrap coset : Nat) : Nat :=
  let usable := mem - mem / pipeHeadroomDiv
  if stark + wrap ≤ usable then 0
  else if stark - coset + wrap ≤ usable then 1
  else 2

/-- The device memory the policy is fitted to (VRAM total, or the host budget of a unified
    GPU), under the limits; `none` without a usable GPU. -/
def policyMem (s : St) : Option Nat :=
  if gpusUsable s = 0 then none
  else
    let m := if s.unified then hostBudget s else s.gpuTotal
    some (if s.limGpuMemOn then min s.limGpuMem m else m)

def policy (s : St) : Nat :=
  match policyMem s with
  | none => 2
  | some m => policyOf m s.peakStark s.peakWrap s.peakCoset

/-- STM-019: the wrap tail: the wrap peak less the resident cosets with full overlap and no
    eviction, else the whole wrap peak. -/
def wrapTail (s : St) : Nat := if s.policy = 0 then s.peakWrap - s.peakCoset else s.peakWrap

/-! ## Refusals -/

inductive Refusal where
  | setupStopped       -- STM-005 (a calibration proof refused: no profile)
  | noProfile          -- STM-016 (no profile at the box profile path)
  | profileRefused     -- STM-007
  | noShapeFits        -- STM-012
  | noSchedule         -- STM-009 (prove before a schedule)
  | shapeNoKey         -- STM-013
  | emptyCut           -- STM-013
  | leafOutOfRange     -- STM-013
  | cutNotCovering     -- STM-013
  | leafNotVerified    -- ELA-006
  | earlyWrapRefused   -- STM-019
  | outsideCapability
  deriving DecidableEq, Repr

def Refusal.code : Refusal → String
  | .setupStopped => "setup-stopped"
  | .noProfile => "no-profile"
  | .profileRefused => "profile-refused"
  | .noShapeFits => "no-shape-fits"
  | .noSchedule => "no-schedule"
  | .shapeNoKey => "shape-no-key"
  | .emptyCut => "empty-cut"
  | .leafOutOfRange => "leaf-out-of-range"
  | .cutNotCovering => "cut-not-covering"
  | .leafNotVerified => "leaf-not-verified"
  | .earlyWrapRefused => "early-wrap-refused"
  | .outsideCapability => "outside-capability"

/-- STM-007: a profile with another `format`, a missing field, or a calibration entry whose
    leaf time is not a positive time. -/
def profileOk (s : St) : Bool :=
  s.profFormatOk && s.profFieldsOk &&
  entries.all (fun e => !present s.calMask e || decide (0 < s.calT.getD e 0))

/-- STM-013: `check_free`, the only check on a schedule (no count list, no menu: STM-014). -/
def checkFree (caps : List Nat) (n shape : Nat) (cut : List Nat) : List Refusal :=
  let cap := caps.getD shape 0
  (if shape < shapes ∧ 0 < cap then [] else [.shapeNoKey]) ++
  (if cut = [] then [.emptyCut] else []) ++
  (if cut.all (fun c => decide (1 ≤ c ∧ c ≤ cap)) then [] else [.leafOutOfRange]) ++
  (if cut.sum = n then [] else [.cutNotCovering])

inductive Op where
  | setup (refused : Bool)
  | startRun
  | memLimit (a : Nat)
  | gpus (g : Nat)
  | gpuMemLimit (a : Nat)
  | forceCut (k : Nat)
  | prove
  | newRun
  | moveTo (b : Nat)
  | join (bad : Bool)
  | earlyWrap
  | bad
  deriving DecidableEq, Repr

def decodeOp (op : Nat) : Op :=
  let a := op % 16
  match op / 16 with
  | 0 => if a ≤ 1 then .setup (a == 1) else .bad
  | 1 => if a = 0 then .startRun else .bad
  | 2 => .memLimit a
  | 3 => if a ≤ 1 then .gpus a else .bad
  | 4 => .gpuMemLimit a
  | 5 => .forceCut a
  | 6 => if a = 0 then .prove else .bad
  | 7 => if a = 0 then .newRun else .bad
  | 8 => if a < boxes then .moveTo a else .bad
  | 9 => if a ≤ 1 then .join (a == 1) else .bad
  | 10 => if a = 0 then .earlyWrap else .bad
  | _ => .bad

/-- State domain (not reachability). -/
def Valid (s : St) : Prop :=
  s.caps.length = shapes ∧ s.calT.length = 4 ∧ s.calHost.length = 4 ∧ s.calDev.length = 4 ∧
  s.gpuCount ≤ 1 ∧ s.box < boxes ∧ s.calMask < 16 ∧ (s.scheduled = true → s.profWritten = true)

instance (s : St) : Decidable (Valid s) := by unfold Valid; infer_instance

def validState (s : St) : Bool := decide (Valid s)

def stepRefusals (s : St) (op : Nat) : List Refusal :=
  if validState s = false then [.outsideCapability] else
  match decodeOp op with
  | .setup refused => if refused then [.setupStopped] else []                       -- STM-005
  | .startRun =>
    if s.profWritten = false then [.noProfile]                                       -- STM-016
    else if profileOk s = false then [.profileRefused]                               -- STM-007
    else if pick s = none then [.noShapeFits]                                        -- STM-012
    else []
  | .memLimit _ => []                                                                -- STM-010
  | .gpus _ => []                                                                    -- STM-010
  | .gpuMemLimit _ => []                                                             -- STM-010
  | .forceCut _ => if s.scheduled then [] else [.noSchedule]                         -- STM-013
  | .prove =>
    if s.scheduled then checkFree s.caps s.n s.schShape s.cut                        -- STM-013
    else [.noSchedule]                                                               -- STM-009
  | .newRun => []                                                                    -- ELA-001
  | .moveTo _ => []                                                                  -- ELA-005
  | .join bad => if bad then [.leafNotVerified] else []                              -- ELA-006
  | .earlyWrap =>
    if s.scheduled = false then [.noSchedule]
    else if s.wrapFree < s.starkExtra + wrapTail s + s.margin then [.earlyWrapRefused]  -- STM-019
    else []
  | .bad => [.outsideCapability]

/-- Setup mode (STM-001 to STM-006): read the box, calibrate every shape on the CPU and on the
    GPU, verify every calibration proof, write the profile with this key set. It selects no key,
    no shape and no `plan_d` (SPL-011, UKS-008). -/
def setupOk (s : St) : St :=
  let uni := s.hasGpu && !s.gpuReportsMem
  { s with profWritten := true, profFormatOk := true, profFieldsOk := true,
           profKeyD := s.proverKeyD,
           memBudget := budgetOf s.memAvail s.cgroupHead,                -- STM-003
           unified := uni,                                               -- STM-002
           gpuCount := if s.hasGpu then 1 else 0,                        -- STM-001
           gpuTotal := if uni then 0 else s.hwGpuTotal,
           gpuFree := if uni then 0 else s.hwGpuFree,
           profCores := s.cores,
           calMask := if s.hasGpu then 15 else 10,                       -- STM-004 (CPU entries 1, 3)
           scheduled := false }

/-- STM-009: derive the schedule from the profile, the run length and the limits. -/
def schedule (s : St) (c : Cand) : St :=
  { s with scheduled := true,
           calibrated := decide (s.profKeyD = s.proverKeyD),           -- STM-008
           schShape := c.shape, schDevice := c.dev, schConc := c.conc, schEst := c.est,
           cut := cutAux (s.caps.getD c.shape 0) s.n c.m, forced := false,
           policy := policy s }                                          -- STM-015

def step (s : St) (op : Nat) : St :=
  match decodeOp op with
  | .setup refused => if refused then { s with setupStops := s.setupStops + 1 } else setupOk s  -- STM-005, STM-006
  | .startRun =>
    match pick s with
    | some c => schedule s c                                             -- STM-009, STM-017
    | none => s                                                          -- STM-012 (refused)
  | .memLimit a => { s with limMemOn := true, limMem := a * s.memUnit, scheduled := false }      -- STM-010
  | .gpus g => { s with gpusOff := g == 0, scheduled := false }                                  -- STM-010
  | .gpuMemLimit a => { s with limGpuMemOn := true, limGpuMem := a * s.memUnit, scheduled := false } -- STM-010
  | .forceCut k => { s with cut := balCut s.n k, forced := true }                                -- STM-013
  | .prove =>                                                                                    -- ELA-001, ELA-002, ELA-007
    { s with runs := s.runs + 1, recBox := s.box, recShape := s.schShape, recDevice := s.schDevice,
             recLeaves := s.cut.length, recCalibrated := s.calibrated, recKeys := s.proverKeyD,
             recClaim := claimOf s.prog s.input s.rP s.salt }
  | .newRun => { s with salt := s.salt + 1, scheduled := false }                                 -- ELA-001
  | .moveTo b => { s with box := b, profWritten := false, scheduled := false }                   -- ELA-005
  | .join _ => { s with nodes := s.nodes + 1 }                                                   -- ELA-006
  | .earlyWrap => { s with earlyWraps := s.earlyWraps + 1 }                                      -- STM-019
  | .bad => s

def apply (s : St) (op : Nat) : St := if stepRefusals s op = [] then step s op else s

def run (s : St) : List Nat → St
  | [] => s
  | op :: ops => run (apply s op) ops

def runRefusals (s : St) (_ops : List Nat) : List Refusal :=
  if validState s then [] else [.outsideCapability]

end Scheduling
