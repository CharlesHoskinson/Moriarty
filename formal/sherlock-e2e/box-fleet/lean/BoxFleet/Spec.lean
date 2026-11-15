/-!
# Box Fleet: owned boxes, heavy-slot admission, `gpu.lock`, box bus, GPU paths, prover daemon

Lean oracle of the Box Fleet bounded context (slice B, `bus/ddd-B.md` section 1).
Specs: `sherlock-e2e-box-fleet` (BXF-001 to BXF-014), `sherlock-e2e-gpu-proving`
(GPV-001 to GPV-007), `sherlock-e2e-prover-daemon` (PDM-001 to PDM-009); reused PRD-001 to
PRD-005, GPW-007 (processor-final), LPC-005, LPC-007 (completion).

Abstraction: boxes are 0 muninn, 1 huginn, 2 odin, 3 a fourth owned box. Memory is in
units of 8 GB. Digests and manifests are natural numbers. One `gpu.lock` (one box) is
modelled; every box runs the same rule.

An operation is one natural number `op = code * 16 + arg` (`arg < 16`):

| code | operation | arg |
|---|---|---|
| 0 | `submitJob h`: a proving job names host `h` | host 0..3 |
| 1 | `sharedJob m`: a shared `heavy-slot` job asks for `8 m` GB | m 0..15 |
| 2 | `exclusiveJob`: `heavy-slot --exclusive` | 0 |
| 3 | `endShared` | 0 |
| 4 | `endExclusive` | 0 |
| 5 | `addBox h`: the user adds box `h` to the roster | host 0..3 |
| 6 | `gpuAcquire k`: a process takes `gpu.lock`; k 1 prover, 2 `wrapd`, 3 sampler or monitor of an ended job whose parent is gone, 4 other | 1..4 |
| 7 | `gpuRelease`: the holder releases the lock | 0 |
| 8 | `idle m`: the holder makes no progress for `m` minutes | 0..15 |
| 9 | `sigterm`: the box agent logs the holder and sends SIGTERM | 0 |
| 10 | `wait t`: `10 t` seconds pass after the SIGTERM | 0..15 |
| 11 | `sigkill`: the box agent sends SIGKILL | 0 |
| 12 | `transfer a`: a result file moves from `a / 4` to `a % 4` | 0..15 |
| 13 | `outbox b`: box `b` writes a line to its outbox | 0..3 |
| 14 | `linkDrop b`: the link to box `b` drops | 1..2 |
| 15 | `queueJob b`: a job is queued on box `b` | 1..2 |
| 16 | `daemonStart m`: `wrapd serve` starts with key manifest `m` | 0..15 |
| 17 | `request k`: k 0 `prove`, 1 `call`, 2 `stats`, 3 `quit`, 4..15 another operation | 0..15 |
| 18 | `makeSlot o`: a blinding slot is made; o 0 by the daemon, 1 by a helper | 0..1 |
| 19 | `exportSlot`: a slot is serialized off the machine | 0 |
| 20 | `proveShard f`: one shard is proved; f 1 = a device error | 0..1 |
| 21 | `startRun a`: a run starts; planned minutes `10 * (a / 2)`, as a unit when `a % 2 = 1` | 0..15 |
| 22 | `speedClaim a`: a speed gain from a run on box `a / 4` against a run on box `a % 4` | 0..15 |
| 23 | `gpuQuery q`: q 0 `nvidia-smi --query-gpu`, 1 CUDA memory call, 2 `nvidia-smi conf-compute` | 0..2 |
-/

namespace BoxFleet

