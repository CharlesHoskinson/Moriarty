# GPU path of the wrap prover: results

**Verdict: the GPU path proves the run decider at k = 20 and k = 21, and the decider-shaped proxy at k = 19. Every GPU proof is byte-identical to the CPU proof and passes the library verifier and ledger 9 `VerifierKey::verify` (MEASURED). On the same degraded host the full GPU path is 2.56x (k = 20) and 2.85x (k = 21) faster than the CPU path (MEASURED). Against the quiet-host CPU baseline (111.8 s at k = 21) the full GPU path is about 1.35x faster, and the GPU path without MSM about 1.45x (ESTIMATE, inputs in section 2). The quotient numerator itself is 11x to 14x faster on the GPU; the CPU work that stays (advice synthesis, lookup and permutation products, h(X) and multiopen) now sets the prove time.**

## 1. Table

All rows MEASURED, one proof per cell (n = 1), RAYON_NUM_THREADS = 20, `GW_DET=1` (test seed 20261111; chip seed 20261111). Runs: `~/.cache/gpu-wrap/runs/chain-20261111T194102/{k19p,k20,k21}/`. The host was degraded (see section 2). "GPU" is the `all` config (MSM, NTT, coset and quotient on the GPU); the second GPU line per k is the `ntt,coset,quotient` config (MSM on the CPU). Phase seconds come from the timers in the vendored prover (`hooks::PHASE_SECONDS`). Peak RSS is VmHWM of the process, reset before each prove, including the resident proving key and SRS. GPU mem counts only mori-gpu allocations (MSM base tables, NTT scratch, quotient slots and cosets), not the sppark internal pools. The GPU is unified memory: device bytes are not part of RSS and add to it on the shared 120 GB. "Verifies" means the library verifier, the in-process ledger 9 `VerifierKey::verify` at the workspace pin `a2b30128`, and the negative control (one flipped public input rejected) all pass. Byte-identical means the same SHA-256 as the CPU proof of the same run.

| k | Phase | CPU s | GPU s | Speedup | Peak RSS CPU / GPU (GB) | GPU mem (GB) | Verifies | Byte-identical |
|---|---|---:|---:|---:|---|---:|---|---|
| 19 (proxy) | prove, all | 118.8 | 34.8 | 3.41x | 8.48 / 9.91 | 3.08 | yes | yes |
| 19 (proxy) | prove, ntt,coset,quotient | 118.8 | 45.1 | 2.63x | 8.48 / 11.27 | 3.13 | yes | yes |
| 19 (proxy) | quotient numerator | 58.1 | 4.0 | 14.5x | | | | |
| 19 (proxy) | advice / rest of trace / finalise | 12.6 / 31.3 / 16.5 | 7.3 / 14.5 / 8.1 | | | | | |
| 20 | prove, all | 243.3 | 95.0 | 2.56x | 15.83 / 18.28 | 5.88 | yes | yes |
| 20 | prove, ntt,coset,quotient | 243.3 | 131.6 | 1.85x | 15.83 / 18.52 | 5.98 | yes | yes |
| 20 | prove, msm only | 243.3 | 197.2 | 1.23x | 15.83 / 16.35 | 0.30 | yes | yes |
| 20 | quotient numerator | 101.6 | 8.4 | 12.1x | | | | |
| 20 | MSM, 45 calls | | 23.2 | | | | | |
| 20 | advice / rest of trace / finalise | 30.1 / 84.8 / 26.5 | 30.6 / 40.7 / 14.0 | | | | | |
| 21 | prove, all | 441.7 | 155.2 | 2.85x | 30.55 / 35.25 | 11.49 | yes, and standalone ledger 9 (below) | yes |
| 21 | prove, ntt,coset,quotient | 441.7 | 247.9 | 1.78x | 30.55 / 35.57 | 11.69 | yes | yes |
| 21 | quotient numerator | 191.8 | 17.4 | 11.0x | | | | |
| 21 | MSM, 45 calls | | 40.2 | | | | | |
| 21 | advice / rest of trace / finalise | 32.1 / 161.3 / 55.0 | 36.6 / 74.2 / 24.8 | | | | | |

Proof SHA-256 prefixes: k = 20 `1cb0edf55b34`, k = 21 `a51733814a40`. Keygen: k = 19 proxy 61.0 s, k = 20 102.4 s, k = 21 214.7 s (CPU, not accelerated). At k = 21 `vk_equals_bundle` is true: the proofs use the M-k21 decider key (SHA-256 `ef102478...b380`). The k = 21 GPU proof (`all`) also passes the unpatched standalone verifier `k21-ledger-verify` (ledger 9 checkout `f15fb9d`): `Ok(())`, 0.180 s first and 0.025 s repeat, 70 of 70 lane negatives and the flipped byte rejected.

