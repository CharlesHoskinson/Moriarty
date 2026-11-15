import BoxFleet.Spec

/-!
# CONSTANT-CHECKED theorems of Box Fleet

Each theorem checks a recorded value by `decide` (no `native_decide`). Values were read by
the Formal-Lean-B box-fleet fork on 2026-11-15 at the cited source. CODE = a constant in the
code; MEASURED = a run record that labels the number MEASURED; DERIVED = the source labels it
DERIVED. Times are in centiseconds.
-/

namespace BoxFleet

/-- CODE `~/.local/bin/heavy-slot:27` `SLOTS = 3`, `:28` `BUDGET = 64` (GB); `:115` refuses a
    shared `--mem` over `BUDGET` ("use --exclusive"). -/
def codeSlots : Nat := 3
def codeBudgetG : Nat := 64

/-- BXF-002, BXF-003: the oracle's admission constants are the tool's. The verification
    case of BXF-003 (80 GB shared refused, 80 GB exclusive admitted) holds on them. -/
theorem bxf003_heavy_slot_code : codeSlots = heavySlots ∧ codeBudgetG = heavyBudgetG ∧
    80 > codeBudgetG ∧ (stepRefusals (init 7 true 0 0) (1 * 16 + 10)).map Refusal.code = ["over-budget"] ∧
    stepRefusals (init 7 true 0 0) (2 * 16) = [] := by decide

/-- CODE Moriarty `t14b` 5e54263da `tools/sherlock-wrap-gpu/src/serve.rs:210`:
    `Permissions::from_mode(0o600)`. -/
def codeSocketMode : Nat := 0o600

/-- PDM-002: the socket mode in the code is 0600. -/
theorem pdm002_socket_mode : codeSocketMode = mode0600 := by decide

/-- MEASURED huginn h79 (`~/Projects/Moriarty/llm-wiki/wiki/log.md:3691`): an end-to-end call
    113.01 s with one process per call, 71.59 s with the resident daemon (T10m keys). -/
def huginnColdCs : Nat := 11301
def huginnDaemonCs : Nat := 7159

/-- PDM-001, BXF-011, LPC-005: the daemon gain is a same-box comparison (huginn against
    huginn) and the warm call is faster. -/
theorem pdm001_daemon_gain_same_box : huginnDaemonCs < huginnColdCs ∧
    (stepRefusals (init 7 true 0 0) (22 * 16 + 1 * 4 + 1)) = [] := by decide

/-- DERIVED (the source says DERIVED, the spec status of GPV-001 says MEASURED): Moriarty
    `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:52`, huginn
    GPU app proof 0.225 s against 3.595 s on the CPU. See FINDINGS.md F-BF-1. -/
def huginnCpuAppMs : Nat := 3595
def huginnGpuAppMs : Nat := 225

/-- GPV-001 (and BXF-011): the GPU app proof is faster on the same box (huginn): 15.98 times,
    which the source rounds to "16 times" (16 * 225 = 3600 > 3595). -/
theorem gpv001_gpu_faster_huginn : huginnGpuAppMs < huginnCpuAppMs ∧ 15 * huginnGpuAppMs ≤ huginnCpuAppMs := by
  decide

/-- MEASURED odin h39 (`~/.cache/box-bus/all.log:42`): the WRAP baseline w1 on odin, cold
    28.98 s, warm 27.77 s; the same message says "STARK lfr_call half not run". PDM-004
    cites this line for the STARK worker. See FINDINGS.md F-BF-2. -/
def odinWrapColdCs : Nat := 2898
def odinWrapWarmCs : Nat := 2777

theorem pdm004_cited_record_is_wrap_only : odinWrapWarmCs < odinWrapColdCs := by decide

/-- BXF-005, BXF-013, BXF-009: the spec's time bounds. -/
theorem bxf_time_bounds : orphanIdleMin = 10 ∧ killWaitS = 30 ∧ unitMinutes = 30 := by decide

end BoxFleet