/-- Observable state. Field names are the `--step` JSON keys. -/
structure St where
  /-- bit h set: box h is in the owned-box roster of the user (BXF-001) -/
  roster : Nat
  /-- shared `heavy-slot` jobs running on muninn (BXF-002, BXF-003) -/
  slotsUsed : Nat
  /-- an exclusive `heavy-slot` job runs (BXF-003) -/
  exclRunning : Bool
  /-- proving jobs started on boxes other than muninn (BXF-001) -/
  remoteJobs : Nat
  /-- holder of `gpu.lock`: 0 none, 1 prover, 2 `wrapd`, 3 eligible orphan, 4 or more other (BXF-004) -/
  gpuHolder : Nat
  /-- minutes the holder made no progress (BXF-005) -/
  idleMin : Nat
  /-- SIGTERM sent to the holder (BXF-005, BXF-013) -/
  termSent : Bool
  /-- seconds since the SIGTERM (BXF-013) -/
  termAgeS : Nat
  /-- lines of `~/.cache/box-bus/all.log` (BXF-005, BXF-008, BXF-013) -/
  busLog : Nat
  /-- the sha256 of the moving file equals the sha256 in the bus message (BXF-007) -/
  shaOk : Bool
  /-- accepted file transfers (BXF-007, BXF-014) -/
  transfers : Nat
  /-- the owner is away (BXF-010) -/
  ownerAway : Bool
  /-- queued jobs of huginn and odin (BXF-010) -/
  q1 : Nat
  q2 : Nat
  /-- `wrapd serve` runs (PDM-001) -/
  daemonUp : Bool
  /-- mode of the daemon socket; 384 = 0o600 (PDM-002) -/
  socketMode : Nat
  /-- keys, SRS, tables and the CUDA context are loaded (PDM-001, PDM-004) -/
  keysLoaded : Bool
  /-- key manifest the daemon loaded (PDM-006) -/
  daemonManifest : Nat
  /-- key manifest the next request names (PDM-006) -/
  reqManifest : Nat
  /-- the next request comes from a peer of the same user on the same machine (PRD-001) -/
  reqPeerOk : Bool
  /-- leaf count of the next `call` (PDM-007) -/
  reqLeaves : Nat
  /-- `prove` and `call` requests served (PDM-005, PDM-009) -/
  served : Nat
  /-- error answers to unknown operations (PDM-003) -/
  errors : Nat
  /-- `stats` answers, and witness words in them (PDM-008) -/
  statsReplies : Nat
  statsWitness : Nat
  /-- unused blinding slots, slots made, slots consumed (PRD-002, PDM-005, PDM-009) -/
  poolFree : Nat
  slotsMade : Nat
  slotsSpent : Nat
  /-- path of the last served call: 0 none, 1 full tree, 3 path C (PDM-007, PRD-004) -/
  lastPath : Nat
  /-- the build has the cargo feature `cuda` (GPV-001, GPV-004, GPV-005, GPV-007) -/
  cuda : Bool
  /-- `T7_NODE_GPU`: 0 unset, 1 `0`, 2 `off`, 3 another value (GPV-004) -/
  nodeEnv : Nat
  /-- `T14_L_GPU`: 0 unset, 1 `off`, 2 another value (GPV-005) -/
  lEnv : Nat
  /-- shards proved (GPV-006) -/
  shardsProved : Nat
  /-- the run stopped on a device error, naming the shard and the device (GPV-006) -/
  runStopped : Bool
  stopShard : Nat
  stopDevice : Nat
  /-- device of the last shard: 0 CPU, 1 GPU; app and leaf layer, nodes, `L` -/
  lastAppDev : Nat
  lastNodeDev : Nat
  lastLDev : Nat
  /-- runs started, started as `systemd-run --user` units (BXF-009) -/
  runsStarted : Nat
  unitRuns : Nat
  /-- speed claims stated (BXF-011), GPU data reads (BXF-012) -/
  claims : Nat
  gpuQueries : Nat
  deriving DecidableEq, Repr

/-- CODE `~/.local/bin/heavy-slot:27`, `:28`: `SLOTS=3`, `BUDGET=64` (GB). -/
def heavySlots : Nat := 3
def heavyBudgetG : Nat := 64
/-- BXF-005: more than 10 minutes with no progress. BXF-013: 30 seconds after SIGTERM. -/
def orphanIdleMin : Nat := 10
def killWaitS : Nat := 30
/-- BXF-009: a run planned over 30 minutes is a unit. -/
def unitMinutes : Nat := 30
/-- PDM-002: socket mode 0600. -/
def mode0600 : Nat := 384

inductive Refusal where
  | hostNotInRoster     -- BXF-001
  | noSlot              -- BXF-002, BXF-003 (admission waits)
  | overBudget          -- BXF-003
  | noJob               -- BXF-002 (nothing to end)
  | gpuLocked           -- BXF-004
  | noHolder            -- BXF-004
  | holderProtected     -- BXF-006
  | notEligible         -- BXF-005, BXF-013
  | alreadySignalled    -- BXF-005
  | notViaMuninn        -- BXF-014
  | shaMismatch         -- BXF-007
  | daemonRunning       -- PDM-001
  | daemonDown          -- PDM-001
  | peerRefused         -- PRD-001
  | manifestMismatch    -- PDM-006
  | noBlindingSlot      -- PRD-002, PDM-005
  | helperSlot          -- PRD-002
  | slotOffMachine      -- PRD-002, PRD-003
  | runStopped          -- GPV-006
  | needsUnit           -- BXF-009
  | crossBoxClaim       -- BXF-011
  | confComputeForbidden -- BXF-012
  | outsideCapability
  deriving DecidableEq, Repr

