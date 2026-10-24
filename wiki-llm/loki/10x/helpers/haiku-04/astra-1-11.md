# astra-1-11: Adversarial review of astra-1 PROPOSAL.md (2026-10-24 draft)

Scope: read-only audit of `/home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/astra-1/PROPOSAL.md`. No repo edits, no builds, no heavy runs, no `sherlock-next` run. Checked: same-key claims, ledger-9 reader and final pairing, new-key pricing obligations, F201-F206 dependencies/tier/check, source provenance.

## Question

Which claims in the draft go beyond the evidence, and what exact wording fixes them?

## Answer

### A. Verified (no change needed)

- **Receipt.** `docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json`: `phases.prove.duration_seconds = 882.589555363`, `phases.setup_pk.duration_seconds = 62.328385318`. Nested `951.614` and `954.127` present under `nested_wall_observations`. Binary lineage is NOT the Moriarty 0.8.3 workspace (astra-1 correction; Source44 binary/private recl9 backend). Scale reference only; the draft's wording must say so (it does, in Line 27 'uncontrolled reference').
- **Historical chain arithmetic.** Leaves 163.630576714 + node 867.279750364 + decider 586.381689547 = 1617.292016625 s (matches `out/node-b1/w.stats.json` `prove_s` 867.28 and `out/decider-b1/batch-1.stats.json` `prove_s` 586.38). The draft's 1.933x (10x node) and 2.156x (infinite node) and 2.226 batches/h are correct.
- **Cache-only factor.** (882.590 + 62.328) / 882.590 = 1.07062 (correct).
- **Speedup formula.** `S = 1/[f+(1-f)/s+h]`. f <= 0.0421 at s=20, h=0.01 (correct, since 0.04/0.95 = 0.04211). Ceilings 9.09x, 4.76x, 6.45x, 4.00x all recompute.
- **Scenario arithmetic.** R = 0.20/8 + 0.78/20 + 0.02 + 0.01 = 0.094; 10.638x; 82.963 s = 22.065 + 34.421 + 17.652 + 8.826 (correct). Bulk-only R = 0.269 gives 3.717x; with witness 8x, 2.862x; product 11.39x (correct). Witness 2x gives 5.917x; 5x gives 9.174x; threshold witness >= 6.452x (correct). Both 8x gives 6.557x (correct).
- **Memory table.** 32 MiB per k20 scalar polynomial, 128/256 MiB extended, 72 extended buffers = 9.00/18.00 GiB, 45 fixed+sigma columns x 2 forms = 2.8125 GiB, 41 buffers = 1.28125 GiB; totals 19.09 and 28.09 GiB recompute. The 45-column figure in recl9 `REPORT.md:` node `arch` row (15 advice, 45 fixed, 5 lookups, 18 permutations) is consistent with the draft's 27 fixed + 18 sigma, since 45 = 27 + 18 (inference, not stated in the report).
- **cuSnark.** Proof 2 (chunk_inner, 2^20 rows, 1135 columns) in `astra-1/raw/snarkify-cusnark.md`: total 1762.75 -> 463.13 (3.81x). Eval_h 1115.43 -> 36.19 (30.82x). "Generate Advice" GPU 306.44 + "Generate Lookups" GPU 56.84 = **363.28** (matches the draft). 4.85x ceiling = 1762.75/363.28 (correct). Hardware is 4x RTX 3090, as the draft says.
- **Snarkify zkBLS.** 2.16 s for four proofs = 540 ms per proof; baseline ~490 s per proof, so ~907x (draft "~900x" is fine). Microbench 2^22 NTT 1.13 ms, inverse 1.23 ms, MSM 28.40 ms (`snarkify-zprize.md` table, milliseconds, 2^22 column).
- **Nocy.** Capture table: RTX 5090 GPU 4.4 s vs EPYC CPU 12.3 s at k17; 8.8 vs 24.6 at k18; 2.8x. The draft's "12.3->4.4" is CPU->GPU; correct. Nocy MSM uses 4-bit windowed scalar multiplication per point (`nocy-msm.md`, `window_bits = 4`), consistent with the draft's "not a bucket MSM".
- **ZKProphet.** "NTT up to 90% of proof latency" present; "studies Groth16" present.
- **Code lines.** Verified by printing source: `P/plonk/prover.rs:57,242,460,501,533,540,108,282,581`; `lookup/prover.rs:396` (sort); `permutation/prover.rs:106,143`; `evaluation.rs:285,363,443`; `domain.rs:37,81,224`; `vanishing/prover.rs:98,108,112,138`; `kzg/mod.rs:94,105,129,166,176`; `utils/arithmetic.rs:95` (kate_division); `kzg/msm.rs:163` (msm_specific zero filter); `g1.rs:628`; `single_pass.rs:89,109`; `Z/lib.rs:1581,1593,1880`; `native_chip.rs:367,774,1253`; `sherlock_link.rs:1029` (cost_model), `:1041`; `decider.rs:163,186,201,238`; `leaf.rs:7` (trust note); `REPORT.md:191` (child_proof_swap: ACCEPT at node, decider rejects); `chain/rc5check/ledger-rc5/transient-crypto/src/proofs.rs` exists.
- **Kate division recurrence.** `utils/arithmetic.rs:95-110` is synthetic division by (x - b): with the code's `b = -b`, q_{i-1} = a_i + b*q_i. The draft's "q_i = a_(i+1) + b*q_(i+1)" is correct, and the affine-map scan is valid.
- **Helper files exist.** `helpers/haiku-01/astra-1-8.md`, `haiku-02/astra-1-2.md`, `haiku-03/astra-1-3.md`, `haiku-03/astra-1-5.md`, `haiku-04/astra-1-6.md`.
- **MSM dispatch.** 0.8.2 `msm.rs:176-177` (BLST `multi_exp` only for `len <= 2<<18`); 0.8.3 `msm.rs:176-178` (always BLST). The draft's attribution to recl9/0.8.2 and the warning against mixing is correct.
- **Lookup HashMap.** `plonk/lookup/prover.rs:399-430`. Draft's line 113 wording is correctly scoped (full-proof bytes under fixed RNG only).
- **Decider embeds parent VK.** `decider.rs:238` writes `vk_bytes(&self.w_vk)`.

