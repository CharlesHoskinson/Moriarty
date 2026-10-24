# astra-1-2: Serial floor audit of the real prover (Sherlock parent)

## Question
Trace `create_proof` (midnight-proofs 0.8.3, with circuits 7.2.4 and recl9 Sherlock parent synthesis). If MSM, FFT, quotient, lookup sorting, grand products, inversions, multiopen and witness field arithmetic move to GPU, what stays CPU? Count transcript barriers, host synthesis dependencies, row counts and available timings. Cite path:line. Read LOKI node 09 and node 01.

Source note: node files are `wiki-llm/loki/nodes/loki-01-prover-profile.md` and `loki-09-cpu-witness-field.md` (no `LOKI-DESIGN/nodes01` directory exists; the design doc is `wiki-llm/loki/LOKI-DESIGN.md`). All evidence below is local code, local logs or local result docs. No web pages were relied on, so there are no Scrapling captures.

Path abbreviations:
- `P` = `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3`
- `P2` = same root, `midnight-proofs-0.8.2`
- `S` = same root, `midnight-zk-stdlib-2.3.5`
- `C` = same root, `midnight-circuits-7.2.4`
- `MC` = same root, `midnight-curves-0.3.1`
- `B` = same root, `blst-0.3.17`
- `R` = `/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9`
- `D` = `/home/charles/repos/Moriarty-factory/docs/results/sherlock-native-observations-20261022`

## Answer

### 0. Which prover code is being traced
- Moriarty's lock pins `midnight-proofs 0.8.3` (`Moriarty-factory/Cargo.lock:2031`). The Sherlock parent receipt (882.6 s) is NOT a Moriarty-workspace build. Its source snapshot (`docs/results/sherlock-native-source44-20261022/README.md:7`) belongs to the qualified Fenrir recl9 backend and needs a private patched SDK closure. Its proofs version is not recorded; the recl9 lockfile pins 0.8.2. The receipt is a completed reference scale only, not a matched 0.8.3 baseline.
- recl9 pins `midnight-proofs =0.8.2` (`R/Cargo.toml:25`; `R/Cargo.lock:647`), with `midnight-zk-stdlib =2.3.5` and a patched circuits crate (`R/Cargo.toml:35`).
- `diff -r P2/src P/src` differs in one file only: `poly/kzg/msm.rs`. The prover path (`plonk/prover.rs`, `lookup`, `permutation`, `vanishing`, `evaluation.rs`, `domain.rs`, `kzg/mod.rs`) is byte-identical across both. The MSM dispatch does differ (section 6). This matters for baselines.

### 1. create_proof order, with CPU/GPU classification
Entry: `P/src/plonk/prover.rs:348` `create_proof` calls `compute_trace` (`:371`) then `finalise_proof` (`:385`). The stdlib entry is `S/src/utils/plonk_api.rs:88-112`. It calls `create_proof` with `nb_instance_commitments=1`, so the `committed-instances` path is active.

Phase A, `compute_trace` (`prover.rs:57-240`):
1. VK hash into transcript (`:98`). Host.
2. `compute_instances` (`:392-446`). The committed instance is interpolated and committed by MSM (`:434-436`). This is one extra n-length MSM before any advice work, and it is not in the GPU list. Host.
3. `parse_advices` (`:460-592`), serial over phases (`:501`) and circuits (`:533` synthesis per phase per circuit). For each phase:
   - full synthesis (see section 3);
   - `batch_invert_rational` over all denominators of the phase (`:540`);
   - blinding rows drawn with `F::random` serially (`:557-559`);
   - advice commitments computed by a serial `.map(commit_lagrange)` (`:568-569`), then written to transcript (`:571-573`);
   - challenge squeeze for the phase (`:580-582`).