def Refusal.code : Refusal → String
  | .hostNotInRoster => "host-not-in-roster"
  | .noSlot => "no-slot"
  | .overBudget => "over-budget"
  | .noJob => "no-job"
  | .gpuLocked => "gpu-locked"
  | .noHolder => "no-holder"
  | .holderProtected => "holder-protected"
  | .notEligible => "not-eligible"
  | .alreadySignalled => "already-signalled"
  | .notViaMuninn => "not-via-muninn"
  | .shaMismatch => "sha256-mismatch"
  | .daemonRunning => "daemon-running"
  | .daemonDown => "daemon-down"
  | .peerRefused => "peer-refused"
  | .manifestMismatch => "manifest-mismatch"
  | .noBlindingSlot => "no-blinding-slot"
  | .helperSlot => "helper-slot"
  | .slotOffMachine => "slot-off-machine"
  | .runStopped => "run-stopped"
  | .needsUnit => "needs-unit"
  | .crossBoxClaim => "cross-box-claim"
  | .confComputeForbidden => "conf-compute-forbidden"
  | .outsideCapability => "outside-capability"

inductive Op where
  | submitJob (h : Nat) | sharedJob (m : Nat) | exclusiveJob | endShared | endExclusive
  | addBox (h : Nat) | gpuAcquire (k : Nat) | gpuRelease | idle (m : Nat) | sigterm
  | wait (t : Nat) | sigkill | transfer (src dst : Nat) | outbox (b : Nat) | linkDrop (b : Nat)
  | queueJob (b : Nat) | daemonStart (m : Nat) | request (k : Nat) | makeSlot (o : Nat)
  | exportSlot | proveShard (f : Nat) | startRun (minutes : Nat) (unit : Bool)
  | speedClaim (a b : Nat) | gpuQuery (q : Nat) | bad
  deriving DecidableEq, Repr

def decodeOp (op : Nat) : Op :=
  let a := op % 16
  match op / 16 with
  | 0 => if a < 4 then .submitJob a else .bad
  | 1 => .sharedJob a
  | 2 => if a = 0 then .exclusiveJob else .bad
  | 3 => if a = 0 then .endShared else .bad
  | 4 => if a = 0 then .endExclusive else .bad
  | 5 => if a < 4 then .addBox a else .bad
  | 6 => if 1 ≤ a ∧ a ≤ 4 then .gpuAcquire a else .bad
  | 7 => if a = 0 then .gpuRelease else .bad
  | 8 => .idle a
  | 9 => if a = 0 then .sigterm else .bad
  | 10 => .wait a
  | 11 => if a = 0 then .sigkill else .bad
  | 12 => if a / 4 ≠ a % 4 then .transfer (a / 4) (a % 4) else .bad
  | 13 => if a < 4 then .outbox a else .bad
  | 14 => if a = 1 ∨ a = 2 then .linkDrop a else .bad
  | 15 => if a = 1 ∨ a = 2 then .queueJob a else .bad
  | 16 => .daemonStart a
  | 17 => .request a
  | 18 => if a ≤ 1 then .makeSlot a else .bad
  | 19 => if a = 0 then .exportSlot else .bad
  | 20 => if a ≤ 1 then .proveShard a else .bad
  | 21 => .startRun (10 * (a / 2)) (a % 2 = 1)
  | 22 => .speedClaim (a / 4) (a % 4)
  | 23 => if a ≤ 2 then .gpuQuery a else .bad
  | _ => .bad

/-- GPV-001, GPV-007: app shards and leaf-layer proofs run on the GPU where the build has `cuda`. -/
def appDev (cuda : Bool) : Nat := if cuda then 1 else 0
/-- GPV-004: nodes on the GPU where `cuda` and `T7_NODE_GPU` is not `0` or `off`. -/
def nodeDev (cuda : Bool) (env : Nat) : Nat := if cuda = true ∧ env ≠ 1 ∧ env ≠ 2 then 1 else 0
/-- GPV-005: `L` on the GPU where `cuda` and `T14_L_GPU` is not `off`. -/
def lDev (cuda : Bool) (env : Nat) : Nat := if cuda = true ∧ env ≠ 1 then 1 else 0

/-- GPV-002, GPV-003: the public values of a shard are a function of the shard alone; the
    device enters no public value. (The oracle cannot compute STARK public values; this
    is the consistency statement that the device choice of GPV-001/004/005/007 never
    reaches them.) -/
def shardPublics (shard : Nat) (_device : Nat) : Nat := shard

