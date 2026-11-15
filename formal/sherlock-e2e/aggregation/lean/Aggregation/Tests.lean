import Aggregation.Spec

/-!
# Step-level pins and reachable witnesses of Aggregation

Expected values are written from the requirement sentences. Each pin is checked with
`decide +kernel` and with `#guard`.
-/

namespace Aggregation

def codes (s : St) (op : Nat) : List String := (stepRefusals s op).map Refusal.code

/-- A run of program 5 under `key_root` 9, `reg_root` 0, `S_MAX` 100, a MERGE-WORK key of
    5,284 columns (the T10m measurement) and arity 2; candidate: an honest LEAF of chip set 3,
    steps 0..3, `pre_d` 10, `post_d` 11, Halted with outcome 42. -/
def base : St :=
  { init 5 9 0 100 5284 2 with
    cKind := kLeaf, cProg := 5, cKeyRoot := 9, cPre := 10, cPost := 11, cHPre := 0, cSteps := 3,
    cAirs := allAirs, cChipSet := 3, cStatus := 1, cOutcome := 42, cManifestOk := true }

def s1 : St := apply base 0
/-- Second leaf of chip set 0, linked: `pre_d` 11 = `post_d` of the first, `h_pre` 3. -/
def leafB (s : St) : St := { s with cPre := 11, cPost := 12, cHPre := 3, cSteps := 2, cChipSet := 0 }
def s2 : St := apply (leafB s1) 0

-- NODE-001, NAG-009, NAG-004: two linked leaves of chip sets 3 and 0 merge into the SEG
-- (pre 10, post 12, h 0..5) and both chip sets are recorded.
theorem pin_merge_chain : s2.segPre = 10 ∧ s2.segPost = 12 ∧ s2.segLo = 0 ∧ s2.segHi = 5 ∧
    s2.chipSets = 9 ∧ s2.leaves = 2 := by decide +kernel
#guard s2.segHi == 5 && s2.chipSets == 9

-- NAG-009 (scenario "Leaves in the wrong order"): the second leaf first, then the first.
theorem pin_nag009_wrong_order :
    codes (apply (leafB base) 0) 0 = ["link-broken"] := by decide +kernel
#guard codes (apply (leafB base) 0) 0 == ["link-broken"]

-- NAG-002: a child without one of the three required AIRs is refused.
theorem pin_nag002 : codes { base with cAirs := 6 } 0 = ["air-missing"] := by decide +kernel
#guard codes { base with cAirs := 3 } 0 == ["air-missing"]

-- LFP-007, NAG-003: a leaf of program B in a run of program A; a changed key_root.
theorem pin_nag003 : codes { base with cProg := 6 } 0 = ["prog-d-mismatch"] ∧
    codes { base with cKeyRoot := 8 } 0 = ["key-root-mismatch"] := by decide +kernel
#guard codes { base with cProg := 6 } 0 == ["prog-d-mismatch"]

-- NODE-002: zero steps and steps above S_MAX are refused for a LEAF.
theorem pin_node002 : codes { base with cSteps := 0 } 0 = ["range"] ∧
    codes { base with cSteps := 101 } 0 = ["range"] := by decide +kernel
#guard codes { base with cSteps := 101 } 0 == ["range"]

-- NAG-006, NAG-007: each forged witness (MW-F01, AFK-F7, GP2-F4.2, AFK-F9 b, GP2-C2.1).
theorem pin_forgeries : (List.range 5).all (fun f => codes { base with cForged := f + 1 } 0 == ["forged-witness"]) = true := by
  decide +kernel
#guard (List.range 5).all (fun f => codes { base with cForged := f + 1 } 0 == ["forged-witness"])

-- NODE-008, NAG-011: a nonzero reg_root in N1 is accepted (LEAF child carries none);
-- a NODE child must carry the parent's reg_root.
def r77 : St := { base with regRoot := 77 }
theorem pin_nag011_reg_root : codes r77 0 = [] ∧ codes { r77 with cRegRoot := 77 } 0 = ["reg-root-mismatch"] ∧
    codes { r77 with cKind := kNode, cRegRoot := 77 } 0 = [] ∧
    codes { r77 with cKind := kNode, cRegRoot := 0 } 0 = ["reg-root-mismatch"] := by decide +kernel
#guard (codes r77 0).isEmpty

-- NAG-005: a MERGE-WORK key of 5,401 columns is refused; 5,400 and 5,284 are accepted.
theorem pin_nag005 : codes { base with mwColumns := 5401 } (2 * 16) = ["columns"] ∧
    codes { base with mwColumns := 5400 } (2 * 16) = [] ∧ codes base (2 * 16) = [] := by decide +kernel
