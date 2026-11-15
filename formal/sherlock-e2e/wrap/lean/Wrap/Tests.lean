import Wrap.Spec

/-!
# Known answers, written from the requirement sentences

Each answer is pinned twice: `decide +kernel` (kernel) and `#guard` (compiled
evaluator). Bound, bound + 1 and the honest path are pinned for each guard.
-/

namespace Wrap

def o (code arg : Int) : Int := code * opBase + arg

/-- The honest path of one call (T10m measured values, LOOP.md:31). -/
def honest : List Int :=
  [o 1 21, o 2 1685845, o 3 101325, o 4 1, o 5 11515, o 6 1]

-- WRP-002: k = 22 stops before keygen; k = 21 is accepted
theorem pin_k22 : (run 0 [o 1 22]).stop = stopK22 ∧ (run 0 [o 1 22]).keygenRan = false := by decide +kernel
#guard (run 0 [o 1 22]).stop == stopK22 && !(run 0 [o 1 22]).keygenRan
theorem pin_k21 : (run 0 [o 1 21]).phase = 1 := by decide +kernel
#guard (run 0 [o 1 21]).phase == 1

-- WRP-003: 1,782,574 rows stop before keygen; 1,782,573 pass
theorem pin_rows_over : (run 0 [o 1 21, o 2 1782574]).stop = stopRows := by decide +kernel
#guard (run 0 [o 1 21, o 2 1782574]).stop == stopRows
theorem pin_rows_gate : (run 0 [o 1 21, o 2 1782573]).phase = 2 := by decide +kernel
#guard (run 0 [o 1 21, o 2 1782573]).phase == 2

-- WRP-004: a 50,001-byte VK stops without wrap.vk; 50,000 bytes pass
theorem pin_vk_over : (run 0 [o 1 21, o 2 1, o 3 150001]).stop = stopVk ∧
    (run 0 [o 1 21, o 2 1, o 3 150001]).vkFileWritten = false := by decide +kernel
#guard (run 0 [o 1 21, o 2 1, o 3 150001]).stop == stopVk
theorem pin_vk_bound : (run 0 [o 1 21, o 2 1, o 3 150000]).phase = 3 := by decide +kernel
#guard (run 0 [o 1 21, o 2 1, o 3 150000]).phase == 3

-- WRP-008: a key that differs from E6_EXPECT_VK stops before any output file
theorem pin_keydiff : (run 0 [o 1 21, o 2 1, o 3 101325, o 4 0]).stop = stopKeyDiff ∧
    (run 0 [o 1 21, o 2 1, o 3 101325, o 4 0]).files = 0 := by decide +kernel
#guard (run 0 [o 1 21, o 2 1, o 3 101325, o 4 0]).stop == stopKeyDiff

-- WRP-007, ANC-013: slot 0 in lane 6 is refused at decode
theorem pin_slot0 : (run 0 (honest.take 4 ++ [o 5 11500])).stop = stopDecode := by decide +kernel
#guard (run 0 (honest.take 4 ++ [o 5 11500])).stop == stopDecode
-- WRP-007: chunk write key 3 in lane 15 is refused
theorem pin_slot3 : (run 0 (honest.take 4 ++ [o 5 10315])).stop = stopDecode := by decide +kernel
#guard (run 0 (honest.take 4 ++ [o 5 10315])).stop == stopDecode
-- ANC-016: ordinary call (mode 0) with the anchor-write template is refused
theorem pin_template : (run 0 (honest.take 4 ++ [o 5 101515])).stop = stopDecode := by decide +kernel
#guard (run 0 (honest.take 4 ++ [o 5 101515])).stop == stopDecode
-- ordinary call without the template passes
theorem pin_ordinary : (run 0 (honest.take 4 ++ [o 5 1515])).phase = 5 := by decide +kernel
#guard (run 0 (honest.take 4 ++ [o 5 1515])).phase == 5