/-- GPW-007: the label of a time figure. -/
inductive TimeLabel where | measured | pending
  deriving DecidableEq, Repr
def timeLabel (hasRecord hasLoad : Bool) : TimeLabel :=
  if hasRecord && hasLoad then .measured else .pending

/-- State domain (not reachability). Every conjunct is an invariant that a requirement states. -/
def Valid (s : St) : Prop :=
  s.slotsUsed ≤ heavySlots ∧                                   -- BXF-002, BXF-003 (3 slots)
  (s.exclRunning = true → s.slotsUsed = 0) ∧                   -- BXF-003 (--exclusive)
  (s.termSent = true → s.gpuHolder = 3) ∧                      -- BXF-006 (never a prover or wrapd)
  (s.daemonUp = true → s.socketMode = mode0600 ∧ s.keysLoaded = true) ∧  -- PDM-001, PDM-002, PDM-004
  s.poolFree + s.slotsSpent = s.slotsMade ∧                    -- PRD-002 (one use per slot)
  s.served = s.slotsSpent ∧                                    -- PDM-005, PDM-009 (one fresh slot per proof)
  s.statsWitness = 0 ∧                                         -- PDM-008
  (s.runStopped = true → s.stopDevice = 1) ∧                   -- GPV-006
  s.unitRuns ≤ s.runsStarted                                   -- BXF-009

instance (s : St) : Decidable (Valid s) := by unfold Valid; infer_instance

def validState (s : St) : Bool := decide (Valid s)

def inRoster (s : St) (h : Nat) : Bool := (s.roster / 2 ^ h) % 2 == 1

def admit (s : St) : List Refusal :=                             -- BXF-002, BXF-003
  if s.exclRunning = true ∨ s.slotsUsed ≥ heavySlots then [.noSlot] else []

def stepRefusals (s : St) (op : Nat) : List Refusal :=
  if validState s = false then [.outsideCapability] else
  match decodeOp op with
  | .submitJob h =>                                              -- BXF-001, BXF-002
    if inRoster s h = false then [.hostNotInRoster]
    else if h = 0 then admit s else []
  | .sharedJob m =>                                              -- BXF-003
    (if 8 * m > heavyBudgetG then [.overBudget] else []) ++ admit s
  | .exclusiveJob =>                                             -- BXF-003
    if s.exclRunning = true ∨ s.slotsUsed > 0 then [.noSlot] else []
  | .endShared => if s.slotsUsed = 0 then [.noJob] else []
  | .endExclusive => if s.exclRunning = false then [.noJob] else []
  | .addBox _ => []                                              -- BXF-001 (the user writes the roster)
  | .gpuAcquire _ => if s.gpuHolder ≠ 0 then [.gpuLocked] else []          -- BXF-004
  | .gpuRelease => if s.gpuHolder = 0 then [.noHolder] else []
  | .idle _ => if s.gpuHolder = 0 then [.noHolder] else []
  | .sigterm =>                                                  -- BXF-005, BXF-006
    if s.gpuHolder = 1 ∨ s.gpuHolder = 2 then [.holderProtected]
    else if s.gpuHolder ≠ 3 ∨ s.idleMin ≤ orphanIdleMin then [.notEligible]
    else if s.termSent = true then [.alreadySignalled] else []
  | .wait _ => []
  | .sigkill =>                                                  -- BXF-013, BXF-006
    if s.gpuHolder = 1 ∨ s.gpuHolder = 2 then [.holderProtected]
    else if s.gpuHolder ≠ 3 ∨ s.termSent = false ∨ s.termAgeS < killWaitS then [.notEligible]
    else []
  | .transfer src dst =>                                         -- BXF-014, BXF-007
    (if src ≠ 0 ∧ dst ≠ 0 then [.notViaMuninn] else []) ++
    (if s.shaOk = false then [.shaMismatch] else [])
  | .outbox _ => []                                              -- BXF-008
  | .linkDrop _ => []                                            -- BXF-010
  | .queueJob _ => []
  | .daemonStart _ => if s.daemonUp = true then [.daemonRunning] else []   -- PDM-001
  | .request k =>                                                -- PDM-003, PDM-006, PRD-001, PRD-002
    if s.daemonUp = false then [.daemonDown]
    else if s.reqPeerOk = false then [.peerRefused]
    else if k ≤ 1 then
      (if s.reqManifest ≠ s.daemonManifest then [.manifestMismatch] else []) ++
      (if s.poolFree = 0 then [.noBlindingSlot] else [])
    else []
  | .makeSlot o =>                                               -- PRD-002
    (if s.daemonUp = false then [.daemonDown] else []) ++ (if o = 1 then [.helperSlot] else [])
  | .exportSlot => [.slotOffMachine]                             -- PRD-002, PRD-003
  | .proveShard f =>                                             -- GPV-006
    if s.runStopped = true then [.runStopped]
    else if f = 1 ∧ s.cuda = false then [.outsideCapability] else []
  | .startRun minutes unit =>                                    -- BXF-009
    if minutes > unitMinutes ∧ unit = false then [.needsUnit] else []
  | .speedClaim a b => if a ≠ b then [.crossBoxClaim] else []    -- BXF-011
  | .gpuQuery q => if q = 2 then [.confComputeForbidden] else [] -- BXF-012
  | .bad => [.outsideCapability]

