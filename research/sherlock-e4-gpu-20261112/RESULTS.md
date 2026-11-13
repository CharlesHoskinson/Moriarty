# E4: `L_Fr` prover speed on the GPU (results)

Plan: `PLAN.md` beside this file. Branch `e4-gpu` in three worktrees: Moriarty `~/Projects/Moriarty-wt/e4-gpu` (docs, `crates/mori-gpu`), stark-backend `~/.cache/m7-wrap/src/stark-backend-e4`, openvm `~/.cache/m7-wrap/src/openvm-e4`. Logs `~/.cache/e4-gpu/logs/` (`runs.log` has the start, end and load of every heavy job).

## Status

- MSM re-time: stopped on the owner update of 2026-11-12 (the existing benchmark is the upper bound); branch `msm-rust` unchanged.
- Step 1 (profile harness): done.
- Step 2 (CPU baseline and profile): done; the CPU prover was not deterministic (grind), fixed on `e4-gpu`.
- Step 3 increment 1 (GPU Midnight Poseidon for the Merkle trees): done, byte-identical, prove 824.4 s CPU against 228.6 s GPU (section 3).
- Step 3 increment 2 (fused GPU encode and hash, chunked grind): byte-identical; the timing run was memory-throttled (section 4).
- Step 3 increment 2b (streaming fused encode): byte-identical, prove **139.9 s** MEASURED on the GPU path, exclusive slot (section 5). This is the gate number.
- Step 4 (gate verdict): section 6. `L_Fr` on the GPU fails the call-time budget by a wide margin.
- CPU baseline on the current tree: 896.5 s MEASURED in an exclusive slot (rerun `e4-abfinal2` after the tmpfs leftover was removed), but at host load 28 to 36 from work outside heavy-slot; a quiet host was not available (section 7.1).
- Step 3 tuning (owner 21:00, tune first): the Poseidon kernel knobs give no gain; the GPU fractional sumcheck saves about 7 s. **Tuned `L_Fr` prove 105.0 s** MEASURED (median of 3, 99.6 to 105.8 s), byte-identical (section 7).
- Owner options to close the remaining gap: section 8 (recommended: option A).

## 1. Profile harness

`crates/sherlock-lfr/tests/e4_profile.rs` (openvm `e4-gpu`), ignored test `e4_profile_lfr_prove`: builds the I10 tape from the real `L` proof (fixture `~/.cache/e2-lfr/fixture`), keygen at `lfr_params()` (or loads the key from `$E4_PK_CACHE`, bitcode), one honest `engine.prove`, the stock verifier, proof bytes to `$E4_PROOF_OUT`, peak RSS (`VmHWM`), and the backend tracing spans (`setup_tracing`). The CPU baseline runs a frozen binary of the unchanged tree (openvm c4bd312, stark-backend 5f698b5): `~/.cache/e4-gpu/bin/e4_profile-cpu-c4bd312`.

## 2. CPU baseline (MEASURED)

