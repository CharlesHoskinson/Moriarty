import LeafProving.Spec

/-!
# Step-level pins and reachable witnesses of Leaf Proving

Expected values come from the requirement sentences. Each pin: `decide +kernel` and `#guard`.
-/

namespace LeafProving

/-- An honest run of `t02 increment`-like shape: 5 app AIRs, no hash, 3 shards of 4 steps
    (10 steps), statement and tape bound. -/
def honest : St := init 5 false false 9000 false true 7 11 11 13 13 10 4 21 1

def proved : St := run honest [0, 16, 16, 16]

-- LFP-003: 9 app AIRs are refused (over the bound and outside the bucket).
theorem pin_lfp003_over : (stepRefusals { honest with appAirs := 9 } 0).map Refusal.code =
    ["air-count-over-bound", "outside-bucket"] := by decide +kernel
#guard (stepRefusals { honest with appAirs := 9 } 0).map Refusal.code == ["air-count-over-bound", "outside-bucket"]

-- LFP-003 scenario: SHA-256 and Keccak-256 in one program (8 AIRs) are refused, no proof.
theorem pin_lfp003_mixed :
    (stepRefusals { honest with appAirs := 8, hasSha := true, hasKeccak := true } 0).map
      Refusal.code = ["mixed-hash"] ∧
    (stepRefusals { honest with hasSha := true, hasKeccak := true } 16).map Refusal.code =
      ["not-admitted"] := by decide +kernel
#guard (stepRefusals { honest with appAirs := 8, hasSha := true, hasKeccak := true } 0).map Refusal.code == ["mixed-hash"]

-- RSL-002: 7 app AIRs (8 with FP-1) are outside the bucket.
theorem pin_rsl002 : (stepRefusals { honest with appAirs := 7 } 0).map Refusal.code =
    ["outside-bucket"] := by decide +kernel
#guard (stepRefusals { honest with appAirs := 7 } 0).map Refusal.code == ["outside-bucket"]

-- LFP-004: 30,001 columns refused, 30,000 admitted.
theorem pin_lfp004 : (stepRefusals { honest with appCols := 30001 } 0).map Refusal.code =
    ["column-bound"] ∧ stepRefusals { honest with appCols := 30000 } 0 = [] := by decide +kernel
#guard (stepRefusals { honest with appCols := 30001 } 0).map Refusal.code == ["column-bound"]

-- LFP-002: three shards give three app proofs and three leaf-layer proofs; a fourth is refused.
theorem pin_lfp002 : proved.appProofs = 3 ∧ proved.leafProofs = 3 ∧
    (stepRefusals proved 16).map Refusal.code = ["all-shards-proved"] := by decide +kernel
#guard proved.appProofs == 3 && (stepRefusals proved 16).map Refusal.code == ["all-shards-proved"]

-- LFP-005: swapped heights are refused.
theorem pin_lfp005 : (stepRefusals (apply { honest with shapeOk := false } 0) 16).map
    Refusal.code = ["shape-mismatch"] := by decide +kernel
#guard (stepRefusals (apply { honest with shapeOk := false } 0) 16).map Refusal.code == ["shape-mismatch"]

-- LFP-006, LFP-011: the leaf slot PROG_D holds the program's prog_d (7).
theorem pin_lfp006 : proved.leafProgD = 7 := by decide +kernel
#guard proved.leafProgD == 7

-- LFP-007: a leaf of program B (3) is refused (wrong-prog-d) and the run still merges its honest
-- leaves (Formal-Quint D-leaf-2, lead ruling). A leaf of the same program (7) joins.
theorem pin_lfp007 :
    (stepRefusals (run proved [3 * 16]) (2 * 16 + 3)).map Refusal.code = ["wrong-prog-d"] ∧
    stepRefusals (run proved [3 * 16, 2 * 16 + 3]) (4 * 16) = [] ∧
    stepRefusals (run proved [3 * 16, 2 * 16 + 7]) (4 * 16) = [] := by decide +kernel
#guard (stepRefusals (run proved [3 * 16]) (2 * 16 + 3)).map Refusal.code == ["wrong-prog-d"]

-- LFP-008: 12 more steps (10 -> 22, shard length 4) give 6 shards instead of 3, and the same key_root and manifest.
theorem pin_lfp008 : shards (apply (apply honest 0) (5 * 16 + 2)) = 6 ∧
    (apply (apply honest 0) (5 * 16 + 2)).keyRoot = 21 ∧
    (apply (apply honest 0) (5 * 16 + 2)).manifestD = 1 ∧
    (stepRefusals proved (5 * 16)).map Refusal.code = ["run-started"] := by decide +kernel
#guard shards (apply (apply honest 0) (5 * 16 + 2)) == 6

-- LFP-009: the record is published after the 3 leaves and names 3 leaves; not before.
theorem pin_lfp009 : (apply proved (3 * 16)).recordLeaves = 3 ∧
    (stepRefusals (run honest [0, 16]) (3 * 16)).map Refusal.code = ["leaves-incomplete"] ∧
    (stepRefusals proved (4 * 16)).map Refusal.code = ["not-published"] := by decide +kernel
#guard (apply proved (3 * 16)).recordLeaves == 3

-- LFP-010: FP-1 words of another outcome are refused.
theorem pin_lfp010 : (stepRefusals (apply { honest with fp1Outcome := 14 } 0) 16).map
    Refusal.code = ["statement-unbound"] := by decide +kernel
#guard (stepRefusals (apply { honest with fp1Outcome := 14 } 0) 16).map Refusal.code == ["statement-unbound"]

-- LFP-012: a trace that consumed tape B is refused.
theorem pin_lfp012 : (stepRefusals (apply { honest with tapeD := 12 } 0) 16).map
    Refusal.code = ["input-mismatch"] := by decide +kernel
#guard (stepRefusals (apply { honest with tapeD := 12 } 0) 16).map Refusal.code == ["input-mismatch"]

-- RSL-008: a non-canonical input word is refused.
theorem pin_rsl008 : (stepRefusals (apply { honest with nonCanonical := true } 0) 16).map
    Refusal.code = ["noncanonical-word"] := by decide +kernel
#guard (stepRefusals (apply { honest with nonCanonical := true } 0) 16).map Refusal.code == ["noncanonical-word"]

-- Witness: the honest run reaches a merged N1 with the universal pre-hash (RSL-001).
theorem witness_merged : (run proved [3 * 16, 4 * 16]).merged = true ∧
    n1PreHashUniversal proved = true := by decide +kernel
#guard (run proved [3 * 16, 4 * 16]).merged

-- 9 app AIRs with FP-1 = 8 app AIRs (one hash kit) is admitted.
theorem witness_sha_shape : stepRefusals { honest with appAirs := 8, hasSha := true } 0 = [] := by
  decide +kernel
#guard (stepRefusals { honest with appAirs := 8, hasSha := true } 0).isEmpty

-- outside the capability
theorem pin_bad : (stepRefusals honest (6 * 16)).map Refusal.code = ["outside-capability"] ∧
    (stepRefusals { honest with shardCap := 0 } 0).map Refusal.code = ["outside-capability"] := by
  decide +kernel
#guard (stepRefusals honest (1 * 16 + 1)).map Refusal.code == ["outside-capability"]

end LeafProving