def clearLock (s : St) : St := { s with gpuHolder := 0, idleMin := 0, termSent := false, termAgeS := 0 }

/-- One transition (applied only when `stepRefusals` is empty). -/
def step (s : St) (op : Nat) : St :=
  match decodeOp op with
  | .submitJob h =>
    if h = 0 then { s with slotsUsed := s.slotsUsed + 1 }        -- BXF-002: muninn jobs go through heavy-slot
    else { s with remoteJobs := s.remoteJobs + 1 }               -- BXF-001
  | .sharedJob _ => { s with slotsUsed := s.slotsUsed + 1 }      -- BXF-003
  | .exclusiveJob => { s with exclRunning := true }              -- BXF-003
  | .endShared => { s with slotsUsed := s.slotsUsed - 1 }
  | .endExclusive => { s with exclRunning := false }
  | .addBox h => { s with roster := if inRoster s h then s.roster else s.roster + 2 ^ h }  -- BXF-001
  | .gpuAcquire k => { (clearLock s) with gpuHolder := k }       -- BXF-004: exclusive flock
  | .gpuRelease => clearLock s                                   -- BXF-004
  | .idle m => { s with idleMin := s.idleMin + m }               -- BXF-005
  | .sigterm => { s with busLog := s.busLog + 1, termSent := true, termAgeS := 0 }  -- BXF-005: log, then SIGTERM
  | .wait t => { s with termAgeS := s.termAgeS + 10 * t }        -- BXF-013
  | .sigkill => { clearLock s with busLog := s.busLog + 1 }      -- BXF-013
  | .transfer _ _ => { s with transfers := s.transfers + 1 }     -- BXF-007, BXF-014
  | .outbox _ => { s with busLog := s.busLog + 1 }               -- BXF-008: box-relay appends the line
  | .linkDrop b =>                                               -- BXF-010
    if s.ownerAway = true then
      (if b = 1 then { s with q1 := 0, q2 := s.q2 + s.q1 } else { s with q2 := 0, q1 := s.q1 + s.q2 })
    else s
  | .queueJob b => if b = 1 then { s with q1 := s.q1 + 1 } else { s with q2 := s.q2 + 1 }
  | .daemonStart m =>                                            -- PDM-001, PDM-002, PDM-004
    { s with daemonUp := true, socketMode := mode0600, keysLoaded := true, daemonManifest := m }
  | .request k =>
    if k = 0 then                                                -- PRD-002, PDM-005, PDM-009
      { s with served := s.served + 1, poolFree := s.poolFree - 1, slotsSpent := s.slotsSpent + 1 }
    else if k = 1 then                                           -- PDM-007, PRD-004: one leaf takes path C
      { s with served := s.served + 1, poolFree := s.poolFree - 1, slotsSpent := s.slotsSpent + 1,
               lastPath := if s.reqLeaves = 1 then 3 else 1 }
    else if k = 2 then { s with statsReplies := s.statsReplies + 1 }   -- PDM-008: no witness word
    else if k = 3 then { s with daemonUp := false, keysLoaded := false }  -- quit
    else { s with errors := s.errors + 1 }                       -- PDM-003: error, keep serving
  | .makeSlot _ => { s with poolFree := s.poolFree + 1, slotsMade := s.slotsMade + 1 }  -- PRD-002
  | .exportSlot => s
  | .proveShard f =>
    if f = 1 then                                                -- GPV-006: stop, name shard and device
      { s with runStopped := true, stopShard := s.shardsProved, stopDevice := 1 }
    else                                                         -- GPV-001, GPV-004, GPV-005, GPV-007
      { s with shardsProved := s.shardsProved + 1, lastAppDev := appDev s.cuda,
               lastNodeDev := nodeDev s.cuda s.nodeEnv, lastLDev := lDev s.cuda s.lEnv }
  | .startRun _ unit =>                                          -- BXF-009
    { s with runsStarted := s.runsStarted + 1, unitRuns := s.unitRuns + (if unit then 1 else 0) }
  | .speedClaim _ _ => { s with claims := s.claims + 1 }         -- BXF-011
  | .gpuQuery _ => { s with gpuQueries := s.gpuQueries + 1 }     -- BXF-012
  | .bad => s

