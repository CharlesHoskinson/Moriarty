# astra-2-4: ECIP divisor certificate in place of the foreign MSM ladder

> SUPERSEDED IN PART by `astra-2-7.md`. The 765-element witness estimate and the "7 multiplicative constraints" count below do not apply to the Garaga certificate. The Garaga verifier is one division per nonzero base -3 digit (about 107 per full-width scalar, derived). Fiat-Shamir binding in Garaga is unimplemented in the captured code. Use astra-2-7.md.

## Question

Can an Eagen-style ECIP divisor certificate replace the AssignedMsm collapse ladder (foreign-EC MSM) in the Midnight stock PLONK/KZG verifier, with at least 75% foreign-EC row reduction, same VK constraints? Need the actual BLS MSM sizes and field-operation counts, the Fiat-Shamir challenge dependency, the auxiliary witness cost, the subgroup/exception checks, and whether stock gates can express a noninteractive challenge over all ECIP data. Honest feasibility with thresholds, no promised benchmark. Read-only; no repo writes.

Scope note: the bus asked for raw captures under `astra-2/raw/haiku05-*.md`. That path is outside my write scope (`helpers/haiku-05/` only), so captures are in `helpers/haiku-05/raw/ecip-*.md` and `ecip-divisor-techniques.{pdf,txt}` instead.

## Answer