#guard codes { base with mwColumns := 5401 } (2 * 16) == ["columns"]

-- NAG-011, SHP-003: MERGE-WORK with one linked child is accepted and adds k steps; without a
-- prior child (AFK-F9 b, unlinked) it is refused; k = 0 is refused.
def w0 : St := apply base (2 * 16)
def w1 : St := apply w0 0
def wLinked : St := { w1 with cPre := 11, cPost := 13, cHPre := 3 }
theorem pin_work : codes wLinked (1 * 16 + 2) = [] ∧ (apply wLinked (1 * 16 + 2)).segHi = 5 ∧
    codes w0 (1 * 16 + 2) = ["no-child", "forged-witness"].take 1 ∧
    codes wLinked (1 * 16) = ["range"] ∧ codes { wLinked with cPre := 99 } (1 * 16 + 1) = ["link-broken"] ∧
    codes (apply base 0) (1 * 16 + 1) = ["no-work-key", "link-broken"] := by decide +kernel
#guard codes w0 (1 * 16 + 2) == ["no-child"]

-- NODE-003, CONT-003: a Running state is not sealed; a seal fixes n_steps = h_hi.
theorem pin_seal : codes { s2 with cStatus := 0 } (3 * 16) = ["not-terminal"] ∧
    (apply s2 (3 * 16)).sealed = true ∧ (apply s2 (3 * 16)).claimSteps = 5 := by decide +kernel
#guard (apply s2 (3 * 16)).claimSteps == 5

-- NAG-008: L refuses a changed outcome word and accepts the proved one.
def sealed2 : St := apply s2 (3 * 16)
theorem pin_nag008 : codes { sealed2 with cOutcome := 43 } (6 * 16) = ["outcome-mismatch"] ∧
    codes sealed2 (6 * 16) = [] ∧ codes s2 (6 * 16) = ["not-sealed", "outcome-mismatch"] ∧
    codes { s2 with cOutcome := 0 } (6 * 16) = ["not-sealed"] := by decide +kernel
#guard codes { sealed2 with cOutcome := 43 } (6 * 16) == ["outcome-mismatch"]

-- CLM-004, GPH-002: after a consume, plain SEAL refuses the non-empty chain; SEAL-WITH-DEPS
-- needs DL.chain_d = deps_d; a SEG is not consumed as a claim (CLM-002).
def c1 : St := apply s2 (4 * 16 + 3)
theorem pin_clm004 : c1.depsD = 4 ∧ codes c1 (3 * 16) = ["deps-nonempty"] ∧
    codes { c1 with cChain := 5 } (5 * 16) = ["deps-mismatch"] ∧ codes { c1 with cChain := 4 } (5 * 16) = [] ∧
    codes { s2 with cKind := kNode } (4 * 16) = ["not-a-claim"] := by decide +kernel
#guard codes c1 (3 * 16) == ["deps-nonempty"]

-- GPH-003: N3 accepts an FV unit at a foreign row (7) and refuses it at a node row (3).
def fv : St := { base with cKind := kForeign, cKeyRow := 7 }
theorem pin_gph003 : codes fv (7 * 16) = [] ∧ codes { fv with cKeyRow := 3 } (7 * 16) = ["key-row"] ∧
    codes { fv with cKind := kNode } (7 * 16) = ["key-selector"] := by decide +kernel
#guard codes { fv with cKeyRow := 3 } (7 * 16) == ["key-row"]

-- GPH-006, CONT-002, CONT-004: suspend, then resume with the key manifest; a resume with
-- another manifest is refused; no merge while suspended.
def sus : St := apply s2 (8 * 16)
theorem pin_gph006 : codes sus (9 * 16) = [] ∧ codes { sus with cManifestOk := false } (9 * 16) = ["manifest-mismatch"] ∧
    stmtView (apply sus (9 * 16)) = stmtView s2 ∧ codes (leafB sus) 0 = ["suspended", "link-broken"] := by
  decide +kernel
#guard stmtView (apply sus (9 * 16)) == stmtView s2

-- Witness: the honest run reaches an L-accepted root (reachable, so NAG-008's accepting
-- branch fires).
theorem witness_root : (run base [0] |> fun s => run (leafB s) [0, 3 * 16, 6 * 16]).lAccepted = true := by
  decide +kernel
#guard (run base [0] |> fun s => run (leafB s) [0, 3 * 16, 6 * 16]).lAccepted

-- outside the capability
theorem pin_bad : codes base (10 * 16) = ["outside-capability"] ∧ codes base 1 = ["outside-capability"] := by
  decide +kernel
#guard codes base (15 * 16) == ["outside-capability"]

end Aggregation
