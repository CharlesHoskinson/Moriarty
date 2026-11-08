import Clean.Sherlock.Aggregation

/-!
Premise file of `Clean.Sherlock.Aggregation` (task 2.12, row F21).

No `axiom` is declared in either file. Every premise is a hypothesis of the theorem that uses it:

* `Premises.Step`, `halted`, `result`: the general reference machine (SRC-017: one machine, the
  program as a parameter). Premise because the machine is the Sherlock semantics defined elsewhere
  (`universalMachine.qnt`, the Rust interpreter); the theorems hold for any step relation.
* `Premises.digest_inj`: `digest` is injective on the carrier domain. Premise because it is the
  collision resistance of Poseidon over the canonical carrier encoding (SRC-001/SAS O1), a
  cryptographic assumption, not a theorem of this file.
* `Premises.progDigest_inj`: `prog_d` is injective on committed programs. Same justification.
* `Premises.leaf_sound`: an accepted leaf statement has a witness run of exactly `steps` machine
  steps (SAS O2 for the S-slot leaf, SRC-014). Premise because it is the in-circuit leaf
  verifier's soundness (the fixed leaf key, `verify_child_incircuit_with`), a SRC-005 test
  obligation and a circuit-level fact proven elsewhere.
* The certificate tree itself: a node proof accepted under the self key yields the slot
  statements it verified. Premise because it is the accumulator and pairing soundness
  (`tree.md` §4, BCMS20-class; SRC-005/SRC-006 test obligations). It is the modelling choice that
  `NodeCert` is the witness tree, as `tree.md` §4 states.
* `AccAlgebra` fields (L-D): the fold has a neutral element, scaling by a bit is zero or the
  identity, `collapse` preserves the pairing-check value. Premises because they are the algebra
  of `AssignedAccumulator` in `midnight-circuits`, checked by the SRC-005/006 raw-witness tests.
* `det` and `stuck` of `result_unique` / `statement_independent_of_S`: the reference machine is
  deterministic and halted states are terminal. Premises because they are properties of the
  Sherlock machine, stated per theorem rather than imposed on every machine.

The allowed `#print axioms` output is therefore the empty set or the standard Lean axioms
`propext`, `Quot.sound`, `Classical.choice`. `scripts/qa/check-lean-aggregation.sh` enforces this
and that every theorem of the module is queried below.
-/

open Clean.Sherlock.Aggregation

-- Sanity checks (decided by the kernel, not asserted).
example : (chainOf [⟨0, 0, 1, 0, 1⟩, ⟨0, 1, 2, 1, 1⟩] : NodeCert Nat) =
    .node (.node .off (some ⟨0, 0, 1, 0, 1⟩) .off) (some ⟨0, 1, 2, 1, 1⟩) .off := rfl
example : (chainOf [⟨0, 0, 1, 0, 1⟩, ⟨0, 1, 2, 1, 1⟩] : NodeCert Nat).stmt 1 =
    some ⟨0, 0, 2, 0, 2⟩ := by decide
-- A tree over the same two leaves certifies the same segment.
example : (NodeCert.node (.node .off (some ⟨0, 0, 1, 0, 1⟩) .off) none
    (.node .off (some ⟨0, 1, 2, 1, 1⟩) .off) : NodeCert Nat).stmt 1 = some ⟨0, 0, 2, 0, 2⟩ := by
  decide
-- A boundary mismatch, an empty node, a zero-step leaf and a leaf above S are refused.
example : (NodeCert.node .off (some ⟨0, 0, 1, 0, 1⟩) (.node .off (some ⟨0, 5, 2, 1, 1⟩) .off) :
    NodeCert Nat).stmt 1 = none := by decide
example : (NodeCert.node .off none .off : NodeCert Nat).stmt 8 = none := rfl
example : (NodeCert.node .off (some ⟨0, 0, 1, 0, 0⟩) .off : NodeCert Nat).stmt 8 = none := by decide
example : (NodeCert.node .off (some ⟨0, 0, 1, 0, 9⟩) .off : NodeCert Nat).stmt 8 = none := by decide
example : (NodeCert.node .off (some ⟨0, 0, 1, 0, 8⟩) .off : NodeCert Nat).stmt 8 =
    some ⟨0, 0, 1, 0, 8⟩ := by decide
-- `assert_lower_than_fixed` with bound S + 1 = 9 (k = 3, diff = 1) over a toy field p = 64.
example : ∀ x, x < 64 → ((x < 2 ^ 3 ∨ (x + (64 - (9 - 2 ^ 3))) % 64 < 2 ^ 3) ↔ x < 9) := by decide
-- The weaker check pair admits 2^64.
example : (2 ^ 64 - 1 : Nat) + 1 = 2 ^ 64 := by decide

