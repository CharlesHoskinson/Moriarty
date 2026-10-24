---
title: "GPU NTT and a resident quotient pipeline for Midnight"
project: loki
node: loki-03-gpu-ntt
author: gpt-6.1-sol
date: 2026-10-24
tags: [loki, gpu, ntt, fft, bls12-381, quotient, halo2, midnight]
sources:
  - https://dev.ingonyama.com/api/rust-bindings/ntt
  - https://dev.ingonyama.com/3.1.0/icicle/libraries
  - https://github.com/ingonyama-zk/icicle/blob/main/docs/versioned_docs/version-4.0.0/api/cpp/ntt.md
  - https://dev.ingonyama.com/api/rust-bindings/program
  - https://github.com/ingonyama-zk/icicle/blob/main/examples/c%2B%2B/best-practice-ntt/README.md
  - https://www.ingonyama.com/oldblogs/user-guide-zk-acceleration-of-gnark-using-icicle
  - https://github.com/supranational/sppark
  - https://github.com/supranational/sppark/blob/main/ntt/ntt.cuh
  - https://eprint.iacr.org/2023/1410
  - https://arxiv.org/abs/2209.01290
  - https://arxiv.org/abs/2509.22684
  - https://www.zprize.io/blog/announcing-zprize-results
  - https://github.com/Tao-Lu-123/cuZK
  - https://github.com/ingonyama-zk/icicle
---

## Question

**PROPOSAL:** Accelerate the k20 parent and terminal decider by moving extended-domain transforms and the entire quotient calculation to the GPU, while preserving the existing Midnight verifier, circuit, transcript, SRS and VK. Scope is research and design; no implementation, builds or benchmarks were performed.

## Summary

- **PROPOSAL:** Integrate a device-resident quotient backend encompassing column extension, custom gates, permutation, lookups, trash arguments, vanishing division and inverse NTT. An FFT-only adapter is a useful differential prototype, but leaves the main residency opportunity unresolved.
- **CODE:** Midnight calls the relevant operation `compute_nu_poly` plus `Evaluator::evaluate_numerator`; `evaluate_h` is the name in the separate Halo2 checkout. See `midnight-proofs-0.8.3/src/plonk/prover.rs:595` and `~/repos/halo2/src/plonk/evaluation.rs:265`.
- **CODE:** Midnight uses the BLS12-381 scalar field named `Fq`, an order-three coset shift `ZETA`, and extended-domain rotation stride `N/n`. These must match exactly: `midnight-curves-0.3.1/src/bls12_381/fq.rs:22`; `midnight-proofs-0.8.3/src/poly/domain.rs:81`; `src/plonk/evaluation.rs:307`.
- **ESTIMATE:** At `n=2^20`, `N=2^22`, 64 live extended columns occupy `64×2^22×32=8 GiB`. A worked allocation below totals 19 GiB including reserves; changing to `N=2^23` raises it to 28 GiB. Actual column counts and library workspace remain unknown.
- **PROPOSAL:** Keep the proof format unchanged. Benchmark ICICLE and sppark against the same pinned field/domain tests; select by full quotient time and peak VRAM. Require unchanged reference-verifier acceptance before any performance promotion.

## Findings

### Code provenance and exact hooks

**CODE:** Citation shorthand: package-relative paths beginning `midnight-*` resolve under `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`; a following `src/...` in the same citation refers to that package. Moriarty pins proofs 0.8.3, circuits 7.2.4 and stdlib 2.3.5 in `~/repos/Moriarty-factory/crates/mori-backend/Cargo.toml:14`. The Fenrir `recl9` manifest instead pins proofs 0.8.2 and patches circuits locally: `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/Cargo.toml:25`. Treat their build identities separately.

**CODE:** The supplied `midnight-zk` revision `31435e9` has cached/pruned transforms and a flattened evaluator, but also LogUp arguments and an optional single quotient commitment. Those are substantive differences from the installed 0.8.3 implementation: `~/repos/midnight-zk/curves/src/fft.rs:110`; `proofs/src/plonk/evaluation.rs:830`; `proofs/src/plonk/prover.rs:695`. Its optimizations are design references, not evidence that replacing the installed library wholesale preserves ledger compatibility.