### B. Overstated or unverified (fix before publication)

1. **[RETRACTED by astra-1 review; my earlier flag was wrong.]** The capture contains both statements: line 19 "up to 25x performance boost" (for ICICLE's ezkl-halo2 fork; baseline is not named as CPU), and line 31 "In this work we didn't focus on optimizing the witness generation." Draft line 48 is supported. Add the baseline caveat: the 25x is against the ezkl-halo2 fork, not a CPU baseline. `astra-1/raw/icicle-halo2-v2.md` contains the RTX 4080 / i9-13900K benchmark, but the speedup figures are in screenshots and the text does not contain "25x" or any witness-exclusion statement. Fix: "ICICLE-Halo2 v2 benchmarks on RTX 4080 vs i9-13900K (speedup figures in images, not extractable from the capture); not a BLS12-381 Sherlock benchmark." Remove "up to 25x" unless the image is captured and read.
2. **[RESOLVED: draft correct.]** `docs/results/sherlock-native-source44-20261022/source/sherlock_link.rs:1230-1234` starts the Prove phase before `prove::<...>`, then `drop(pk)` at 1233 and ends the phase at 1234, so the receipt prove phase includes PK drop. (Earlier inference superseded.) `sherlock_link.rs:1044` `drop(pk)` is inside the parent prover function, but I did not verify the receipt's `prove` phase wraps that function. Fix: "the prove phase likely includes PK drop (inferred from `sherlock_link.rs:1041-1044`; not confirmed by the receipt's instrumentation)."
3. **[RESOLVED by astra-1 rerun at 22:55Z: A2 first failing.]** Not rerun by me. The A1 check (`scripts/qa/check-a1.sh`) is live and its result is unknown to me. Fix: cite the output with timestamp, or say "by reading predicates.yaml: A1 is live, A2 is the first unimplemented heavy predicate; the runtime result was not captured."
4. **F-table incomplete for the factory schema (Lines 125-134). [Accepted; T1 point narrowed.]** Heavy routing: `factory/cli.py:99` sends heavy predicates through `heavy` even under `check`; `CONTRACT.md:43-44` requires `budget` for heavy checks. So heavy at T1 is not illegal. `CONTRACT.md:106` bounds T1 at under 20 min, so F202 stays T1 only if its heavy GPU differential fits that bound; otherwise make it T2. `models/doctorwatson/factory/predicates.py:112-113` requires a `budget` for every `heavy: true` predicate, and the table omits `heavy` and `budget` for all six. F201, F203, F204, F205, F206 must be heavy with budget keys (proposed `f201-phase-profile`, etc.). Budgets live in `qa/sherlock/budgets.json`. F202 (GPU differentials) is heavy on the GPU device; a T1 tier would run it under `qa/tiers/t1.sh`, which does not run heavy checks, so the tier is inconsistent. Fix: add `heavy` and `budget` columns; move F202 to T2 or make it non-heavy with CPU-only differentials at T1.
5. **[ACCEPTED by astra-1: F201 depends on A2 intentionally.]** Original concern: The draft's F201 depends on A2, which is an unimplemented heavy stub. Then F201 cannot run until A2 passes, and A2 needs the same k20-scale artifacts. That is a valid gate but should be stated; otherwise it reads as a free profiling step. Fix: state "F201 depends on A2 (unimplemented); the profile is therefore blocked until A2 is implemented."
6. **"None of steps 1-5 requires a new VK or ledger contract" (Line 74).** Conditional. Step 5 changes witness assignment order and possibly the floor planner; the VK is unchanged only if the circuit rows and copy constraints are preserved (the draft says so, Line 68). Fix: "None of steps 1-5 requires a new VK provided the VK-byte equality and layout checks pass."
7. **Astra-2 numbers quoted without verification (Line 117, 140).** "20,466 rows", "k15 capacity 32,761" (arithmetic correct: 12274 + 8192 = 20466; 2^15 - 7 = 32761), and "2-4 engineer-weeks" and "6-12+ weeks" are astra-2's estimates. Fix: attribute explicitly: "astra-2 estimate, not measured."
8. **"Crossing 10x probably requires optimized witness construction" (Line 33).** Opinion. The draft's own scenario shows bulk 20x alone gives 3.72x and witness 6.45x is needed in the 8x-bulk case. Acceptable as ESTIMATE, but say "under the draft's assumed phase shares."
9. **"Bulk 20x ... R=.269" and the product 11.39x (Lines 99-103).** Arithmetic correct, but the table uses the 882.590 s prove time as the base while the cache step assumes PK construction (62.328 s) is fully removed. That is a warm-service assumption. Fix: mark the row "warm-service assumption: PK construction fully removed."
10. **Helper attribution (Line 146).** Says haiku-02 "traced ... the critical 0.8.2/0.8.3 MSM difference" and haiku-03 "corrected for 27 fixed + 18 sigma". My check agrees with both facts (the MSM difference is verified; the 27+18 split is inferred from 45 fixed in the REPORT). Fine, but label the 27+18 as inferred.
11. **Speedup-budget title "Evidence against a 4x hardware ceiling" (Line 41).** The table gives published ceilings (2.80x Nocy, 3.81x/30.82x cuSnark, 25x ICICLE unverified). Title overstates: cuSnark's own witness-bound ceiling is 4.85x, and Nocy's 2.8x is a per-commit-sync implementation. Fix: "Published GPU speedups: what they bound and what they do not."

