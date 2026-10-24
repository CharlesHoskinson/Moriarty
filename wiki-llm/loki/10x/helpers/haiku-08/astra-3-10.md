# astra-3-10: review of PROPOSAL.md (math, denominators, caches, Send/Sync)

## Question
Read `/home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/astra-3/PROPOSAL.md`. Check the mathematical bounds, denominators, caches, and Send/Sync feasibility. Mark factual errors with paragraph and correction. No edits to the lead's file. No new broad search.

## Summary
- **Arithmetic is correct** in every checked speedup, ceiling, and memory line (see P1-P10 below).
- **One modeling error in the headline:** the 10.45x pipeline result charges the CPU lane `cpu_fraction = 0.25` as a wall share at an unspecified core count. The PROPOSAL's own measured CPU data (2.69 and 4.01 effective cores) implies 0.34 to 0.50 at the assumed two cores per worker. That moves the result to 9.7x and 7.4x before any contention is modeled. The PROPOSAL says the core assumption must be measured, but still headlines 10.45x.
- **Bulk GPU speed of 10x is outside the design's own range.** LOKI-DESIGN (L3/L4) gives about 1.6x MSM-only and "about 3x and more" with the quotient. The PROPOSAL's sensitivity table tests 8x and 20x only. At 3x the same model gives 3.8x to 4.6x (astra-3-7 grid).
- **Naming error:** the "parent" in the Fenrir-chain 1.933x and 2.156x figures is the arity-4 node (867.28 s), not the Sherlock A102/A80 parent (882.59 s). The arithmetic is for the node.
- **My own earlier PK estimate double-counted permutation columns** (corrected in astra-3-4 addendum 2). The PROPOSAL's column accounting is right.
- **Send/Sync section is correct** and matches my inspection.

## Findings, by paragraph

### Verdict section
- **V1 (verified arithmetic):** "(62.328+882.590)/882.590 = 1.07062x" is correct (944.918/882.590 = 1.07062). It excludes the 5.69 s `cost_model` phase in the same parent (`phases.model.duration_seconds`, receipt line for `model`); the PROPOSAL does not mention it. Minor.
- **V2 (FACTUAL, naming):** "making only the parent 10x faster improves total proving by 1.933x; even a zero-time parent has 2.156x". Verified: 1617.292 / (1617.292 - 0.9 x 867.280) = 1.9328; 1617.292 / (1617.292 - 867.280) = 2.1563. Both use the node (867.28 s), so "parent" must read "node". The Sherlock parent is a different 882.59 s object. Correction: rename to "arity-4 node".
- **V3 (headline overstatement):** "A reproducible queue model reaches 10.45x" omits that the model's CPU lane charges 0.25 wall at unspecified cores. With the measured node/decider core counts (2.69/4.01 on 2-core workers) the same code gives 9.71x / 7.39x (astra-3-7 table). The paragraph should say so.

### Section 1 (baseline)
- **B1 (verified):** PK setup sum 157.751410713 s = node 104.463 + decider 46.892 + leaves 6.396 (`recl9/out/leaves-b1/leaves.stats.json` `keygen_pk_s` 6.396236471). Correct.
- **B2 (verified):** node 2615.48 CPU-s / 973.73 s = 2.686 cores; decider 2546.75 / 634.85 = 4.012 cores. Correct (GNU time logs, `node-prove.log` and `decider-prove.log` lines 5-9).
- **B3 (unverified detail):** "the 0.8.2 dispatch uses generic MSM above `2<<18` nonzero scalars". The zero-filter in `msm_specific` runs before the length check in 0.8.3 (`msm.rs:166-172`). I did not re-check 0.8.2's filter lines, so "nonzero" is unverified for 0.8.2.
- **B4 (accurate):** "Historical CPU allocation is not established by today's 20-core host" — the logs show 268% and 401% of whatever was available; the machine has 20 logical CPUs (`nproc`).

