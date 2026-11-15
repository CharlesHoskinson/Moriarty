import KeyManagement.Spec

/-!
# CONSTANT-CHECKED theorems of Key Management

Each theorem checks a recorded value against the bound of its requirement by `decide`
(kernel evaluation; no `native_decide`, so the axiom audit stays clean). Every value
cites its source. MEASURED = a run record; CODE = a constant in the code at the cited
commit (read by Formal-Lean-B on 2026-11-15); LIST = `~/.cache/rekey-t11/LIST.md` at
sha256 639220981074832c1e935914ba0ec2c8e9d46b36a78d7ada718476de74e081db (mtime
2026-11-15 10:31).
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

/-- LIST: rows present on 2026-11-15 10:31: FP-3 (bit 0, "FP-3 lane" table) and anchor
    slot 15 (bit 1, row T12-SLOT15). No row for range checks, R_SL statement bindings,
    the R_SL key leaf index or the era-check words. -/
def listRowsMeasured : Nat := 3

/-- MEASURED: Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:91`, `:96`: D3 on
    the T10m chain keys; weakest shift 101.0110 bits (leaf). In millibits. All terms
    reported "at least 100" and every grindable term "at least 101". -/
def d3WeakestMillibitsT10m : Nat := 101011

/-- The measured configuration of 2026-11-15: the T25 rekey on the list of 10:31, from the
    `rekey-chips` dcf1b4c tree, which does not descend from `t10q-ntfix` e3b407d (checked
    with `git merge-base --is-ancestor e3b407d dcf1b4c`: not an ancestor). -/
def measured20261115 : St :=
  init listRowsMeasured true false plannedRslKeyIdx codeKeydagIdx 1 101 101

/-- UKS-003, UKS-009, UKS-010 on the recorded values: the rekey runner refuses to start,
    with the missing rows, the tree that lacks a lane and the key leaf collision.
    (RKY-002 holds: every row present has a commit.) -/
theorem t25_refused_20261115 :
    (stepRefusals measured20261115 0).map Refusal.code =
      ["list-row-missing", "tree-not-descendant", "key-leaf-collision"] := by decide

/-- UKS-003: the planned R_SL key leaf index equals `KEYDAG_IDX`. -/
theorem uks003_collision_measured : plannedRslKeyIdx = codeKeydagIdx := by decide

/-- UKS-010: four of the six rows are still missing on the list of 10:31 (the spec status of
    2026-11-15 says six; FP-3 and slot 15 were added since). -/
theorem uks010_rows_missing : allRows - listRowsMeasured = 60 ∧ listRowsMeasured ≠ allRows := by
  decide

/-- RKY-005 on the T10m keys: the weakest term is above both bounds (100 and 101 bits). -/
theorem rky005_d3_t10m : d3Bits * 1000 ≤ d3WeakestMillibitsT10m ∧
    d3GrindBits * 1000 ≤ d3WeakestMillibitsT10m := by decide

end KeyManagement
