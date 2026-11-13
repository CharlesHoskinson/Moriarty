# X8: foreign adapter probe and the foreign lift relation

Date: 2026-11-12. Loop item X-EXP of `research/sherlock-e-path-20261112/LOOP.md`. Branch `x-exp` (from `final-design-2` at `efb0eb78a`). Experiment definition: `research/sherlock-final-design-20261111/ARCHITECTURE.md` section 5.4, row X8: "foreign adapter probe: one Compact adapter class and the foreign lift relation". It decides the scope of the GP-3 native path (N-10) and gates the GP-3 hook and I-13.

Labels: MEASURED, DERIVED, ESTIMATE and READ as in X14. All numbers here are DERIVED from X10 and X14 measurements or READ. No new run.

## Result

**X8 cannot be measured as written.** The repository has no Compact adapter class and no foreign lift relation (READ: no adapter circuit in `crates/`; the kit GP-3 hook lists the `midnight-circuits` verifier gadget as the pinned API, ARCHITECTURE line 2598, and no STARK code uses it). Under AD-22 (a) (owner decision 2026-11-12), the foreign lift fully verifies the adapter proof and the inner foreign proof in circuit, with no accumulator folded. So the lift's cost is set by the parts below, and the dominant part is blocked by the same choice as X14.

The foreign lift relation (ARCHITECTURE line 2562) and its parts on the STARK node:

| Part | Cost on the STARK node | Label |
|---|---|---|
| verification of the adapter-class proof (a halo2 KZG proof over BLS12-381) | the X14 unit sequence: parse, transcript, expression evaluation, MSM about 25,000 `g1_add` rows, final check about 20,000 `fp_mul` rows | DERIVED from X14 |
| verification of the inner foreign proof | the same sequence again, under its own registry entry; with another SRS only the pairing constants change (AD-22 (a)) | DERIVED from X14 |
| membership of the adapter key and the inner key in `reg_root` | 2 paths. A 255-level binary path is 255 two-to-one Poseidon2 compressions (one width-16 permutation each) per path, so 510 permutation rows. That is one Poseidon2 AIR of 2^9 rows and about 300 columns, with 0 interactions. The measured `HeaderOpenAir` is 307 columns and 362 constraints for 8 rows | DERIVED from X10 |
| foreign claim `H(SHK_CLAIM, KIND_FOREIGN, vk_id_inner, adapter_id, out_d)` and the one-claim SET | 1 to 2 permutations (about 4 + 8 words) | DERIVED |
| adapter application on `pi_d` | one sponge over the verified public inputs: 1 permutation per 8 words after the BLS12-381 scalars are encoded as BabyBear limbs. The encoding is not fixed (I-13 / ENC) | DERIVED, encoding PENDING |

The old halo2 figures (a 255-level path about 5,610 hash rows; the lift 550,000 to 750,000 rows, ARCHITECTURE line 2574) are prototype history (DG-007). On the STARK node, the membership and claim parts are small (one AIR of 2^9 rows, DERIVED). All of the lift's cost is in the two in-circuit verifications.

## Gate status

- **N-10, the GP-3 native path:** the native path is "the foreign lift with the adapter-class proof verified in circuit". By AD-22 (a) it uses the same operation AIRs as the general path. So the native and general paths share one blocker, the X14 encoding choice (operation AIRs within 100 interactions per AIR). The native path is not a cheaper way around it. N-10 stays open: "native first as a milestone" still holds as a schedule, but on cost the native path is about the general path plus 2 membership paths.
- **GP-3 hook, I-13:** FREEZE-PENDING on X14 option 1, 2 or 3, and on a Compact adapter class. The adapter class needs its circuit (the `midnight-circuits` verifier gadget configuration) chosen before any probe can run.

## What is needed next

1. The owner's (or lead's) choice of X14 option 1, 2 or 3 (operation AIR encoding under the internal-set bounds).
2. One Compact adapter class named with its circuit, so a real adapter proof can be costed with `x14_inventory.rs` (its `CircuitModel` gives the counts of the X14 inventory table).
3. Then the probe: the unit sequence of X14 over that proof, plus the two membership paths and the claim hash, in nodes at the internal set.