### Section 2 (caches)
- **C1 (verified):** `P/src/plonk/mod.rs:416` — serialized reads rebuild coefficient and coset transforms (`mod.rs:416-428`). Correct.
- **C2 (verified):** `P/src/poly/kzg/params.rs:40` downsize recomputes the Lagrange basis (`g_lagrange = g_to_lagrange(&self.g, new_k)`, params.rs:48). Correct.
- **C3 (verified):** ledger `PK_CACHE_SIZE = 5` (`midnight-ledger/transient-crypto/src/proofs.rs:250`, `lru::LruCache` at :21), `num_workers` default 2 (`proof-server/src/main.rs:44`), `spawn_blocking` at `worker_pool.rs:236`. Correct.
- **C4 (memory, arithmetic correct; one fix):** "retained PK payload `32 MiB x [(27+18)x6+12] = 8.8125 GiB` per heavy shape" — 282 units x 32 MiB = 9024 MiB = 8.8125 GiB. The 45 includes permutation columns: verified in `midnight-proofs-0.8.3/src/dev/cost_model.rs:244-246` (`fixed_columns: self.fixed.len() + self.permutation.columns`, `permutations: self.permutation.columns`). So 45 = 27 fixed + 18 permutation. My astra-3-4 section 5 used 45 fixed + 18 permutation (double count) and gave 11.6 GiB; the corrected figure is about 8.25-8.8 GiB (ESTIMATE).
- **C5 (cross-check):** `recl9/logs/node-prove.log` line 3, `[node] pk 104.5s rss 11799136 kB` = 11.25 GiB after PK setup, which includes the SRS and other state. An 8.8 GiB PK plus about 2.5 GiB of other resident state is consistent with this.
- **C6 (correct):** "Two full decoded keys ... 17.625 GiB" = 2 x 8.8125.
- **C7 (unit slip in my own astra-3-4 text):** I wrote peak RSS as "14.97 GB" and "14.99 GB". The GNU time values are kB: 14,965,544 kB = 14.27 GiB = 15.33 GB decimal. The B7 bound is decimal 16 GB; the measured peak is under it, and the log shows `Swaps: 0`.

### Section 3 (threads, Send/Sync)
- **S1 (verified, path ambiguous):** `midnight-circuits-7.2.4/src/field/native/native_chip.rs:117-118` holds `Rc<RefCell<...>>` (`cached_fixed`, `committed_instance_offset`). The PROPOSAL cites `src/field/native/native_chip.rs:117`; the crate is midnight-circuits, not zk-stdlib. Correct line, ambiguous path.
- **S2 (verified):** `Z/src/lib.rs:1529` = `nb_public_inputs: Rc::new(RefCell::new(None))` in `MidnightCircuit::new`. Correct.
- **S3 (verified):** `P/src/plonk/prover.rs:371,515,568` — compute_trace call (371), per-circuit loop in `parse_advices` (515), advice commitment (568-571). Correct.
- **S4 (verified):** `R/blst-0.3.17/src/lib.rs:41` = `da_pool()`, the separate BLST pool. Correct.
- **S5 (verified; completes the Send/Sync analysis):** `R: Relation` has no Send/Sync bound (`midnight-zk-stdlib-2.3.5/src/lib.rs:1795`), and `MidnightPK<R>` and `ProvingKey` contain no `Rc`/`RefCell` (grep of `midnight-proofs-0.8.3/src/{plonk,poly,circuit,utils}`). So `MidnightPK<R>` is `Send + Sync` iff `R` is. The PROPOSAL's "may require separation" is accurate. The one gap: Moriarty's `midnight_zkir_v3` IR relation types are not in the local registry, so their auto-traits are unverified.
- **S6 (verified):** "lookup leftovers iterate a randomized HashMap (`P/src/plonk/lookup/prover.rs:399,430`)" — `HashMap::with_capacity` at 399, iteration at 430. Correct (std HashMap uses per-process RandomState).
- **S7 (verified as cited; production path unconfirmed):** "the foreign ECC chip independently samples OsRng (`midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:994`)" — `let mut rng = OsRng;` at 994, inside a function taking `scalar_field_chip`. I did not confirm whether that function is test-only; the PROPOSAL should say so.
- **S8 (unverified here):** "vanishing randomness depends on Rayon width" — the vanishing prover imports `current_num_threads` (`plonk/vanishing/prover.rs:6`); I did not trace the RNG consumption order.