The k = 19 row is a proxy. The run decider has 929,170 rows and does not fit k = 19. `PoseidonChain` (`tools/gpu-prover/src/proxy.rs`) has the decider's `ZkStdLibArch` (BLS12-381 and Poseidon chips, 4 pow2range columns), so the same gates, columns, lookups and degree. 22,000 Poseidon hashes give 440,001 rows (0.84 x 2^19). It is not a wrap circuit.

## 2. What the numbers mean

**Host state, MEASURED.** The CPU path in this session is 3.6x (k = 20) and 4.0x (k = 21) slower than M-k21 (68.12 s and 111.83 s). An A/B in one lock hold settles the cause (`~/.cache/gpu-wrap/runs/ab-20261111T203826/`). The M-k21 harness binary (`sherlock_gp`, the build that produced the baseline) proved the k = 20 decider in 233.7 s and 219.8 s. `wrap_prove` on the CPU path took 228.6 s and 238.2 s. The two builds are at parity, and the host is degraded. The heavy jobs run in `heavy.slice` at CPUWeight 30 while other lanes run at the default weight. systemd accounting shows the prover scopes got about 6 of 20 cores (k = 21 stage: 110 min CPU in 17.7 min wall). Same-run ratios are valid on this host. They overstate the speedup on a quiet host, because the GPU kernels are not slowed by CPU contention.

**Quiet-host projection, ESTIMATE.** Method: keep the measured GPU kernel seconds, scale the remaining CPU seconds by the quiet-to-degraded ratio of the CPU path (k = 21: 111.83 / 441.7 = 0.253; k = 20: 68.12 / 243.3 = 0.280).
- k = 21, all: kernels 40.2 (MSM) + 13.7 (quotient) + 4.8 (NTT) = 58.7 s; residual (155.2 - 58.7) x 0.253 = 24.4 s; total about 83 s against 111.8 s, **about 1.35x**.
- k = 21, ntt,coset,quotient: kernels 13.7 + 5.2 = 18.8 s; residual (247.9 - 18.8) x 0.253 = 58.0 s; total about 77 s, **about 1.45x**.
- k = 20, all: kernels 23.2 + 6.8 + 2.2 = 32.2 s; residual 62.8 x 0.280 = 17.6 s; about 50 s against 68.1 s, about 1.37x. k = 20, ntt,coset,quotient: about 43 s, about 1.57x.
- The uniform scaling is a simplification: the CPU phases do not all scale alike with core count. The figure needs a run on a quiet host to become MEASURED.

**MSM.** The GPU MSM does not pay on a quiet host. G1 micro-benchmark (`~/.cache/gpu-wrap/g1-bench.jsonl`, MEASURED, host load 41 to 44): 0.48 s at 2^20 and 0.75 s at 2^21 for random scalars. The quiet BLST figure in `research/sherlock-speed-replan-20261108/gpu.md` is 0.51 s at 2^20, so the ratio is about 1.06x, under the G3 gate (100 ms) and over the kill line (250 ms). The coordinator's floor estimate for this device is about 28 ms at 2^20, so sppark `msm_t` is far from the device limit here. On the real decider, 45 MSM calls took 622 s to 647 s with the old guard: range-checked columns repeat many small values below its n / 64 threshold, and sppark accumulates each bucket serially. The mask MSM(s + r) - MSM(r) (r a fixed splitmix64 vector independent of the witness; MSM(r) cached per base table and length) brings 45 calls to 23.2 s (k = 20) and 40.2 s (k = 21), with proof bytes unchanged. The best quiet-host configuration is therefore expected to be `ntt,coset,quotient` with MSM on the CPU, until the MSM is replaced.

**NTT and coset, MEASURED.** Bit-exact against `best_fft` for log_n 1 to 23. 79 NTT calls per proof take 2.2 to 2.7 s at k = 20 and 4.8 to 5.2 s at k = 21. Micro-benchmark: 0.034 s for 2^21 and 0.100 s for 2^23 on the GPU, against 0.49 s and 3.13 s on the CPU in the same loaded run.

**Quotient, MEASURED.** Max gate degree 5, so the extended domain is 4n (cs_degree 5, extended_k = k + 2 at each k). The k = 20 numerator has 2,269 gate calculations, 177 value slots, 5 lookups, 6 permutation sets and 39 resident cosets. A check run compared all 4,194,304 extended values of the GPU numerator with the CPU numerator: 0 differences. The kernel is a grid-stride graph interpreter with intermediates in global scratch (177 slots x 32 B x grid stride). All advice, instance, permutation-product, lookup and trash cosets are resident on the device at once (untiled), so device memory is 5.9 GB at k = 20 and 11.5 GB at k = 21. Fixed and sigma cosets and l0, l_last and l_active are read in place from host memory over unified memory; a discrete GPU would need copies. The coordinator estimates the k = 20 kernel moves about 0.91 TB through global scratch, with a floor of 0.34 s to 0.67 s for a fused kernel, against 6.6 s to 7.9 s measured. The stride sweep (`MORI_GPU_Q_STRIDE` 3072, 6144, 12288, to fit the scratch in the 24 MB L2) was queued and then cancelled for the A/B; NOT MEASURED.

