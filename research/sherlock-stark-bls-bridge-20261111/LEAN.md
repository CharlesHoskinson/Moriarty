**BUILD PASS** (`lake build` of `SherlockSpec`, 30 jobs, Lean v4.33.1). Sorry count 0; no `admit`, no `native_decide`, no `axiom` declarations. `#print axioms bridge_sound`: `[propext, Classical.choice, Quot.sound]`.

# Bridge round, step 3: Lean formalization of the candidate

Date: 2026-11-11. Branch `bridge-lean` (worktree `~/Projects/Moriarty-wt/bridge-lean`). Source plan: `CANDIDATE.md` section 11 and two owner decisions of 2026-11-11 at its end: internal digests on `sherlock-value/2` (Poseidon2 over BabyBear), and proven 100-bit security for `L` (folding and `mu` PoW 21 bits, BCHKS25 Theorem 1.5 as stated at `m = 3`).

Files (project `clean/fork`, Lean `v4.33.1`, no mathlib, library `SherlockSpec`):

- `clean/fork/Clean/Sherlock/FrLog.lean`: limb encoding, packing, bounded-quotient multiplication, the expression compiler, squeeze digits, log entries, encoding, onion digest, the deferred-or-emulated split.
- `clean/fork/Clean/Sherlock/Bridge.lean`: the verifier of `L`, the wrap relation, `Premises`, `bridge_sound`, the zero-knowledge corollary, supporting lemmas.
- `clean/fork/Clean/Sherlock/BridgeAxioms.lean`: the premise documentation, kernel sanity checks, a consistency model of `Premises`, and `#print axioms` for every theorem (84).

Build commands: `lake build` (all of `SherlockSpec`) and `lake env lean Clean/Sherlock/BridgeAxioms.lean`, from `clean/fork`, under `flock ~/.cache/cost-opt/measure/heavy.lock heavy-run --mem 16G`. Logs: `~/.cache/bridge-lean/final.log`.

## 1. The theorem

```lean
theorem bridge_sound (P : Premises) {pub : List Nat} {prf : P.LedgerProof}
    (h : P.LedgerAccepts pub prf) : P.Run pub
```

`pub` is the wrap's whole public input, and it is the statement (the claim). `P.Run pub` says the reference machine makes that claim true. Proof route (in `Bridge.lean`):