### Section 4 (memory)
- **M1 (correct):** 32 MiB per k20 field vector; 128 MiB per k22 extended vector.
- **M2 (correct, consistent with C4):** 8.8125 GiB per heavy shape.
- **M3 (missing computation):** the 64 GiB experiment with five slots. Measured node peak 14,988,176 kB = 14.29 GiB; 11.25 GiB of it is resident after PK setup (C5), and about 3.0 GiB is added during prove. With an Arc-shared PK (8.8 GiB) and five slots at about 3 GiB each, the total is about 24 GiB, which fits 64 GiB. With duplicated PKs, five slots is about 5 x 14.3 = 71.4 GiB, which exceeds 64 GiB. The PROPOSAL states the constraint but does not compute it; five slots needs the shared-key design. Consistent with astra-3-7 section 4.

### Speedup budget section (arithmetic checked)
- **P1 (verified):** T = 1617.292 s gives 2.22594 batches/h (3600/1617.292). Correct.
- **P2 (verified):** setup-inclusive 1775.043 / 1617.292 = 1.09754. Correct.
- **P3 (verified):** 0.25 + 0.75/10 + 0.01 = 0.335; 1/0.335 = 2.98507; T' = 541.793 s. Correct.
- **P4 (verified):** 541.793 / 154.705 = 3.50211; 2.98507 x 3.50211 = 10.4541 = 1617.292/154.705. Correct.
- **P5 (verified):** a 4.54% interval slowdown takes 10.454 to 10.000. Correct.
- **P6 (verified):** GPU-only ceiling 1/0.075 = 13.33x. Correct.
- **P7 (verified):** 1/0.26 = 3.846x for infinite GPU at 25% CPU and 1% overhead. Correct.
- **P8 (verified):** R-denominator challenge: 10.454/2 = 5.227; 13.333/2 = 6.667; 4/0.25/2 = 8.0. Correct.
- **P9 (verified as a ceiling):** (2615.48 + 2546.75)/20 = 258.11 s; 1775.043/258.11 = 6.88x. It assumes node and decider CPU time is unchanged and all 20 cores are busy the whole time, so it is a bound, not a forecast.
- **P10 (verified):** dependence limits 1617.292/508.297 = 3.18 and 1617.292/282.001 = 5.735 (PROPOSAL writes 5.74; rounding).
- **P11 (MODEL ISSUE, same as V3):** the 4-worker CPU lane is charged `0.25 x T` per stage regardless of cores. With the measured effective cores, the CPU-lane input is 0.336 (node) to 0.50 (decider), giving 9.71x and 7.39x (astra-3-7). The PROPOSAL's "treat the assumed two effective cores as a quantity to measure" is correct but must be applied to the headline.
- **P12 (MODEL ISSUE):** the sensitivity set omits bulk speed 3x (the LOKI-DESIGN range). Results at 3x with 5 slots: 3.81x (cpu 0.25), 4.06x (cpu 0.336), 4.55x (cpu 0.5).

### Validation and risk sections
- No numerical claims to verify there beyond those already noted. The parent-preparation rejection citation (`F/REPORT.md:191`) is unverified here.

## Recommended edits (for the lead to decide; not applied)
1. V2: say "arity-4 node", not "parent", in the 1.933x and 2.156x sentence.
2. V3 and P11: state 10.45x as conditional on a 0.25 CPU wall share at an unmeasured core count, and report 9.7x / 7.4x at the measured effective cores in the same paragraph.
3. P12: add a bulk-speed row at 3x (the LOKI-DESIGN range) beside the 10x row.
4. C4: note that 45 = 27 fixed + 18 permutation, so the payload is about 8.8 GiB per shape, not 11.6 GiB.
5. M3: compute five-slot memory for duplicated keys (about 71.4 GiB, over 64 GiB) and shared keys (about 24 GiB).

## Caveats
- Verified items come from direct reads of the cited files or log lines. Unverified items are labelled.
- The 3x sweep is the astra-3-7 queue model run as written (not modified), with `bulk_speed=3`.
- Memory figures convert GNU time kB at 1024 bytes per kB.