### C. Missing caveats (add)

- Cross-version: recl9 numbers use proofs 0.8.2 (BLST cut-over at 2^19 scalars). Draft correctly warns (Line 39), but the budget section (Line 95-103) still uses 882.590 s from the 0.8.3 receipt while the sequential chain uses 0.8.2 numbers in the 1617 s analysis. Keep the two in separate sections, and say so.
- **Ledger reader (corrected per astra-1).** `chain/rc5check/ledger-rc5` is at git HEAD `3997c6c8d1`, not a tag (`git describe --tags` returns nothing). `chain/REPORT.md:33` describes it as built from `ledger-9.1.0.0-rc.5`; that label is unverified by git. Resolve the reader from `ledger-verify` Cargo.lock/Cargo.toml, not the directory name. recl9 measured decider verify is rc.6 (`REPORT.md:160-166`). (Superseded text follows.) Ledger reader: `chain/rc5check/ledger-rc5` is the rc.5 tag source; recl9's decider verify is rc.6 (`REPORT.md`). Draft (Line 115) names rc5check as the tested reader, but the recl9 measured verify was rc.6. Fix: "the measured local verify was ledger-9 rc.6 (recl9 REPORT); the rc.5 reader in rc5check is a separate frozen source."
- SRS: draft's Line 119 "ledger's embedded 2^14 verifier parameters do not impose a k14 circuit limit" is correct per astra-1's note. Keep it; the decider prover used 2p20 (REPORT.md:154).

## Raw numbers (checked)

| Item | Value | Source |
|---|---|---|
| prove / setup_pk | 882.589555363 s / 62.328385318 s | receipts/parent-proof.json |
| leaves / node / decider | 163.630576714 / 867.279750364 / 586.381689547 s | out/*/stats.json |
| cuSnark Proof 2 | 1762.75 -> 463.13 s; Eval_h 1115.43 -> 36.19; advice+lookup GPU 363.28 s | astra-1/raw/snarkify-cusnark.md |
| Snarkify BLS 4 proofs | 2.16 s (540 ms/proof) | astra-1/raw/snarkify-zprize.md:228 |
| Nocy 5090 k17/k18 | 4.4/8.8 s GPU vs 12.3/24.6 s CPU | astra-1/raw/haiku-01-haiku01-nocy-repo.md:280-281 |
| ICICLE-Halo2 25x | not in capture text | astra-1/raw/icicle-halo2-v2.md |

## Caveats

- Not run: `sherlock-next`, `check-a1.sh`, any build, any GPU job.
- Not checked: the Snarkify microbench claim "repeats the generator as every MSM base" (only the `generator[24]` array was seen); ZKProphet's "different CPU baselines"; the 5.687 s cost-model figure (`model.duration_seconds` in the receipt matches it, so that one is verified).
- The "roughly 120-second keygen anecdote" and "951.6/954.1" intervals were checked only for existence in the receipt.

## Corrections after astra-1 review (2026-10-24 22:55Z)

- ICICLE 25x and witness-scope statement: both present in the capture (lines 19 and 31). Draft line 48 is supported; add the ezkl-halo2 baseline caveat.
- Prove interval includes PK drop: confirmed at `docs/results/sherlock-native-source44-20261022/source/sherlock_link.rs:1230-1234`.
- sherlock-next gives A2 first failing: from astra-1's rerun; not rerun by me.
- F201 depends on A2: accepted as intentional.
- F202 at T1 is allowed only within the 20-min T1 bound (`CONTRACT.md:106`); heavy routing exists (`cli.py:99`).
- rc5check HEAD `3997c6c8d1` is not a tag; the rc.5 label comes from `chain/REPORT.md:33` and is unverified by git.