def apply (s : St) (op : Nat) : St := if stepRefusals s op = [] then step s op else s

def run (s : St) : List Nat → St
  | [] => s
  | op :: ops => run (apply s op) ops

def runRefusals (s : St) (_ops : List Nat) : List Refusal :=
  if validState s then [] else [.outsideCapability]

/-- Initial state: the roster the user wrote, the build features and the environment
    inputs; nothing runs. -/
def init (roster : Nat) (cuda : Bool) (nodeEnv lEnv : Nat) : St :=
  { roster, slotsUsed := 0, exclRunning := false, remoteJobs := 0, gpuHolder := 0, idleMin := 0,
    termSent := false, termAgeS := 0, busLog := 0, shaOk := true, transfers := 0,
    ownerAway := false, q1 := 0, q2 := 0, daemonUp := false, socketMode := 0,
    keysLoaded := false, daemonManifest := 0, reqManifest := 0, reqPeerOk := true,
    reqLeaves := 1, served := 0, errors := 0, statsReplies := 0, statsWitness := 0,
    poolFree := 0, slotsMade := 0, slotsSpent := 0, lastPath := 0, cuda, nodeEnv, lEnv,
    shardsProved := 0, runStopped := false, stopShard := 0, stopDevice := 0, lastAppDev := 0,
    lastNodeDev := 0, lastLDev := 0, runsStarted := 0, unitRuns := 0, claims := 0,
    gpuQueries := 0 }

/-! ## Theorems -/

/-- Every applied operation keeps `Valid`: the heavy-slot bound, the exclusive rule, the
    protected lock holders, the daemon socket and keys, one blinding slot per proof, no
    witness word in `stats`, and the named device of a stopped run. -/
theorem apply_valid (s : St) (op : Nat) (h : Valid s) : Valid (apply s op) := by
  unfold apply
  split
  · rename_i hr
    have hv : validState s = true := by simp [validState, h]
    simp only [stepRefusals, hv, Bool.true_eq_false, ite_false] at hr
    unfold Valid at h ⊢
    obtain ⟨h1, h2, h4, h5, h6, h7, h8, h9, h10⟩ := h
    unfold step
    split <;> rename_i hop <;> simp only [hop] at hr <;>
      (try simp only [admit, List.append_eq_nil_iff] at hr) <;>
      (repeat' split) <;> (repeat' split at hr) <;>
      simp_all [clearLock, heavySlots, mode0600] <;> omega
  · exact h

theorem init_valid (r : Nat) (c : Bool) (n l : Nat) : Valid (init r c n l) := by
  simp [Valid, init, heavySlots]

theorem run_valid (s : St) (ops : List Nat) (h : Valid s) : Valid (run s ops) := by
  induction ops generalizing s with
  | nil => exact h
  | cons op ops ih => exact ih _ (apply_valid s op h)

