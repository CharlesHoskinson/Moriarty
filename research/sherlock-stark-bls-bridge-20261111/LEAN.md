**BUILD PASS** (`lake build` of `SherlockSpec`, 31 jobs, Lean v4.33.1). Sorry count 0; no `admit`, no `native_decide`, no `axiom` declarations. `#print axioms bridge_sound`: `[propext, Classical.choice, Quot.sound]`. Round 2 closes the four gaps of the lead review: lemma 23 is proved, `coset_fold`, `whir_round` and `pv_eq` are specific lemmas, `pvFirst` and the proof-of-work part of `ProvenRegime` are derived from the transcript of `L`, and the model applies `d_0 = P2(TAG_EMPTY)` and a fixed `pv` length per key.

# Bridge round, step 3: Lean formalization of the candidate

Date: 2026-11-11. Branch `bridge-lean` (worktree `~/Projects/Moriarty-wt/bridge-lean`). Source plan: `CANDIDATE.md` section 11, the two owner decisions of 2026-11-11 at its end (internal digests on `sherlock-value/2`, Poseidon2 over BabyBear; proven 100-bit security for `L`, folding and `mu` PoW 21 bits, BCHKS25 Theorem 1.5 as stated at `m = 3`), and the four lead amendments of the same date.

Files (project `clean/fork`, Lean `v4.33.1`, no mathlib, library `SherlockSpec`):

- `clean/fork/Clean/Sherlock/FrLog.lean`: limb encoding, packing, bounded-quotient multiplication, the expression compiler, the sumcheck, fold and `eq` checks, squeeze digits, log entries, encoding, the fixed-length `pv` vector, the onion digest, the deferred-or-emulated split.
- `clean/fork/Clean/Sherlock/Bridge.lean`: the transcript of `L`, the verifier of `L`, the wrap relation, `Premises`, `bridge_sound`, the zero-knowledge corollary, supporting lemmas, the example key.
- `clean/fork/Clean/Sherlock/PvBinding.lean`: lemma 23 (`pv_binding_count`) and the primality of BabyBear `p`.
- `clean/fork/Clean/Sherlock/BridgeAxioms.lean`: the premise documentation, kernel sanity checks, a consistency model of `Premises`, and `#print axioms` for every theorem (112).

Build commands: `lake build` (all of `SherlockSpec`) and `lake env lean Clean/Sherlock/BridgeAxioms.lean`, from `clean/fork`, under `flock ~/.cache/cost-opt/measure/heavy.lock heavy-run --mem 16G`. Logs: `~/.cache/bridge-lean/final.log`.

## 1. The theorem

```lean
theorem bridge_sound (P : Premises) {pub : List Nat} {prf : P.LedgerProof}
    (h : P.LedgerAccepts pub prf) : P.Run pub
```

`pub` is the wrap's whole public input, and it is the statement (the claim). `P.Run pub` says the reference machine makes that claim true. Proof route (in `Bridge.lean`):

