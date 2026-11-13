# E4: `L_Fr` prover speed on the GPU (results)

Plan: `PLAN.md` beside this file. Branch `e4-gpu` in three worktrees: Moriarty `~/Projects/Moriarty-wt/e4-gpu` (docs, `crates/mori-gpu`), stark-backend `~/.cache/m7-wrap/src/stark-backend-e4`, openvm `~/.cache/m7-wrap/src/openvm-e4`. Logs `~/.cache/e4-gpu/logs/` (`runs.log` has the start, end and load of every heavy job).

## Status

- MSM re-time: stopped on the owner update of 2026-11-12 (the existing benchmark is the upper bound); branch `msm-rust` unchanged.
- Step 1 (profile harness): done.
- Step 2 (CPU baseline and profile): done; the CPU prover was not deterministic (grind), fixed on `e4-gpu`.
- Step 3 increment 1 (GPU Midnight Poseidon for the Merkle trees): done, byte-identical, prove 824.4 s CPU against 228.6 s GPU (section 3).
- Step 3 increment 2 (fused GPU encode and hash, chunked grind): byte-identical; the timing run was memory-throttled (section 4).

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
