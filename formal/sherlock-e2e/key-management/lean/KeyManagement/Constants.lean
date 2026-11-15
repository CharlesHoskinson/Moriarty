import KeyManagement.Spec

/-!
# CONSTANT-CHECKED theorems of Key Management

Each theorem checks a recorded value against the bound of its requirement by `decide`
(kernel evaluation; no `native_decide`, so the axiom audit stays clean). Every value
cites its source. MEASURED = a run record; CODE = a constant in the code at the cited
commit (read by Formal-Lean-B on 2026-11-15); LIST = `~/.cache/rekey-t11/LIST.md`, read at two
versions: sha256 639220981074832c... (mtime 10:31) and 8a315c07c5861308... (mtime 11:22).
-/

namespace KeyManagement

/-- CODE: openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/stmt.rs:90` `UROWS = 7`. -/
def codeUrows : Nat := 7
/-- CODE: openvm `rekey-chips` dcf1b4c `crates/sherlock-wrap-export/examples/t7_chain.rs:457`
    (7 node entries) and `:459` (2 foreign entries). -/
def codeNodeEntries : Nat := 7
def codeForeignEntries : Nat := 2

/-- UKS-002: `keys_d` is built from 7 node rows (and the 2 foreign rows of the record). -/
theorem uks002_node_rows : codeUrows = nodeRows ∧ codeNodeEntries = nodeRows ∧
    codeForeignEntries = 2 := by decide

/-- UKS-001, CPR-003: the universal key set has exactly four keys. -/
theorem uks001_four_universal_keys : universalKeys.length = 4 := by decide

/-- UKS-006: the rekey runner reruns twelve named gates, and DONE needs all twelve bits. -/
theorem uks006_twelve_gates : gateNames.length = 12 ∧ allGates = 2 ^ 12 - 1 := by decide

/-- CODE: openvm `rekey-chips` dcf1b4c `t7_chain.rs:363` `KEYDAG_IDX = 3`.
    LIST: line 17 plans "key_root menu id 3" for R_SL chip set 3. -/
def codeKeydagIdx : Nat := 3
def plannedRslKeyIdx : Nat := 3

/-- LIST at 10:31 (sha256 639220981074832c...): rows present are FP-3 (bit 0, "FP-3 lane"
    table) and anchor slot 15 (bit 1, row T12-SLOT15). -/
def listRows1031 : Nat := 3

/-- LIST at 11:22 (sha256 8a315c07c5861308...): adds RSL-BIND (bit 3), the T22 era check row
    (bit 5; premise A3 failed, the ELD-007 fallback takes the row's place as UKS-010's status
    states) and the prekey-fixes rows for the R_SL key leaf index (bit 4) and canonical range
    checks (bit 2). All six rows. -/
def listRows1122 : Nat := 63

/-- LIST (both versions): the wrap-levers C3.1 row is "staged, NOT committed" and the T11
    blinders row has a Moriarty "commit pending", so RKY-002 fires on the recorded list. -/
def rowsCommittedRecorded : Bool := false

/-- CODE: openvm rekey-int ca65f27 (11:27) `crates/sherlock-lfr/src/stmt.rs:271`
    `KEYDAG_IDX = 3`, `:275` `RSL_KEY_IDX0 = KEYDAG_IDX + 1` (the UKS-003 fix of prekey-fixes
    70a56d4). Every openvm commit named in LIST (e3b407d, a2eeece, 4ea3d05, dcf1b4c, 36cdbf0,
    fe3601c, 6f773b2, c281a5f, 637ca2e, bbf6ad3, 699539c, d8e5671) is an ancestor of ca65f27
    (`git merge-base --is-ancestor`, checked 2026-11-15). -/
def rslKeyIdx0RekeyInt : Nat := 4

/-- The spec's evidence configuration: the list of 10:31 and the keygen tree `rekey-chips`
    dcf1b4c, which does not descend from `t10q-ntfix` e3b407d. -/
def atSpecEvidence : St :=
  init listRows1031 rowsCommittedRecorded false plannedRslKeyIdx codeKeydagIdx 1 101 101

/-- The configuration of 11:27: the list of 11:22 and the integration tree rekey-int ca65f27. -/
def atRekeyInt : St :=
  init listRows1122 rowsCommittedRecorded true rslKeyIdx0RekeyInt codeKeydagIdx 2 101 101

/-- UKS-003, UKS-009, UKS-010, RKY-002 on the spec's evidence: four refusals. -/
theorem t25_refused_spec_evidence :
    (stepRefusals atSpecEvidence 0).map Refusal.code =
      ["list-row-missing", "row-uncommitted", "tree-not-descendant", "key-leaf-collision"] := by
  decide

/-- On rekey-int ca65f27 with the list of 11:22 only RKY-002 still refuses: two rows have no
    commit. UKS-003, UKS-009 and UKS-010 hold. -/
theorem t25_refused_rekey_int :
    (stepRefusals atRekeyInt 0).map Refusal.code = ["row-uncommitted"] := by decide

/-- UKS-003: the planned R_SL key leaf index of LIST:17 equals `KEYDAG_IDX` (the spec's GAP);
    the rekey-int fix gives index 4. -/
theorem uks003_collision_measured : plannedRslKeyIdx = codeKeydagIdx ∧
    rslKeyIdx0RekeyInt ≠ codeKeydagIdx := by decide

/-- UKS-010: 4 of the six rows missing at 10:31 (the spec status says six), none at 11:22. -/
theorem uks010_rows_missing : listRows1031 ≠ allRows ∧ listRows1122 = allRows := by decide

/-- MEASURED: Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:91`, `:96`: D3 on
    the T10m chain keys; weakest shift 101.0110 bits (leaf). In millibits. All terms
    reported "at least 100" and every grindable term "at least 101". -/
def d3WeakestMillibitsT10m : Nat := 101011

/-- RKY-005 on the T10m keys: the weakest term is above both bounds (100 and 101 bits). -/
theorem rky005_d3_t10m : d3Bits * 1000 ≤ d3WeakestMillibitsT10m ∧
    d3GrindBits * 1000 ≤ d3WeakestMillibitsT10m := by decide

end KeyManagement