1. A1 gives a wrap witness `w` with `WrapRel X B vkL pub w`.
2. `wrap_refines` (a theorem) gives: the reference verifier of `L` accepts `(pub, w.pv, w.π)`, including the reference transcript run (`run_refines`); the wrap table `w.T` has exactly `B` entries; every entry is well-formed (canonical values) and `Valid`; `w.pv = pvOf vkL.pvLen w.T` (the encoding of `w.T`, zero-padded to the key's fixed length).
3. A2, with its three hypotheses discharged (`packed_hash_inj` from A2b; `pvFirstT` and `ProvenRegime` from the static key check, both computed from the key's transcript program), gives a root acceptance with some digest `d`.
4. A3 gives a carrier log tree `t` with `pvOf vkL.pvLen (pad B (flatten t)) = w.pv` and digest `d`.
5. `pvOf_inj` (from `encode_append_inj`) gives `pad B (flatten t) = w.T`, so every entry of `flatten t` is valid.
6. A4 gives a carrier tree `t'` with digest `d` and `RunWith (flatten t') pub`.
7. `digest_inj` (a theorem from A5) gives `t' = t`.
8. `runWith_all_valid` turns the run into one whose Fr operations are all valid (deferred ones from step 5, emulated ones from the leaves); A6 closes.

Companion theorem for the zero-knowledge boundary:

```lean
theorem wrap_view_only_claim (P : Premises) {pub : List Nat} {w₁ w₂ : WrapWit}
    (h₁ : WrapRel P.X P.B P.vkL pub w₁) (h₂ : WrapRel P.X P.B P.vkL pub w₂) :
    P.Indist (P.prove pub w₁) (P.prove pub w₂)
```

Two witnesses for the same claim (different logs, used-entry counts, splits, proofs of `L`) give indistinguishable ledger proofs.

## 2. Premises (fields of `Premises`, no `axiom` declared)

| Name | Field | Plain meaning |
|---|---|---|
| A1 | `wrapKnowledgeSound` | If ledger 9 accepts a proof for `pub`, a witness of the wrap circuit exists (midnight-proofs PLONK with KZG, knowledge sound in the AGM with a q-type assumption). |
| A2 | `lastLayerSound` | If the reference verifier of `L` accepts, the root AIR of `L` accepted with some log digest. The proximity-gap part is the proven Johnson-radius statement (citations per REVIEW.md, not read by this lane): BCHKS25 Theorem 1.5 as stated (`m = max(ceil(sqrt(rho)/(2 eta)), 3) = 3` at S4), with Theorems 4.5 (batching) and 4.6 (mutual correlated agreement); with 21-bit folding and `mu` PoW this gives 100.2 and 102.3 bits (paper review, `research/sherlock-design-paper-review-20261111/REVIEW.md` section 4). The unproved p.23 remark (`m < 3`) is not a premise. The rest: WHIR round-by-round soundness, Fiat-Shamir in the random-oracle model (non-interactive error `Q_RO 2^-100 + 3 (Q_RO^2 + 1) / 2^kappa`), the `eq` binding error `c m / |EF|` (its counting core is lemma 23, proved), the squeeze bias. It holds only under three hypotheses that the proof discharges: the packed hash is injective, the transcript program absorbs every word of `pv` before its first challenge or proof of work (`pvFirstT`), and the key is in `ProvenRegime`. |
| A2b | `frSpongeInj` | Midnight Poseidon over Fr has no collision on its carrier domain `FrDom` (the transcript and Merkle hash of `L`, and the public digest form). |
| A3 | `rootBinds` | The root AIR binds its public values to a carrier log tree (at most `B` entries, all well-formed), padded with `NOP` to `B` entries and then with zero words to the key's `pvLen`, with the digest that the recursion carries. |
| A4 | `recSound` | The recursion (leaves, discharge shards, nodes) proves the run against the flattened log of a carrier tree with that digest; each Fr operation is either in the log or proved inline by tier E. |
| A5 | `p2Inj` | Poseidon2 over BabyBear has no collision on its carrier domain `P2Dom` (the log onion and the internal `sherlock-value/2` digests). Separate from A2b, as the owner decision requires. |
| A6 | `oracleElim` | A run whose Fr operations are all valid is a run of the reference machine (a premise until the Sherlock step relation is imported). |
| A7 | `wrapZK` | The blinded wrap is zero knowledge: for a true statement the proof distribution equals the simulator's on `pub` alone. Not used by `bridge_sound`; used by `wrap_view_only_claim`. `indist_symm`, `indist_trans` make indistinguishability an equivalence. |
| (static) | `vkL_ok` | Not cryptographic: `KeyOK vkL`. The transcript program of the frozen key absorbs `pv` before its first challenge (`pvFirstT`), the key is in `ProvenRegime` (proof-of-work bits read from the transcript, `m` from Theorem 1.5), and every compiled expression has a tracked bound below `r`. Decidable on the concrete key (`lKey_ok` decides it for the example key). |

`Prims` gained one data fact, `frSponge_lt` (the Fr sponge output is below `r`), which the squeeze refinement needs. It is a property of the function's type in the real system, not a cryptographic assumption.

The carriers `P2Dom` and `FrDom` are abstract, as `Carrier` is in `Aggregation.lean`. An unguarded injectivity premise for a hash with a finite digest would be false in Lean (pigeonhole) and would make every theorem vacuous. `BridgeAxioms.consistent` is a model of all premises (a trivial one: nothing is accepted, with the example key `lKey 21 21 20 s4Whir`, `nStmt = 2`, `pvLen = 4`), which shows they are jointly satisfiable. It does not show that the real primitives meet them.

## 3. Where each soundness point sits

"On path" lemmas are used by the proof of `bridge_sound`. "Supporting" lemmas are proved but not used by it.

| Point | Definition clause | On path | Supporting | Negative lemma (the check is needed) |
|---|---|---|---|---|
| Bounded-quotient encoding of BabyBear multiplication in Fr | `ArithGadget`: `c < p`, `q < 2^223`, both sides `= c + q p` in Fr; `ArithOK`: tracked bound `< r` | `bounded_quotient_sound`, `arith_refines`, `check_refines`, `wrap_refines` | `fmul_sound`, `ef_mul_sound`, `horner_refines`, `reduce_sound` | `unbounded_quotient_unsound`: with `q = p^(-1) mod r`, `1 * 1 = 0 + q p` in Fr |
| WHIR sumcheck rounds (lemma 11) | `SumcheckRound` (coefficients, `alpha`, claim, next); two `arith` checks per round: `roundSumExpr cs = claim` and `hornerExpr alpha cs = next` | `arith_refines` (each check is an `arith` check of the key) | `sumcheck_round_refines` (gives `g(0) + g(1) = claim` and `g(alpha) = next` mod `p`), `whir_round_refines` (every round of a WHIR round), `whir_round_chain` (with `Chained`, the claim of round `j + 1` is `g_j(alpha_j)`) | n/a |
| WHIR coset fold (lemma 11) | `foldLhs`, `foldRhs`: the division-free fold `2 x y = x (f(x) + f(-x)) + beta (f(x) - f(-x))` | `arith_refines` | `coset_fold_refines` (the wrap check gives the reference fold relation mod `p`), `fold_complete` (the honest fold of `a + b t` passes) | n/a |
| `eq` evaluation of `pv` (lemma 13) | `eqExpr`, `eqSumExpr rho tbl`: `sum_i eq(rho, bits_i) pv_i = claim` | `arith_refines` | `pv_eq_refines` (the wrap check gives the reference value mod `p`), `eqVal_bool` (on Boolean points `eq` is the indicator, so the sum is the multilinear extension of `pv`) | n/a |
| `pv` binding count (lemma 23) | `PvBinding.mle`, `points`, `agree` over `Z/q` | none (the error term sits inside A2) | `pv_binding_count` (two tables that differ on `{0,1}^m` have extensions that agree on at most `m q^(m-1)` of `q^m` points, for `q` with Euclid's property), `euclid_of_prime`, `pBB_prime` (kernel trial division), `pv_binding_count_babybear` | n/a |
| Canonical injective packing of 8 x 31-bit limbs with checked length | `pack`, `chunks8`, `packChunks`; `PackedHashInj` requires equal lengths and words `< 2^31` | `pack_injective`, `packChunks_inj`, `packed_hash_inj` (discharges A2's first hypothesis), `packChunksR_eq` in `run_refines` | `public_digest_binds`, `pack_gadget_refines` | `pack_length_check_needed` (`[1]` and `[1, 0]`), `pack_range_check_needed` (`[2^31]` and `[0, 1]`) |
| Canonical logged values | `CanonGadget`: 9 limbs `< 2^30`, lexicographically below the limbs of `r`; `EntryGadget` | `canon_check_sound`, `replay_sound`, `pvOf_inj` | `dec_inj`, `canon_mod_inj`, `limb_embed_inj` | `alias_without_canon`: limbs of `5` and of `5 + r` recompose to the same Fr element |
| Squeezed challenge digits (amendment 3) | `TOp.squeeze` in the transcript; `runGadget` uses `SqueezeGadget` (5 digits `< p`, recomposition `< r`), `runRef` the reference digits | `squeeze_digits_sound` inside `run_refines` | | `squeeze_alias`: the digits of `r` recompose `0` in Fr |
| Log entries absorbed before the first challenge | `KeyL.transcript : List TOp`; `absorbedBeforeChallenge`; `pvFirstT nStmt pvLen transcript`, computed from the transcript; `VerifyL` places `pv` at indices `nStmt .. nStmt + pvLen - 1` and runs `runRef` | A2's second hypothesis, discharged by `vkL_ok.1` | `pv_absorbed_before_challenge` (every word of an accepted `pv` is absorbed before the first challenge), `lKey_ok` (the stark-backend preamble order) | `late_pv_rejected`, `partial_pv_rejected` |
| Proven proximity regime (owner decision) | `ProvenRegime vk`: `m = thm15Mult` (`m` as Theorem 1.5 sets it), and the weakest folding, `mu` and query PoW of the transcript (`powList`, `powMin`) at least 21, 21 and 20; `runRef` checks each `TOp.pow` (output has `bits` low zero bits); `s4Whir` = log blowup 4, `eta = 1/8`, `m = 3` | A2's third hypothesis, discharged by `vkL_ok.2.1` | `s4_mult_three`, `lKey_ok` | `s4_pow15_not_proven` (the former 15-bit PoW), `remark_m1_not_proven` (the `m = 1` reading of the p.23 remark) |
| Fixed log capacity and fixed `pv` length (amendment 4), overflow to STARK shards | `WrapRel`: `w.T.length = B`, `(encode w.T).length <= pvLen`, `w.pv = pvOf pvLen w.T`; `VerifyL`: `pv.length = pvLen`; `LogTree.Carrier`: at most `B` entries; `pad`; `RunWith`: each operation deferred or emulated | `pad_length`, `pad_wf`, `pvOf_length`, `pvOf_inj`, `digest_inj`, `runWith_all_valid` | `spill_equiv`, `discharge_sound` (any split of a valid run, and a discharge shard, give the same claim) | n/a |
| Empty onion is a hash output (amendment 1) | `onion X [] = X.p2 [tagEmpty]` | `digest_inj` | `onion_empty` | n/a |
| Public statement is only the claim | `WrapRel X B vk pub w`: `pub` is the only public input, of the key's fixed length `nStmt`; `VerifyL` reads the statement from `pub` | A1 to A6 are all stated on `pub` | A7 and `wrap_view_only_claim` | n/a |

Limits of this modeling that the lead should know:

- The wrap's arithmetic gadget in `WrapRel` is `ArithGadget` (unsigned values, `c + q p` with `0 <= q < 2^223`). The plan's signed `ReduceRel` is formalized and proved (`reduce_sound`) but is not the gadget that `WrapRel` uses; a wrap that uses signed lazy reduction needs `ArithGadget` replaced by `ReduceRel`.
- The transcript of `L` is a program in the key (`KeyL.transcript : List TOp`: absorb BabyBear words, absorb Fr elements, squeeze, proof of work), and the verifier `VerifyL` runs it (`runRef`). So `pvFirstT` and the proof-of-work bits of `ProvenRegime` are derived from what the verifier executes, not declared. What stays inside A2 is that this `TOp` program is the transcript of the real stark-backend verifier of `L`; that link is a property of the code that freezes the key, outside Lean.
- Only the WHIR analysis parameters stay key metadata (`WhirParams`: `logBlowup`, `eta`, `mult`). Reason: `mult` (the Guruswami-Sudan multiplicity) and `eta` (the slack below the Johnson radius) are not verifier operations; they occur only in the soundness proof of the proximity gap. `logBlowup` occurs in the verifier only through the domain constants of the `arith` checks, and this model has no Reed-Solomon code to tie those constants to (core Lean, no mathlib). Deriving them would need a formal WHIR verifier with its code, which is outside this lane. `ProvenRegime` checks them against Theorem 1.5 (`thm15Mult`), and the proximity-gap theorem itself stays inside A2.
- `coset_fold_refines`, `whir_round_refines` and `pv_eq_refines` are specific lemmas about named expression shapes. `bridge_sound` does not need them: every such check is an `arith` check of the key, and `arith_refines` covers all of them at once. The specific lemmas state what the reference relation is for each shape.
- Lemma 23 is proved over the prime field `Z/q` (BabyBear). The binding challenge of `L` lies in the extension field `EF`; the same induction applies over any field, but this lane proves the prime-field case, and the bound enters `bridge_sound` only through A2.

## 4. Statements changed from the plan (CANDIDATE.md section 11)

1. `Params` removed. `p` and `r` are fixed literals (`pBB`, `rFr`), with `w = 30`, `l = 9`, base `2^31`, 8 words per Fr element, and every bound checked by `decide`. Reason: the plan's `hr : 2^248 < r` is too weak for the lazy reduction. `ReduceRel` with `b <= 252` needs `|x - (q p + t)| < 2^252 + 2^253 + 2^31 < r`, which needs `r > 2^254`. True for BLS12-381 Fr (`r_big`, `reduce_bits_ok`). No design change.
2. `castP`, `castR` and `|.|` replaced by `%` on `Nat`/`Int` and two-sided bounds; `ReduceRel` takes explicit bounds `X`, `Q` with the static condition `X + Q p + 2^31 <= r`. `lift_eq` is stated in difference form (`-r < x - y < r`). Reason: no mathlib absolute value; the explicit form is what the wrap's static bound check verifies.
3. `verifyL` returns `Prop`, not `Bool`, and is defined (`VerifyL` over `Check` and the transcript program), not a `Premises` field. `WrapRel`, `RunWith`, `Carrier` are also definitions. Reason: as opaque fields, `wrap_refines`, `encode_inj` and `digest_inj` cannot be proved and none of the soundness points would be in the theorem.
4. `RunWith` is defined from a new field `RunOps` (the leaves' relation over the run's whole Fr operation sequence) and a split into deferred and emulated operations. `oracleElim` (A6) is stated on `RunOps`. Reason: makes lemmas 20 and 21 provable and puts the overflow path in the proof.
5. A2 (`lastLayerSound`) has three hypotheses: `PackedHashInj X FrDom`, `pvFirstT vkL.nStmt vkL.pvLen vkL.transcript = true` and `ProvenRegime vkL = true`. Reason: these are the conditions under which the soundness of `L` holds; stating them puts packing, transcript order and the proven proximity regime on the formal path. All three are discharged inside `bridge_sound`. The proximity-gap theorem itself stays inside A2: core Lean has no Reed-Solomon codes, so the formal content is that the key's parameters are the ones Theorem 1.5 covers as stated, not the theorem.
6. New premises A2b (`frSpongeInj`, Fr hash) and A7 (`wrapZK`), and the static field `vkL_ok`. Reason: owner decision (small-field digest premise separate from the Fr transcript hash premise) and the task (ZK boundary as a premise of the blinded wrap). The premise count is therefore 8 named assumptions plus one static key fact, not 6.
7. `p2Inj` is guarded by an abstract `P2Dom` (not "well-formed tagged inputs"), and `LogTree.Carrier` includes `DigestIn`: every `p2` input of the tree's digest is in `P2Dom`. Reason: a guard that admits every BabyBear vector makes the premise false (finite digest), see section 2.
8. `Prims.p2` carries `p2_width : (p2 a).length = 8`. Reason: without a fixed width the input `tag :: (d_a ++ d_b)` cannot be split, so `digest_inj` fails. `Prims.frSponge` carries `frSponge_lt` (output below `r`), used by the squeeze refinement in `run_refines`.
9. The route compares padded tables: it gives `pad B (flatten t) = T_w`, not `flatten t = T_w`. Reason: `L` pads with `NOP`. Entries have a fixed arity per operation (`Op.arity`) and a two-word code; without the fixed arity the concatenation is not injective.
10. `Valid` takes the statement (for `PUB(i)`). `PATH(d)` is membership with private bits and siblings. `DIGEST` checks `out = frSponge [tag, pack d2]` for 8 BabyBear words `d2` (the `sherlock-value/2` public form).
11. `rootBinds` returns `pvOf vkL.pvLen (pad B (flatten t)) = pv` (the plan had `encode (flatten t) = pv`). Reasons: item 9, and the fixed `pv` length of amendment 4.
12. Lemma 7 (`add_bound`, `mul_bound`) is `Expr.eval_le_bound`. Lemma 8 uses an explicit 12-cell environment and a tuple `efMul`. Lemmas 11 and 13 are specific theorems: `sumcheck_round_refines`, `whir_round_refines`, `whir_round_chain` and `coset_fold_refines` (with `fold_complete`), and `pv_eq_refines` (with `eqVal_bool`). The plan's `coset_fold` is stated in the division-free form `2 x y = x (f(x) + f(-x)) + beta (f(x) - f(-x))`, which is what a constraint can check. Lemma 10's `sponge_refines` and `merkle_path_refines` are the `hash` and `merkle` cases of `check_refines` (native Fr in the wrap, so the refinement is the packing gadget `packChunksR_eq`).
13. Lemma 12 (`constraint_dag_refines`) is `arith_refines` over expression trees (`Expr`), not a DAG generated from OpenVM's Lean emitter. A DAG is a tree with sharing; the semantics are the same.
14. Lemma 23 (`pv_binding_count`) is proved (`PvBinding.lean`), in the form: two tables `T`, `U` on `{0,1}^m` that differ mod `q` have multilinear extensions that agree on at most `m q^(m-1)` of the `q^m` points of `(Z/q)^m`. The proof is by induction on `m` with a one-variable root count (`line_roots`); the only field fact it uses is Euclid's property, proved from primality (`euclid_of_prime`). Primality of `p = 2013265921` is checked by the kernel with `decide +kernel` on trial division to 44,871 (`pBB_prime`), not with `native_decide`. It is off the trust path (its error term sits inside A2).
15. The onion has two extra domain tags, `tagStep` (chain step) and `tagEnt` (entry hash), beside `TAG_EMPTY` and `TAG_NODE`: `d_(i+1) = P2(tagStep, d_i, P2(tagEnt, enc(e_i)))`, with `d_0 = P2(TAG_EMPTY)` (`onion_empty`, amendment 1). One variable-length `p2` stands for both the two-to-one hash and the entry hash `H2s` of the plan; the tags separate the uses.
16. The key fixes the `pv` length (amendment 4): `KeyL.pvLen`, `VerifyL` requires `pv.length = pvLen`, and the wrap sets `pv = pvOf pvLen T` (the encoding of the table, zero-padded) with the check `(encode T).length <= pvLen`. The capacity `B` still counts entries; a table whose encoding is longer than `pvLen` has no wrap witness, so `pvLen` must cover `B` entries of the longest arity (or the wrap bounds `n` per `SPONGE(n)` slot). The padding is injective because `encode` is prefix-free on tables with the same entry count (`encode_append_inj`).
17. The statement length is fixed per key (`KeyL.nStmt`, checked in `WrapRel` and `VerifyL`), so the `pv` indices in the transcript are static.
18. `Check.squeeze` is removed. The squeeze is now a transcript operation (`TOp.squeeze`) whose input is the sponge state of the absorbed transcript, not a free cell. The plan's `squeeze` check had no link to what was absorbed.
19. The paper citations in A2 (BCHKS25 Theorems 1.5, 4.5, 4.6; SWIRL Cor 4.2.6) are taken from `research/sherlock-design-paper-review-20261111/REVIEW.md` section 4. This lane did not read the papers. Lemma 23 is the standard Schwartz-Zippel count for multilinear polynomials and is proved here, not cited.

## 5. Design findings

1. Empty onion must be a hash output (amendment 1, applied). With `d_0 = TAG_EMPTY` as a raw constant (section 3.1 of the candidate), tree-digest injectivity needs a preimage assumption (`p2` never outputs `TAG_EMPTY`) that collision resistance does not give: an empty segment could equal a `TAG_NODE` merge. The model uses `d_0 = P2([TAG_EMPTY])` (`onion_empty`), at one extra permutation per empty segment.
2. The quotient bound is a hard requirement, not a cost choice (amendment 2, in the model as `ArithGadget`). Without `q < 2^223` (any bound with `c + q p < r`), a prover can claim a false product: for any claimed residue `c`, `q = (a b - c) p^(-1) mod r` satisfies the Fr equation (`unbounded_quotient_unsound` is the instance `1 * 1 = 0`). Every bounded-quotient gadget of the wrap must range-check `q`. The static bound `< r` of every lazily evaluated expression must be checked when the key is frozen (`vkL_ok`).
3. Packing needs both checks the candidate names: equal (key-fixed) absorb lengths and words below `2^31`; each alone is not enough (`pack_length_check_needed`, `pack_range_check_needed`). In the transcript of `L` the absorb lengths are fixed by the key (`TOp.absorbBB` lists its variables), so the length check is static.
4. The squeeze needs a canonical check on the recomposition (amendment 3, in the model as `SqueezeGadget` inside `runGadget`; `squeeze_alias`), the same `v` against `v + r` issue as for logged values.
5. The `pv` length is fixed per key (amendment 4, applied as `pvLen` and `pvOf`). Sizing input for M9: `pvLen` must be at least the encoded length of `B` entries of the longest arity, or `SPONGE(n)` slots need a bound on `n`.
6. Pigeonhole: hash premises in Lean must be carrier-guarded with an abstract domain. Stating the small-field digest premise as "injective on all BabyBear vectors" would be false and make the theorem vacuous. The formalization follows `Aggregation.lean`.
7. The proven regime needs the 21-bit folding and `mu` PoW in the transcript of `L` (owner decision). `ProvenRegime` reads the weakest PoW of each kind from the transcript program, so a key frozen with one 15-bit PoW, or with `m = 1`, fails `vkL_ok` and `bridge_sound` does not apply to it. A transcript with no PoW of a kind also fails (the minimum is `0`). The bit figures (100.2, 102.3) are taken from the paper review and are not computed in Lean; M4 (the soundness calculator) should confirm them for S4.
8. The transcript must absorb all of `pv` before the first challenge or proof of work; absorbing a part first is rejected (`partial_pv_rejected`). The stark-backend preamble order (commitments, then public values, then challenges) passes (`lKey_ok`).
9. No design change found for: the fixed capacity with spill (`spill_equiv`, `discharge_sound` hold for any split), the ZK boundary (only `pub` is public), and the case-A digest form `PoseidonFr(tag, pack d2)` (`public_digest_binds`).

## 6. Axioms

Output of `lake env lean Clean/Sherlock/BridgeAxioms.lean` (log `~/.cache/bridge-lean/final.log`):

```
'Clean.Sherlock.Bridge.bridge_sound' depends on axioms: [propext, Classical.choice, Quot.sound]
'Clean.Sherlock.Bridge.wrap_view_only_claim' depends on axioms: [propext, Quot.sound]
'Clean.Sherlock.Bridge.PvBinding.pv_binding_count' depends on axioms: [propext, Classical.choice, Quot.sound]
```

All 112 theorems of `FrLog.lean`, `Bridge.lean` and `PvBinding.lean` are queried: 23 depend on no axioms, 16 on `[propext]`, 46 on `[propext, Quot.sound]`, 27 on `[propext, Classical.choice, Quot.sound]`. None uses another axiom (no `Lean.ofReduceBool`, so no `native_decide`). The cryptographic assumptions are not axioms: they are the `Premises` fields of section 2, hypotheses of every theorem that uses them.

## 7. Open items

1. OOD term under the `m = 3` interpolant reading (no new proof work in this round). `research/sherlock-call-time-opt-20261111/RESULT.md` section 4.1 finds that under reading B of Lemma 3.1 (list size `D_Y` of the `m = 3` interpolant, 14 at rate `2^-4`) the OOD term of the adopted option (b) is 98.0 bits after round 0 (99.0 after round 1), with no PoW to raise it. This term sits inside A2, so `bridge_sound` is unchanged, but the 100-bit claim for `L` under A2 does not hold under reading B. Recommended closure (owner decision pending, S5 item 1): two OOD samples per round, error `(L^2/2)(2^m_i/|EF|)^2`, with the bound derived and proved in Lean beside `pv_binding_count` (a union over list pairs and a Schwartz-Zippel count per independent sample, the same kind of counting as lemma 23). Alternative: name the Guruswami-Sudan list bound at the radius multiplicity as premise P4c, which adds an external theorem to A2.
2. The `L` parameter set changes if the owner accepts RESULT.md section 4.5 (S5 item 3): blowup 4, radius `(4/3) sqrt(rho)`, queries 52/27/18, two OOD samples per round, `mu` PoW 20, query PoW 20, and the per-fold PoW vector of section 4.3 (S5 item 2: round 0 folds 15, 14, 13, 12; round 1 folds 19, 18, 17, 16; round 2 folds 22, 21, 20, 19). The present `ProvenRegime` requires every folding and `mu` PoW to be at least 21 bits, so that key fails it. If the owner accepts the change, `ProvenRegime` must check the PoW of each fold position against the vector (the transcript tags each `TOp.pow` already; the fold index must be added to `PowKind.fold`), `mu` against 20, `s4Whir` must take the new radius (`eta` from `(4/3) sqrt(rho)`, with `thm15Mult` still `3`), and `lKey_ok` and the negative lemmas must be redone on the new key.
