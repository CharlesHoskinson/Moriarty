import Bridge.Spec

/-!
# Step-level pins and reachable witnesses of Bridge

Expected values come from the requirement sentences. Each pin: `decide +kernel` and `#guard`.
-/

namespace Bridge

/-- An honest chunked run of 2 slices with k = 2, inside every bound. -/
def honest : St := init 1995 99 15 15 1 2 2 true false true true true true true
/-- An honest ordinary call. -/
def honestOrd : St := init 1995 99 15 15 0 1 1 true false true true true true true

/-- keygen, plan, L on the full tree, L_Fr, stock verify, publish, honest transfer, wrap. -/
def path : List Nat := [0, 16, 2 * 16 + 1, 3 * 16, 4 * 16, 5 * 16, 6 * 16, 7 * 16]
def done : St := run honest path

-- Witness: the whole hand-off happens; the record names the proof's sha256 (LTR-006).
theorem witness_handoff : done.records = 1 ∧ done.wraps = 1 ∧ done.recordSha = done.proofSha ∧
    done.phase = 4 := by decide +kernel
#guard done.records == 1 && done.wraps == 1

-- LTR-003: 2,201 columns or 101 interactions: the key generator refuses the key.
theorem pin_ltr003 : (stepRefusals { honest with lCols := 2201 } 0).map Refusal.code = ["l-key-bounds"] ∧
    (stepRefusals { honest with lInter := 101 } 0).map Refusal.code = ["l-key-bounds"] ∧
    stepRefusals { honest with lCols := 2200, lInter := 100 } 0 = [] := by decide +kernel
#guard (stepRefusals { honest with lCols := 2201 } 0).map Refusal.code == ["l-key-bounds"]

-- CLB-004: slot 0 on both components is refused (anchor-slot).
theorem pin_clb004 : (stepRefusals { honest with slotIface := 0, slotWrap := 0 } 0).map Refusal.code =
    ["anchor-slot"] := by decide +kernel
#guard (stepRefusals { honest with slotIface := 0, slotWrap := 0 } 0).map Refusal.code == ["anchor-slot"]

-- CLB-008 (scenario "The wrap still reads slot 0"): interface 15, decoder 0: refused.
theorem pin_clb008 : (stepRefusals { honest with slotWrap := 0 } 0).map Refusal.code =
    ["slot-mismatch"] := by decide +kernel
#guard (stepRefusals { honest with slotWrap := 0 } 0).map Refusal.code == ["slot-mismatch"]

-- LTR-007: a second L key is refused; path C and the full tree record the one key.
def both : St := run honest [0, 16, 2 * 16]
theorem pin_ltr007 : (stepRefusals (apply honest 0) 0).map Refusal.code = ["keys-exist"] ∧
    both.keyPathC = 1 ∧ (run honest [0, 16, 2 * 16 + 1]).keyFull = 1 := by decide +kernel
#guard (stepRefusals (apply honest 0) 0).map Refusal.code == ["keys-exist"]

-- CLB-007: a call cut into 3 chunks is refused before a proof starts.
theorem pin_clb007 : (stepRefusals (apply { honest with k := 3, slices := 3 } 0) 16).map Refusal.code =
    ["k-max"] := by decide +kernel
#guard (stepRefusals (apply { honest with k := 3, slices := 3 } 0) 16).map Refusal.code == ["k-max"]

-- CLB-002: a lane word the guest did not write is refused by L.
theorem pin_clb002 : (stepRefusals (run { honest with laneOk := false } [0, 16]) (2 * 16 + 1)).map
    Refusal.code = ["lane-unbound"] := by decide +kernel
#guard (stepRefusals (run { honest with laneOk := false } [0, 16]) (2 * 16 + 1)).map Refusal.code == ["lane-unbound"]

-- CLB-003 (scenario: 2 slices, k = 3 is above K_MAX, so use 1 slice and k = 2).
theorem pin_clb003 : (stepRefusals (run { honest with slices := 1 } [0, 16]) (2 * 16 + 1)).map
    Refusal.code = ["k-mismatch"] := by decide +kernel
#guard (stepRefusals (run { honest with slices := 1 } [0, 16]) (2 * 16 + 1)).map Refusal.code == ["k-mismatch"]

-- CLB-006: an ordinary call with the anchor-write template is refused.
theorem pin_clb006 : (stepRefusals (run { honestOrd with tplWrite := true } [0, 16]) (2 * 16 + 1)).map
    Refusal.code = ["ordinary-anchor-write"] := by decide +kernel
#guard (stepRefusals (run { honestOrd with tplWrite := true } [0, 16]) (2 * 16 + 1)).map Refusal.code == ["ordinary-anchor-write"]

-- CLB-006 does not touch a chunk call: a chunk carries the write template.
theorem pin_clb006_chunk : stepRefusals (run { honest with tplWrite := true } [0, 16]) (2 * 16 + 1) = [] := by
  decide +kernel

-- LTR-004, LW-021, LTR-005, CLB-005: the L_Fr refusals.
def atL (s : St) : St := run s [0, 16, 2 * 16 + 1]
theorem pin_lfr : (stepRefusals (atL { honest with tapeExposed := false }) (3 * 16)).map Refusal.code =
      ["tape-not-exposed"] ∧
    (stepRefusals (atL { honest with tapeTyped := false }) (3 * 16)).map Refusal.code = ["ill-typed-tape"] ∧
    (stepRefusals (atL { honest with transcriptOk := false }) (3 * 16)).map Refusal.code =
      ["transcript-mismatch"] ∧
    (stepRefusals (atL { honest with lockOk := false }) (3 * 16)).map Refusal.code = ["lock-mismatch"] ∧
    stepRefusals (atL { honestOrd with lockOk := false }) (3 * 16) = [] := by decide +kernel
#guard (stepRefusals (atL { honest with lockOk := false }) (3 * 16)).map Refusal.code == ["lock-mismatch"]

-- LTR-008: the stock verifier refuses: no record.
def forged : St := run { honest with stockAccepts := false } [0, 16, 2 * 16 + 1, 3 * 16, 4 * 16]
theorem pin_ltr008 : (stepRefusals forged (5 * 16)).map Refusal.code = ["not-verified"] ∧
    (apply forged (5 * 16)).records = 0 := by decide +kernel
#guard (apply forged (5 * 16)).records == 0

-- LTR-006 (scenario "A corrupted transfer"): one changed byte: the wrap does not start.
def corrupted : St := run honest [0, 16, 2 * 16 + 1, 3 * 16, 4 * 16, 5 * 16, 6 * 16 + 1]
theorem pin_ltr006 : (stepRefusals corrupted (7 * 16)).map Refusal.code = ["digest-mismatch"] := by
  decide +kernel
#guard (stepRefusals corrupted (7 * 16)).map Refusal.code == ["digest-mismatch"]

theorem pin_bad : (stepRefusals honest (8 * 16)).map Refusal.code = ["outside-capability"] ∧
    (stepRefusals honest (2 * 16 + 2)).map Refusal.code = ["outside-capability"] := by decide +kernel
#guard (stepRefusals honest (15 * 16)).map Refusal.code == ["outside-capability"]

end Bridge
