import CompactComp.Spec

/-!
# Known answers, written from the requirement sentences

Each answer is pinned twice: `decide +kernel` (kernel) and `#guard` (compiled evaluator).
`adm st ir kind shape clash keccak` builds the admit operation of one circuit.
-/

namespace CompactComp

def o (code arg : Int) : Int := code * opBase + arg
def adm (st ir kind shape clash kc : Int) : Int :=
  o 1 (st + 10 * ir + 100 * kind + 1000 * shape + 10000 * clash + 100000 * kc)
/-- A stock ZKIR 3.1 circuit with field inputs only. -/
def ok1 : Int := adm 1 1 0 1 0 0
def fin : Int := o 2 0

-- ECC-012, ECC-003: every circuit admitted: exit 0, one image and one ABI per circuit
theorem pin_compile_ok : (run 0 [ok1, ok1, fin]).phase = 1 ∧ (run 0 [ok1, ok1, fin]).images = 2 ∧
    (run 0 [ok1, ok1, fin]).abis = 2 ∧ (run 0 [ok1, ok1, fin]).exitCode = 1 := by decide +kernel
#guard (run 0 [ok1, ok1, fin]).images == 2 && (run 0 [ok1, ok1, fin]).exitCode == 1

-- ECC-013 scenario: one of two circuits states ZKIR 3.2: MYC-REF-032/irVersion, no image
theorem pin_ir32 : (run 0 [ok1, adm 1 2 0 1 0 0, fin]).phase = 2 ∧
    (run 0 [ok1, adm 1 2 0 1 0 0, fin]).images = 0 ∧
    (run 0 [ok1, adm 1 2 0 1 0 0, fin]).exitCode = 2 ∧
    (run 0 [ok1, adm 1 2 0 1 0 0, fin]).lastRefusal = refIrVersion := by decide +kernel
#guard (run 0 [ok1, adm 1 2 0 1 0 0, fin]).lastRefusal == refIrVersion

-- ECC-001: a non-stock input is refused (carrier)
theorem pin_carrier : (run 0 [adm 0 1 0 1 0 0]).lastRefusal = refCarrier := by decide +kernel
#guard (run 0 [adm 0 1 0 1 0 0]).lastRefusal == refCarrier

-- CF-014: an input type with no kit is refused and named
theorem pin_nokit : (run 0 [adm 1 1 4 1 0 0]).lastRefusal = refInputType := by decide +kernel
#guard (run 0 [adm 1 1 4 1 0 0]).lastRefusal == refInputType

-- ECC-005: a form outside the 13 shapes is refused
theorem pin_shape : (run 0 [adm 1 1 0 0 0 0]).lastRefusal = refShape := by decide +kernel
#guard (run 0 [adm 1 1 0 0 0 0]).lastRefusal == refShape

-- MYC-SEAL-021, ECC-009: a field at the anchor slot is refused
theorem pin_clash : (run 1 [adm 1 1 0 1 1 0]).lastRefusal = refAnchorSlot := by decide +kernel
#guard (run 1 [adm 1 1 0 1 1 0]).lastRefusal == refAnchorSlot

-- ECC-014: a Jubjub point is two tape words and one curve check; a scalar one word
theorem pin_point : (run 0 [adm 1 1 1 1 0 0]).tapeWords = 2 ∧ (run 0 [adm 1 1 1 1 0 0]).curveChecks = 1 ∧
    (run 0 [adm 1 1 1 1 0 0]).accepted = 1 := by decide +kernel
#guard (run 0 [adm 1 1 1 1 0 0]).tapeWords == 2
theorem pin_scalar : (run 0 [adm 1 1 2 1 0 0]).tapeWords = 1 ∧ (run 0 [adm 1 1 2 1 0 0]).curveChecks = 0 := by
  decide +kernel
#guard (run 0 [adm 1 1 2 1 0 0]).tapeWords == 1

-- CF-013: secp256k1 inputs lower to the kit
theorem pin_secp : (run 0 [adm 1 1 3 1 0 0]).secpKit = 1 ∧ (run 0 [adm 1 1 3 1 0 0]).refused = 0 := by
  decide +kernel
#guard (run 0 [adm 1 1 3 1 0 0]).secpKit == 1

-- ECC-007: keccak256 is one chip instruction, no tier 3
theorem pin_keccak : (run 0 [adm 1 1 0 1 0 1]).keccakChip = 1 ∧ (run 0 [adm 1 1 0 1 0 1]).tier3 = 0 := by
  decide +kernel
#guard (run 0 [adm 1 1 0 1 0 1]).keccakChip == 1

-- ECC-009, MYC-SEAL-005: slot 15 in the deploy profile, cell 0 in the spike
theorem pin_slot : (run 1 []).anchorSlot = 15 ∧ (run 0 []).anchorSlot = 0 := by decide +kernel
#guard (run 1 []).anchorSlot == 15

-- ECC-004: two calls with different tapes give equal bytes
theorem pin_bytes : (run 0 [o 3 7001, o 3 7002]).img1Hash = (run 0 [o 3 7001, o 3 7002]).img2Hash := by
  decide +kernel
#guard (run 0 [o 3 7001, o 3 7002]).img1Hash == (run 0 [o 3 7001, o 3 7002]).img2Hash

-- ECC-002: no gate result: not used; six passing gates: used; a surviving mutant: not used
def gates6 : List Int := [o 4 1, o 4 3, o 4 5, o 4 7, o 4 9, o 4 11]
theorem pin_nogates : (run 0 [o 5 0]).settleUses = 0 ∧ (run 0 [o 5 0]).settleBlocked = 1 := by decide +kernel
#guard (run 0 [o 5 0]).settleBlocked == 1
theorem pin_gates : (run 0 (gates6 ++ [o 5 0])).settleUses = 1 := by decide +kernel
#guard (run 0 (gates6 ++ [o 5 0])).settleUses == 1
theorem pin_mutant : (run 0 (gates6 ++ [o 4 8, o 5 0])).settleUses = 0 ∧
    (run 0 (gates6 ++ [o 4 8, o 5 0])).failedGate = 5 := by decide +kernel
#guard (run 0 (gates6 ++ [o 4 8, o 5 0])).failedGate == 5

-- step domain
theorem pin_refuse_empty_finish : stepRefusals (init false) fin = [.opNotEnabled] := by decide +kernel
theorem pin_refuse_code : stepRefusals (init false) (o 7 0) = [.notAnOperation] := by decide +kernel
theorem pin_refuse_digit : stepRefusals (init false) (adm 2 1 0 1 0 0) = [.notAnOperation] := by decide +kernel
#guard stepRefusals (init false) fin == [.opNotEnabled]

end CompactComp