- **The ECIP certificate is not a noninteractive stock-PLONK gadget.** Eagen's construction (Construction 5 / Fig. 4) is a commit-and-prove argument: the prover commits to the first-round message `(m, a(x), b(x))` with a Pedersen commitment `cm` on a second group `G_out`; the verifier challenge is `(A0, A1) = H(cm)`; a third-round helper vector `(h0, h1, h2, g)` is sent; and a Bulletproofs argument `BP.Prove` proves the verifier relation `R_V`. Source: zkSecurity divisor notes PDF, `ecip-divisor-techniques.txt` lines ~1530-1545 and Fig. 4 at ~1625-1655 (`raw/ecip-divisor-techniques.pdf`). Midnight's ledger does not verify Bulletproofs or Pedersen over `G_out`, so this is a new verifier, not a stock one.
- **Challenge dependency:** the challenge depends on the prover's commitment `cm` to all first-round data, including the ECIP points. Eagen's paper says the logarithmic-derivative step enables the same technique as Bulletproofs++ permutation, and that it "can be random or structured" (eprint 2022/596 abstract, `raw/ecip-eprint-2022-596.md`). zkSecurity says the protocol is interactive and must be made non-interactive via Fiat-Shamir ROM; they treat it as needing extraction proofs and a ROM argument (`raw/ecip-zksecurity-divisor-notes.md` "The Trick"; PDF ~1264-1300, 1499).
- **Stock Midnight can express a second-phase challenge, but not the stock reader for it.** `midnight-proofs-0.8.3` has phases: `Advice::new(phase)` (`plonk/circuit.rs:128-191`) and per-phase challenge squeezes in the verifier (`plonk/verifier.rs:97`). So an in-circuit Poseidon-of-commitment challenge is possible at the prover-protocol level. However, `MidnightVK` reconstructs the constraint system only from a stock `ZkStdLibArch` (`midnight-zk-stdlib-2.3.5/src/lib.rs:1547, 1581`, per node 06). A custom phase-2 construction needs new VK/reader integration, so it is a class-C change (new keys, reader, and ledger acceptance), not a same-key speedup.
- **The "7 multiplicative constraints" figure is an R1CS count, not Midnight PLONK rows.** Fig. 5 (PDF ~1693-1700) lists: curve equation 3 mults, `h_i D(A_i)=D'(A_i)` 3 mults, `g L(-P)=-1` 1 mult, `a1=1` 0, the sum 0. Total 7. That is the Bulletproofs verifier relation for one fixed-basis ECIP over the commitment group, excluding the Bulletproofs IP argument cost. zkSecurity states "an MSM of arbitrary length can be verified in 7 multiplicative constraints" (`raw/ecip-zksecurity-divisor-notes.md`), which is the same claim in Bulletproofs terms.
- **Committed witness size is the real cost.** The prover commits to `d + k + 2` field elements; for a 256-bit curve with `k = 256`, `d = k+1 = 257`, that is 515 committed elements (PDF ~1545-1547). For BLS12-381 G1 (base field `Fp`, 381 bits; `midnight-curves-0.3.1/src/bls12_381/fp.rs:553`), the same formula gives about `d+k+2 = 765` elements (estimate from the formula, not a source figure). Each `Fp` element is 7 limbs of 56 bits in Midnight's foreign encoding (`field/foreign/params.rs:277-283`), so the auxiliary witness is roughly 765 × 7 ≈ 5,355 native cells before range checks. Estimate only.
- **Field mismatch:** the ECIP coefficients live over the curve's base field. For BLS12-381 G1 that is `Fp`, which is foreign to the Midnight circuit (native field `Fq`). Every ECIP multiplication is therefore an emulated `Fp` operation. Jubjub is native over `Fq`, but it is not the MSM group. So the certificate does not avoid foreign arithmetic; it moves it from ~342-row EC additions to emulated `Fp` multiplications inside the verifier relation, whose row cost is not in the stock fixtures.
- **Stock baseline (the ladder being replaced):** from `midnight-circuits-7.2.4/goldenfiles/cost-model.json` (`raw` in node 06 and my own read): `foreign_ecc_bls_over_bls/add` 342 rows, `double` 89, `msm_3` 39,935 rows (three full-width 255-bit scalars, `instructions/ecc.rs:555-600`), `msm_by_bounded_scalars_3` 30,630 rows (widths 4, 12, 255 bits; `ecc.rs:605-630`), `mul_by_constant` 31,418. The msm fixtures include point assignment and checks, so a per-term average (about 13.3k rows) is an upper bound, not a marginal cost.
- **Subgroup and exception checks** remain required even with ECIP: `assign_with_subgroup_check` on BLS G1 is 15,410 rows (stock fixture), and the incomplete-addition/exceptional-point checks in the existing ladder are what Fenrir's hardening protects (`verifier-patch/PATCH-REPORT.md:25, 83-88`). The ECIP relation would need equivalent checks for the committed points and for the `h_i` helpers, which is not covered by the 7-constraint count.
- **Garaga is not a usable benchmark for this.** The captured `garaga` README (`raw/ecip-garaga-readme.md` lines 280-286, 364-501) gives Starknet Cairo step counts for Groth16 BLS12-381, BLS pairings, and SECP256 MSMs. Its BLS12-381 MSM table is absent from the captured page. The units are Cairo steps and builtins, not PLONK rows, so they cannot be converted to Midnight rows. (The "work in progress, not production" wording appeared only in the search snippet, not in the captured page, so I do not rely on it.)
- **Feasibility verdict:** the ECIP certificate may cut the number of *MSM terms* (one divisor replaces a multi-scalar ladder), but there is no published BLS12-381 MSM row count or emulated-`Fp` multiplication count for this construction in any source I captured. The 75% threshold is therefore unevaluated. Concretely: to reach ≥75% reduction of the foreign-EC share (for the parent's `msm_3`-class collapse, see astra-2-1), the certificate's total rows (7 R1CS mults times emulated-`Fp` cost, plus `d+k+2` committed elements, plus IP argument rows, plus the challenge hash and new reader) must come in under 25% of today's collapse rows. No source supports that number; the 75% route is not shown feasible on current evidence.
- **Same-VK requirement fails:** a divisor certificate with a Bulletproofs (or phase-2 PLONK) challenge changes the verifier relation, the transcript, and the terminal VK. This violates the same-key constraint of LOKI-DESIGN §"The hard constraint" and falls under Track B/C (new keys). Consistent with node 06 and astra-2's own framing.

## Raw numbers

| Quantity | Value | Source |
|---|---:|---|
| ECIP fixed-basis R1CS mults | 7 (3 + 3 + 1 + 0 + 0) | `ecip-divisor-techniques.txt` Fig. 5 (~1693-1700) |
| Committed elements, 256-bit curve, k=256, d=257 | 515 (`d+k+2`) | PDF ~1545-1547 |
| Committed elements, 381-bit (BLS G1 base), formula | 765 (`d=382`, `k=381`) | estimate from formula |
| Foreign limbs per Fp element | 7 × 56-bit | `field/foreign/params.rs:277-283` |
| Stock BLS foreign ECC add / double | 342 / 89 rows | `cost-model.json` (`foreign_ecc_bls_over_bls`) |
| Stock msm_3 (3 full-width) | 39,935 rows | `cost-model.json`; `ecc.rs:555-600` |
| Hardened msm_3 | 55,630 rows | `PATCH-REPORT.md:83-88` |
| Stock msm_by_bounded_scalars_3 (4,12,255 bits) | 30,630 rows | `cost-model.json`; `ecc.rs:605-630` |
| Stock assign_with_subgroup_check (BLS G1) | 15,410 rows | `cost-model.json` |
| ECC advice columns | 15 | `weierstrass_chip.rs:105` |
| Phases available | 3 (`FirstPhase/SecondPhase/ThirdPhase`) | `midnight-proofs-0.8.3/src/plonk/circuit.rs:128-148` |
| Garaga Groth16 BLS12-381 test | 337,407 steps (Cairo, not PLONK rows) | `raw/ecip-garaga-readme.md` line ~366 |

Derived threshold: a 75% reduction needs the certificate path at ≤25% of today's foreign-EC collapse rows. No row count for the certificate is available, so this is unevaluated.

## Caveats

- No measured Midnight row cost exists for an ECIP certificate, emulated `Fp` multiplication, or the Bulletproofs IP argument in Midnight. All ECIP sizes above come from the paper's formulas or from the stock fixtures. Nothing was built or benchmarked.
- The `765` committed-element figure and the `~5,355` native-cell figure are my extrapolations from the paper's formula (`d+k+2`, `k=381`), not from a source.
- The stock msm fixture figures include point assignment and checks, so the per-term cost is an upper bound.
- zkSecurity's "7 multiplicative constraints" is an R1CS count for the Bulletproofs verifier relation; it is not comparable to Midnight PLONK row counts without the emulated-field cost.
- Eagen's 10× witness / 100× multiplication claims (eprint abstract) are relative to unspecified "existing techniques" and depend on the proof system; they do not transfer to Midnight.
- The eprint PDF itself was not captured (only the abstract page); the protocol details come from the zkSecurity notes PDF, which is a secondary source by zkSecurity and states it reviews Eagen's protocol for MAGIC Grants (May 2026).
- Garaga: captured README is partial (a cached snapshot per the search summary); the BLS MSM benchmark table was not found in it.

## Sources

- Eagen, eprint 2022/596 (captured abstract): https://eprint.iacr.org/2022/596 (`helpers/haiku-05/raw/ecip-eprint-2022-596.md`)
- zkSecurity, "Notes and Proofs for Divisor Techniques" (May 2026): https://blog.zksecurity.xyz/posts/divisor-notes/ (`raw/ecip-zksecurity-divisor-notes.md`); PDF https://blog.zksecurity.xyz/posts/divisor-notes/divisor-techniques.pdf (`raw/ecip-divisor-techniques.pdf`, text `raw/ecip-divisor-techniques.txt`)
- Garaga README: https://github.com/keep-starknet-strange/garaga/ (`raw/ecip-garaga-readme.md`)
- Local code: `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/{circuit.rs,verifier.rs}`, `midnight-circuits-7.2.4/{goldenfiles/cost-model.json,src/ecc/foreign/weierstrass_chip.rs,src/instructions/ecc.rs}`, `midnight-zk-stdlib-2.3.5/src/lib.rs`, `projectfenrir/recursion/stagenet-strawman/verifier-patch/PATCH-REPORT.md`
