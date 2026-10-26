# prior-profile — local source snapshot
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/nodes/loki-01-prover-profile.md
> Collected: 2026-10-26
> Published: Unknown

---
title: "Midnight reference prover: phase profile and acceleration boundaries"
project: loki
node: 01
author: gpt-6.1-sol
date: 2026-10-24
tags: [prover, profiling, recursion, halo2, kzg, gpu, memory]
sources:
  - https://github.com/midnightntwrk/midnight-zk/tree/31435e9
  - https://github.com/midnightntwrk/midnight-ledger
  - https://github.com/ingonyama-zk/icicle
  - https://github.com/supranational/sppark
  - https://eprint.iacr.org/2022/1321
  - https://eprint.iacr.org/2024/973
  - https://www.zprize.io/blog/announcing-zprize-results
  - https://www.zprize.io/blog/announcing-the-2023-zprize-winners
  - https://zcash.github.io/halo2/design/implementation/proofs.html
  - https://dev.ingonyama.com/2.8.0/icicle/primitives/ntt
---

## Question

PROPOSAL — Identify the reference prover's computational and memory costs, then define acceleration boundaries that preserve acceptance by the existing Midnight ledger-9 verifier.

CODE — Citation prefixes below expand to these local paths: `R = /home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f`; `P = R/midnight-proofs-0.8.3/src`; `C = R/midnight-curves-0.3.1/src`; `Z = R/midnight-zk-stdlib-2.3.5/src`; `M = R/midnight-circuits-7.2.4/src`; `F = /home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9`; `L = /home/charles/repos/midnight-ledger`; `N = /home/charles/repos/midnight-node`. References use `path:line` under these explicitly defined roots.

## Summary

- CODE — `create_proof` separates trace construction from finalisation. Witness synthesis, commitment MSMs, lookup/permutation products, extended-domain constraint evaluation, quotient commitments and KZG multiopening all contribute; there is no single recursive pairing performed by the host prover that explains the entire cost ([P/plonk/prover.rs:371](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:371), [P/plonk/prover.rs:267](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:267)).
- CODE — The deployed dependency graph pins 0.8.3, which uses sorted lookups. Checkout `31435e9` instead contains LogUp and additional parallel scheduling; substituting it requires compatibility investigation ([/home/charles/repos/Moriarty-factory/crates/mori-backend/Cargo.toml:14](/home/charles/repos/Moriarty-factory/crates/mori-backend/Cargo.toml:14); [/home/charles/repos/midnight-zk/proofs/src/plonk/prover.rs:174](/home/charles/repos/midnight-zk/proofs/src/plonk/prover.rs:174)).
- CODE — BLS12-381 commitments already dispatch to BLST Pippenger. Rayon handles FFT butterflies and quotient evaluation, while sorting, scans, synthesis and several orchestration loops remain serial ([P/poly/kzg/msm.rs:176](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:176); [C/fft.rs:46](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/fft.rs:46); [P/plonk/permutation/prover.rs:143](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/permutation/prover.rs:143)).
- ESTIMATE — At k20, each scalar polynomial occupies `32 × 2^20 = 32 MiB`; simultaneously retained ordinary and extended polynomials readily explain roughly 12–15 GiB. This is an allocation model, not an attribution of the reported peak.
- PROPOSAL — Measure phase boundaries and preserve decoded keys first; introduce exact BLS12-381 MSM acceleration next, then device-resident FFT/quotient arithmetic. Require unchanged SRS, VK, transcript and proof encoding throughout.

## Findings

### Version and evidence boundaries