**Byte identity.** Definition: same proving key, witness, test seed, `GW_CHIP_SEED`, and rayon pool size (20); lookup HashMap leftovers in sorted order (`DETERMINISTIC_LOOKUP`, test-only). The foreign ECC chip in midnight-circuits 7.3.0 draws an incomplete-addition point from `OsRng` on every synthesis, and that point is a witness cell. Without a fixed seed, two CPU proofs of one witness differ. This is why the earlier k = 20 check run reported `byte_identical_to_reference: false` while the proof verified. The vendored crate `tools/gpu-prover/vendor/midnight-circuits-7.3.0-seed` (registry archive SHA-256 `79ba2331...3448`, Apache-2.0, patch `patches/chip-seed.diff`, SHA-256 `129e288c...82af1b0`) takes the point from a seeded `StdRng` when `GW_CHIP_SEED` is set. Unset, it behaves as the registry crate. Seeded proofs are test-only (owner ZK rule); production keeps `OsRng` and fresh blinding.

**Code.**
- `crates/mori-gpu`: CUDA kernels, FFI and differential tests. Feature `cuda`, default off.
  - Vendored sppark subset: commit `08033103c14f`, Apache-2.0. nvcc 13.4, sm_121. blst 0.3.17 for the C source.
- `tools/gpu-prover`: a separate workspace with its own lockfile.
  - vendored midnight-proofs 0.8.3 plus `patches/gpu-hooks.diff` (557 inserted lines against a design target of under 100);
  - the circuits crate above;
  - the `wrap_prove` harness.
- The root workspace never sees the patched crates.

## 3. What remains for a production GPU wrap

1. **Fused quotient kernel.** Compile the gate graph to straight-line CUDA (NVRTC, keyed by the circuit's vk hash) and keep intermediates in registers. Tile the cosets so device memory stays bounded. Target: under 1 s at k = 21 (ESTIMATE from the coordinator's floor). Effort ESTIMATE: 2 to 4 days.
2. **MSM.** Replace sppark `msm_t` with a skew-robust MSM: sort digits, split heavy buckets, and batch the same-size commitments of one phase into one launch. Or evaluate the Yrrid design. Until then, keep MSM on the CPU in the default configuration. Effort ESTIMATE: 3 to 5 days.
3. **The CPU residual.**
   - Grand products (K4): permutation, lookup and trash products.
   - Evaluations and multiopen (K5).
   - Counting-sort lookup permutation (K6).
   - The rest of the trace is the largest phase after the quotient at every k (74 s at k = 21 on this host).
   - Advice synthesis (30 to 37 s here) stays on the CPU.
4. **Quiet-host measurement.**
   - Rerun the table on a quiet host.
   - Use reps >= 3 and report first-call and steady-state figures separately. Each cell here is n = 1, and `all` is the first GPU proof in its process, so it includes the MSM table builds and the mask MSMs.
   - Add per-prove CPU seconds (getrusage) to the record.
5. **Hardening.**
   - Fuzz the quotient ABI and the job builder against random constraint systems.
   - Add a check mode in CI that compares the GPU numerator with the CPU numerator.
   - Add a device-loss and fallback path test.
   - Extend `scripts/qa/check-profile-isolation.py` to `tools/gpu-prover`.
   - Add formal models and receipts per the OpenSpec process.
6. **Portability.**
   - The kernels read host pointers over unified memory; a discrete GPU needs explicit copies or managed memory.
   - The hooks are pinned to midnight-proofs 0.8.3. A version bump needs the patch re-applied and the check-mode diff rerun.
7. **Patch size.** Move the job builder out of the vendored crate behind a narrower trait, to approach the design target of under 100 lines.

## Log

