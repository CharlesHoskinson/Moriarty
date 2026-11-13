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

## Update under the owner rulings of 2026-11-12 20:35 (from the new X14 numbers)

X14 option 2 is ruled: 200 interactions per AIR, 64 AIRs, 3,000 columns, stock `fr_mul` and `fp_mul`, and G1 as `fp_mul`-class rows. The X14 blocker of this file is removed for the arithmetic. New cost of the foreign lift on the STARK node (DERIVED from `../sherlock-x14-20261112/RESULTS.md`, follow-up section):

| Part | Units at the internal set | Label |
|---|---|---|
| adapter-class proof: MSM (about 23,000 additions in a bucket MSM, 255 doublings) | 1 addition unit (`L_add`, `X3`, `Y3` at 2^15 to 2^16) + 1 doubling unit | DERIVED (unit shapes MEASURED) |
| adapter-class proof: final check, AD-22 (a) | 1 `fp_mul` unit (about 20,000 rows) | ESTIMATE (unit shape MEASURED) |
| inner foreign proof | the same 3 units again, under its own registry entry | DERIVED |
| transcript and expression evaluation, both proofs | PENDING (no AIR) | |
| 2 membership paths in `reg_root` and the foreign claim | one Poseidon2 AIR of 2^9 rows (about 300 columns, 0 interactions) and 1 to 2 permutations, inside any unit's spare columns | DERIVED |

So the lift costs about 6 arithmetic units plus the transcript and expression-evaluation units. Each unit is a node proof at the internal set. The parent's cost of verifying a unit is about 1 % over a plain node (X14 (b), MEASURED). The exceptional-case rows are now built and measured (X14 (d)): `INV` or `INV_y` per chain, `id` as row index, and a `PointsIn` producer. With them the addition unit is 45 AIRs / 3,138 columns, over the 3,000-column bound, and the doubling unit without `INV_y` is 45 / 2,946 (MEASURED; its point producer checks inputs on the curve). The unit count above holds with a 3,300-column bound for the FV work units (AFK pick for owner confirmation, D3 unchanged at 101.1643). Otherwise the producer and on-curve conditions move to another unit, and the count may grow by one producer unit per proof (DERIVED).

**N-10:** the native path (adapter class) and the general path use the same units. The native path is a schedule milestone, not a cheaper circuit. **Still needed:** a named Compact adapter class with its circuit (to cost its proof with `x14_inventory.rs`), the transcript AIR for the midnight transcript hash, and the owner's choice on the column overflow of the exceptional-case rows (X14 (d)). The GP-3 hook and I-13 stay FREEZE-PENDING on these.
