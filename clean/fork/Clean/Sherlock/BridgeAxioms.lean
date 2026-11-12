import Clean.Sherlock.Bridge
import Clean.Sherlock.PvBinding

/-!
Premise file of `Clean.Sherlock.Bridge` and `Clean.Sherlock.FrLog` (bridge round, QUEUE.md
step 3; CANDIDATE.md section 11.3).

No `axiom` is declared in any of the bridge files. Every assumption is a field of
`Clean.Sherlock.Bridge.Premises` and enters `bridge_sound` as a hypothesis, by name:

* (A1) `wrapKnowledgeSound`: a proof that ledger 9 `VerifierKey::verify` accepts for public input
  `pub` has a witness of `WrapRel`, the wrap that verifies `L_Fr` (option E, OD-24). Knowledge soundness of midnight-proofs PLONK with KZG over
  BLS12-381 in the algebraic group model with a q-type assumption, as for today's decider.
* (A2) `lastLayerSound`: a reference verification of `L` that accepts yields an accepted root AIR
  with some log digest. Unique decoding (owner decision OD-19), statements as recorded in
  `research/sherlock-design-paper-review-20261111/G5-PAPER-MAP.md`: WHIR Theorem 5.2 with list
  size 1, mutual correlated agreement from WHIR Lemma 4.10, Corollary 4.11 and the unique
  decoding branch of Theorem 4.8, OOD from WHIR Lemma 4.25, `mu` batching from SWIRL
  Theorem 4.2.4 (unique decoding branch), LogUp from SWIRL Theorem 4.2.1 (total interactions
  below `p`), composition by SWIRL Theorem 4.2.5, Fiat-Shamir in the random-oracle model (SWIRL
  Corollary 4.2.6), the `eq` binding of `pv` (error at most `c m / |EF|`) and the 5-digit
  squeeze bias. It has three hypotheses that `bridge_sound` discharges: `PackedHashInj` (proved
  from A2b by `packed_hash_inj`), `pvFirstT` and `ProvenRegime` (gate G4: `UDSet.gate` on the
  declared parameter set with the proof of work read from the key's transcript program; every
  folding, `mu` and query term at least 101 bits, every OOD and `gamma` term at least 100).
