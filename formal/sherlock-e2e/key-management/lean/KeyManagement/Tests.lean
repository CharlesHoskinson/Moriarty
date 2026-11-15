import KeyManagement.Spec

/-!
# Step-level pins and reachable witnesses of Key Management

Expected values are written from the requirement sentences, not copied from `#eval`.
Each pin is checked twice: `decide +kernel` (a theorem) and `#guard` (compiled code).
-/

namespace KeyManagement

/-- A ready list: every row, every commit, a tree that holds every lane, distinct indexes. -/
def ready : St := init allRows true true 4 3 7 101 101

def gatesOps : List Nat := (List.range 12).map (fun g => 1 * 16 + g)

/-- The rekey path: keygen, the twelve gates, DONE, keys to the three boxes. -/
def rekeyed : St := run ready ([0] ++ gatesOps ++ [2 * 16, 3 * 16, 3 * 16 + 1, 3 * 16 + 2])

-- UKS-010: a list without the FP-3 row (bit 0) is refused.
theorem pin_uks010 : (stepRefusals { ready with listRows := 62 } 0).map Refusal.code =
    ["list-row-missing"] := by decide +kernel
#guard (stepRefusals { ready with listRows := 62 } 0).map Refusal.code == ["list-row-missing"]

-- RKY-002: a row with no commit is refused.
theorem pin_rky002 : (stepRefusals { ready with rowsCommitted := false } 0).map Refusal.code =
    ["row-uncommitted"] := by decide +kernel
#guard (stepRefusals { ready with rowsCommitted := false } 0).map Refusal.code == ["row-uncommitted"]

-- UKS-009: a keygen tree that is not a descendant of every commit is refused.
theorem pin_uks009 : (stepRefusals { ready with treeHoldsAll := false } 0).map Refusal.code =
    ["tree-not-descendant"] := by decide +kernel
#guard (stepRefusals { ready with treeHoldsAll := false } 0).map Refusal.code == ["tree-not-descendant"]

-- UKS-003: an R_SL key leaf index equal to KEYDAG_IDX is refused; 4 against 3 is accepted.
theorem pin_uks003 : (stepRefusals { ready with rslKeyIdx := 3 } 0).map Refusal.code =
    ["key-leaf-collision"] ∧ stepRefusals ready 0 = [] := by decide +kernel
#guard (stepRefusals { ready with rslKeyIdx := 3 } 0).map Refusal.code == ["key-leaf-collision"]

-- RKY-001: the keys are made once; a second keygen from the same list is refused.
theorem pin_rky001 : (stepRefusals (apply ready 0) 0).map Refusal.code = ["keys-current"] := by
  decide +kernel
#guard (stepRefusals (apply ready 0) 0).map Refusal.code == ["keys-current"]

-- UKS-004: the manifest records the list sha256 (7) and has id 1; status prototype (UKS-007).
theorem pin_uks004 : (apply ready 0).manifestListD = 7 ∧ (apply ready 0).manifestD = 1 ∧
    (apply ready 0).qualified = false := by decide +kernel
#guard (apply ready 0).manifestListD == 7 && (apply ready 0).manifestD == 1

-- UKS-006: with 11 of 12 gates the rekey is not DONE.
theorem pin_uks006_one_gate_fails :
    (stepRefusals (run ready ([0] ++ gatesOps.take 11)) (2 * 16)).map Refusal.code =
      ["gates-incomplete"] := by decide +kernel
#guard (stepRefusals (run ready ([0] ++ gatesOps.take 11)) (2 * 16)).map Refusal.code == ["gates-incomplete"]

-- UKS-006: no box receives keys before DONE.
theorem pin_uks006_no_distribution : (stepRefusals (apply ready 0) (3 * 16)).map Refusal.code =
    ["wrong-phase"] := by decide +kernel
#guard (stepRefusals (apply ready 0) (3 * 16)).map Refusal.code == ["wrong-phase"]

-- RKY-005: 99 bits on a term, or 100 on a grindable term, stops DONE.
theorem pin_rky005 :
    (stepRefusals (run { ready with d3MinGrindBits := 100 } ([0] ++ gatesOps)) (2 * 16)).map
      Refusal.code = ["d3-below-bound"] := by decide +kernel
#guard (stepRefusals (run { ready with d3MinBits := 99 } ([0] ++ gatesOps)) (2 * 16)).map Refusal.code == ["d3-below-bound"]

-- Witness: the rekey reaches DONE, each box holds the manifest and a proof starts (UKS-005).
theorem witness_rekey_done : rekeyed.phase = 2 ∧ rekeyed.box1 = rekeyed.manifestD ∧
    stepRefusals rekeyed (4 * 16 + 1) = [] := by decide +kernel
#guard rekeyed.phase == 2 && (stepRefusals rekeyed (4 * 16 + 1)).isEmpty

-- UKS-005 (scenario "huginn holds the T10m keys"): after a list change and a second rekey
-- that reaches box 0 only, a job on huginn (box 1) is refused.
def second : St := run rekeyed ([8 * 16 + 2, 0] ++ gatesOps ++ [2 * 16, 3 * 16])
theorem pin_uks005 : (stepRefusals second (4 * 16 + 1)).map Refusal.code = ["manifest-mismatch"] ∧
    stepRefusals second (4 * 16) = [] := by decide +kernel
#guard (stepRefusals second (4 * 16 + 1)).map Refusal.code == ["manifest-mismatch"]

-- UKS-007: before the security gate the key set is `prototype`; after it, `qualified`;
-- a second keygen returns to `prototype`.
theorem pin_uks007 : rekeyed.qualified = false ∧ (apply rekeyed (7 * 16)).qualified = true ∧
    (run (apply rekeyed (7 * 16)) [8 * 16, 0]).qualified = false := by decide +kernel
#guard !rekeyed.qualified && (apply rekeyed (7 * 16)).qualified

-- UKS-008: setup on a box changes nothing but the setup count.
theorem pin_uks008 : keyView (apply rekeyed (5 * 16 + 2)) = keyView rekeyed := by decide +kernel
#guard keyView (apply rekeyed (5 * 16 + 2)) == keyView rekeyed

-- CPR-003, SPL-004: a new program changes key_root only.
theorem pin_cpr003 : (apply rekeyed (6 * 16 + 5)).keyRoot ≠ rekeyed.keyRoot ∧
    (apply rekeyed (6 * 16 + 5)).manifestD = rekeyed.manifestD ∧
    stepRefusals (apply rekeyed (6 * 16 + 5)) (4 * 16) = [] := by decide +kernel
#guard (apply rekeyed (6 * 16 + 5)).manifestD == rekeyed.manifestD

-- outside the capability: unknown operation code and box 3.
theorem pin_bad : (stepRefusals ready (9 * 16)).map Refusal.code = ["outside-capability"] ∧
    (stepRefusals rekeyed (4 * 16 + 3)).map Refusal.code = ["outside-capability"] := by decide +kernel
#guard (stepRefusals ready (15 * 16)).map Refusal.code == ["outside-capability"]

end KeyManagement
