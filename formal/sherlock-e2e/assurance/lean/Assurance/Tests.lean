import Assurance.Spec

/-!
# Step-level pins and reachable witnesses of Assurance

Expected values are written from the requirement sentences. Each pin is checked twice:
`decide +kernel` (a theorem) and `#guard` (compiled code).
-/

namespace Assurance

/-- A production build, no seed variable, roster {0, 1, 2} (muninn, huginn, odin), a clean
    counterexample record and a G5 mapping of the current sets. -/
def prod : St := init false false false 7 0 0 0 true
def deployedRun : St := run prod [0, 1 * 16]

-- ZK-012: r_P drawn once; a second preparation of the deployment is refused.
theorem pin_zk012 : (apply prod 0).rpDraws = 1 ∧ (apply prod 0).deploySecret = true ∧
    (stepRefusals (apply prod 0) 0).map Refusal.code = ["already-deployed"] := by decide +kernel
#guard (stepRefusals (apply prod 0) 0).map Refusal.code == ["already-deployed"]

-- ZK-013: two runs take two different salts.
theorem pin_zk013 : (run prod [0, 16]).salt = 1 ∧ (run prod [0, 16, 16]).salt = 2 := by decide +kernel
#guard (run prod [0, 16]).salt != (run prod [0, 16, 16]).salt

-- ZK-014 / ZKE-001: a production build with a seed variable or a constant r_P path: exit 4.
theorem pin_zk014 : (stepRefusals { prod with seedEnv := true } 0).map Refusal.code =
    ["seeded-without-feature"] ∧
    (stepRefusals { prod with constRpPath := true } 0).map Refusal.code = ["seeded-without-feature"] ∧
    (stepRefusals { deployedRun with seedEnv := true } (3 * 16)).map Refusal.code =
      ["seeded-without-feature"] := by decide +kernel
#guard (stepRefusals { prod with seedEnv := true } 0).map Refusal.code == ["seeded-without-feature"]

-- ZK-003: a test-seed build with GW_DET may use the fixed blinder.
theorem pin_zk003_testseed : (run (init true true false 7 0 0 0 true) [0, 16, 48, 48]).blinders = [0, 0] := by
  decide +kernel
#guard (run (init true true false 7 0 0 0 true) [0, 16, 48, 48]).blinders == [0, 0]

-- ZKE-005: with deploy.secret absent the call is refused and no new r_P is drawn.
theorem pin_zke005 : (stepRefusals (run prod [0, 2 * 16]) 16).map Refusal.code = ["secret-missing"] ∧
    (run prod [0, 2 * 16, 16]).rpDraws = 1 := by decide +kernel
#guard (stepRefusals (run prod [0, 2 * 16]) 16).map Refusal.code == ["secret-missing"]

-- ZKE-007, ZK-015, ZK-017: two CPU and two GPU proofs of one statement: four distinct blinders.
def four : St := run deployedRun [3 * 16, 3 * 16, 3 * 16 + 1, 3 * 16 + 1]
theorem pin_zke007 : four.blinders = [4, 3, 2, 1] ∧ four.blinders.Nodup ∧ four.devices = [1, 1, 0, 0] := by
  decide +kernel
#guard four.blinders == [4, 3, 2, 1]

-- ZKE-002: a test-seed binary does not submit; a production proof does.
theorem pin_zke002 :
    (stepRefusals (run (init true false false 7 0 0 0 true) [0, 16, 48]) (4 * 16)).map Refusal.code =
      ["test-seed-binary"] ∧ (apply four (4 * 16)).submitted = 1 := by decide +kernel
#guard (stepRefusals (run (init true false false 7 0 0 0 true) [0, 16, 48]) (4 * 16)).map Refusal.code == ["test-seed-binary"]

-- ZKE-004, ZK-018: odin (host 2) admitted; host 5 outside the roster refused.
theorem pin_zke004 : stepRefusals deployedRun (5 * 16 + 2) = [] ∧
    (stepRefusals deployedRun (5 * 16 + 5)).map Refusal.code = ["host-outside-roster"] := by decide +kernel
#guard (stepRefusals deployedRun (5 * 16 + 5)).map Refusal.code == ["host-outside-roster"]

-- ZK-016: a reused blinding slot is refused.
theorem pin_zk016 : stepRefusals prod (6 * 16 + 3) = [] ∧
    (stepRefusals (apply prod (6 * 16 + 3)) (6 * 16 + 3)).map Refusal.code = ["slot-reused"] := by
  decide +kernel
