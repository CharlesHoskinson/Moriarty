import BoxFleet.Spec

/-!
# Step-level pins and reachable witnesses of Box Fleet

Expected values are written from the requirement sentences, not copied from `#eval`.
Each pin is checked twice: `decide +kernel` (a theorem) and `#guard` (compiled code).
Ops: `code * 16 + arg` (see `Spec.lean`).
-/

namespace BoxFleet

def codes (s : St) (op : Nat) : List String := (stepRefusals s op).map Refusal.code

/-- The factory fixtures muninn, huginn, odin in the roster (bits 0..2); a CUDA build. -/
def fleet : St := init 7 true 0 0

-- BXF-001: a job on a host outside the roster is refused; after the user adds box 3, admitted.
theorem pin_bxf001 : codes fleet (0 * 16 + 3) = ["host-not-in-roster"] ∧
    codes (apply fleet (5 * 16 + 3)) (0 * 16 + 3) = [] := by decide +kernel
#guard codes fleet 3 == ["host-not-in-roster"] && (codes (apply fleet 83) 3).isEmpty

-- BXF-002: a muninn job takes a heavy-slot slot; a fourth one waits.
theorem pin_bxf002 : (run fleet [0, 0, 0]).slotsUsed = 3 ∧ codes (run fleet [0, 0, 0]) 0 = ["no-slot"] := by
  decide +kernel
#guard codes (run fleet [0, 0, 0]) 0 == ["no-slot"]

-- BXF-003: `heavy-slot --mem 80G` (10 units) is refused; `--exclusive --mem 80G` runs.
theorem pin_bxf003 : codes fleet (1 * 16 + 10) = ["over-budget"] ∧ codes fleet (1 * 16 + 8) = [] ∧
    codes fleet (2 * 16) = [] ∧ codes (apply fleet (2 * 16)) (1 * 16 + 1) = ["no-slot"] := by decide +kernel
#guard codes fleet 26 == ["over-budget"] && (codes fleet 32).isEmpty

-- BXF-004: a second GPU job waits on gpu.lock; after release it runs.
theorem pin_bxf004 : codes (apply fleet (6 * 16 + 1)) (6 * 16 + 1) = ["gpu-locked"] ∧
    codes (run fleet [6 * 16 + 1, 7 * 16]) (6 * 16 + 1) = [] ∧ codes fleet (6 * 16 + 1) = [] := by
  decide +kernel
#guard codes (apply fleet 97) 97 == ["gpu-locked"]

/-- An eligible orphan sampler holding the lock. -/
def orphan : St := apply fleet (6 * 16 + 3)

-- BXF-005: 10 minutes is not more than 10; 11 minutes is; the SIGTERM writes a bus line first.
theorem pin_bxf005 : codes (apply orphan (8 * 16 + 10)) (9 * 16) = ["not-eligible"] ∧
    codes (apply orphan (8 * 16 + 11)) (9 * 16) = [] ∧
    (run orphan [8 * 16 + 11, 9 * 16]).busLog = 1 := by decide +kernel
#guard codes (apply orphan 138) 144 == ["not-eligible"] && (codes (apply orphan 139) 144).isEmpty

-- BXF-006: a prover or wrapd holder is left running.
theorem pin_bxf006 : codes (run fleet [6 * 16 + 1, 8 * 16 + 15]) (9 * 16) = ["holder-protected"] ∧
    codes (run fleet [6 * 16 + 2, 8 * 16 + 15]) (11 * 16) = ["holder-protected"] := by decide +kernel
#guard codes (run fleet [97, 143]) 144 == ["holder-protected"]

-- BXF-013: SIGKILL only 30 s after SIGTERM (20 s is too early).
def termed : St := run orphan [8 * 16 + 11, 9 * 16]
theorem pin_bxf013 : codes (apply termed (10 * 16 + 2)) (11 * 16) = ["not-eligible"] ∧
    codes (apply termed (10 * 16 + 3)) (11 * 16) = [] ∧
    (run termed [10 * 16 + 3, 11 * 16]).gpuHolder = 0 ∧
    (run termed [10 * 16 + 3, 11 * 16]).busLog = 2 := by decide +kernel
#guard (codes (apply termed 163) 176).isEmpty

-- BXF-007: a file whose sha256 differs from the bus message is refused.
theorem pin_bxf007 : codes { fleet with shaOk := false } (12 * 16 + 1) = ["sha256-mismatch"] ∧
    codes fleet (12 * 16 + 1) = [] := by decide +kernel
#guard codes { fleet with shaOk := false } 193 == ["sha256-mismatch"]

-- BXF-014: huginn (1) to odin (2) directly is refused; huginn to muninn and muninn to odin pass.
theorem pin_bxf014 : codes fleet (12 * 16 + 1 * 4 + 2) = ["not-via-muninn"] ∧
    codes fleet (12 * 16 + 1 * 4 + 0) = [] ∧ codes fleet (12 * 16 + 0 * 4 + 2) = [] := by decide +kernel
#guard codes fleet 198 == ["not-via-muninn"]

-- BXF-008: two outbox lines give two all.log lines.
theorem pin_bxf008 : (run fleet [13 * 16 + 1, 13 * 16 + 2]).busLog = 2 := by decide +kernel
#guard (run fleet [209, 210]).busLog == 2

-- BXF-010: owner away, huginn drops: its 2 queued jobs move to odin; owner present: they stay.
theorem pin_bxf010 :
    (run { fleet with ownerAway := true } [15 * 16 + 1, 15 * 16 + 1, 14 * 16 + 1]).q2 = 2 ∧
    (run fleet [15 * 16 + 1, 15 * 16 + 1, 14 * 16 + 1]).q1 = 2 := by decide +kernel