Run 1, 2026-11-12 15:20 to 15:36, heavy lock held, `RAYON_NUM_THREADS=20` on 20 cores. Host load average 7.3 at the start and 20.7 at the end (other agents' lanes outside the lock: two Gemini lanes and their tools). Not a quiet host by load; it is the quietest window available today. Log `~/.cache/e4-gpu/logs/cpu-base1.log`.

| Quantity | Value |
|---|---|
| Keygen | 188.8 s (175 s of it is the Merkle tree of the 10-column program table) |
| `engine.prove` | 700.2 s |
| Verify | 1.87 s |
| Proof | 2,516,530 bytes (equal to the E2 I10 size), 2 commitments |
| Peak RSS | 28,061 MiB (10,106 MiB after keygen) |

Stage profile of the prove (tracing spans, seconds of 700):

| Stage | s | share |
|---|---:|---:|
| main trace commit: RS encode and Merkle | 603 | 86.1 % |
| of which leaf (row) hashing, Midnight Poseidon | 537 | 76.7 % |
| of which digest layers (Poseidon compress) | 31.3 | 4.5 % |
| of which batch DFT (LDE, 2^23 rows, 33 columns) | 18.3 | 2.6 % |
| of which transpose and iDFT of the `l_skip` chunks | 14.7 | 2.1 % |
| LogUp GKR and zerocheck | 26.0 | 3.7 % |
| stacked opening reduction | 4.3 | 0.6 % |
| WHIR opening | 64.8 | 9.3 % |
| of which WHIR round DFT and Merkle | about 52 | 7 % |
| of which proof-of-work grinding (Poseidon) | about 11.5 | 1.6 % |

Run 2, 2026-11-12 17:36 to 17:49, the same binary with the cached proving key (`~/.cache/e4-gpu/pk.bitcode`, loaded in 27.1 s), host load average 33 (other agents' jobs ran next to it, so this is not a timing): prove 823.3 s, row hashing 617 s, peak RSS 27,210 MiB, proof 2,516,530 bytes, accepted (MEASURED).

**The CPU prover is not deterministic at 20 threads** (MEASURED): the two proofs differ from byte 53,546 to the end (2,392,741 of 2,516,530 bytes differ; SHA-256 `52a4d34a...` and `bd7c5e95...`), while the first 53,546 bytes (the commitments) are equal, so the cached key round-trips. Cause: `FiatShamirTranscript::grind` (stark-backend `transcript/traits.rs`) searches the proof-of-work witness with rayon `find_any`, which returns whichever thread finds a witness first; every later challenge depends on it. Fix on `e4-gpu`: `find_first` (the smallest witness), so a proof is a function of its inputs and the byte-identity gate is meaningful. The CPU reference for the GPU increments is therefore the `e4-gpu` tree with the GPU off (`MORI_GPU=off`), not the c4bd312 binary.

Reading: Midnight Poseidon over Fr is about 600 s of the 700 s (leaf hashes, digest layers, WHIR Merkle trees and grinding). The NTT work is about 35 s. So the first GPU increment is the Poseidon kernel for Merkle hashing; the NTT comes after it (DERIVED from the table).

## 3. GPU increment 1: Midnight Poseidon for the Merkle trees (MEASURED)

Code: `crates/mori-gpu/cuda/poseidon.cuh` and `src/poseidon.rs` (Moriarty `e4-gpu`): the permutation on the sppark `fr_t` (8 x 32-bit Montgomery limbs, K0), constants uploaded from midnight-circuits `PoseidonField` at run time (no transcribed constants), two modes: the plain rounds (mode 0) and sparse partial rounds (mode 1, default; the Poseidon paper appendix B factorisation, table built on the host in stark-sdk `gpu::sparse_partial`). Hooks: `MerkleHasher::{has_batch, hash_rows, digest_layers}` (stark-backend, default `None`), called by cpu-backend `rs_encode_and_merkle_cpu`, `build_digest_layers`, the WHIR tree `build_ef_merkle_tree_packed` and the reference `MerkleTree::new` (keygen); `FrMerkleHasher` overrides them under the stark-sdk feature `gpu` (sherlock-lfr feature `gpu`). `MORI_GPU=off` at run time takes the CPU path in the same binary.

Unit test `tests/e4_gpu_hash.rs` (shared slot, host load 33): both modes equal `hash_slice` on rows of width 1, 2, 3, 10, 33 and 48 (random values plus 0 and p - 1) and equal the stark-backend digest-layer order at rows per query 1, 4 and 16. Rate at 2^20 permutations including host copies, loaded host: mode 0 1.33 to 1.71 M/s, mode 1 1.72 to 2.14 M/s; one CPU thread 11,356/s.

Full `L_Fr` prove, A/B in one exclusive slot (`heavy-slot --exclusive`, 2026-11-12 18:22 to 18:41; load average 28 at the start falling to 19.5, of which about 13 cores were this prover; no other heavy job; GPU: only the two browser GPU processes, SM clock 656 to 747 MHz under load). Log `~/.cache/e4-gpu/logs/ab-gpu1.log`, binary `~/.cache/e4-gpu/bin/e4_profile-gpu1`.

| Path | prove | row hash | digest layers | WHIR trees | grind | peak RSS | proof SHA-256 |
|---|---:|---:|---:|---:|---:|---:|---|
| CPU reference (`MORI_GPU=off`, `find_first` grind) | 824.4 s | 579 s | 33.4 s | 23.9 s | 68.7 s | 27,271 MiB | `c17dadf7...3554987` |
| GPU hashing | 228.6 s | 57.2 s | 3.25 s | 2.6 s | 97.0 s | 27,212 MiB | `c17dadf7...3554987` |

The two proofs are byte-identical (2,516,530 bytes, accepted). Hashing on the GPU takes row hashing from 579 s to 57.2 s (10.1 times) and digest layers from 33.4 s to 3.25 s. The rest of the GPU-path prove: grinding 97 s, batch DFT 18.7 s, transpose 12.5 s, LogUp GKR and zerocheck 23.6 s, other WHIR work about 10 s.

Grinding cost: a single `find_first` over the whole candidate range costs 68.7 s (CPU run) and 97.0 s (GPU run) against 11.5 s for `find_any` in baseline run 1, because rayon's halves far from the answer keep scanning. The fix (next build) searches ordered chunks of 2^16 candidates with `find_first` in each; it returns the same smallest witness, so the proof bytes do not change.

## 4. GPU increment 2: fused Reed-Solomon encode and leaf hash; chunked grind (MEASURED, correctness only)

Code: `mori_pos_encode_hash` (`cuda/mori_gpu.cu`) and `poseidon::encode_hash_rows`: the PLE coefficients come from the CPU (`eval_to_coeff_cpu`, about 1 s); each of the 33 columns is copied to the device, zero-padded to 2^23 and transformed with the sppark forward NTT (natural order, standard root, the same values as `Radix2DitParallel::dft_batch`); the rows are hashed from the column-major device matrix (coalesced reads), and a device transpose writes the row-major codeword back in 256 MiB chunks. Hook `MerkleHasher::encode_hash_rows` (default `None`), called first in `rs_encode_and_merkle_cpu`; `E4_GPU_ENCODE=0` keeps increment 1 only. The grind now searches ordered chunks of 2^16 candidates (section 3).

Unit test `e4_gpu_encode_equals_cpu` (shared slot): codeword and row hashes equal the CPU DFT and `hash_slice` at (width, height, blowup) = (33, 2^11, 8), (10, 2^12, 8), (3, 2^10, 4), (1, 2^12, 2); the increment 1 tests pass again.

Full prove in an exclusive slot (2026-11-12 18:53 to 19:06; load 5 at the start; log `~/.cache/e4-gpu/logs/ab-gpu2.log`): proof SHA-256 `c17dadf7...3554987`, **byte-identical to the CPU reference of section 3**, accepted. Stage times: main commit 87.7 s (encode and hash 81.4 s, digest layers 3.7 s), grinding 4.8 s in total (was 97 s), WHIR 15.0 s, stacked reduction 4.5 s. The prove took 697.7 s because the LogUp fractional sumcheck took 570 s (10 to 25 s in the other runs): the process was throttled by the `heavy.slice` memory limit (`MemoryHigh` 64 GiB; the busy thread sat in `__mem_cgroup_handle_over_high`, system time twice user time). The slice held 30.5 GiB of tmpfs (`/tmp/bounded-depth-final-2848992`, 31 GB, another lane's leftover) plus this job's 32 GiB (peak RSS 29,397 MiB plus 8.9 GiB of device buffers for the 33 columns). So this run is a correctness result, not a timing.

## 5. GPU increment 2b: streaming fused encode, low device memory (MEASURED)

The resident path of section 4 holds all 33 codeword columns on the device (8.9 GiB). The streaming path keeps only the sponge states of the 2^23 rows (805 MiB) and two column buffers on the device: per column pair it copies and transforms two columns, absorbs them, permutes, and writes the two codeword columns straight into the host row-major matrix (this GPU reads and writes pageable host memory, `cudaDevAttrPageableMemoryAccess` = 1; 32-bit stores, since a Rust `Vec<Fq>` is only 8-byte aligned and a vector store gave `cudaErrorMisalignedAddress`). It is the default where the attribute is set; `MORI_POS_STREAM=0` selects the resident path. The encode test passes on both paths.

Full prove, exclusive slot, 2026-11-12 19:40 to 19:43, load average 15 at the start (decaying from the previous jobs) and 10.5 at the end, no other heavy job; GPU SM clock 754 MHz (throttle reason 0x400) for the whole run; log `~/.cache/e4-gpu/logs/ab-gpu3.log`, binary `~/.cache/e4-gpu/bin/e4_profile-gpu3`:

| Stage | GPU path (s) |
|---|---:|
| `engine.prove` | **139.9** |
| main trace commit (PLE coefficients 0.9, encode and row hash on the GPU 70.6, digest layers on the GPU 3.7) | 76.9 |
| LogUp GKR (fractional sumcheck 28.9) and zerocheck (5.2), CPU | 42.8 |
| stacked opening reduction, CPU | 4.5 |
| WHIR (round DFT and GPU trees 7.0, grinding 4.7, rest 3.9) | 15.6 |

Proof SHA-256 `c17dadf7...3554987`, byte-identical to the CPU reference, accepted, 2,516,530 bytes. Peak RSS 23,738 MiB (CPU path 27,271 MiB); system memory in use rose by 22.4 GiB over the run (61.7 to 84.1 GiB, sampled every 2 s), which includes the device buffers.

The encode stage is the Midnight Poseidon permutation count: 2^23 rows x 17 permutations = 1.43 x 10^8 permutations in about 70 s, about 2.0 M permutations per second at 754 MHz (DERIVED), the rate of the section 3 unit test.

## 6. Gate verdict and open items

**Measured `L_Fr` prove on the GPU path:** 139.9 s, peak host RSS 23,738 MiB, system memory in use +22.4 GiB over the run (host and device share memory on this machine), all MEASURED (section 5, exclusive slot, load 15 falling to 10.5, GPU at 754 MHz). The device memory that the streaming encode holds is the sponge states (805 MiB) plus two column buffers (DERIVED from the sizes; no separate device-memory counter was read).

**Budget** (PROPOSAL 6.3, ESTIMATE): prove stages 6.31 / 8.90 / 16.20 s and warm call 11.1 / 19.0 / 45.0 s (lo / mid / hi), with no `L_Fr` stage. `L_Fr` adds to the call:

| Quantity | Value |
|---|---|
| Mid warm call plus `L_Fr` | 19.0 + 139.9 = 158.9 s (DERIVED) |
| `L_Fr` as a multiple of the mid warm call | 7.4x (DERIVED) |
| `L_Fr` as a multiple of the mid prove-stage figure 8.90 s | 15.7x (DERIVED) |

**Verdict: FAIL.** The GPU path is 5x faster than the CPU path (139.9 s against 700 to 824 s), but `L_Fr` is still about 7 times the whole mid warm call. With these kernels, `L_Fr` is not a per-call stage.

**CPU baselines, as measured:** 700.2 s (run 1, old heavy lock, load 7.3 to 20.7, rayon `find_any` grind) and 824.4 s (exclusive slot, load 28 falling to 19.5, unchunked `find_first` grind 68.7 s). Neither is a quiet baseline of the current tree. With the grind at the chunked cost (about 11.5 s, run 1), the 824.4 s run gives about 767 s (DERIVED). The CPU-to-GPU ratio is about 5x on every pairing (DERIVED).

**Where the 139.9 s goes and the next levers** (section 5 profile): (1) the Midnight Poseidon kernel, about 74 s of encode, row hash and digest at about 2.0 M permutations/s with the GPU throttled to 754 MHz. Next tries are block size (an env knob, staged in `~/.cache/e4-gpu/mori_gpu.cu.tune`, not built) and squaring in the x^5 S-box (`poseidon.cuh.tune`). (2) The LogUp GKR fractional sumcheck, 28.9 s on the CPU, which can move to the GPU. (3) WHIR, 15.6 s. Even if all three fall to zero, the Poseidon floor at today's kernel rate (about 74 s) stays several times the call budget (DERIVED). Getting within budget needs a much cheaper Merkle hash for `L_Fr` or a much smaller `L_Fr` trace, not only kernel tuning.

**Incidents for the coordinator:** (a) the CPU prover was not deterministic (rayon `find_any` in the grind), fixed on stark-backend `e4-gpu` 4199bc1 with ordered chunks and `find_first`. (b) heavy.slice has MemoryHigh 64 GiB, and another lane's 31 GB tmpfs leftover `/tmp/bounded-depth-final-2848992` counts as shmem in it. This throttled the section 4 run (fractional sumcheck 570 s). It is not deleted, because it belongs to another lane.

**Pickup for `e4-abfinal`** (systemd user unit, `~/.cache/e4-gpu/ab.sh final`). The exclusive slot came free at 20:10. The CPU reference (`MORI_GPU=off`) then ran at about 0.7 cores for 21 minutes: heavy.slice was at 69.4 GB against its 68.7 GB MemoryHigh, with 32.7 GB of shmem from `/tmp/bounded-depth-final-2848992`. The prove thread sat in `__mem_cgroup_handle_over_high`, the same throttle as section 4. The agent stopped the unit at 20:31 with no timing. To rerun: first remove that tmpfs leftover (owner or its lane), then run `systemd-run --user --collect --unit e4-abfinal ~/.cache/e4-gpu/ab.sh final`. The binary is `~/.cache/e4-gpu/bin/e4_profile-final` (the tree of section 5). Logs go to `~/.cache/e4-gpu/logs/ab-final.log` and `logs/mem-final.log`, and proofs to `~/.cache/e4-gpu/proofs/{cpu,gpu}-final.bin`. Both SHA-256 values must equal `c17dadf7a2a65eaa1785f219f10caf9900f3bd2e69f2ee0a89bb7315a3554987`. Its CPU time is the quiet baseline of the current tree.

## 7. Tuning: Poseidon kernel, GPU fractional sumcheck, final A/B (MEASURED)

### 7.1 Final A/B of the section 5 binary

Unit `e4-abfinal2`, exclusive slot 21:56 to 22:15, binary `~/.cache/e4-gpu/bin/e4_profile-final`, log `~/.cache/e4-gpu/logs/ab-final.log`. Host load average 28 to 36 for the whole slot: other lanes' cargo and Lean builds run outside heavy-slot, so an exclusive slot does not give a quiet host.

| Path | Prove | Main commit | Fractional sumcheck | WHIR | Peak RSS | Proof SHA-256 |
|---|---:|---:|---:|---:|---:|---|
| CPU (`MORI_GPU=off`) | 896.5 s | 802 s (row hash 728) | 12.0 s | 62.3 s | 27,273 MiB | `c17dadf7...3554987` |
| GPU | 123.0 s | 68.3 s (encode and hash 60.0) | 11.0 s | 23.9 s | 23,830 MiB | `c17dadf7...3554987` |

The fractional sumcheck took 11 to 12 s here against 28.9 s in section 5, so the section 5 run was most likely also slowed by the heavy.slice memory throttle of section 6 (the tmpfs leftover was present then).

### 7.2 Poseidon kernel knobs

The kernels take `MORI_POS_BLOCK` (threads per block, default 128) and `MORI_POS_SQR=1` (the x^5 S-box with `sqr()`; same values). The equality test `e4_gpu_hash` passes at sqr and block sizes 64 and 256. Throughput, test `e4_gpu_poseidon_rate` (2^20 permutations including copies), exclusive slot 22:54 to 23:17, 3 interleaved rounds of 3 reps per knob, load 7.6 to 23.6 (log `~/.cache/e4-gpu/logs/rate1.log`):

| Knob | Mean of 9 reps (M permutations/s) |
|---|---:|
| block 128 (default) | 2.34 |
| sqr S-box | 2.30 |
| block 64 | 2.28 |
| block 256 | 2.22 |
| block 512 | 2.40 |

Every rep lies between 1.80 and 2.95, so the spread inside a knob (about 0.35) is larger than any difference between knobs: no knob is a measurable gain. The defaults stay. The GPU stays at 754 MHz SM clock (maximum 3,003 MHz, event reason 0x400) at 96 percent utilization in every run; the kernel is bound by that clock, not by its launch shape. The cause of the clock cap was not investigated (no system change without the owner).

### 7.3 GPU fractional sumcheck

mori-gpu `frac` (`crates/mori-gpu/cuda/frac.cuh`, `src/frac.rs`): the segment tree layer by layer, and per GKR layer the round polynomial at 1, 2, 3 (block sums, then one reduction block) and the fold, with the transcript kept on the host through a callback. Hook: stark-backend `StarkProtocolConfig::frac_gkr_accel` (default `None`; trait `FracGkrAccel`), implemented by stark-sdk `gpu::FracGpu` for layers of at least 2^12 points; `E4_GPU_FRAC=0` takes the CPU path. Test `e4_gpu_frac`: proof, point and transcript state equal to the CPU at 2^12, 2^13 and 2^17 leaves.

Full prove, exclusive slot 23:36 to 23:53, interleaved, load 9.4 to 23.9 (log `~/.cache/e4-gpu/logs/abt-t2.log`), every proof SHA-256 `c17dadf7...3554987`:

| Run | GPU fractional sumcheck (tuned) | CPU fractional sumcheck |
|---|---:|---:|
| 1 | 105.78 s (stage 2.33 s) | 109.49 s (stage 10.4 s) |
| 2 | 99.64 s (stage 2.93 s) | 118.76 s (stage 10.2 s) |
| 3 | 105.02 s (stage 3.16 s) | 110.62 s (stage 9.84 s) |
| Median | **105.0 s** | 110.6 s |

**Tuned `L_Fr` prove on the GPU path: 105.0 s** (median of 3; 99.6 to 105.8 s), MEASURED. Peak RSS 23,265 to 23,533 MiB; system memory in use rose 21.7 to 25.9 GiB per run (host and GPU share memory). Stages of run 3: encode and leaf hash 55.6 s (GPU), digest layers about 3.4 s (GPU), LogUp GKR and zerocheck 19.0 s (fractional sumcheck 3.2 s on the GPU, the rest on the CPU), stacked reduction 5.0 s, WHIR 19.4 s (CPU apart from its Merkle trees).

Against the budget (PROPOSAL 6.3, ESTIMATE): 105.0 s is 5.5x the 19.0 s mid warm call and 11.8x the 8.90 s mid prove-stage figure; the mid warm call with `L_Fr` becomes about 124 s (DERIVED). **The gate stays FAIL.**

## 8. Owner options to close the remaining gap

No option below reaches the 19.0 s warm call; kernel tuning is exhausted at the 754 MHz clock. Each option is a decision for the owner; nothing here is applied. Wrap rows: E5 final circuit 1,436,529 rows against the k = 21 gate of 1,782,573 (MEASURED, `~/Projects/Moriarty-wt/e5-wrap/research/sherlock-e5-wrap-20261112/RESULTS.md` section 6g); per-query costs from its section 8 model (about 9,600 rows per round-0 query plus 688 of `mu` batching, about 1,010 per round 1 and 2 query, DERIVED there).

**Option A (recommended): relax the target; `L_Fr` is an added stage of about 105 s per call.**
- Prove time 105.0 s, MEASURED (section 7.3); mid warm call about 124 s, DERIVED (19.0 + 105.0).
- D3: no change. Keys: none. Wrap: none. Ledger: none.
- Further engineering inside the design, no key change: the CPU stages left (WHIR about 15 to 19 s, GKR input and zerocheck about 12 to 16 s, stacked reduction about 5 s) to the GPU would give about 65 to 75 s (ESTIMATE). If the GPU ran at its 3,003 MHz maximum instead of 754 MHz, the Poseidon stage (about 59 s) could fall up to 4x (ESTIMATE, not tested; the cause of the cap is unknown).
- Why recommended: it keeps every hard rule, every key and the wrap headroom, and options B and D do not reach the budget either.

**Option B: smaller `L_Fr` trace, log blowup 3 to 2** (queries 120/85/82).
- Prove 96.2 s MEASURED in the same slot as a 138.6 s run at log blowup 3 (section 7, run `abt-t1`, load 26 to 36), so about 31 percent less. At the section 7.3 conditions about 72 to 80 s (ESTIMATE: the encode stage scales with the codeword length, 70.5 to 41.5 s in that slot). Proof 3,016,786 bytes against 2,516,530 (MEASURED). Peak RSS 16,782 MiB. Stock verifier accepts; there is no CPU reference proof at this set.
- D3: the query terms of 120/85/82 are at least 101 bits (M7 RESULTS 2b.2, the D3 calculator at the Fr field, MEASURED); the field-size terms stay above 200 bits; no PoW change.
- Keys: the `L_Fr` key regenerates (log blowup and query vector are key constants) and the wrap key regenerates (its circuit has more queries). Ledger: none.
- Wrap: about +223,000 rows (22 more round-0 queries at about 10,300 rows, one path level less, 2 more round 1 and 2 queries; ESTIMATE), so about 1.66 M of 1,782,573 (93 percent) and about 120,000 rows of headroom left for E3's public-value carrier.
- Log blowup 1 (196/89/82) proves in 86.2 s (MEASURED, proof 4,763,762 bytes) but adds about 1.0 M wrap rows (ESTIMATE) and does not fit k = 21, so it is excluded by the never-k = 22 rule.

**Option C: a cheaper Merkle hash for `L_Fr`** (for example Poseidon2 over Fr, width 3).
- Prover: Poseidon is about 59 s of the 105.0 s. A permutation about 1.5x cheaper (ESTIMATE, from the multiplication counts of the sparse Midnight rounds against Poseidon2's internal rounds) saves about 20 s, to about 85 s (ESTIMATE).
- D3: no term moves. The calculator has no hash term; Merkle binding rests on the hash's collision resistance, a premise outside D3 (the Fiat-Shamir term of SWIRL Cor 4.2.6 uses its kappa). The new hash would need its own security premise.
- Keys: the `L_Fr` key and its hash tag (OD-18) change, and the wrap key changes. The wrap must check the new hash in circuit. `ZkStdLibArch` (midnight-zk-stdlib 2.4.0) has one algebraic hash, Midnight Poseidon; its other hashes are bit-oriented (SHA-2, SHA-3, Keccak, Blake2b) and cost far more rows (NOT MEASURED). A Poseidon2 chip takes the wrap out of the stdlib architecture, and the unchanged ledger 9.1.0.1 verifier then cannot verify it (M7 verdict). So this breaks the hard rule "ledger 9.1.0.1 verifier unchanged".
- Not recommended.

**Option D: options A and B together.** About 72 to 80 s now (ESTIMATE), about 50 to 60 s with the engineering of option A (ESTIMATE), at the key and wrap-headroom cost of option B. Still about 3x the mid warm call.