1. A1 gives a wrap witness `w` with `WrapRel X B vkL pub w`.
2. `wrap_refines` (a theorem) gives: the reference verifier of `L` accepts `(pub, w.pv, w.π)`; the wrap table `w.T` has exactly `B` entries; every entry is well-formed (canonical values) and `Valid`; `w.pv = encode w.T`.
3. A2, with its three hypotheses discharged (`packed_hash_inj` from A2b; `pvFirst` and `ProvenRegime` from the static key check), gives a root acceptance with some digest `d`.
4. A3 gives a carrier log tree `t` with `encode (pad B (flatten t)) = w.pv` and digest `d`.
5. `encode_inj` gives `pad B (flatten t) = w.T`, so every entry of `flatten t` is valid.
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
| A2 | `lastLayerSound` | If the reference verifier of `L` accepts, the root AIR of `L` accepted with some log digest. The proximity-gap part is the proven Johnson-radius statement (citations per REVIEW.md, not read by this lane): BCHKS25 Theorem 1.5 as stated (`m = max(ceil(sqrt(rho)/(2 eta)), 3) = 3` at S4), with Theorems 4.5 (batching) and 4.6 (mutual correlated agreement); with 21-bit folding and `mu` PoW this gives 100.2 and 102.3 bits (paper review, `research/sherlock-design-paper-review-20261111/REVIEW.md` section 4). The unproved p.23 remark (`m < 3`) is not a premise. The rest: WHIR round-by-round soundness, Fiat-Shamir in the random-oracle model (non-interactive error `Q_RO 2^-100 + 3 (Q_RO^2 + 1) / 2^kappa`), the `eq` binding error `c m / |EF|`, the squeeze bias. It holds only under three hypotheses that the proof discharges: the packed hash is injective, the transcript absorbs `pv` before the first challenge, and the key's WHIR parameters are in `ProvenRegime`. |
| A2b | `frSpongeInj` | Midnight Poseidon over Fr has no collision on its carrier domain `FrDom` (the transcript and Merkle hash of `L`, and the public digest form). |
| A3 | `rootBinds` | The root AIR binds its public values to a carrier log tree (at most `B` entries, all well-formed), padded with `NOP` to `B`, with the digest that the recursion carries. |
| A4 | `recSound` | The recursion (leaves, discharge shards, nodes) proves the run against the flattened log of a carrier tree with that digest; each Fr operation is either in the log or proved inline by tier E. |
| A5 | `p2Inj` | Poseidon2 over BabyBear has no collision on its carrier domain `P2Dom` (the log onion and the internal `sherlock-value/2` digests). Separate from A2b, as the owner decision requires. |
| A6 | `oracleElim` | A run whose Fr operations are all valid is a run of the reference machine (a premise until the Sherlock step relation is imported). |
| A7 | `wrapZK` | The blinded wrap is zero knowledge: for a true statement the proof distribution equals the simulator's on `pub` alone. Not used by `bridge_sound`; used by `wrap_view_only_claim`. `indist_symm`, `indist_trans` make indistinguishability an equivalence. |
| (static) | `vkL_ok` | Not cryptographic: the frozen key of `L` absorbs `pv` first, its WHIR parameters are in `ProvenRegime`, and every compiled expression has a tracked bound below `r`. Decidable on the concrete key. |

The carriers `P2Dom` and `FrDom` are abstract, as `Carrier` is in `Aggregation.lean`. An unguarded injectivity premise for a hash with a finite digest would be false in Lean (pigeonhole) and would make every theorem vacuous. `BridgeAxioms.consistent` is a model of all premises (a trivial one: nothing is accepted), which shows they are jointly satisfiable. It does not show that the real primitives meet them.

## 3. Where each soundness point sits

"On path" lemmas are used by the proof of `bridge_sound`. "Supporting" lemmas are proved but not used by it.