#guard (stepRefusals (apply prod (6 * 16 + 3)) (6 * 16 + 3)).map Refusal.code == ["slot-reused"]

def allChecksOps : List Nat := (List.range 6).map (fun c => 7 * 16 + c)

-- SSG-006 + UKS-007: G1..G5 and FP-8 pass: record with the manifest; status qualified.
def gated : St := run prod (allChecksOps ++ [8 * 16])
theorem pin_ssg006 : gated.gateRecord = true ∧ gated.recordManifest = gated.manifestD ∧
    gated.qualified = true := by decide +kernel
#guard gated.qualified

-- SSG-006 scenario "G5 names old parameters": no record, status stays prototype.
theorem pin_ssg006_old_g5 :
    (stepRefusals (run { prod with g5Current := false } allChecksOps) (8 * 16)).map Refusal.code =
      ["g5-old-params"] := by decide +kernel
#guard (stepRefusals (run { prod with g5Current := false } allChecksOps) (8 * 16)).map Refusal.code == ["g5-old-params"]

-- SQG-015: with FP-8 missing the gate does not pass.
theorem pin_sqg015 : (stepRefusals (run prod (allChecksOps.take 5)) (8 * 16)).map Refusal.code =
    ["checks-incomplete"] := by decide +kernel
#guard (stepRefusals (run prod (allChecksOps.take 5)) (8 * 16)).map Refusal.code == ["checks-incomplete"]

-- SQG-016: an UNVERIFIED step blocks the gate.
theorem pin_sqg016 : (stepRefusals (run { prod with unverified := 1 } allChecksOps) (8 * 16)).map
    Refusal.code = ["step-unverified"] := by decide +kernel
#guard (stepRefusals (run { prod with unverified := 1 } allChecksOps) (8 * 16)).map Refusal.code == ["step-unverified"]

-- SSG-003: a CONFIRMED row without a fix blocks the gate; the fix clears it.
theorem pin_ssg003 : (stepRefusals (run { prod with confirmedNoFix := 1 } allChecksOps) (8 * 16)).map
    Refusal.code = ["confirmed-defect"] ∧
    (run { prod with confirmedNoFix := 1 } (allChecksOps ++ [14 * 16, 8 * 16])).qualified = true := by
  decide +kernel
#guard (stepRefusals (run { prod with confirmedNoFix := 1 } allChecksOps) (8 * 16)).map Refusal.code == ["confirmed-defect"]

-- SSG-004: NOT ATTEMPTED rows are listed open (they do not block the record).
theorem pin_ssg004 : (run { prod with notAttempted := 3 } (allChecksOps ++ [8 * 16])).recordOpen = 3 := by
  decide +kernel
#guard (run { prod with notAttempted := 3 } (allChecksOps ++ [8 * 16])).recordOpen == 3

-- SSG-007: a run record before the gate reads not qualified; after, qualified; new keys: not.
theorem pin_ssg007 : (apply prod (10 * 16)).lastRunQualified = false ∧
    (apply gated (10 * 16)).lastRunQualified = true ∧
    (run gated [9 * 16, 10 * 16]).lastRunQualified = false := by decide +kernel
#guard !(apply prod (10 * 16)).lastRunQualified && (apply gated (10 * 16)).lastRunQualified

-- SSG-005, LPC-004, LPC-016: a landed change gets a row; DONE needs its lead verdict.
theorem pin_lpc016 : (apply prod (11 * 16)).changeRows = 1 ∧
    (stepRefusals (apply prod (11 * 16)) (13 * 16)).map Refusal.code = ["no-lead-verdict"] ∧
    stepRefusals (run prod [11 * 16, 12 * 16]) (13 * 16) = [] ∧
    (stepRefusals prod (12 * 16)).map Refusal.code = ["no-open-change"] := by decide +kernel
#guard (stepRefusals (apply prod (11 * 16)) (13 * 16)).map Refusal.code == ["no-lead-verdict"]

-- outside the capability
theorem pin_bad : (stepRefusals prod (17 * 16)).map Refusal.code = ["outside-capability"] ∧
    (stepRefusals prod (7 * 16 + 6)).map Refusal.code = ["outside-capability"] := by decide +kernel
#guard (stepRefusals prod (3 * 16 + 2)).map Refusal.code == ["outside-capability"]

end Assurance