* (A2') `lfrSound`: if the reference verifier of `L_Fr` accepts `(s, pv, π)`, the reference
  verifier of `L` accepts an embedded `L` proof with the same statement and public values
  (knowledge soundness of the Fr STARK `L_Fr`). Hypotheses that `bridge_sound` discharges: A2b
  and `KeyOKFr` (`pv` absorbed first, tag separation of `L_Fr`'s hash domains from `L`'s).
  `lfrSound_of_emul` (a theorem) shows that an `L_Fr` enforcing the Fr-cell emulation `EmulL`
  meets it.
* (A2b) `frSpongeInj`: collision resistance of Midnight Poseidon over Fr (the transcript and
  Merkle hash of `L`, and the public digest form `PoseidonFr(tag, pack d2)`), guarded by the
  carrier `FrDom`. Separate from A5.
* (A3) `rootBinds`: the root AIR of `L` binds its public values to a carrier log tree: at most
  `B` entries, each well-formed, `pv = encode (pad B (flatten t))`, and the digest of `t` is the
  digest the recursion carries. A circuit fact of `L` (its log table, the `eq` binding and the
  onion recompute), measured in M9 and tested in M14.
* (A4) `recSound`: recursion soundness of leaves, discharge shards and nodes: an accepted digest
  is the digest of a carrier tree whose flattened log the run was proved against
  (`RunWith`: every operation either deferred into the log or proved inline by tier E).
* (A5) `p2Inj`: collision resistance of Poseidon2 over BabyBear, guarded by the carrier `P2Dom`
  as `Aggregation.Premises.digest_inj` is. It covers the log onion and the internal
  `sherlock-value/2` digests (owner decision 2026-11-11). Tree-digest injectivity
  (`digest_inj`) is a theorem from it.
* (A6) `oracleElim`: a run whose Fr operations are all `Valid` is a run of the reference machine.
  A premise until the Sherlock step relation is imported (as `Aggregation.Premises.Step`).
* (A7) `wrapZK`: zero knowledge of the blinded wrap: for every satisfying witness the proof
  distribution equals the simulator's on the public input alone. Not used by `bridge_sound`;
  used by `wrap_view_only_claim`. `indist_symm` and `indist_trans` say that indistinguishability
  is an equivalence.
* `vkL_ok`: not cryptographic. Static facts of the frozen key of `L` (`pvFirstT` of its
  transcript program, `ProvenRegime`, and the bound `< r` of every compiled
  expression), decided on the concrete key.
* `vkLFr_ok`: not cryptographic. `KeyOKFr vkL vkLFr`: `pvFirstFr`, `tagSep`, and the statement and
  `pv` lengths of `L`.

The carriers `P2Dom` and `FrDom` are abstract on purpose: a hash with a finite digest is not
injective on all inputs, so an unguarded injectivity premise would be false in Lean and make every
theorem vacuous. `consistent` below shows the premises are jointly satisfiable.

The allowed `#print axioms` output is the empty set or `propext`, `Quot.sound`,
`Classical.choice`.
-/

open Clean.Sherlock.Bridge

-- Sanity checks (decided by the kernel, not asserted).
example : pack [1, 2] = 1 + 2 * 2 ^ 31 := by decide
example : digitsOf pBB 5 7 = [7, 0, 0, 0, 0] := by decide
example : (Op.sponge 3).arity = 4 := rfl
example : encode [nopE] = [0, 0] := rfl
example : pad 3 [nopE] = [nopE, nopE, nopE] := rfl
example : efRaw3.bound < rFr := by decide

/-- The premises are jointly satisfiable (a trivial model: nothing is accepted). This shows
consistency only, not that the real primitives satisfy them. -/
def consistent : Premises where
  B := 0
  X := { frPerm := id, frSponge := fun _ => 0, jubjubMul := fun _ q => q, jubjubGen := (0, 0),
         p2 := fun _ => List.replicate 8 0, p2_width := fun _ => by simp,
         frSponge_lt := fun _ => by decide }
  vkL := { checks := [.arith (.const 0) (.const 1)], transcript := lTranscriptUD,
           nStmt := 2, pvLen := 4, whir := lUD }
  vkLFr := lfrKey
  LFrAccept := fun _ => False
  P2Dom := fun _ => False
  FrDom := fun _ => False
  LedgerProof := Unit
  LedgerAccepts := fun _ _ => False
  RootAccepts := fun _ _ _ => False
  RecAccepts := fun _ _ => False
  RunOps := fun _ _ => False
  Run := fun _ => True
  vkL_ok := ⟨lKey_ok.1, lKey_ok.2, by
    intro c hc
    simp only [List.mem_singleton] at hc
    subst hc
    exact ⟨by decide, by decide⟩⟩
  vkLFr_ok := lfrKey_ok
  wrapKnowledgeSound := fun _ _ h => h.elim
  lfrSound := fun _ _ _ _ _ h => h.2.2.2.1.elim
  lastLayerSound := by
    intro _ _ _ s pv π h
    have := (h.2.2.1 _ (List.mem_singleton_self _)).2
    change 0 % pBB = 1 % pBB at this
    exact absurd this (by decide)
  frSpongeInj := fun _ _ h => h.elim
  rootBinds := fun _ _ _ h => h.elim
  recSound := fun _ _ h => h.elim
  p2Inj := fun _ _ h => h.elim
  oracleElim := fun _ _ _ _ => trivial
  Coins := Unit
  prove := fun _ _ _ => ()
  sim := fun _ _ => ()
  Indist := fun _ _ => True
  indist_symm := fun _ _ _ => trivial
  indist_trans := fun _ _ _ _ _ => trivial
  wrapZK := fun _ _ _ => trivial

-- Every theorem of FrLog.lean, UDBits.lean, Bridge.lean, PvBinding.lean.
#print axioms Clean.Sherlock.Bridge.pBB_eq
#print axioms Clean.Sherlock.Bridge.pBB_pos
#print axioms Clean.Sherlock.Bridge.limb_fits
#print axioms Clean.Sherlock.Bridge.limbs_cover
#print axioms Clean.Sherlock.Bridge.pack_fits
#print axioms Clean.Sherlock.Bridge.packBase_gt
#print axioms Clean.Sherlock.Bridge.qMax_fits
#print axioms Clean.Sherlock.Bridge.r_big
#print axioms Clean.Sherlock.Bridge.add_mul_mod_congr
#print axioms Clean.Sherlock.Bridge.hornerR_eq
#print axioms Clean.Sherlock.Bridge.horner_lt
#print axioms Clean.Sherlock.Bridge.horner_inj
#print axioms Clean.Sherlock.Bridge.digits_unique
#print axioms Clean.Sherlock.Bridge.dec_inj
#print axioms Clean.Sherlock.Bridge.canon_mod_inj
#print axioms Clean.Sherlock.Bridge.limb_embed_inj
#print axioms Clean.Sherlock.Bridge.decR_canon
#print axioms Clean.Sherlock.Bridge.ltLE_sound
#print axioms Clean.Sherlock.Bridge.rLimbs_props
#print axioms Clean.Sherlock.Bridge.canon_check_sound
#print axioms Clean.Sherlock.Bridge.alias_without_canon
#print axioms Clean.Sherlock.Bridge.pack_injective
#print axioms Clean.Sherlock.Bridge.pack_length_check_needed
#print axioms Clean.Sherlock.Bridge.pack_range_check_needed
#print axioms Clean.Sherlock.Bridge.pack_gadget_refines
#print axioms Clean.Sherlock.Bridge.chunks8_nil
#print axioms Clean.Sherlock.Bridge.chunks8_cons
#print axioms Clean.Sherlock.Bridge.packChunksR_eq
#print axioms Clean.Sherlock.Bridge.packChunks_inj
#print axioms Clean.Sherlock.Bridge.packChunks_lt
#print axioms Clean.Sherlock.Bridge.bounded_quotient_sound
#print axioms Clean.Sherlock.Bridge.fmul_sound
#print axioms Clean.Sherlock.Bridge.unbounded_quotient_unsound
#print axioms Clean.Sherlock.Bridge.lift_eq
#print axioms Clean.Sherlock.Bridge.reduce_sound
#print axioms Clean.Sherlock.Bridge.reduce_bits_ok
#print axioms Clean.Sherlock.Bridge.Expr.evalP_eq
#print axioms Clean.Sherlock.Bridge.Expr.evalR_eq
#print axioms Clean.Sherlock.Bridge.Expr.eval_le_bound
#print axioms Clean.Sherlock.Bridge.reduce_side
#print axioms Clean.Sherlock.Bridge.arith_refines
#print axioms Clean.Sherlock.Bridge.arith_var_sound
#print axioms Clean.Sherlock.Bridge.efRaw_ok
#print axioms Clean.Sherlock.Bridge.ef_mul_sound
#print axioms Clean.Sherlock.Bridge.hornerExpr_eval
#print axioms Clean.Sherlock.Bridge.horner_refines
#print axioms Clean.Sherlock.Bridge.sumExpr_eval
#print axioms Clean.Sherlock.Bridge.roundSumExpr_eval
#print axioms Clean.Sherlock.Bridge.sumcheck_round_refines
#print axioms Clean.Sherlock.Bridge.whir_round_refines
#print axioms Clean.Sherlock.Bridge.whir_round_chain
#print axioms Clean.Sherlock.Bridge.coset_fold_refines
#print axioms Clean.Sherlock.Bridge.fold_complete
#print axioms Clean.Sherlock.Bridge.eqExpr_eval
#print axioms Clean.Sherlock.Bridge.eqSumExpr_eval
#print axioms Clean.Sherlock.Bridge.pv_eq_refines
#print axioms Clean.Sherlock.Bridge.eqVal_bool
#print axioms Clean.Sherlock.Bridge.squeeze_digits_sound
#print axioms Clean.Sherlock.Bridge.squeeze_alias
#print axioms Clean.Sherlock.Bridge.Op.code_length
#print axioms Clean.Sherlock.Bridge.Op.code_inj
#print axioms Clean.Sherlock.Bridge.nopE_wf
#print axioms Clean.Sherlock.Bridge.pad_length
#print axioms Clean.Sherlock.Bridge.flatten_append_inj
#print axioms Clean.Sherlock.Bridge.enc_append_inj
#print axioms Clean.Sherlock.Bridge.encode_inj
#print axioms Clean.Sherlock.Bridge.encode_append_inj
#print axioms Clean.Sherlock.Bridge.pvOf_length
#print axioms Clean.Sherlock.Bridge.pvOf_inj
#print axioms Clean.Sherlock.Bridge.pad_wf
#print axioms Clean.Sherlock.Bridge.replay_sound
#print axioms Clean.Sherlock.Bridge.nop_pad_valid
#print axioms Clean.Sherlock.Bridge.onion_snoc
#print axioms Clean.Sherlock.Bridge.onion_empty
#print axioms Clean.Sherlock.Bridge.onionRev_length
#print axioms Clean.Sherlock.Bridge.onionRev_inj
#print axioms Clean.Sherlock.Bridge.onionRev_cases
#print axioms Clean.Sherlock.Bridge.seg_ne_node
#print axioms Clean.Sherlock.Bridge.digest_length
#print axioms Clean.Sherlock.Bridge.digest_inj
#print axioms Clean.Sherlock.Bridge.filter_split
#print axioms Clean.Sherlock.Bridge.spill_equiv
#print axioms Clean.Sherlock.Bridge.runWith_all_valid
#print axioms Clean.Sherlock.Bridge.discharge_sound
#print axioms Clean.Sherlock.Bridge.lUD_gate
#print axioms Clean.Sherlock.Bridge.intUD_gate
#print axioms Clean.Sherlock.Bridge.lUD_bits
#print axioms Clean.Sherlock.Bridge.lUD_fold_min
#print axioms Clean.Sherlock.Bridge.intUD_bits
#print axioms Clean.Sherlock.Bridge.lUD_mu5_fails
#print axioms Clean.Sherlock.Bridge.lUD_q97_fails
#print axioms Clean.Sherlock.Bridge.lUD_no_ood_fails
#print axioms Clean.Sherlock.Bridge.lUD_two_ood
#print axioms Clean.Sherlock.Bridge.appUD_gate
#print axioms Clean.Sherlock.Bridge.leafUD_gate
#print axioms Clean.Sherlock.Bridge.appUD_bits
#print axioms Clean.Sherlock.Bridge.leafUD_bits
#print axioms Clean.Sherlock.Bridge.run_refines
#print axioms Clean.Sherlock.Bridge.trStart_eq_cap
#print axioms Clean.Sherlock.Bridge.cap_inj
#print axioms Clean.Sherlock.Bridge.pv_absorbed_before_challenge
#print axioms Clean.Sherlock.Bridge.ood_batch_refines
#print axioms Clean.Sherlock.Bridge.ood_point_digits
#print axioms Clean.Sherlock.Bridge.getD_lt
#print axioms Clean.Sherlock.Bridge.check_refines
#print axioms Clean.Sherlock.Bridge.emulL_refines
#print axioms Clean.Sherlock.Bridge.runFr_refines
#print axioms Clean.Sherlock.Bridge.tags_separated
#print axioms Clean.Sherlock.Bridge.transcripts_separated
#print axioms Clean.Sherlock.Bridge.lfrSound_of_emul
#print axioms Clean.Sherlock.Bridge.wrap_refines
#print axioms Clean.Sherlock.Bridge.packed_hash_inj
#print axioms Clean.Sherlock.Bridge.bridge_sound
#print axioms Clean.Sherlock.Bridge.wrap_view_only_claim
#print axioms Clean.Sherlock.Bridge.merkle_binding_or_collision
#print axioms Clean.Sherlock.Bridge.public_digest_binds
#print axioms Clean.Sherlock.Bridge.lKey_ok
#print axioms Clean.Sherlock.Bridge.lKey_powVec
#print axioms Clean.Sherlock.Bridge.lKey_mu5_not_proven
#print axioms Clean.Sherlock.Bridge.lKey_query19_not_proven
#print axioms Clean.Sherlock.Bridge.lKey_logup17_not_proven
#print axioms Clean.Sherlock.Bridge.lKey_missing_folds_not_proven
#print axioms Clean.Sherlock.Bridge.lKey2_ok
#print axioms Clean.Sherlock.Bridge.ood_interleaved_rejected
#print axioms Clean.Sherlock.Bridge.lKey_no_ood_not_proven
#print axioms Clean.Sherlock.Bridge.lKey_fold_order_not_proven
#print axioms Clean.Sherlock.Bridge.s4_mult_three
#print axioms Clean.Sherlock.Bridge.lKeyLD_ok
#print axioms Clean.Sherlock.Bridge.s4_pow15_not_proven
#print axioms Clean.Sherlock.Bridge.remark_m1_not_proven
#print axioms Clean.Sherlock.Bridge.lfrKey_ok
#print axioms Clean.Sherlock.Bridge.lfr_tag_reuse_rejected
#print axioms Clean.Sherlock.Bridge.late_pv_rejected
#print axioms Clean.Sherlock.Bridge.partial_pv_rejected
#print axioms Clean.Sherlock.Bridge.PvBinding.euclid_of_prime
#print axioms Clean.Sherlock.Bridge.PvBinding.mle_mod
#print axioms Clean.Sherlock.Bridge.PvBinding.points_length
#print axioms Clean.Sherlock.Bridge.PvBinding.countP_le_one
#print axioms Clean.Sherlock.Bridge.PvBinding.line_roots
#print axioms Clean.Sherlock.Bridge.PvBinding.flat_count
#print axioms Clean.Sherlock.Bridge.PvBinding.mul_pow_pred_le
#print axioms Clean.Sherlock.Bridge.PvBinding.pv_binding_count
#print axioms Clean.Sherlock.Bridge.PvBinding.noDivFrom_sound
#print axioms Clean.Sherlock.Bridge.PvBinding.prime_of_noDiv
#print axioms Clean.Sherlock.Bridge.PvBinding.pBB_prime
#print axioms Clean.Sherlock.Bridge.PvBinding.pv_binding_count_babybear