#guard (run { fleet with ownerAway := true } [241, 241, 225]).q2 == 2

-- BXF-009: a 40-minute run outside a unit is refused; as a unit it starts; 30 minutes needs none.
theorem pin_bxf009 : codes fleet (21 * 16 + 8) = ["needs-unit"] ∧ codes fleet (21 * 16 + 9) = [] ∧
    codes fleet (21 * 16 + 6) = [] := by decide +kernel
#guard codes fleet 344 == ["needs-unit"]

-- BXF-011: a speed claim across huginn and odin is refused; on huginn alone it is stated.
theorem pin_bxf011 : codes fleet (22 * 16 + 1 * 4 + 2) = ["cross-box-claim"] ∧
    codes fleet (22 * 16 + 1 * 4 + 1) = [] := by decide +kernel
#guard codes fleet 358 == ["cross-box-claim"]

-- BXF-012: query-gpu and the CUDA memory call pass; conf-compute is refused.
theorem pin_bxf012 : codes fleet (23 * 16) = [] ∧ codes fleet (23 * 16 + 1) = [] ∧
    codes fleet (23 * 16 + 2) = ["conf-compute-forbidden"] := by decide +kernel
#guard codes fleet 370 == ["conf-compute-forbidden"]

/-- The daemon up with manifest 5, two own blinding slots. -/
def daemon : St := run fleet [16 * 16 + 5, 18 * 16, 18 * 16]
def asking (m : Nat) : St := { daemon with reqManifest := m }

-- PDM-001, PDM-002, PDM-004: socket 0600, keys loaded.
theorem pin_pdm002 : daemon.socketMode = 384 ∧ daemon.keysLoaded = true ∧ daemon.daemonUp = true := by
  decide +kernel
#guard daemon.socketMode == 384

-- PDM-006: a request with another manifest is refused; with manifest 5 it is served.
theorem pin_pdm006 : codes (asking 4) (17 * 16) = ["manifest-mismatch"] ∧ codes (asking 5) (17 * 16) = [] := by
  decide +kernel
#guard codes (asking 4) 272 == ["manifest-mismatch"]

-- PRD-002, PDM-005, PDM-009: each proof consumes its own slot; a third proof finds none.
theorem pin_prd002 : (run (asking 5) [17 * 16, 17 * 16 + 1]).slotsSpent = 2 ∧
    codes (run (asking 5) [17 * 16, 17 * 16 + 1]) (17 * 16) = ["no-blinding-slot"] ∧
    codes daemon (18 * 16 + 1) = ["helper-slot"] ∧ codes daemon (19 * 16) = ["slot-off-machine"] := by
  decide +kernel
#guard codes (run (asking 5) [272, 273]) 272 == ["no-blinding-slot"]

-- PDM-003: `dump` (another operation) gets an error and the daemon keeps serving.
theorem pin_pdm003 : (apply daemon (17 * 16 + 7)).errors = 1 ∧ (apply daemon (17 * 16 + 7)).daemonUp = true := by
  decide +kernel
#guard (apply daemon 279).errors == 1

-- PRD-001: a peer of another user is refused.
theorem pin_prd001 : codes { asking 5 with reqPeerOk := false } (17 * 16) = ["peer-refused"] := by decide +kernel
#guard codes { asking 5 with reqPeerOk := false } 272 == ["peer-refused"]

-- PDM-007, PRD-004: a one-leaf call takes path C; a three-leaf call takes the tree.
theorem pin_pdm007 : (apply (asking 5) (17 * 16 + 1)).lastPath = 3 ∧
    (apply { asking 5 with reqLeaves := 3 } (17 * 16 + 1)).lastPath = 1 := by decide +kernel
#guard (apply (asking 5) 273).lastPath == 3

-- PDM-008: stats answers carry no witness word.
theorem pin_pdm008 : (apply daemon (17 * 16 + 2)).statsReplies = 1 ∧ (apply daemon (17 * 16 + 2)).statsWitness = 0 := by
  decide +kernel
#guard (apply daemon 274).statsWitness == 0

-- GPV-001, GPV-004, GPV-005, GPV-007: a CUDA build with switches unset proves every stage
-- on the GPU; T7_NODE_GPU=off and T14_L_GPU=off move nodes and L to the CPU; no cuda: CPU.
theorem pin_gpv_devices : (apply fleet (20 * 16)).lastAppDev = 1 ∧ (apply fleet (20 * 16)).lastNodeDev = 1 ∧
    (apply fleet (20 * 16)).lastLDev = 1 ∧ (apply (init 7 true 2 1) (20 * 16)).lastNodeDev = 0 ∧
    (apply (init 7 true 2 1) (20 * 16)).lastLDev = 0 ∧ (apply (init 7 false 0 0) (20 * 16)).lastAppDev = 0 := by
  decide +kernel
#guard (apply (init 7 true 1 0) 320).lastNodeDev == 0

-- GPV-006: a device error after 2 shards stops the run at shard 2 on the GPU; no more shards.
theorem pin_gpv006 : (run fleet [20 * 16, 20 * 16, 20 * 16 + 1]).stopShard = 2 ∧
    codes (run fleet [20 * 16, 20 * 16, 20 * 16 + 1]) (20 * 16) = ["run-stopped"] := by decide +kernel
#guard codes (run fleet [320, 320, 321]) 320 == ["run-stopped"]

-- outside the capability: unknown op code.
theorem pin_bad : codes fleet (24 * 16) = ["outside-capability"] := by decide +kernel
#guard codes fleet 384 == ["outside-capability"]

end BoxFleet