-- L2 linkage
#print axioms Clean.Sherlock.Aggregation.link_ident_left
#print axioms Clean.Sherlock.Aggregation.link_ident_right
#print axioms Clean.Sherlock.Aggregation.link_ident_left_iff
#print axioms Clean.Sherlock.Aggregation.link_eq_some_iff
#print axioms Clean.Sherlock.Aggregation.link_eq_none_iff
#print axioms Clean.Sherlock.Aggregation.link_assoc
#print axioms Clean.Sherlock.Aggregation.linkOpt_assoc
#print axioms Clean.Sherlock.Aggregation.link_len
#print axioms Clean.Sherlock.Aggregation.link_fields
#print axioms Clean.Sherlock.Aggregation.foldl_linkOpt
#print axioms Clean.Sherlock.Aggregation.linkAll_single
#print axioms Clean.Sherlock.Aggregation.linkAll_cons
#print axioms Clean.Sherlock.Aggregation.linkAll_append
#print axioms Clean.Sherlock.Aggregation.linkAll_replace_left
#print axioms Clean.Sherlock.Aggregation.linkAll_replace_right
#print axioms Clean.Sherlock.Aggregation.Bracket.segs_ne_nil
#print axioms Clean.Sherlock.Aggregation.Bracket.fold_eq_linkAll
#print axioms Clean.Sherlock.Aggregation.bracket_independent
-- L3 padding
#print axioms Clean.Sherlock.Aggregation.NodeCert.stmt_node
#print axioms Clean.Sherlock.Aggregation.NodeCert.slot_off
#print axioms Clean.Sherlock.Aggregation.NodeCert.slot_node
#print axioms Clean.Sherlock.Aggregation.NodeCert.slot_cases
#print axioms Clean.Sherlock.Aggregation.NodeCert.empty_node_refused
#print axioms Clean.Sherlock.Aggregation.NodeCert.unary_left
#print axioms Clean.Sherlock.Aggregation.NodeCert.unary_right
#print axioms Clean.Sherlock.Aggregation.NodeCert.leaf_only
#print axioms Clean.Sherlock.Aggregation.NodeCert.leaf_out_of_range
#print axioms Clean.Sherlock.Aggregation.ident_prog
#print axioms Clean.Sherlock.Aggregation.ident_pre
#print axioms Clean.Sherlock.Aggregation.ident_post
#print axioms Clean.Sherlock.Aggregation.ident_lo
#print axioms Clean.Sherlock.Aggregation.ident_hi
#print axioms Clean.Sherlock.Aggregation.link_ident_left_eq
#print axioms Clean.Sherlock.Aggregation.link_ident_right_eq
#print axioms Clean.Sherlock.Aggregation.linkOpt_some_eq_some
#print axioms Clean.Sherlock.Aggregation.gated_empty
#print axioms Clean.Sherlock.Aggregation.gated_eq_linkAll
-- L7 shape, depth and S
#print axioms Clean.Sherlock.Aggregation.NodeCert.leaves_off
#print axioms Clean.Sherlock.Aggregation.NodeCert.stmt_eq_linkAll_leaves
#print axioms Clean.Sherlock.Aggregation.NodeCert.same_leaves_same_stmt
#print axioms Clean.Sherlock.Aggregation.NodeCert.stmt_independent_of_S
#print axioms Clean.Sherlock.Aggregation.NodeCert.stmt_mono_S
#print axioms Clean.Sherlock.Aggregation.chainOf_leaves
-- L4 coverage
#print axioms Clean.Sherlock.Aggregation.Covers.le
#print axioms Clean.Sherlock.Aggregation.Covers.bounds
#print axioms Clean.Sherlock.Aggregation.Covers.pairwise
#print axioms Clean.Sherlock.Aggregation.Covers.hpre_lt
#print axioms Clean.Sherlock.Aggregation.Covers.nodup
#print axioms Clean.Sherlock.Aggregation.Covers.sum
#print axioms Clean.Sherlock.Aggregation.Covers.prog_fixed
#print axioms Clean.Sherlock.Aggregation.linkAll_covers
#print axioms Clean.Sherlock.Aggregation.NodeCert.cert_covers
#print axioms Clean.Sherlock.Aggregation.NodeCert.cert_len_pos
-- L1 soundness
#print axioms Clean.Sherlock.Aggregation.Run.trans
#print axioms Clean.Sherlock.Aggregation.Run.unique
#print axioms Clean.Sherlock.Aggregation.Run.halt_unique
#print axioms Clean.Sherlock.Aggregation.sound_link
#print axioms Clean.Sherlock.Aggregation.sound_linkAll
#print axioms Clean.Sherlock.Aggregation.sound_leaf
#print axioms Clean.Sherlock.Aggregation.segment_sound
#print axioms Clean.Sherlock.Aggregation.chain_sound
-- L6 decider pinning, L-C
#print axioms Clean.Sherlock.Aggregation.other_program_refused
#print axioms Clean.Sherlock.Aggregation.other_init_refused
#print axioms Clean.Sherlock.Aggregation.non_genesis_refused
#print axioms Clean.Sherlock.Aggregation.decider_pinning
#print axioms Clean.Sherlock.Aggregation.result_unique
#print axioms Clean.Sherlock.Aggregation.statement_independent_of_S
-- L5 height arithmetic
#print axioms Clean.Sherlock.Aggregation.height_no_wrap
#print axioms Clean.Sherlock.Aggregation.weaker_pair_insufficient
#print axioms Clean.Sherlock.Aggregation.field_sum_exact
#print axioms Clean.Sherlock.Aggregation.field_eq_exact
-- L-D accumulator
#print axioms Clean.Sherlock.Aggregation.genesis_scale_by_bit
#print axioms Clean.Sherlock.Aggregation.non_genesis_keeps_proof
-- L-E range check
#print axioms Clean.Sherlock.Aggregation.lower_than_fixed_correct