- 2026-11-11 17:2x. Cause of `byte_identical_to_reference: false` in the k = 20 `cpu;quotient` check run found. `ForeignWeierstrassEccChip::new` (midnight-circuits 7.3.0, `src/ecc/foreign/weierstrass_chip.rs:1003`) samples its incomplete-addition point with `OsRng` on every synthesis, and `msm_randomness` assigns that point as a witness cell. Two syntheses of the same decider witness therefore give two different advice columns, so two CPU proofs also differ. The GPU path is not the cause: the quotient check in the same run found 0 differences in 4,194,304 extended-domain values. The k = 16 proxy was byte-identical because `PoseidonChain` builds the BLS12-381 chip but assigns no point. Fix (test-only): `vendor/midnight-circuits-7.3.0-seed`, a copy of the registry crate (SHA-256 79ba2331...3448, Apache-2.0) with one change: if `GW_CHIP_SEED` is set, the point comes from `StdRng::seed_from_u64(seed)`; otherwise `OsRng` as before. `wrap_prove` sets `GW_CHIP_SEED = GW_SEED` when `GW_DET=1`. Patch: `tools/gpu-prover/patches/chip-seed.diff`.
- 2026-11-11 17:50 to 18:50. Run `det-k20` (`~/.cache/gpu-wrap/runs/det-k20-20261111T173426/`), real decider at k = 20, `GW_DET=1` (chip seed 20261111), RAYON_NUM_THREADS=20, configs cpu; ntt,coset,quotient; all; cpu; ntt,coset,quotient; all; msm. MEASURED: all 7 proofs have one SHA-256 (`1cb0edf55b34...`), all pass library verify, ledger 9 `VerifierKey::verify` and the negative control, and `gpu_path_valid` is true with 0 fallbacks. Byte identity of the GPU path is therefore shown on the real decider. Timings are contaminated (host load average 17 to 44 from other lanes; M-k21 measured 68.1 s for the CPU decider prove on a quiet host): cpu 341.5 s and 314.8 s; ntt,coset,quotient 161.6 s and 175.3 s (quotient kernel 7.93 s and 6.68 s, 79 NTT calls 2.72 s and 2.51 s, device peak 5.68 to 5.98 GB, RSS 17.9 to 18.5 GB); all 710.0 s and 725.3 s; msm 828.4 s. The GPU MSM is a disaster on the real witness: 45 MSM calls take 622 to 647 s (about 14 s per call), against 0.48 s for a 2^20 MSM of random scalars. Cause: witness columns repeat many small values (range-checked limbs) below the n / 64 threshold of the `split_heavy` guard, and sppark accumulates each bucket serially. Fix under test: a witness-independent mask, MSM(s) = MSM(s + r) - MSM(r), r a fixed splitmix64 vector, MSM(r) cached per base table and length. s + r has uniform digits whatever the witness repeats; the group element is unchanged, so proof bytes are unchanged.
- 2026-11-11 20:10. Chain stage k20 (`~/.cache/gpu-wrap/runs/chain-20261111T194102/k20/`), MSM skew mask on, MEASURED, load average 16 to 24 (contaminated; quiet CPU reference is M-k21 68.1 s). All proofs byte-identical to the cpu reference and accepted by ledger 9. Phase seconds [advice, rest of trace, quotient numerator, finalise]: cpu 243.3 s total [30.1, 84.8, 101.6, 26.5]; all 95.0 s [30.6, 40.7, 8.4, 14.0], MSM 23.2 s over 45 calls; msm only 197.2 s [30.0, 51.2, 98.1, 16.9], MSM 21.7 s. The mask removed the 14 s per call skew (622 s down to 23 s). The quotient numerator moves from 101.6 s to 8.4 s (12x) on this host. Advice synthesis (30 s) stays on the CPU and is the next floor.
- 2026-11-11 20:00. Chain stage k20, last config ntt,coset,quotient (no MSM): 131.6 s, phase seconds [29.5, 71.5, 8.4, 22.3], byte-identical, ledger 9 accepts, load average 15.6. MEASURED, contaminated.
- 2026-11-11 20:04. Chain stage k21: keygen 214.7 s, `vk_equals_bundle` true, so the k = 21 proofs use the same key as the M-k21 baseline. MEASURED.
- 2026-11-11 20:34. Chain stage k21 (cpu 441.7 s, all 155.2 s, ntt,coset,quotient 247.9 s, all byte-identical, `a51733814a40`) and stage k19p (proxy 440,001 rows; cpu 118.8 s, all 34.8 s, ntt,coset,quotient 45.1 s, all byte-identical) done. Standalone ledger 9 verifier `f15fb9d` accepts the k = 21 GPU proof. MEASURED.
- 2026-11-11 21:17. A/B in one lock hold (`~/.cache/gpu-wrap/runs/ab-20261111T203826/`): M-k21 `sherlock_gp` decider prove at k = 20 took 233.7 s and 219.8 s; `wrap_prove` cpu took 228.6 s and 238.2 s. The harness is at parity with the baseline build; the host is degraded (heavy.slice CPUWeight 30, about 6 of 20 cores received). MEASURED. Stride sweep cancelled for this A/B.