| Label | Installed 0.8.3 hook, relative to `midnight-proofs-0.8.3/` | Device responsibility / checkout counterpart |
|---|---|---|
| **CODE** | `src/poly/domain.rs:251`, `EvaluationDomain::coeff_to_extended` | Coset scaling, zero extension, forward FFT. Checkout: `~/repos/midnight-zk/proofs/src/poly/domain.rs:285`. |
| **CODE** | `src/plonk/prover.rs:595`, `compute_nu_poly` | Advice/instance extension and numerator orchestration. Checkout: `~/repos/midnight-zk/proofs/src/plonk/prover.rs:634`. |
| **CODE** | `src/plonk/evaluation.rs:285`, `Evaluator::evaluate_numerator` | Gate, permutation, lookup and trash evaluation. Checkout: `~/repos/midnight-zk/proofs/src/plonk/evaluation.rs:818`. |
| **CODE** | `src/poly/domain.rs:321`, `divide_by_vanishing_poly` | Multiply numerator evaluations by cached inverse vanishing values. |
| **CODE** | `src/poly/domain.rs:272`, `extended_to_coeff` | Inverse FFT, normalization and inverse coset scaling. |
| **CODE** | `src/plonk/vanishing/prover.rs:86`, `Committed::construct` | Quotient conversion, truncation, splitting, blinding and commitment. Checkout analogue: `~/repos/midnight-zk/proofs/src/plonk/prover.rs:701`, `compute_h_poly`. |
| **CODE** | `src/plonk/keygen.rs:333`, `keygen_pk` body | Reusable fixed/permutation cosets and boundary selectors; evaluator construction at line 345. |
| **CODE** | `src/plonk/prover.rs:267`, proof-finishing caller | Chooses numerator path before `vanishing.construct` at line 280. Both sites must participate to retain the quotient on device. |

**CODE:** `PolynomialRepresentation::coeff_to_self` routes helper-polynomial extension through the domain, but `Polynomial` owns a host `Vec<F>`: `midnight-proofs-0.8.3/src/poly/mod.rs:72`, `:155`, `:170`. Replacing that trait alone cannot express resident device ownership. Furthermore, `best_fft` is generic over field elements **and curve groups**, so scalar GPU dispatch must not silently capture every instantiation: `midnight-curves-0.3.1/src/fft.rs:6`, `:31`.

### Algebra that the accelerator must reproduce

**CODE:** The scalar modulus is `0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`, represented by four little-endian Montgomery `u64` limbs; two-adicity is 32. This is the scalar field, despite Midnight's `Fq` naming: `midnight-curves-0.3.1/src/bls12_381/fq.rs:1`, `:25`, `:43`, `:126`.

**CODE:** For constraint-system degree `d`, the constructor chooses the smallest power-of-two `N≥n(d−1)` and derives its root from Midnight's `ROOT_OF_UNITY`. Forward extension computes `A_i=Σ_j a_j ζ^j ω_N^(ij)`, with zero padding above `n`. Inversion applies inverse NTT normalization and `ζ^(−j)`. The shift has period three, so scaling needs only `[1,ζ,ζ²]`, not an `N`-element powers table. See `midnight-proofs-0.8.3/src/poly/domain.rs:37`, `:55`, `:81`, `:251`, `:272`, `:349`.

**CODE:** A circuit rotation `r` accesses `(i+r×N/n) mod N`, including negative rotations and permutation's last-row offset. Division uses only `N/n` cached inverses of `(ζω_N^i)^n−1`; no per-row inversion is needed. See `midnight-proofs-0.8.3/src/plonk/evaluation.rs:12`, `:307`, `:359`; `src/poly/domain.rs:84`, `:321`.