4. theta squeeze (`:108`). Then per lookup, serially (`:110-124`): compress expressions with `evaluate` over rows, `permute_expression_pair` (sort plus HashMap), two commitments, two transcript writes (`lookup/prover.rs:58-135`).
5. beta and gamma squeezes (`:138`, `:141`).
6. Permutation commitments per advice set. Outer loop over column chunks, serial. Products are parallel per column, but `batch_invert` is serial (`permutation/prover.rs:106`). The prefix product `z` is a serial loop (`:141-146`). Blinding is serial (`:148-151`). `last_z` is carried from one set to the next (`permutation/prover.rs:70`, `:153`), so the sets cannot run concurrently.
7. Lookup products (`lookup/prover.rs:155-296`):
   - parallel numerators, serial `batch_invert` of n denominators (`:182-196`);
   - serial prefix scan (`:229-`);
   - blinding `F::random` (`:239`, serial);
   - `commit_lagrange` and one write (`:283-287`).
8. trash squeeze (`:175`). Trash commits and writes (`plonk/trash/prover.rs:24-63`).
9. vanishing random commitment: a parallel random fill (`vanishing/prover.rs:47-62`), then one coefficient-basis commit and write (`:79`).
10. y squeeze (`:205`). Then `advice_polys` iFFT to coefficients (`:211-220`).

Phase B, `finalise_proof` (`prover.rs:242-346`):
11. `compute_nu_poly` (`:267`, impl `:595-656`). Serial coset FFTs of every advice and instance column (`:603-620`), then `evaluate_numerator` (`plonk/evaluation.rs:285-`), which is rayon-parallel per chunk for custom gates (`:317-340`), lookups (`:455`), trash (`:524`) and permutation (`:373`). This is the GPU quotient target.
12. Vanishing `construct`: divide by `t(X)` (`domain.rs:321-340`, parallel), extended iFFT (`domain.rs:272-292`, FFT-bound), coefficient split, `blind_quotient_limbs` (serial random, `vanishing/prover.rs:138-146`), then one coefficient-basis commit per h piece, serial `map` (`:123-124`) and write (`:127-129`).
13. x squeeze (`:282`). Evaluations written: `write_evals_to_transcript` (`:284`), vanishing evaluate (`:293`), permutation evaluate (`:299-302`), lookup evaluate (`:305-316`), trash evaluate (`:316`, `trash/prover.rs:70-76`). Each eval is `eval_polynomial` (`utils/arithmetic.rs:53-`), which is chunked-parallel for large polynomials.
14. KZG `multi_open` (`P/src/poly/kzg/mod.rs:94-200`): x1 and x2 squeezes (`:105-106`); query polynomials are cloned (`kzg/mod.rs:113`); `inner_product` (serial outer loop with an allocation per term, `utils/arithmetic.rs:263-273`); `kate_division` per point per set (serial recurrence, `utils/arithmetic.rs:95-115`, applied at `kzg/mod.rs:150-153`); f commit (`:163`) and write; x3 squeeze (`:166`); q evaluations written; x4 squeeze (`:176`); pi = `kate_division` (`:191-`) then commit and write (`:199`).