| Point | Definition clause | On path | Supporting | Negative lemma (the check is needed) |
|---|---|---|---|---|
| Bounded-quotient encoding of BabyBear multiplication in Fr | `ArithGadget`: `c < p`, `q < 2^223`, both sides `= c + q p` in Fr; `ArithOK`: tracked bound `< r` | `bounded_quotient_sound`, `arith_refines`, `check_refines`, `wrap_refines` | `fmul_sound`, `ef_mul_sound`, `horner_refines`, `sumcheck_round_refines`, `reduce_sound` | `unbounded_quotient_unsound`: with `q = p^(-1) mod r`, `1 * 1 = 0 + q p` in Fr |
| Canonical injective packing of 8 x 31-bit limbs with checked length | `pack`, `chunks8`, `packChunks`; `PackedHashInj` requires equal lengths and words `< 2^31` | `pack_injective`, `packChunks_inj`, `packed_hash_inj` (discharges A2's first hypothesis) | `public_digest_binds`, `pack_gadget_refines` | `pack_length_check_needed` (`[1]` and `[1, 0]`), `pack_range_check_needed` (`[2^31]` and `[0, 1]`) |
| Canonical logged values | `CanonGadget`: 9 limbs `< 2^30`, lexicographically below the limbs of `r`; `EntryGadget` | `canon_check_sound`, `replay_sound`, `encode_inj` | `dec_inj`, `canon_mod_inj`, `limb_embed_inj` | `alias_without_canon`: limbs of `5` and of `5 + r` recompose to the same Fr element |
| Squeezed challenge digits | `SqueezeGadget`: 5 digits `< p`, recomposition `< r` | `squeeze_digits_sound` | | `squeeze_alias`: the digits of `r` recompose `0` in Fr |
| Log entries absorbed before the first challenge | `pvFirst` on the key's transcript schedule `sched`; `KeyOK` | A2's second hypothesis, discharged by `vkL_ok.1` | `lPreamble_pvFirst` (the stark-backend preamble order) | `late_pv_rejected` |
| Proven proximity regime (owner decision) | `WhirParams`, `thm15Mult` (`m` as Theorem 1.5 sets it), `ProvenRegime` (`m = thm15Mult`, folding and `mu` PoW at least 21, query PoW at least 20); `s4Whir` = log blowup 4, `eta = 1/8`, `m = 3`, PoW 21/21/20 | A2's third hypothesis, discharged by `vkL_ok.2.1` | `s4_mult_three`, `s4_proven` | `s4_pow15_not_proven` (the former 15-bit PoW), `remark_m1_not_proven` (the `m = 1` reading of the p.23 remark) |
| Fixed log capacity, overflow to STARK shards | `WrapRel`: `w.T.length = B`; `LogTree.Carrier`: at most `B` entries; `pad`; `RunWith`: each operation deferred or emulated | `pad_length`, `pad_wf`, `encode_inj`, `digest_inj`, `runWith_all_valid` | `spill_equiv`, `discharge_sound` (any split of a valid run, and a discharge shard, give the same claim) | n/a |
| Public statement is only the claim | `WrapRel X B vk pub w`: `pub` is the only public input; `VerifyL` reads the statement from `pub` | A1 to A6 are all stated on `pub` | A7 and `wrap_view_only_claim` | n/a |

Limits of this modeling that the lead should know:

- The wrap's arithmetic gadget in `WrapRel` is `ArithGadget` (unsigned values, `c + q p` with `0 <= q < 2^223`). The plan's signed `ReduceRel` is formalized and proved (`reduce_sound`) but is not the gadget that `WrapRel` uses; a wrap that uses signed lazy reduction needs `ArithGadget` replaced by `ReduceRel`.
- `sched` and `whir` are declared metadata of the key. Nothing in Lean derives them from `checks`. So "`pv` before the first challenge" and the proven regime enter `bridge_sound` as static flags that gate A2, and the link between the flags and the real verifier of `L` (that its transcript really absorbs `pv` in the preamble, that its PoW checks really have 21 bits) stays inside A2. The proximity-gap theorem itself is not a Lean proposition (core Lean has no Reed-Solomon codes); the formal content is the parameter gate.
- WHIR folding, WHIR rounds and the `eq` evaluation of `pv` are not separate lemmas: they are `arith` checks of the key, covered generically by `arith_refines`.

## 4. Statements changed from the plan (CANDIDATE.md section 11)

1. `Params` removed. `p` and `r` are fixed literals (`pBB`, `rFr`), with `w = 30`, `l = 9`, base `2^31`, 8 words per Fr element, and every bound checked by `decide`. Reason: the plan's `hr : 2^248 < r` is too weak for the lazy reduction. `ReduceRel` with `b <= 252` needs `|x - (q p + t)| < 2^252 + 2^253 + 2^31 < r`, which needs `r > 2^254`. True for BLS12-381 Fr (`r_big`, `reduce_bits_ok`). No design change.
2. `castP`, `castR` and `|.|` replaced by `%` on `Nat`/`Int` and two-sided bounds; `ReduceRel` takes explicit bounds `X`, `Q` with the static condition `X + Q p + 2^31 <= r`. `lift_eq` is stated in difference form (`-r < x - y < r`). Reason: no mathlib absolute value; the explicit form is what the wrap's static bound check verifies.
3. `verifyL` returns `Prop`, not `Bool`, and is defined (`VerifyL` over `Check`), not a `Premises` field. `WrapRel`, `RunWith`, `Carrier` are also definitions. Reason: as opaque fields, `wrap_refines`, `encode_inj` and `digest_inj` cannot be proved and none of the soundness points would be in the theorem.
4. `RunWith` is defined from a new field `RunOps` (the leaves' relation over the run's whole Fr operation sequence) and a split into deferred and emulated operations. `oracleElim` (A6) is stated on `RunOps`. Reason: makes lemmas 20 and 21 provable and puts the overflow path in the proof.
5. A2 (`lastLayerSound`) has three hypotheses: `PackedHashInj X FrDom`, `pvFirst vkL.sched = true` and `ProvenRegime vkL.whir = true`. Reason: these are the conditions under which the soundness of `L` holds; stating them puts packing, transcript order and the proven proximity regime on the formal path. All three are discharged inside `bridge_sound`. `KeyL` gained the field `whir : WhirParams` for the third (owner decision, 2026-11-11). The proximity-gap theorem itself stays inside A2: core Lean has no Reed-Solomon codes, so the formal content is that the key's parameters are the ones Theorem 1.5 covers as stated, not the theorem.
6. New premises A2b (`frSpongeInj`, Fr hash) and A7 (`wrapZK`), and the static field `vkL_ok`. Reason: owner decision (small-field digest premise separate from the Fr transcript hash premise) and the task (ZK boundary as a premise of the blinded wrap). The premise count is therefore 8 named assumptions plus one static key fact, not 6.
7. `p2Inj` is guarded by an abstract `P2Dom` (not "well-formed tagged inputs"), and `LogTree.Carrier` includes `DigestIn`: every `p2` input of the tree's digest is in `P2Dom`. Reason: a guard that admits every BabyBear vector makes the premise false (finite digest), see section 2.
8. `Prims.p2` carries `p2_width : (p2 a).length = 8`. Reason: without a fixed width the input `tag :: (d_a ++ d_b)` cannot be split, so `digest_inj` fails.
9. `encode_inj` is used on padded tables: the route gives `pad B (flatten t) = T_w`, not `flatten t = T_w`. Reason: `L` pads with `NOP`. Entries have a fixed arity per operation (`Op.arity`) and a two-word code; without the fixed arity the concatenation is not injective.
10. `Valid` takes the statement (for `PUB(i)`). `PATH(d)` is membership with private bits and siblings. `DIGEST` checks `out = frSponge [tag, pack d2]` for 8 BabyBear words `d2` (the `sherlock-value/2` public form).
11. `rootBinds` returns `encode (pad B (flatten t)) = pv` (the plan had `encode (flatten t) = pv`). Same reason as item 9.
12. Lemma 7 (`add_bound`, `mul_bound`) is `Expr.eval_le_bound`. Lemma 8 uses an explicit 12-cell environment and a tuple `efMul`. Lemmas 11 and 13 (`coset_fold_refines`, `whir_round_refines`, `pv_eq_refines`) are not separate theorems: every WHIR, folding, sumcheck and `eq` check is an `arith` check of the key and is covered by `arith_refines`; `horner_refines` and `sumcheck_round_refines` are given as instances. Lemma 10's `sponge_refines` and `merkle_path_refines` are the `hash` and `merkle` cases of `check_refines` (native Fr in the wrap, so the refinement is the packing gadget `packChunksR_eq`).
13. Lemma 12 (`constraint_dag_refines`) is `arith_refines` over expression trees (`Expr`), not a DAG generated from OpenVM's Lean emitter. A DAG is a tree with sharing; the semantics are the same.
14. Lemma 23 (`pv_binding_count`, the Schwartz-Zippel count in table form) was NOT ATTEMPTED. It is off the trust path (its error term sits inside A2). It is not false: it could be proved with the one-variable root count taken as a hypothesis; it was left out for time.
15. The onion has two extra domain tags, `tagStep` (chain step) and `tagEnt` (entry hash), beside `TAG_EMPTY` and `TAG_NODE`: `d_(i+1) = P2(tagStep, d_i, P2(tagEnt, enc(e_i)))`. One variable-length `p2` stands for both the two-to-one hash and the entry hash `H2s` of the plan; the tags separate the uses.
16. The capacity `B` counts entries, not BabyBear words. Because `SPONGE(n)` has `n + 1` values, `pv` has no fixed length for a fixed `B`. A wrap with a fixed public-value table needs either a word capacity or a bound on `n` per slot (a design detail for M9).
17. The paper citations in A2 (BCHKS25 Theorems 1.5, 4.5, 4.6; SWIRL Cor 4.2.6) are taken from `research/sherlock-design-paper-review-20261111/REVIEW.md` section 4. This lane did not read the papers.

## 5. Design findings

1. Empty onion must be a hash output. With `d_0 = TAG_EMPTY` as a raw constant (section 3.1 of the candidate), tree-digest injectivity needs a preimage assumption (`p2` never outputs `TAG_EMPTY`) that collision resistance does not give: an empty segment could equal a `TAG_NODE` merge. The formalization uses `d_0 = P2([TAG_EMPTY])`. The candidate should change `d_0` to `P2(TAG_EMPTY)` (one extra permutation per empty segment) or add the preimage premise.
2. The quotient bound is a hard requirement, not a cost choice. Without `q < 2^223` (any bound with `c + q p < r`), a prover can claim a false product: for any claimed residue `c`, `q = (a b - c) p^(-1) mod r` satisfies the Fr equation (`unbounded_quotient_unsound` is the instance `1 * 1 = 0`). Every bounded-quotient gadget of the wrap must range-check `q`. The static bound `< r` of every lazily evaluated expression must be checked when the key is frozen (`vkL_ok`).
3. Packing needs both checks the candidate names: equal (key-fixed) absorb lengths and words below `2^31`; each alone is not enough (`pack_length_check_needed`, `pack_range_check_needed`). In the transcript of `L` the absorb lengths are fixed by the key, so the length check is static.
4. The squeeze needs a canonical check on the recomposition (`squeeze_alias`), the same `v` against `v + r` issue as for logged values. The candidate's sampling section should state it as a wrap constraint.
5. Pigeonhole: hash premises in Lean must be carrier-guarded with an abstract domain. Stating the small-field digest premise as "injective on all BabyBear vectors" would be false and make the theorem vacuous. The formalization follows `Aggregation.lean`.
6. The proven regime needs the 21-bit folding and `mu` PoW in the key of `L` (owner decision). It is a static key fact (`ProvenRegime`), so a key frozen with 15-bit PoW or with `m = 1` fails `vkL_ok` and `bridge_sound` does not apply to it. The bit figures (100.2, 102.3) are taken from the paper review and are not computed in Lean; M4 (the soundness calculator) should confirm them for S4.
7. No design change found for: the fixed capacity with spill (`spill_equiv`, `discharge_sound` hold for any split), `pv` before the first challenge (a static key property), the ZK boundary (only `pub` is public), and the case-A digest form `PoseidonFr(tag, pack d2)` (`public_digest_binds`).

## 6. Axioms

Output of `lake env lean Clean/Sherlock/BridgeAxioms.lean` (log `~/.cache/bridge-lean/final.log`):

```
'Clean.Sherlock.Bridge.bridge_sound' depends on axioms: [propext, Classical.choice, Quot.sound]
'Clean.Sherlock.Bridge.wrap_view_only_claim' depends on axioms: [propext, Quot.sound]
```

All 84 theorems of `FrLog.lean` and `Bridge.lean` are queried: 25 depend on no axioms, 12 on `[propext]`, 28 on `[propext, Quot.sound]`, 19 on `[propext, Classical.choice, Quot.sound]`. None uses another axiom (no `Lean.ofReduceBool`, so no `native_decide`). The cryptographic assumptions are not axioms: they are the `Premises` fields of section 2, hypotheses of every theorem that uses them.