**CODE:** Numerator construction already separates four families. Gates evaluate a shared expression graph; permutation uses boundary terms and product recurrences; each lookup extends product/permuted-input/permuted-table polynomials; trash constraints use compressed expressions and a selector. All feed the same ordered Horner accumulator in `y`: `midnight-proofs-0.8.3/src/plonk/evaluation.rs:325`, `:355`, `:443`, `:516`. Lookup or trash omission would produce a different polynomial.

### What the external evidence establishes

**SOURCE:** ICICLE documents BLS12-381 NTT/vector operations. Its Rust API supports in-place transforms, batching, explicit cosets, ordering, device buffers and asynchronous streams. These capabilities fit the required boundary; they do not establish a Midnight integration. [Supported operations](https://dev.ingonyama.com/3.1.0/icicle/libraries), capture [raw/03/icicle-libraries.md](../raw/03/icicle-libraries.md); [Rust NTT API](https://dev.ingonyama.com/api/rust-bindings/ntt), capture [raw/03/icicle-ntt.md](../raw/03/icicle-ntt.md).

**SOURCE:** ICICLE's CUDA documentation distinguishes radix-two and mixed-radix transforms, says performance depends on field, size, batch and ordering, and offers extra-memory “fast twiddles.” Its `Program` abstraction fuses elementwise expressions, which does not by itself supply rotated indexing. [CUDA NTT documentation](https://github.com/ingonyama-zk/icicle/blob/main/docs/versioned_docs/version-4.0.0/api/cpp/ntt.md), capture [raw/03/icicle-cpp-ntt.md](../raw/03/icicle-cpp-ntt.md); [Program API](https://dev.ingonyama.com/api/rust-bindings/program), capture [raw/03/icicle-program.md](../raw/03/icicle-program.md).

**SOURCE:** Ingonyama's historical Gnark integration reports an 8× NTT improvement, concurrent NTT/INTT pairs, and 460 ms total `computeH` including conversion/copy overhead on an RTX 3090 Ti and i9-12900K for circuits above `2^23` constraints. This is another protocol/workload, not a BLS12-381 Sherlock timing. Its best-practices example specifically recommends overlapping transfers and transforms. [Integration report](https://www.ingonyama.com/oldblogs/user-guide-zk-acceleration-of-gnark-using-icicle), capture [raw/03/icicle-gnark-benchmark.md](../raw/03/icicle-gnark-benchmark.md); [transfer example](https://github.com/ingonyama-zk/icicle/blob/main/examples/c%2B%2B/best-practice-ntt/README.md), capture [raw/03/icicle-practices.md](../raw/03/icicle-practices.md).

**SOURCE:** sppark provides CUDA/C++ templates, Rust integration examples and application-specific benchmarking rather than a universal throughput guarantee. Its NTT code exposes `NN/NR/RN/RR` ordering, forward/inverse directions, standard/coset modes, and `Base_dev_ptr` for existing device buffers. That entry point is useful for avoiding host round trips. [Project](https://github.com/supranational/sppark), capture [raw/03/sppark.md](../raw/03/sppark.md); [NTT source](https://github.com/supranational/sppark/blob/main/ntt/ntt.cuh), capture [raw/03/sppark-ntt.md](../raw/03/sppark-ntt.md).

**SOURCE:** The NTT literature supports reducing global-memory passes and tuning kernel/block geometry. A separate HE polynomial-multiplication paper reports 1.85× average improvement for its mixed-radix NTT and fuses adjacent polynomial work. Its 64-bit modular arithmetic results cannot be applied directly to this 255-bit field. [Özcan–Savaş](https://eprint.iacr.org/2023/1410), capture [raw/03/gpu-ntt-paper.md](../raw/03/gpu-ntt-paper.md); [Shivdikar et al.](https://arxiv.org/abs/2209.01290), capture [raw/03/ntt-fusion-paper.md](../raw/03/ntt-fusion-paper.md).

**SOURCE:** ZKProphet reports that optimized GPU proving can become dominated by NTT and constrained by integer execution resources. That cautions against assuming this pipeline is purely bandwidth bound. [Paper abstract](https://arxiv.org/abs/2509.22684), capture [raw/03/zkprophet.md](../raw/03/zkprophet.md).

**SOURCE:** ZPrize's 2022 NTT category was FPGA-based; its separate GPU Plonk-DIZK result was 48 versus 80 minutes. cuZK publishes BLS12-381 V100 Groth-protocol results, including 1.68 versus 4.45 seconds at `2^20`. Neither is a comparable ICICLE-versus-sppark scalar NTT benchmark. [ZPrize results](https://www.zprize.io/blog/announcing-zprize-results), capture [raw/03/zprize-results.md](../raw/03/zprize-results.md); [cuZK results](https://github.com/Tao-Lu-123/cuZK), capture [raw/03/cuzk-repo.md](../raw/03/cuzk-repo.md).

## Proposed design

### Backend boundary and data flow

**PROPOSAL:** Prefer ICICLE as the initial integration candidate because its public batch/coset interface is explicit; retain sppark as a competing implementation behind the same adapter. Compare three approaches: an FFT-only replacement has the smallest integration surface; a resident quotient backend removes intermediate host materialization; a new arithmetic stack offers maximum tuning but the largest validation burden. Implement the resident backend in a future code task after an FFT-only differential prototype.

**PROPOSAL:** Add an optional prover-side module `gpu_poly` with these conceptual APIs, specialized to Midnight's scalar field:

- **PROPOSAL:** `DomainPlan::from_reference(domain)` exports `n,N,ω_N,ζ,N⁻¹`, periodic vanishing inverses and representation metadata.
- **PROPOSAL:** `DevicePolynomialEngine::extend_batch(coeff_handles, plan) -> ExtendedHandles` scales, pads and transforms batches on one device.
- **PROPOSAL:** `QuotientProgram::compile(reference_evaluator, cs)` emits an ordered arithmetic program with explicit rotation loads and argument boundaries.
- **PROPOSAL:** `evaluate_quotient(pk_cache, trace_inputs, challenges, arena) -> DeviceCoefficients` extends helpers, evaluates numerator, divides and inverse-transforms without returning an extended host vector.
- **PROPOSAL:** `download_quotient_coefficients(handle)` returns coefficients at the existing splitting/blinding boundary; later shares device ownership with [[loki-02-gpu-msm]].

**PROPOSAL:** Dispatch once at the proof-finishing caller and add an internal coefficient-input counterpart to `Committed::construct`. Share the existing truncation, `(n−1)`-coefficient splitting, blinding, commitment and transcript-writing code. Otherwise, returning a GPU numerator to the unchanged constructor would download `N` elements and repeat quotient work on CPU. Preserve CPU fallback and public developer APIs; cache by circuit/VK, domain and backend version.

**PROPOSAL:** Keep transcript challenges and randomness under the existing CPU orchestrator. Upload challenges only after the original transcript phase; first extend advice/instance columns, then permutation products, lookup helpers and trash polynomials when available. Cache fixed/sigma cosets and boundary selectors across proofs under the same key. Return only quotient coefficients initially; a later MSM handoff must preserve blinding and transcript order.

**PROPOSAL:** Initially retain witness generation, lookup sorting and grand-product construction before `compute_nu_poly` on CPU. They are inputs to this quotient pipeline, not omitted constraints. Extend those earlier phases only after their separate profile and differential checks justify it.

### Constraint kernels and representation

**PROPOSAL:** Start with natural-order extended values and exact reference roots. For sppark, use standard NTT plus explicit Midnight `ζ` scaling unless its configured coset is proven identical. For ICICLE, test explicit `coset_gen=ζ`; if the backend cannot use Midnight's root, implement the mathematically determined index permutation or reject that configuration. A field match alone is insufficient.

**PROPOSAL:** Store independent columns contiguously as `[column][row]` for batch NTT, with adjacent row threads reading adjacent field elements. Prototype both eight-`u32` array-of-structures and tiled limb-SoA storage `[column][tile][limb][lane]`. SoA can improve limb access, but transposing before every transform can erase that gain; share the chosen backend's layout through quotient kernels. Prefer Montgomery arithmetic for custom kernels. If the NTT backend requires canonical values, convert on device at its boundaries and count the cost. Check limb order, Montgomery factor, reduction range and alignment; never assume raw Rust `Fq` bytes equal a library scalar ABI.

**PROPOSAL:** Configure the ICICLE baseline with `batch_size=B`, `columns_batch=false`, `ordering=kNN`, device input/output buffers and a dedicated stream. Use `coset_gen=ζ` only after the domain-equality predicate passes. Compare radix-two, mixed-radix and fast-twiddle settings while accounting for their actual workspace. Add bit-reversed residency only if rotated indexing and inverse consumption are verified together.

**PROPOSAL:** Compile the installed evaluator's `Calculation` graph into straight-line field operations with liveness-based temporary reuse. Assign one row per thread initially; specialize kernels per circuit and split oversized graphs before register spilling dominates. Preserve Horner ordering across gates, permutation, every lookup, trash and every circuit instance. Materialize only necessary partition outputs, not one full `N`-element vector per graph node. Emit permutation/lookup boundary constraints alongside their recurrence terms, using explicit wrapped rotation loads.

**PROPOSAL:** Fuse periodic vanishing multiplication into the final numerator write where feasible; fuse inverse normalization and inverse coset scaling into the inverse-transform output pass. Start with separate validated kernels before fusing. ICICLE `Program` may serve unrotated expression partitions; custom CUDA kernels must handle rotated column access. CUDA C++ behind a narrow C ABI is the initial kernel boundary; retain Rust ownership/events around device buffers.

**CODE:** NVIDIA's local CUDA Rust project supplies contexts, streams, buffers and events, but identifies itself as early-stage and lists different compiler requirements for its kernel models: `~/repos/nvidia-cuda-rust/README.md:23`, `:33`, `:45`. Selecting it for custom kernels would be an additional toolchain decision, not a prerequisite for the arithmetic design.

### Residency and batching within the VRAM budget

**ESTIMATE:** Each scalar occupies 32 bytes; at k20 the base vector is `2^20×32=32 MiB`. Fourfold and eightfold extensions cost 128 and 256 MiB per vector respectively. The following is an assumed allocation, not a measured Sherlock column inventory:

| Label | Allocation | `N=2^22` | `N=2^23` |
|---|---|---:|---:|
| **ESTIMATE** | 64 live extended input columns, `64N×32` | 8 GiB | 16 GiB |
| **ESTIMATE** | 64 retained base coefficient vectors, `64n×32` | 2 GiB | 2 GiB |
| **ESTIMATE** | Eight extended scratch/helper vectors, `8N×32` | 1 GiB | 2 GiB |
| **ESTIMATE** | Twiddle/transpose/workspace allowance | 2 GiB | 2 GiB |
| **ESTIMATE** | Separate MSM reserve | 4 GiB | 4 GiB |
| **ESTIMATE** | Runtime/display/allocator reserve | 2 GiB | 2 GiB |
| **ESTIMATE** | Sum | 19 GiB | 28 GiB |

**PROPOSAL:** Set an initial device allocation cap of `min(24 GiB, 0.8×reported_free_bytes)` and account for actual backend workspace before launch. At fourfold extension the example has headroom; eightfold requires reducing simultaneous residency or reserves. Keep the numerator resident, stream lookup triples and trash columns by last use, and batch 4–16 transforms initially. Schedule by argument dependencies; do not evict a gate input before its final rotated read. Coordinate this arena with [[loki-08-recursion-tree-scheduling]] rather than giving concurrent proofs independent full-device budgets.

## Expected speedup

**ESTIMATE:** A radix-two `N=2^22` transform has `(N/2)×22=46,137,344` butterflies. Sixty-four forward transforms plus one inverse imply `65×46,137,344≈3.0 billion` butterflies before quotient field operations. Zero-padding pruning can reduce forward work, but this arithmetic shows why FP32/Tensor throughput is not a latency estimate for multi-limb modular multiplication.

**ESTIMATE:** Assume effective host/device bandwidth of 24 GiB/s, explicitly unmeasured on this WSL2 host. Downloading and re-uploading 64 extended vectors costs `2×8/24≈0.67 s` at fourfold extension, excluding conversion, synchronization and allocation. Uploading 64 base vectors plus downloading a four-piece unblinded quotient costs roughly `(2+0.125)/24≈0.089 s`. Warm fixed-data caching is assumed; cold uploads add cost. Transfer reduction alone does not explain a hundreds-of-seconds improvement.

**ESTIMATE:** Use an illustrative completed-proof baseline `T=500 s`, not the owner's censored `>500 s` attempt. If quotient/transform work is fraction `f`, its device speedup is `s`, and added overhead is 2 s, then `T_gpu=500[(1−f)+f/s]+2`. At assumed `s=8`, results are:

| Label | Assumed fraction `f` | Arithmetic | Whole-proof speedup |
|---|---:|---|---:|
| **ESTIMATE** | 0.35 | `500×(0.65+0.35/8)+2=348.9 s` | `500/348.9=1.43×` |
| **ESTIMATE** | 0.60 | `500×(0.40+0.60/8)+2=239.5 s` | `2.09×` |
| **ESTIMATE** | 0.80 | `500×(0.20+0.80/8)+2=152 s` | `3.29×` |

**OPEN QUESTION:** Actual `f`, GPU quotient throughput and Blackwell backend compatibility are unmeasured. The 8× assumption is a scenario, not a prediction derived from the unrelated ICICLE benchmark. VK generation needs its own synthesis/FFT/MSM breakdown; a proving speedup cannot be applied to the reported approximately 120-second VK build. Obtain those fractions through [[loki-01-prover-profile]].

## Risks and soundness concerns

**PROPOSAL:** This design changes execution only: identical circuit, lookup protocol, transcript challenges, SRS, VK, quotient splitting and proof encoding. Switching to the checkout's LogUp or single-commitment mode would change verifier equations/proof structure; it would require matching native and recursive verifiers, regenerated affected keys and ledger integration review. It is outside this acceleration path. The cost is substantially larger than adding device arithmetic.

**CODE:** The recursive verifier is KZG-based and embeds domain/constraint-system information: `midnight-circuits-7.2.4/src/verifier/mod.rs:14`, `:66`; `~/repos/midnight-zk/circuits/src/verifier/mod.rs:68`. Fenrir's parent uses Poseidon while its terminal proof uses Blake2b: `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/lib.rs:6`, `:15`. Ledger verification calls the stdlib with its stored VK and Blake2b transcript: `~/repos/midnight-ledger/transient-crypto/src/proofs.rs:64`, `:545`.

**PROPOSAL:** Treat carry propagation, Montgomery factors, reduction bounds, sign/wrap handling, root direction, inverse normalization, scratch aliasing and asynchronous lifetimes as correctness obligations. Keep witness values out of logs and erase reused witness allocations. Fail closed on unsupported field/domain/opcodes; CPU fallback must restart from a well-defined checkpoint before transcript side effects.

**SOURCE:** sppark flags FFI-safety caveats in its examples; ICICLE distinguishes its frontend license from backend delivery. Audit the selected artifact and ABI rather than inferring production suitability from a successful demo. [sppark integration caveats](https://github.com/supranational/sppark), capture [raw/03/sppark.md](../raw/03/sppark.md); [ICICLE distribution instructions](https://github.com/ingonyama-zk/icicle), capture [raw/03/icicle-repo.md](../raw/03/icicle-repo.md).

## Validation plan

**PROPOSAL:** Future implementation validation should establish these predicates in order; this research ran none of them:

1. **PROPOSAL — field/domain equality:** Differential add/subtract/multiply/square and conversion tests, including `0,1,p−1` and carry-heavy limbs; compare forward/inverse coset transforms to the pinned reference for small sizes through the actual extended domain. Include basis vectors to detect hidden root/order permutations; round trips alone are insufficient.
2. **PROPOSAL — numerator equality:** Compare every field element after gates, permutation, each lookup, trash and circuit-instance accumulation. Exercise negative/large rotations, blinded boundaries, empty arguments and multiple permutation chunks. Require exact equality of numerator evaluations, quotient evaluations and unblinded coefficients.
3. **PROPOSAL — proof compatibility:** With an identical captured randomness schedule and unchanged CPU orchestrator, require byte-identical proofs where reproducible. Otherwise require both proofs to pass the unchanged reference verifier under exactly the same VK/SRS/public inputs. Production proofs remain randomized. Check transcript bytes/challenges at each boundary to localize divergence. Require rejection after perturbing a quotient coefficient, public input, VK or transcript message; positive acceptance alone is insufficient.
4. **PROPOSAL — recursive/ledger acceptance:** Verify accelerated children in the existing parent, then accelerated parents in the existing terminal decider, including the folded accumulator/pairing checks. Require terminal proof acceptance by the existing ledger-9 transaction path and finally stagenet. Coordinate exact rejection cases and deployed version pins with [[loki-10-integration-verification]].
5. **PROPOSAL — performance:** After correctness, measure cold/warm full quotient wall time, kernel time, transfers, conversion, peak VRAM/RSS and end-to-end proof time. Compare ICICLE/sppark on identical `n,N`, batch counts, coset/root/order and field representation. Record OOM and timeouts; never report a censored attempt as a completed baseline.

**CODE:** The stdlib has separate proving and verification entry points suitable for the compatibility harness: `midnight-zk-stdlib-2.3.5/src/utils/plonk_api.rs:88`, `:132`; checkout counterparts `~/repos/midnight-zk/zk_stdlib/src/utils/plonk_api.rs:89`, `:133`. Fenrir already verifies parent proofs after proving at `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:1042`. Node transaction types wrap the ledger transaction at `~/repos/midnight-node/ledger/src/versions/common/api/transaction.rs:49`; this is an integration seam, not evidence of stagenet acceptance.

## Open questions

- **OPEN QUESTION:** What are the parent/decider's actual degree `d`, extended `N`, column counts, lookup count and peak simultaneously live intermediates?
- **OPEN QUESTION:** Which exact proofs/circuits build identity produces the retained Fenrir parent and terminal artifacts, given the manifest's 0.8.2 versus Moriarty's 0.8.3 pin?
- **OPEN QUESTION:** Does the selected ICICLE CUDA artifact support this Blackwell/CUDA combination and expose the required field representation for custom quotient kernels? Can sppark use exactly Midnight's root without a costly permutation?
- **OPEN QUESTION:** Does expression specialization keep enough temporaries in registers, or should some gates use tiled scratch and several kernels? Which layout wins after counting all transpose/conversion costs?
- **OPEN QUESTION:** How much device memory must the shared MSM backend retain, and is recomputing selected cosets cheaper than spilling them?

## Links

- **PROPOSAL:** Measure stage fractions in [[loki-01-prover-profile]]; share device ownership with [[loki-02-gpu-msm]] and integration precedents with [[loki-04-halo2-gpu-precedents]].
- **PROPOSAL:** Keep the mathematical relation aligned with [[loki-05-recursion-accumulation]] and [[loki-06-recursive-circuit-cost]]; evaluate proof-system changes separately in [[loki-07-folding-alternatives]].
- **PROPOSAL:** Coordinate arena scheduling with [[loki-08-recursion-tree-scheduling]], host conversion/witness work with [[loki-09-cpu-witness-field]], and acceptance predicates with [[loki-10-integration-verification]].
- **CODE:** Higher-level aggregation already has its own setup/SRS/circuit boundary: `~/repos/midnight-zk/aggregation/src/multi_circuit_aggregator/aggregator.rs:28`. Sherlock emits SDK instructions upstream of proving: `~/repos/Moriarty-factory/crates/mori-backend/src/sherlock_machine/mod.rs:4`. The proposed arithmetic hook belongs below those interfaces.