/-- BXF-001: a job starts only on a host of the roster. -/
theorem submit_in_roster (s : St) (op h : Nat) (hop : decodeOp op = .submitJob h)
    (hr : stepRefusals s op = []) : inRoster s h = true := by
  refine Classical.byContradiction fun hc => ?_
  simp_all [stepRefusals] <;> (repeat' split at hr) <;> simp_all <;> omega

/-- BXF-003: a shared job over the 64 GB budget is refused. -/
theorem shared_over_budget (s : St) (op m : Nat) (hop : decodeOp op = .sharedJob m)
    (hb : 8 * m > heavyBudgetG) (hv : validState s = true) :
    Refusal.overBudget ∈ stepRefusals s op := by
  simp [stepRefusals, hv, hop, hb]

/-- BXF-002, BXF-003: in every reachable state at most 3 shared jobs run, and none beside an
    exclusive job. -/
theorem reachable_admission (r : Nat) (c : Bool) (n l : Nat) (ops : List Nat) :
    let t := run (init r c n l) ops
    t.slotsUsed ≤ 3 ∧ (t.exclRunning = true → t.slotsUsed = 0) := by
  have := run_valid _ ops (init_valid r c n l)
  exact ⟨this.1, this.2.1⟩

/-- BXF-004: a second process does not take a held `gpu.lock`. -/
theorem gpu_lock_exclusive (s : St) (op k : Nat) (hop : decodeOp op = .gpuAcquire k)
    (hh : s.gpuHolder ≠ 0) : stepRefusals s op ≠ [] := by
  unfold stepRefusals
  split
  · simp
  · simp [hop]

/-- BXF-006: a prover or `wrapd` holder is never signalled. -/
theorem protected_holder (s : St) (hh : s.gpuHolder = 1 ∨ s.gpuHolder = 2) :
    stepRefusals s (9 * 16) ≠ [] ∧ stepRefusals s (11 * 16) ≠ [] := by
  unfold stepRefusals
  split
  · simp
  · simp [decodeOp]

/-- BXF-005: SIGTERM only to an eligible orphan idle more than 10 minutes, after a bus line. -/
theorem sigterm_guard (s : St) (hr : stepRefusals s (9 * 16) = []) :
    s.gpuHolder = 3 ∧ s.idleMin > orphanIdleMin ∧ (step s (9 * 16)).busLog = s.busLog + 1 := by
  refine ⟨?_, ?_, by simp [step, decodeOp]⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals, decodeOp] <;> (repeat' split at hr) <;> simp_all <;> omega

/-- BXF-013: SIGKILL only to the same eligible orphan, at least 30 s after SIGTERM. -/
theorem sigkill_guard (s : St) (hr : stepRefusals s (11 * 16) = []) :
    s.gpuHolder = 3 ∧ s.termSent = true ∧ s.termAgeS ≥ killWaitS := by
  refine ⟨?_, ?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals, decodeOp] <;> (repeat' split at hr) <;> simp_all <;> omega

/-- BXF-007, BXF-014: an accepted transfer has a matching sha256 and touches muninn. -/
theorem transfer_guard (s : St) (op src dst : Nat) (hop : decodeOp op = .transfer src dst)
    (hr : stepRefusals s op = []) : s.shaOk = true ∧ (src = 0 ∨ dst = 0) := by
  refine ⟨?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals] <;> (repeat' split at hr) <;> simp_all <;> omega

/-- BXF-008: each outbox line appends one line to `all.log`. -/
theorem outbox_appends (s : St) (op b : Nat) (hop : decodeOp op = .outbox b) :
    (step s op).busLog = s.busLog + 1 := by
  simp [step, hop]

/-- BXF-010: with the owner away, a dropped box keeps no queued job and none is lost. -/
theorem link_drop_moves (s : St) (ha : s.ownerAway = true) :
    (step s (14 * 16 + 1)).q1 = 0 ∧ (step s (14 * 16 + 1)).q1 + (step s (14 * 16 + 1)).q2 = s.q1 + s.q2 ∧
    (step s (14 * 16 + 2)).q2 = 0 ∧ (step s (14 * 16 + 2)).q1 + (step s (14 * 16 + 2)).q2 = s.q1 + s.q2 := by
  simp [step, decodeOp, ha]; omega

/-- PDM-001, PDM-002, PDM-004: while the daemon serves, the socket is 0600 and the keys are
    loaded (every reachable state). -/
theorem reachable_daemon (r : Nat) (c : Bool) (n l : Nat) (ops : List Nat) :
    let t := run (init r c n l) ops
    t.daemonUp = true → t.socketMode = mode0600 ∧ t.keysLoaded = true :=
  (run_valid _ ops (init_valid r c n l)).2.2.2.1

/-- PDM-003: an unknown operation gets an error answer and the daemon keeps serving. -/
theorem unknown_op_keeps_serving (s : St) (op k : Nat) (hop : decodeOp op = .request k) (hk : 4 ≤ k)
    (hu : s.daemonUp = true) (hp : s.reqPeerOk = true) (hv : validState s = true) :
    stepRefusals s op = [] ∧ (step s op).daemonUp = true ∧ (step s op).errors = s.errors + 1 := by
  have h1 : ¬ k ≤ 1 := by omega
  have h2 : k ≠ 0 := by omega
  have h3 : k ≠ 1 := by omega
  have h4 : k ≠ 2 := by omega
  have h5 : k ≠ 3 := by omega
  simp [stepRefusals, step, hop, hv, hu, hp, h1, h2, h3, h4, h5]

/-- PDM-006: a `prove` or `call` that names another key manifest is refused; PRD-002: it
    needs a free blinding slot. -/
theorem manifest_guard (s : St) (op k : Nat) (hop : decodeOp op = .request k) (hk : k ≤ 1)
    (hr : stepRefusals s op = []) : s.reqManifest = s.daemonManifest ∧ s.poolFree > 0 := by
  refine ⟨?_, ?_⟩ <;> refine Classical.byContradiction fun hc => ?_ <;>
    simp_all [stepRefusals] <;> (repeat' split at hr) <;> simp_all <;> omega

/-- PRD-002, PDM-005, PDM-009: in every reachable state each served proof consumed its own
    slot: slots spent = proofs served, and free + spent = made (no slot used twice). -/
theorem reachable_one_use_slots (r : Nat) (c : Bool) (n l : Nat) (ops : List Nat) :
    let t := run (init r c n l) ops
    t.served = t.slotsSpent ∧ t.poolFree + t.slotsSpent = t.slotsMade := by
  have := run_valid _ ops (init_valid r c n l)
  exact ⟨this.2.2.2.2.2.1, this.2.2.2.2.1⟩

/-- PRD-002: a helper slot is refused, and no slot leaves the machine. -/
theorem helper_and_export_refused (s : St) :
    stepRefusals s (18 * 16 + 1) ≠ [] ∧ stepRefusals s (19 * 16) ≠ [] := by
  unfold stepRefusals
  split
  · simp
  · simp [decodeOp]

/-- PDM-007, PRD-004: a one-leaf `call` is served on path C. -/
theorem one_leaf_path_c (s : St) (op : Nat) (hop : decodeOp op = .request 1) (h1 : s.reqLeaves = 1) :
    (step s op).lastPath = 3 := by
  simp [step, hop, h1]

/-- PDM-008: no reachable `stats` answer holds a witness word. -/
theorem reachable_stats_no_witness (r : Nat) (c : Bool) (n l : Nat) (ops : List Nat) :
    (run (init r c n l) ops).statsWitness = 0 :=
  (run_valid _ ops (init_valid r c n l)).2.2.2.2.2.2.1

/-- GPV-006: a device error stops the run and names the shard and the device; no further
    shard is proved. -/
theorem device_error_stops (s : St) (op op' : Nat) (hop : decodeOp op = .proveShard 1)
    (hop' : decodeOp op' = .proveShard 0) :
    (step s op).runStopped = true ∧ (step s op).stopShard = s.shardsProved ∧
    (step s op).stopDevice = 1 ∧ stepRefusals (step s op) op' ≠ [] := by
  refine ⟨by simp [step, hop], by simp [step, hop], by simp [step, hop], ?_⟩
  unfold stepRefusals
  split
  · simp
  · simp [step, hop, hop']

/-- GPV-001, GPV-007, GPV-004, GPV-005: the device of each stage follows the build feature
    and the environment switch. -/
theorem device_selection (n l : Nat) :
    appDev true = 1 ∧ appDev false = 0 ∧ nodeDev true 0 = 1 ∧ nodeDev true 1 = 0 ∧
    nodeDev true 2 = 0 ∧ nodeDev true 3 = 1 ∧ nodeDev false n = 0 ∧ lDev true 0 = 1 ∧
    lDev true 1 = 0 ∧ lDev true 2 = 1 ∧ lDev false l = 0 := by
  simp [appDev, nodeDev, lDev]

/-- GPV-002, GPV-003 (consistency): the device enters no public value. -/
theorem publics_device_free (shard d d' : Nat) : shardPublics shard d = shardPublics shard d' := rfl

/-- BXF-009: a run planned over 30 minutes starts only as a unit. -/
theorem long_run_is_unit (s : St) (op m : Nat) (u : Bool) (hop : decodeOp op = .startRun m u)
    (hl : m > unitMinutes) (hr : stepRefusals s op = []) : u = true := by
  refine Classical.byContradiction fun hc => ?_
  simp_all [stepRefusals] <;> (repeat' split at hr) <;> simp_all <;> omega

/-- BXF-011: a speed gain compares two runs on the same box. -/
theorem speed_claim_same_box (s : St) (op a b : Nat) (hop : decodeOp op = .speedClaim a b)
    (hr : stepRefusals s op = []) : a = b := by
  refine Classical.byContradiction fun hc => ?_
  simp_all [stepRefusals] <;> (repeat' split at hr) <;> simp_all <;> omega

/-- BXF-012: no `nvidia-smi conf-compute` call. -/
theorem no_conf_compute (s : St) : stepRefusals s (23 * 16 + 2) ≠ [] := by
  unfold stepRefusals
  split
  · simp
  · simp [decodeOp]

/-- GPW-007: MEASURED needs both the run record and the host load. -/
theorem gpw007_label (r l : Bool) : timeLabel r l = .measured ↔ r = true ∧ l = true := by
  cases r <;> cases l <;> simp [timeLabel]

end BoxFleet