Serial items inside the listed GPU scope. These are movable implementation bottlenecks, not irreducible CPU work. Every one of them can be moved to GPU or parallelized (scan, tree reduction, or a parallel bit-reversal/FFT), and the proof stays valid under the same VK and transcript order. Byte identity additionally needs a deterministic lookup leftover order (see the lookup item):
- Witness synthesis and layout (section 3). Control flow is sequential but its value-producing closures are data-parallel once extracted into a DAG (see astra-1-7).
- Rational witness inversion (`poly/mod.rs:287-318`, one serial batch across the phase; `prover.rs:540`). Batch inversion parallelizes by chunks, each chunk paying one inversion.
- In-circuit eager inverses inside region closures (`C/src/field/native/native_chip.rs:367-377`, `:774`, `:1253`).
- Lookup sort and multiplicity HashMap, per lookup: `lookup/prover.rs:396` (`sort()` over `usable_rows`, a sort, parallelizable), `:399-` (HashMap counts, a histogram). The HashMap iteration order at `:430` is randomized per process (std RandomState), so the leftover placement is not reproducible even today. A GPU sort may pick another valid leftover order under the same VK; acceptance must rely on the lookup multiset equations and independent verification (per astra-1's correction).
- Lookup and permutation batch inversions and prefix scans (`lookup/prover.rs:196`, `:229`; `permutation/prover.rs:106`, `:141-146`). Prefix products are affine-map scans and parallelize. The permutation sets are serialized only by the carried `last_z` (`permutation/prover.rs:70`, `:153`), which is itself a scan boundary.
- Blinding RNG draws (`prover.rs:558`; `lookup/prover.rs:437`; `permutation/prover.rs:149-151`). A counter-based RNG (or pre-drawn stream in fixed order) keeps the transcript RNG ordering and parallelizes the fill.
- FFT host passes: bit-reversal (`MC/src/fft.rs:46-51`) and twiddle table (`:54-60`) are serial loops; the top-level butterfly combine is a serial `for_each` (`:120`) under the rayon join at `:107`. Only the recursive halves run in parallel. This is a code-structure estimate, not a measurement: the span is roughly 2.5n element operations against about (n/2)·log2 n butterflies, an Amdahl ceiling of about 4 to 5x at 20 threads on the CPU. A GPU NTT removes this ceiling; node 01 has the same structure (`loki-01:91`).
- `kate_division` and multiopen outer loops (section 1, step 14). Kate division `q_i = r_i - q_{i-1}·x` is an affine recurrence, so it parallelizes as a scan of 2x2 affine maps. The code has no such scan today (`arithmetic.rs:95-115`). Multiopen's outer loop over queries is independent per query set.
- Transcript hashing on host. Every point absorbed with the Poseidon transcript goes through `as_public_input` (`C/src/ecc/foreign/weierstrass_chip.rs:235-247`; `C/src/hash/poseidon/poseidon_cpu.rs:207-214`). That converts projective to affine (inferred to cost one field inversion per absorbed point, not traced into blst assembly). This is not movable work, but it is cheap relative to the commitments (one absorb per commitment or scalar).
- MSM host preparation, which is also movable: 
  - `msm_specific` zero filter and copy: `filter`, `map(|(s,b)| (*s,*b))`, `unzip` (`P/src/poly/kzg/msm.rs:165-170`). `G1Projective` is `blst_p1`, three 48-byte Fp coordinates, 144 bytes (`MC/src/bls12_381/g1.rs:470`). The copy is about 151 MB per 2^20-base MSM. Caching affine bases on device removes it.
  - `multi_exp` converts all bases to affine on every call (`g1.rs:634-637`) and serializes every scalar with `to_bytes_le` into a fresh Vec (`:639-641`). blst's `p1_affines::from` is pool-parallel only when at least 768 points (`B/src/pippenger.rs:75-87`).
  - Node 09's claim that the SRS affine bases are converted on every MSM is confirmed here.

Residual that is NOT movable (true control and dependency span): the Fiat-Shamir transcript order (section 2: 8 sync groups plus one per challenge-bearing phase, so the host must absorb each commitment in order), the synthesis control flow that decides layout, and the dependency span of the in-circuit chains (MSM accumulator and Poseidon sponge, about 10^3 to 10^4 operations per astra-1-2-followup-witness.md). Everything else in the listed scope is a candidate for GPU or parallel CPU.

Summary of the GPU scope in the Sherlock parent: MSMs (commits, multi-open commitments, h pieces, committed instance, advice, lookup, permutation, trash, vanishing); FFT/iFFT; quotient (`evaluate_numerator`); element-wise numerators and divisions; sorts; scans; inversions; kate division (affine scan); RNG fill. Residual host control: transcript order, synthesis control flow, and the dependency chains above.

### 2. Transcript barriers (Fiat-Shamir squeezes and sync groups)
Squeeze sites on the proof path:
- `plonk/prover.rs:108` theta; `:138` beta; `:141` gamma; `:175` trash; `:205` y; `:282` x.
- `plonk/prover.rs:581` one per challenge in each phase (C total challenges).
- `poly/kzg/mod.rs:105`, `:106` x1, x2; `:166` x3; `:176` x4.

Total squeezes = 10 + C. Node-level sync groups (where a squeeze needs values produced by device work, and no transcript work intervenes):
1. theta: needs all advice commitments.
2. beta + gamma: needs lookup permuted commitments and their writes.
3. trash: needs permutation and lookup product commitments.
4. y: needs trash commitments and the vanishing random commitment.
5. x: needs h piece commitments.
6. x1 + x2: needs all evaluations (scalar writes).
7. x3: needs f commitment.
8. x4: needs q evaluations.
Plus one more group per challenge-bearing phase (`:581`), because the next phase's synthesis reads the challenges.

So the minimum barrier count is 8 + (number of challenge-bearing phases). Phase count and challenge count for the Sherlock parent are not recorded (see section 4).

Transcript write sites on the path: 32 `transcript.write`/`common` sites in `plonk/` and `kzg/` (grep count). Commitment writes per proof:
- advice: A (summed over phases);
- lookups: 2L (permuted input and table) plus L (products);
- permutation products: S = number of column sets, with S = ceil(P_c / (d-2)) where d is the constraint degree (not recorded for the Sherlock parent);
- trash: T;
- vanishing random: 1 (`vanishing/prover.rs:79`);
- h pieces: H (`vanishing/prover.rs:128`);
- f_com: 1 (`kzg/mod.rs:164`) and pi: 1 (`:199`);
- committed instance: 1 common write, not a transcript write (`prover.rs:436`).

Commitment MSM count per proof = 1 (committed instance) + A + 3L + S + T + 1 + H + 2. Not counting setup_pk.

### 3. Host synthesis dependencies
- Synthesis is repeated per advice phase: `prover.rs:501` (phase loop) and `:533` (synthesize). `pk.vk.cs.phases()` (`plonk/circuit.rs:2317-2321`) returns 0..=max advice phase.
- Each synthesis pass runs twice per region: shape pass (`circuit/floor_planner/single_pass.rs:93`) and assignment pass (`:111-114`). Only current-phase advice writes call the value closure (`prover.rs:859` early return before `to()` at `:871`).
- So synthesis work = 2 full passes per advice phase. The Sherlock parent's phase count is not recorded, so this is a formula.
- Stdlib `MidnightCircuit::synthesize` (`S/src/lib.rs:1868-1900`): constructs `ZkStdLib::new` each pass, clones `self.instance` and `self.witness` each pass (`:1880-1883`), runs `relation.circuit`, and loads lookup tables at the end (`zk_std_lib.core_decomposition_chip.load`). Tables are reassigned on every pass.
- The stdlib uses `SimpleFloorPlanner` (`S/src/lib.rs:1845`).
- `setup_pk` synthesizes too: `S/src/lib.rs:1967-1980` (`Value::unknown()`, keygen_pk). Measured separately (section 5).
- recl9 host steps around the parent prove (`R/src/sherlock_link.rs:1020-1050`, function `prove_link_parent`):
  - `rel.check_witness` (:1023);
  - `check_srs` (:1025) and `vk_roundtrip` (:1026);
  - `cost_model(rel, None)` (:1029), which builds a MidnightCircuit and runs a model pass;
  - two off-circuit child verifications (:1031-1033);
  - `reconstruct_acc` and `acc.check` pairing (:1034-1036);
  - `setup_pk` (:1041), `prove` (:1042), `drop(pk)` (:1044), then stdlib verify and off-circuit verify (:1045-1046).
- Witness data: child proof bytes are cloned into witnesses (`R/src/node.rs:125,129`; `R/src/decider.rs:159`; from node 09). The child proofs are 3440 bytes each, A102 at k13 and A80 at k12 (`docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json`, `children[0..1]`). The parent proof is 6432 bytes (`D/samples/parent-proof.json`). The witness size is not measured.
- In-circuit Poseidon and EC witness generation is computed in host closures. I did not trace every gadget; this is partially traced.

### 4. Row and column counts
- Sherlock A102 to A80 parent (Fenrir recl9 backend, proofs version unrecorded): `circuit.*` metrics (advice, fixed, instance, lookup rows, used rows, domain capacity) are all `status: unavailable` (`D/samples/parent-proof.json`, `layers.parent`). Only `circuit_k: 20` is recorded.
- Historical recl9 node arity 4 (k20, 0.8.2): rows 958,095; table rows 1,032,465; advice 15; lookups 5; equality (permutation) columns 18; fixed 45, of which 18 are permutation commitments, so 27 actual fixed (`R/keys/node4/keygen.stats.json`; node 01 `loki-01:43`). usable rows at k: 1,048,569.
- Historical recl9 decider (k20): rows 841,682; table rows 1,032,209; advice 15; lookups 5; permutations 18; fixed 45 (`R/keys/decider/keygen.stats.json`).
- Historical recl9 leaf (k17): rows 112,098; table rows 98,563; advice 9; lookups 4; permutations 10; fixed 32 (`R/keys/leaf4/keygen.stats.json`).
- Node 06 two-child relation: 574,040 advice rows, 1,032,465 table rows (`loki-06` `CODE` line citing REPORT:117).

### 5. Available timings (only measured values; estimates excluded)
Parent (Fenrir recl9 backend, proofs version unrecorded): `D/samples/parent-proof.json` `extensions.observation_scope.phases`:
- setup_pk 62.328385318 s; prove 882.589555363 s (includes explicit PK drop); model 5.687048131 s; native capture 951.6142616319703 s; root 954.127131041023 s.
- producer verification phases (seconds): accumulator_fold 0.0106, child_a102_verify 0.0193, child_a80_verify 0.0191, folded_pairing 0.0007, pairing_verify 0.0130, stdlib_verify 0.0091, write 0.0111.
- Not recorded: CPU time, RSS, cache state (`cache_state: unknown`), timer resolution. The doc says these intervals overlap and must not be added.

recl9 historical (0.8.2, lockfile-inferred; logs do not record the proofs version):
- node prove (arity 4, k20): pk 104.5 s; prove 867.3 s; wall 16:13.73; user 2534.17 s; sys 81.31 s; 268% CPU; max RSS 14,988,176 KB (`R/logs/node-prove.log:3-9,14`).
- node keygen: VK 622.8 s; wall 15:59.50; user 1697.42 s; max RSS 4,502,488 KB (`R/logs/node-keygen.log:3,5-13`; `R/keys/node4/keygen.stats.json` `keygen_vk_s`).
- decider prove (k20): pk 46.9 s; prove 586.4 s; wall 10:34.85; user 2486.95 s; 401% CPU; max RSS 14,965,544 KB (`R/logs/decider-prove.log:3-4,6-9,14`).
- decider keygen: VK 412.0 s; wall 9:05.18; user 1631.08 s; max RSS 4,774,636 KB (`R/logs/decider-keygen.log`).
- leaf keygen (k17): VK 31.0 s (`R/keys/leaf4/keygen.stats.json`).
- historical node cost_model_seconds 335.25 s (`R/keys/node4/keygen.stats.json` `cost`). This is keygen-time cost-model work for node 4, not the parent's model phase (5.7 s).

Host: `nproc` = 20; Intel Core Ultra 9 285K; 90 GB RAM total (`free -g`).

Observation: the node-prove run averaged 268% CPU over 973.7 s wall, and the decider 401%. On a 20-core host, both leave most cores idle. That is consistent with the serial sections above dominating, but the log does not partition time by phase, so no serial fraction can be derived from it.

Node 01 and node 09 Amdahl figures (for example 1.11 to 1.40x for the CPU track; 500 s hypothetical proof phase split) are ESTIMATES and I did not reproduce them as measurements.

### 6. New finding: the recl9 0.8.2 MSM dispatch differs at k20
- In 0.8.3, `msm_specific` always uses BLST `multi_exp` for BLS G1 (`P/src/poly/kzg/msm.rs:181-188`). Node 09 and node 01 describe this correctly for 0.8.3.
- In 0.8.2, the BLST branch is taken only when `coeffs.len() <= (2 << 18)`, that is at most 2^19 = 524,288 nonzero scalars (`P2/src/poly/kzg/msm.rs:176-177`). Otherwise the path is `batch_normalize` plus `msm_best` (`:189-193`).
- Every k20 commitment in the recl9 build has up to 2^20 scalars, so after zero filtering most of them go to `msm_best`, not BLST. This is inferred from the source; it has not been profiled.
- Consequence (candidate, to test): if the 882.6 s parent receipt was produced with the 0.8.2 lock, its k20 MSMs also took the `msm_best` path, and the 0.8.3 BLST path would be a different baseline. The receipt's proofs version is not recorded, so this is unverified. Do not treat the recl9 node/decider runs or the parent receipt as a matched 0.8.3 baseline. Node 09's comment that the "checkout still has a size cutoff" is a checkout-vs-registry difference; the registry 0.8.2 has the cutoff and the registry 0.8.3 does not.

### 7. MSM and FFT counts for the parent (formula; the Sherlock parent's A, L, S, T, H, d are not recorded)
- MSM count per proof = 1 + A + 3L + S + T + 1 + H + 2 (section 2).
- Extended-domain transforms: the quotient needs coset FFTs of every advice, instance, fixed and permutation-product polynomial plus lookup and trash auxiliaries (`prover.rs:603-620`, `evaluation.rs` via `pk.fixed_cosets`). Node 01 (`loki-01:68`) gives 38 ordinary and 39 extended transforms for its assumed geometry (I=2, excluding keys and trash). That count is an ESTIMATE for a different geometry.

## Raw numbers table

| Quantity | Value | Source |
|---|---|---|
| Parent circuit k | 20 | D/samples/parent-proof.json observed.circuit_k |
| Parent proof / VK bytes | 6432 / 2189 (children: 3440 bytes each) | same |
| Parent setup_pk | 62.328385318 s | same, phases.setup_pk |
| Parent prove (incl. PK drop) | 882.589555363 s | same, phases.prove |
| Parent model phase | 5.687048131 s | same, phases.model |
| Parent native / root capture | 951.6142616319703 s / 954.127131041023 s | same, nested_wall_observations |
| Parent circuit rows, advice, lookups | unavailable | same, layers.parent |
| recl9 node4 rows / table rows | 958,095 / 1,032,465 | R/keys/node4/keygen.stats.json |
| recl9 node4 advice / lookups / perm cols / fixed | 15 / 5 / 18 / 45 (27 real fixed) | same, node01 loki-01:43 |
| recl9 node4 usable rows at k20 | 1,048,569 | same |
| recl9 node4 pk / prove / wall | 104.5 s / 867.3 s / 16:13.73 | R/logs/node-prove.log |
| recl9 node4 user / sys / CPU% | 2534.17 s / 81.31 s / 268% | same |
| recl9 node4 max RSS | 14,988,176 KB | same |
| recl9 node4 keygen VK | 622.8 s; RSS 4,502,488 KB | R/logs/node-keygen.log |
| recl9 decider rows / table rows | 841,682 / 1,032,209 | R/keys/decider/keygen.stats.json |
| recl9 decider pk / prove / wall | 46.9 s / 586.4 s / 10:34.85 | R/logs/decider-prove.log |
| recl9 decider user / CPU% / max RSS | 2486.95 s / 401% / 14,965,544 KB | same |
| recl9 decider keygen VK | 412.0 s | R/logs/decider-keygen.log |
| recl9 leaf k17 rows / table rows / keygen | 112,098 / 98,563 / 31.0 s | R/keys/leaf4/keygen.stats.json |
| Host cores / RAM | 20 / 90 GB | nproc; free -g |
| msm BLST cutoff (0.8.2 only) | coeffs.len() <= 2^19 | P2/src/poly/kzg/msm.rs:176 |

## Caveats
- The Sherlock parent receipt (882.6 s) and the recl9 node/decider logs both come from the Fenrir recl9 backend family. Neither records its proofs version. The 0.8.2 attribution is inferred from the recl9 lockfile, and the MSM-dispatch difference (section 6) is a candidate explanation, not a verified one.
- Parent row counts, phase count, challenge count, lookup count, permutation sets and degree are not recorded for the parent receipt. The barrier and MSM counts are formulas, not counts.
- The FFT Amdahl ceiling (about 4 to 5x) is inferred from code structure, not measured.
- Transcript point absorption's inversion cost is inferred from the affine conversion, not traced into blst.
- Synthesis cost is not separated from proving time in any recorded run. No synthesis timing exists for the parent.
- "268% CPU" and "401% CPU" are whole-process averages over wall time, not per-phase utilization.
- No builds, proofs, benchmarks or repo changes were made. Only local source, logs and result docs were read.
- Nothing above is a GPU speedup estimate; the node 01/09 speedup figures are ESTIMATES.