SOURCE — Midnight describes `proofs` as PLONK with KZG commitments and identifies its Halo2 ancestry. This establishes the relevant architecture, not interoperability with arbitrary Halo2 provers. [Repository](https://github.com/midnightntwrk/midnight-zk/tree/31435e9); capture: [midnight-zk.md](../raw/01/midnight-zk.md).

CODE — The local 0.8.3 archive SHA-256 equals the lockfile checksum `19b2e4b1acb581ad741b0cb7c0e9dbdf51142f4e9518886f25fcff40a6702dcc`; its Rust files match the extracted cache. Package provenance points to commit `98d1e964…`, whereas the requested checkout is `31435e9` ([R/midnight-proofs-0.8.3/.cargo_vcs_info.json:3](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/.cargo_vcs_info.json:3); [/home/charles/repos/Moriarty-factory/Cargo.lock:2030](/home/charles/repos/Moriarty-factory/Cargo.lock:2030)). The checkout's manifest even declares 0.8.0; a version string alone cannot identify its algorithms ([/home/charles/repos/midnight-zk/proofs/Cargo.toml:3](/home/charles/repos/midnight-zk/proofs/Cargo.toml:3)).

CODE — Historical four-child recursion artifacts record 15 advice columns, five lookups, 18 equality-enabled columns, and 1,032,465 table rows. Their `fixed_columns=45` includes permutation commitments: actual fixed/selector/table columns are `45−18=27` ([F/keys/node4/keygen.stats.json:4](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/keys/node4/keygen.stats.json:4); [P/dev/cost_model.rs:145](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/dev/cost_model.rs:145), [P/dev/cost_model.rs:244](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/dev/cost_model.rs:244)). This record is a related workload, not the owner's exact Sherlock run.

CODE — Sherlock selects the BLS/Poseidon architecture with four range-check columns ([F/src/sherlock_link.rs:440](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:440)). Its BLS base-field emulation uses seven 56-bit limbs, so synthesis includes foreign-field arithmetic and range-check witnesses ([M/field/foreign/params.rs:278](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/field/foreign/params.rs:278)). Stdlib advice allocation takes the maximum chip width, sharing columns rather than summing every chip's width ([Z/lib.rs:492](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:492)).

CODE — That historical artifact records VK generation 622.827 s, PK generation 104.463 s, proving 867.280 s and peak RSS 14,988,176 KiB. These differ from the owner's supplied >500 s attempt/~12 GiB/~120 s VK figures and must remain separate observations ([F/keys/node4/keygen.stats.json:19](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/keys/node4/keygen.stats.json:19); [F/out/node-b1/w.stats.json:4](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/out/node-b1/w.stats.json:4)).

OPEN QUESTION — Which exact binary, circuit, thread configuration and timer boundaries produced the owner's figures? A >500 s unfinished attempt is a lower bound, not a completed 500 s proof. No builds, proofs or benchmarks were run for this node.

### End-to-end phase trace

CODE — Let `n=2^k`, advice count `A`, fixed count `F_c`, instance count `I`, equality columns `P_c`, lookups `L_c`, constraint degree `d`, permutation sets `S=ceil(P_c/(d−2))`, extended length `m=qn`, and `q=2^ceil(log2(d−1))`. The domain computes precisely this power-of-two extension ([P/poly/domain.rs:37](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/domain.rs:37); [P/plonk/permutation/prover.rs:62](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/permutation/prover.rs:62)).

| Classification | Phase and work | Cost and source location |
|---|---|---|
| CODE | Configure, hash VK, format/pad instances; synthesise advice per circuit phase; resolve rational assignments; blind unused rows. | Synthesis cost follows circuit operations, plus dense `O(An)` storage/conversion; [P/plonk/prover.rs:98](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:98), [P/plonk/prover.rs:501](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:501), [P/poly/mod.rs:287](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/mod.rs:287). |
| CODE | Commit advice in Lagrange basis, then derive challenges. | `A` length-n MSMs across phases; commitments traverse columns sequentially; [P/plonk/prover.rs:568](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:568). |
| CODE | Compress lookup expressions with theta; sort input, build table multiplicities, construct permuted input/table; commit both. | Comparison-sort `O(L_c n log n)`, expected hash work `O(L_c n)`, `2L_c` MSMs and iFFTs; [P/plonk/lookup/prover.rs:80](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/lookup/prover.rs:80), [P/plonk/lookup/prover.rs:396](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/lookup/prover.rs:396). |
| CODE | After beta/gamma, construct permutation and lookup grand products, commit them and convert to coefficients. | `O(P_c n+L_c n)` field work, `S+L_c` MSMs/iFFTs; denominator inversion and prefix products matter; [P/plonk/permutation/prover.rs:85](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/permutation/prover.rs:85), [P/plonk/lookup/prover.rs:182](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/lookup/prover.rs:182). |
| CODE | Trash arguments and random vanishing polynomial; sample y; convert advice to coefficients. | Circuit-dependent trash work, one random-polynomial MSM and `A` iFFTs; [P/plonk/prover.rs:174](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:174), [P/plonk/prover.rs:201](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:201), [P/plonk/prover.rs:210](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:210). |
| CODE | Materialise advice/instance cosets; evaluate all gate, permutation, lookup and trash identities into numerator nu. | Extended FFTs `O((A+I+S+3L_c)m log m)` plus evaluator `O(Gm)` for compiled expression/constraint work `G`; lookup auxiliaries are materialised one lookup at a time; [P/plonk/prover.rs:613](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:613), [P/plonk/evaluation.rs:314](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/evaluation.rs:314), [P/plonk/evaluation.rs:448](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/evaluation.rs:448). |
| CODE | Form `h=nu/(X^n−1)`, extended iFFT, truncate, split into blinded limbs and commit. | `O(m log m)` plus `d−1` length-n MSMs. Limbs split at **n−1**, then receive blinding coefficients; [P/plonk/vanishing/prover.rs:98](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/vanishing/prover.rs:98), [P/plonk/vanishing/prover.rs:110](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/vanishing/prover.rs:110), [P/plonk/vanishing/prover.rs:138](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/vanishing/prover.rs:138). |
| CODE | Evaluate at challenge rotations, construct queries, run KZG multiopen. | Polynomial reductions/evaluations `O(Qn)`, serial synthetic divisions `O(R_o n)` for total opening-point work `R_o`, and two final MSMs; [P/plonk/prover.rs:284](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:284), [P/poly/kzg/mod.rs:108](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:108), [P/poly/kzg/mod.rs:163](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:163), [P/poly/kzg/mod.rs:191](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:191). |

ESTIMATE — A dense Pippenger MSM with scalar bit length `b=255` and window `w` costs roughly `O((b/w)(n+2^w))` group additions, with implementation-dependent parallelism. For the illustrative geometry `A=15, L_c=5, P_c=18, d=5`, hence `S=6`, the main proof uses approximately `15+10+11+1+4+2=43` large MSMs, excluding committed instances and trash. Sparsity can greatly reduce actual terms.

ESTIMATE — The same geometry requires approximately `A+I+3L_c+S=38` ordinary iFFTs and `A+I+3L_c+S+1=39` extended transforms, taking `I=2` and excluding key preparation/trash. At k20/q4, each extended transform has `4×22/20=4.4` times an ordinary transform's butterfly work. Thus the extended transforms alone represent approximately `39×4.4=171.6` ordinary transforms; FFT acceleration deserves measurement alongside MSM acceleration.

CODE — `msm_specific` filters zero scalars, then BLS12-381 uses `G1Projective::multi_exp`; that routine normalises projective bases and builds scalar bytes on every invocation ([P/poly/kzg/msm.rs:165](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:165); [C/bls12_381/g1.rs:628](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/g1.rs:628)). Optimising the generic [C/msm.rs:460](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/msm.rs:460) routine alone misses this principal path.

### Ten priority hotspot candidates

PROPOSAL — These are instrumentation priorities, not a measured ranking. Nested costs must not be summed as independent samples.

| Priority | Classification | Hotspot and exact hook |
|---|---|---|
| 1 | CODE | Advice commitments: [P/plonk/prover.rs:568](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:568) → [P/poly/kzg/mod.rs:83](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:83) → [P/poly/kzg/msm.rs:163](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:163); repeated SRS normalisation/conversion underneath. |
| 2 | CODE | Advice/instance extended FFT materialisation: [P/plonk/prover.rs:614](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:614); clones and zero-padding at [P/poly/domain.rs:257](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/domain.rs:257). |
| 3 | CODE | Quotient expression interpreter and repeated array passes: [P/plonk/evaluation.rs:326](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/evaluation.rs:326), permutation pass `:373`, lookup pass `:455`. |
| 4 | CODE | Recursive witness synthesis and dense rational conversion: [P/plonk/prover.rs:517](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:517), `:533`, `:540`; denominator arrays at [P/poly/mod.rs:290](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/mod.rs:290). |
| 5 | CODE | Lookup sort and multiplicity HashMap: [P/plonk/lookup/prover.rs:396](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/lookup/prover.rs:396), `:399`; sequential lookup scheduling at [P/plonk/prover.rs:119](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:119). |
| 6 | CODE | Permutation denominator inversion and serial prefix product: [P/plonk/permutation/prover.rs:106](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/permutation/prover.rs:106), `:143`. |
| 7 | CODE | Lookup denominator inversion and serial prefix scan: [P/plonk/lookup/prover.rs:196](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/lookup/prover.rs:196), `:231`; associated commitments at `:283`. |
| 8 | CODE | Quotient recovery and limb commitments: [P/plonk/vanishing/prover.rs:102](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/vanishing/prover.rs:102), `:124`. |
| 9 | CODE | Multiopen polynomial clones, reductions, Horner evaluations and divisions: [P/poly/kzg/mod.rs:113](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:113), `:140`, `:150`; serial division recurrence at [P/utils/arithmetic.rs:105](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/utils/arithmetic.rs:105). |
| 10 | CODE | Key preparation: fixed/sigma VK commitments [P/plonk/keygen.rs:283](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/keygen.rs:283), [P/plonk/permutation/keygen.rs:245](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/permutation/keygen.rs:245); PK cosets [P/plonk/keygen.rs:333](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/keygen.rs:333); decoded-PK reconstruction [P/plonk/mod.rs:416](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/mod.rs:416). |

### Threading and key generation

CODE — The FFT serially bit-reverses and generates its twiddle table before recursive Rayon joins; each recursion node combines its halves sequentially ([C/fft.rs:46](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/fft.rs:46), `:54`, `:107`, `:120`). Quotient rows are parallel, but advice-column transforms and commitments are often ordinary iterator loops ([P/plonk/prover.rs:617](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:617), `:569`). Parallel butterflies do not imply every FFT phase scales to 20 cores.

CODE — BLST has a separate `threadpool::ThreadPool`, selected independently from Rayon ([R/blst-0.3.17/src/lib.rs:35](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blst-0.3.17/src/lib.rs:35); [R/blst-0.3.17/src/pippenger.rs:141](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blst-0.3.17/src/pippenger.rs:141)). `RAYON_NUM_THREADS` therefore does not bound all proving threads. Concurrent HTTP jobs can compete through both pools.

CODE — VK generation synthesises fixed assignments/copy constraints, converts selectors, and commits fixed/sigma polynomials; PK generation synthesises again and retains ordinary/coefficient/coset representations ([P/plonk/keygen.rs:257](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/keygen.rs:257), `:267`, `:322`, `:333`). Stdlib `setup_vk` directly uses the SRS k, bypassing automatic k discovery ([Z/lib.rs:1946](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1946)). The Sherlock caller separately invokes the cost model before VK generation and again before proving; its prover reconstructs the PK ([F/src/sherlock_link.rs:971](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:971), `:1029`, `:1041`). Moriarty's child path also calls `setup_pk` per `prove` ([/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_link.rs:736](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_link.rs:736)).

### Why memory reaches this scale

CODE — A PK retains three representations for each fixed and permutation polynomial, plus three extended Lagrange masks ([P/plonk/mod.rs:338](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/mod.rs:338); [P/plonk/permutation/keygen.rs:185](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/permutation/keygen.rs:185)). Advice/instance cosets coexist with the trace while evaluating nu ([P/plonk/prover.rs:614](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:614)). Keygen additionally allocates permutation mappings and auxiliary cycle data: two `(usize,usize)` arrays and one `usize` array per equality column ([P/plonk/permutation/keygen.rs:23](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/permutation/keygen.rs:23), `:43`).

ESTIMATE — For 64-bit indices, keygen mapping payload alone is `(16+16+8)P_c n`; with `P_c=18,k=20`, this is `720 MiB`. Rational cells can require approximately 72 bytes including an enum tag/alignment: `15×72×2^20 = 1.055 GiB`, before denominator/output buffers. Confirm layouts with `size_of` in later instrumentation ([P/utils/rational.rs:11](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/utils/rational.rs:11)).

ESTIMATE — With `B=32n` bytes, retained PK scalar payload is approximately `B[(F_c+P_c)(2+q)+3q]`. Using historical `F_c=27,P_c=18` and **assumed q=4** gives `32 MiB×282=8.8125 GiB`. Advice/instance cosets add `(15+2)×128 MiB=2.125 GiB`; trace coefficients approximately `41×32 MiB=1.28125 GiB`; numerator plus six permutation-product cosets add `7×128 MiB=0.875 GiB`. Two projective SRS bases add approximately `2×144×2^20=288 MiB`; one extended FFT's scalar twiddles add `64 MiB`. Total is approximately **13.44 GiB**, before allocator overhead and phase-dependent temporaries. Permutation cosets and lookup temporaries have different lifetimes; do not sum all buffers ever allocated.

OPEN QUESTION — The exact Sherlock `d`, q, live capacities and peak phase are unrecorded here. The illustrative total explains the observed order of magnitude, not the exact 12 GiB peak. At q=8, the PK term alone becomes `32 MiB×474=14.8125 GiB`. Domain expansion must be logged.

### Proof-server, Docker, WASM and ZKIR overhead

SOURCE — The ledger repository distinguishes a native HTTP proof server from WASM interfaces and browser-proving demos. [Repository](https://github.com/midnightntwrk/midnight-ledger); capture: [midnight-ledger.md](../raw/01/midnight-ledger.md).

CODE — The container runs the native `midnight-proof-server` executable, not a WASM interpreter ([L/flake.nix:207](/home/charles/repos/midnight-ledger/flake.nix:207), `:236`). `/prove` receives tagged bytes, clones supplied key material, queues a closure, creates a Tokio runtime and dispatches versioned ZKIR ([L/proof-server/src/endpoints.rs:251](/home/charles/repos/midnight-ledger/proof-server/src/endpoints.rs:251), `:262`, `:305`). Workers use `spawn_blocking`; defaults are two workers and a 600 s job timeout ([L/proof-server/src/worker_pool.rs:236](/home/charles/repos/midnight-ledger/proof-server/src/worker_pool.rs:236); [L/proof-server/src/main.rs:44](/home/charles/repos/midnight-ledger/proof-server/src/main.rs:44)).

CODE — ZKIRv3 preprocesses the proof preimage, fetches parameters, initialises its key, then invokes stdlib proving ([L/zkir-v3/src/ir.rs:65](/home/charles/repos/midnight-ledger/zkir-v3/src/ir.rs:65)). PK initialisation includes gzip decoding and reconstruction of FFT-derived representations; a global LRU retains five decoded PKs ([L/transient-crypto/src/proofs.rs:250](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:250), `:277`; [P/plonk/mod.rs:418](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/mod.rs:418)). The wrapper also verifies the generated proof ([L/transient-crypto/src/proofs.rs:750](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:750)). Separate WASM proving copies JS byte arrays and enters the same ZKIR interface ([L/zkir-v3-wasm/src/lib.rs:139](/home/charles/repos/midnight-ledger/zkir-v3-wasm/src/lib.rs:139)).

PROPOSAL — Attribute service latency as queue + request/key decoding + SRS acquisition + ZKIR preprocessing + proving + self-verification + response encoding. Record cold and warm paths separately. Docker/WSL, network fetching, cache misses and CPU quotas require measurements; there is no defensible fixed “Docker penalty” percentage.

## Proposed design

PROPOSAL — Compare three approaches: (1) retain keys and improve CPU scheduling, with low integration cost; (2) offload only commitments, with narrow correctness boundaries; (3) retain polynomial arithmetic on the GPU, with greater speed potential and memory-management complexity. Start with (1), then (2), and make (3) conditional on measured phase shares.

| Classification | Proposed module/API | Data flow and existing hooks |
|---|---|---|
| PROPOSAL | `profile::ProverObserver`, `StageSample` | Enter/exit `compute_trace`, `parse_advices`, `compute_nu_poly`, `finalise_proof`, keygen and key decoding. Record wall/CPU time, RSS, allocation high-water, n/m, columns, dense/nonzero MSM terms, FFT direction/size and transfers. Never log witnesses. |
| PROPOSAL | `backend::ProverContext`, `ArithmeticBackend` | Explicit context on additive `create_proof_with_backend`, `keygen_vk_with_k_and_backend`, `keygen_pk_with_backend` entrypoints; current APIs select CPU. Keep context outside serialised keys and domain/VK hashing. |
| PROPOSAL | Context-aware PCS methods | Add default-CPU `commit_with_backend`, `commit_lagrange_with_backend`, `multi_open_with_backend` to `PolynomialCommitmentScheme`. Override KZG hooks at [P/poly/kzg/mod.rs:74](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74), `:83`, `:94`, including its internal commits `:163`, `:196`. Static existing PCS calls otherwise bypass the accelerator. |
| PROPOSAL | `gpu::SrsCache`, `msm_g1` | Key cache by SRS digest, basis and k; normalise coefficient/Lagrange bases once. Upload scalar batches, return exact commitments in original transcript order. Keep existing VK/PCS types and point encoding. |
| PROPOSAL | `poly::DeviceOps` | Context-aware equivalents of domain conversions at [P/poly/domain.rs:228](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/domain.rs:228), `:251`, `:272`; preserve roots, natural ordering, inverse normalisation and ZETA coset. Retain arrays through quotient evaluation; download only required coefficients/evaluations/commitments. |
| PROPOSAL | `service::KeySession` | Reuse decoded PK/parameters and GPU buffers under byte budgets; distinguish queue and arithmetic time. Limit large jobs by memory reservations, not only worker count. |

SOURCE — ICICLE provides Rust-facing acceleration and a BLS12-381 build target; its separate CUDA-backend licensing needs review. [ICICLE](https://github.com/ingonyama-zk/icicle); capture: [icicle.md](../raw/01/icicle.md). Its NTT interface exposes ordering, batching and coset controls. [NTT documentation](https://dev.ingonyama.com/2.8.0/icicle/primitives/ntt); capture: [icicle-primitives.md](../raw/01/icicle-primitives.md).

SOURCE — Sppark offers CUDA/C++ MSM and NTT templates, BLS12-381 instantiation support and Rust integration; its own documentation flags layout/FFI integration concerns. [Sppark](https://github.com/supranational/sppark); capture: [sppark.md](../raw/01/sppark.md).

SOURCE — The ICICLE polynomial-API paper motivates device-independent polynomial operations and managed device memory. CuZK motivates reducing transfers and overlapping transfers/computation, but studies a different proving stack. [ICICLE paper](https://eprint.iacr.org/2024/973), [capture](../raw/01/icicle-paper.md); [cuZK](https://eprint.iacr.org/2022/1321), [capture](../raw/01/cuzk.md).

CODE — Local CUDA Rust supplies contexts, streams, buffers and events, but describes itself as early-stage research ([/home/charles/repos/nvidia-cuda-rust/README.md:23](/home/charles/repos/nvidia-cuda-rust/README.md:23), `:33`). PROPOSAL — Use its host abstractions where useful; avoid making a new Rust field-arithmetic implementation prerequisite to the first MSM experiment.

## Expected speedup

ESTIMATE — A planning distribution for a **hypothetically completed 500 s warm proof** is witness/rational conversion 20% (100 s), all MSMs 35% (175 s), all FFTs 25% (125 s), other field arithmetic 15% (75 s), sorting/allocation/miscellaneous 5% (25 s). These exclusive operation families avoid double-counting MSMs inside lookup phases. Plausible individual ranges are respectively 10–35%, 25–60%, 15–35%, 10–25%, 2–10%; they are uncertainty bounds, not simultaneous measurements.

ESTIMATE — Holding shape/q fixed, approximate 350 s of linear work and 150 s of FFT/sort work at k20 gives `T(k)=350×2^(k−20)+150×k×2^k/(20×2^20)`:

| Classification | k | n | One scalar array | Assumed q=4 array | Illustrative time |
|---|---:|---:|---:|---:|---:|
| ESTIMATE | 18 | 262,144 | 8 MiB | 32 MiB | 121.25 s |
| ESTIMATE | 19 | 524,288 | 16 MiB | 64 MiB | 246.25 s |
| ESTIMATE | 20 | 1,048,576 | 32 MiB | 128 MiB | 500 s |

CODE — The historical table requires over one million rows, so that exact configuration cannot run at k18 or k19 ([F/keys/node4/keygen.stats.json:14](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/keys/node4/keygen.stats.json:14)). PROPOSAL — Use smaller legal recursion/table configurations for scaling experiments; do not truncate the original circuit.

ESTIMATE — Assuming 8× MSM acceleration and 10 s added overhead, MSM-only time is `500×(0.65+0.35/8)+10=356.875 s`, or **1.40×**. Adding 6× FFT and 5× other-field acceleration with 15 s overhead gives `500×(0.20+0.35/8+0.25/6+0.15/5+0.05)+15=197.708 s`, or **2.53×**. These are conditional Amdahl calculations, not RTX 5090 benchmark predictions.

ESTIMATE — For a hypothetical 120 s cold VK build, assuming 50% synthesis, 40% MSM and 10% other work, with 8× MSM and 3× other acceleration plus 5 s overhead: `120×(0.5+0.4/8+0.1/3)+5=75 s`, or **1.60×**. Reusing that same valid VK avoids rebuilding it; PK preparation remains separate.

SOURCE — ZPrize reports GPU MSM competition results and publishes implementation links; its 2023 announcement reports subsecond competition scores. Neither result measures this Midnight circuit or this host. [2022 results](https://www.zprize.io/blog/announcing-zprize-results), [capture](../raw/01/zprize-2022.md); [2023 results](https://www.zprize.io/blog/announcing-the-2023-zprize-winners), [capture](../raw/01/zprize-2023.md).

## Risks and soundness concerns

CODE — The parent uses Poseidon; terminal proving uses Blake2b, matching the ledger wrapper's transcript type ([F/src/sherlock_link.rs:1042](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:1042); [F/src/main.rs:603](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/main.rs:603); [L/transient-crypto/src/proofs.rs:64](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:64)). Node ledger-9 aliases its ledger and transient-crypto implementations explicitly ([N/ledger/src/lib.rs:107](/home/charles/repos/midnight-node/ledger/src/lib.rs:107)). Local stdlib verification alone is insufficient evidence of stagenet acceptance.

PROPOSAL — GPU arithmetic must preserve field modulus, Montgomery/canonical conversions, curve/subgroup handling, zero/identity behavior, quotient limb blinding, challenge widths, SRS and VK. Preserve transcript writes after parallel computations in their original order. Keep witness-dependent data and buffers isolated between jobs.

SOURCE — Halo2's transcript interface couples proof-byte writes with hashing. That is a relevant invariant; its documented IPA/Pasta proof encoding is not Midnight's KZG/BLS12-381 encoding. [Halo2 book](https://zcash.github.io/halo2/design/implementation/proofs.html); capture: [halo2-transcript.md](../raw/01/halo2-transcript.md).

PROPOSAL — MSM/FFT acceleration changes no proof format. Switching sorted lookups to checkout LogUp, changing multiopen, using IPA/FRI/folding, or changing circuit/table geometry may change proof bytes, VK or verifier equations. Such migrations require corresponding recursive verifier/key updates and an existing ledger-compatible terminal wrapper or ledger upgrade; they are outside this acceleration design.

## Validation plan

PROPOSAL — Require these predicates in sequence; none is claimed executed here:

1. **Arithmetic equality:** compare CPU/GPU field operations, FFT/iFFT/cosets and MSMs exactly, including zero, identity, sparse scalars, boundary sizes and all k18–20/extended domains actually used.
2. **Key identity:** unchanged processed VK bytes/digest and decoded polynomial values under the same SRS, circuit and flags; check both newly generated and deserialised PK paths.
3. **Transcript equivalence:** with an identical randomness tape and controlled ordering, compare commitments, evaluations and challenges at every boundary. Fix Rayon count too: random-polynomial generation seeds chunks according to thread count ([P/plonk/vanishing/prover.rs:51](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/vanishing/prover.rs:51)).
4. **Proof acceptance:** verify independently randomised proofs with the unmodified verifier. Byte equality is stronger debugging evidence only when witness construction/order are deterministic; lookup HashMap iteration can change table permutation order even with the same RNG seed ([P/plonk/lookup/prover.rs:430](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/lookup/prover.rs:430)).
5. **Recursive composition:** CPU/GPU child-parent-decider combinations must pass; corrupted child proof bytes and mismatched public lanes, accumulator, key, SRS or terminal binding must fail. Retain terminal Blake2b and parent Poseidon tests.
6. **Ledger acceptance:** pass the exact ledger-9 transaction/verifier path, then the deployed contract on stagenet, with proof verification enabled. The ledger calls `proof_verify`; the node invokes transaction validation ([L/ledger/src/verify.rs:1845](/home/charles/repos/midnight-ledger/ledger/src/verify.rs:1845); [N/pallets/midnight/src/lib.rs:576](/home/charles/repos/midnight-node/pallets/midnight/src/lib.rs:576)). Record transaction/readback evidence separately from native acceptance.
7. **Performance attribution:** cold/warm key paths; stage wall/CPU/RSS/VRAM; transfer bytes; Rayon/BLST thread counts; one/two concurrent jobs. First inspect existing `benchmark_create_proof` stage names ([P/plonk/bench/prover.rs:575](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/bench/prover.rs:575)), then add production-path spans to avoid relying on duplicated benchmark logic.

## Open questions

- OPEN QUESTION — What are the exact Sherlock constraint degree, q, column/query counts, active phases and table geometry?
- OPEN QUESTION — How much of the >500 s attempt was cost-model work, PK reconstruction, synthesis or actual `create_proof`?
- OPEN QUESTION — Does the chosen backend support this Blackwell/CUDA/WSL combination, including compatible binaries, without kernel changes?
- OPEN QUESTION — What decoded-PK/GPU cache budget permits recursion concurrency within 90 GB host RAM and 32 GB device memory?
- OPEN QUESTION — Are backend sorting and randomness schedules deterministic enough for byte comparisons, or must acceptance be the primary end-to-end predicate?

## Links

PROPOSAL — Coordinate arithmetic details with [[loki-02-gpu-msm]] and [[loki-03-gpu-ntt]]; precedent selection with [[loki-04-halo2-gpu-precedents]]; proof-system boundaries with [[loki-05-recursion-accumulation]] and [[loki-07-folding-alternatives]]; geometry with [[loki-06-recursive-circuit-cost]]; memory/concurrency with [[loki-08-recursion-tree-scheduling]]; synthesis with [[loki-09-cpu-witness-field]]; and release predicates with [[loki-10-integration-verification]].