-- WRP-001, WRP-009: one proof, three files
theorem pin_written : (run 0 honest).phase = 6 ∧ (run 0 honest).files = 3 ∧ (run 0 honest).proofs = 1 := by
  decide +kernel
#guard (run 0 honest).files == 3 && (run 0 honest).proofs == 1
-- WRP-009: the library verifier refuses, no file
theorem pin_verify_fail : (run 0 (honest.take 5 ++ [o 6 0])).stop = stopVerify ∧
    (run 0 (honest.take 5 ++ [o 6 0])).files = 0 := by decide +kernel
#guard (run 0 (honest.take 5 ++ [o 6 0])).files == 0

-- WRP-012: deploy with an incomplete record is refused; with all five bits it passes
theorem pin_deploy_unqualified : (run 0 (honest ++ [o 8 0])).lastRefusal = refQual ∧
    (run 0 (honest ++ [o 8 0])).deployed = 0 := by decide +kernel
#guard (run 0 (honest ++ [o 8 0])).lastRefusal == refQual
def qualified : List Int := [o 7 0, o 7 1, o 7 2, o 7 3, o 7 4]
theorem pin_deploy_qualified : (run 0 (qualified ++ honest ++ [o 8 0])).deployed = 1 := by decide +kernel
#guard (run 0 (qualified ++ honest ++ [o 8 0])).deployed == 1
theorem pin_deploy_four_bits : (run 0 (qualified.take 4 ++ honest ++ [o 8 0])).deployed = 0 := by decide +kernel
#guard (run 0 (qualified.take 4 ++ honest ++ [o 8 0])).deployed == 0

-- WRP-010: a second call with another key cannot deploy in the same era
def secondKey : List Int := [o 10 0, o 1 21, o 2 1, o 3 201325, o 4 1, o 5 11515, o 6 1, o 8 0]
theorem pin_second_key : (run 0 (qualified ++ honest ++ [o 8 0] ++ secondKey)).lastRefusal = refSecondKey ∧
    (run 0 (qualified ++ honest ++ [o 8 0] ++ secondKey)).keysDeployed = 1 := by decide +kernel
#guard (run 0 (qualified ++ honest ++ [o 8 0] ++ secondKey)).lastRefusal == refSecondKey

-- WRP-011: a GPU run with another key is refused; with the CPU key it is accepted
theorem pin_gpu_other : (run 0 (honest ++ [o 9 2])).lastRefusal = refGpuKey := by decide +kernel
#guard (run 0 (honest ++ [o 9 2])).lastRefusal == refGpuKey
theorem pin_gpu_same : (run 0 (honest ++ [o 9 1])).gpuVk = 1 := by decide +kernel
#guard (run 0 (honest ++ [o 9 1])).gpuVk == 1

-- WRP-006: two production proofs draw different blinders; a test-seed build repeats
def twoCalls : List Int := honest ++ [o 10 0] ++ honest
theorem pin_blinders : (run 0 twoCalls).prevBlinder = 1 ∧ (run 0 twoCalls).lastBlinder = 2 := by decide +kernel
#guard (run 0 twoCalls).prevBlinder == 1 && (run 0 twoCalls).lastBlinder == 2
theorem pin_seeded : (run 1 twoCalls).prevBlinder = (run 1 twoCalls).lastBlinder := by decide +kernel
#guard (run 1 twoCalls).prevBlinder == (run 1 twoCalls).lastBlinder

-- step domain: unknown code, disabled operation, keygen with key id 0
theorem pin_refuse_code : stepRefusals (init false) (o 11 0) = [.notAnOperation] := by decide +kernel
theorem pin_refuse_disabled : stepRefusals (init false) (o 6 1) = [.opNotEnabled] := by decide +kernel
theorem pin_refuse_vkid0 : stepRefusals (init false) (o 3 1325) = [.notAnOperation] := by decide +kernel
#guard stepRefusals (init false) (o 6 1) == [.opNotEnabled]

end Wrap
