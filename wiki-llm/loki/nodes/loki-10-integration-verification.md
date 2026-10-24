---
title: "GPU backend integration and verification for midnight-proofs"
project: loki
node: 10
author: gpt-6.1-sol
date: 2026-10-24
tags: [gpu, cuda, recursion, midnight-proofs, verification, doctorwatson]
sources:
  - https://github.com/midnightntwrk/midnight-zk
  - https://github.com/midnightntwrk/midnight-zk/blob/main/CONTRIBUTING.md
  - https://github.com/supranational/sppark
  - https://github.com/ingonyama-zk/icicle
  - https://dev.ingonyama.com/start/integration-%26-support/faq_and_troubleshooting
  - https://dev.ingonyama.com/start/architecture/install_gpu_backend
  - https://github.com/axiom-crypto/halo2-gpu
  - https://github.com/Tao-Lu-123/cuZK
  - https://eprint.iacr.org/2022/1321
  - https://docs.nvidia.com/cuda/wsl-user-guide/index.html
  - https://docs.nvidia.com/compute-sanitizer/ComputeSanitizer/index.html
  - https://docs.nvidia.com/cuda/blackwell-compatibility-guide/index.html
  - https://developer.nvidia.com/cuda/gpus
---

## Question

**PROPOSAL —** Add optional GPU MSM, NTT and quotient execution to the pinned Midnight prover while preserving the existing proof system, transcript, SRS, verifying key and ledger acceptance. Keep an independently runnable CPU reference and make correctness precede performance promotion.

## Summary

- **PROPOSAL —** Introduce an explicit per-proof arithmetic backend, with CPU defaults and an opt-in Cargo feature named cuda. Keep proof parsing, transcript sequencing, randomness and verification on the host.
- **CODE —** Target the deployed crate first: Moriarty pins proofs 0.8.3, circuits 7.2.4 and stdlib 2.3.5; the separate checkout has a different PCS interface. See [Moriarty Cargo.toml:14](/home/charles/repos/Moriarty-factory/crates/mori-backend/Cargo.toml:14) and [checkout poly/commitment.rs:54](/home/charles/repos/midnight-zk/proofs/src/poly/commitment.rs:54).
- **PROPOSAL —** Require byte-identical proofs in controlled fixtures with identical randomness and thread configuration, plus independent CPU-verifier acceptance under fresh production randomness.
- **PROPOSAL —** Promote through arithmetic differentials, small cryptographic proofs, recursive accumulator checks, a complete k20 chain, real ledger-9 verification and stagenet effects readback.
- **SOURCE —** Backend choice includes licensing and deployment constraints: sppark publishes Apache-2.0 primitives, whereas ICICLE distinguishes its MIT frontend from separately licensed GPU backends. [sppark](https://github.com/supranational/sppark) ([capture](../raw/10/sppark.md)); [ICICLE](https://github.com/ingonyama-zk/icicle) ([capture](../raw/10/icicle-repo.md)).

## Findings

### The actual integration boundary

**CODE —** In the registry crate, KZG coefficient and Lagrange commitments call msm_specific directly. The existing MSM trait describes accumulation and evaluation; replacing an implementation of that trait alone does not intercept these prover commitments. [proofs 0.8.3/poly/kzg/mod.rs:74](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74), [poly/kzg/msm.rs:109](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:109). Its BLS12-381 branch already uses G1Projective::multi_exp; a GPU comparison must include that CPU implementation. [poly/kzg/msm.rs:176](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:176).

**CODE —** The registry domain methods perform inverse transforms, ordinary transforms and coset extension separately. compute_nu_poly constructs advice/instance cosets and calls Evaluator::evaluate_numerator with gates, challenges, lookup, trash and permutation data. Quotient construction then divides by the vanishing polynomial, inverse-transforms, splits and blinds limbs. These are distinct correctness boundaries. [poly/domain.rs:224](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/domain.rs:224), [plonk/prover.rs:595](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:595), [vanishing/prover.rs:98](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/vanishing/prover.rs:98).

**CODE —** At checkout 31435e9, commit_many returns labeled multi-commitments, transforms use cached twiddles and pruned extension, and numerator evaluation uses a flattened evaluator. Porting a backend is therefore separate from adopting that checkout's protocol changes. The nearby halo2 checkout also has its own PCS signatures. [midnight-zk/poly/kzg/mod.rs:82](/home/charles/repos/midnight-zk/proofs/src/poly/kzg/mod.rs:82), [poly/domain.rs:279](/home/charles/repos/midnight-zk/proofs/src/poly/domain.rs:279), [plonk/evaluation.rs:818](/home/charles/repos/midnight-zk/proofs/src/plonk/evaluation.rs:818), [halo2/poly/commitment.rs:39](/home/charles/repos/halo2/src/poly/commitment.rs:39).

**SOURCE —** Midnight describes its proof system as PLONK with KZG and explicitly documents divergence from its Halo2 origins. A Halo2 accelerator is precedent, not evidence of Midnight compatibility. [Midnight repository](https://github.com/midnightntwrk/midnight-zk) ([capture](../raw/10/midnight-repo.md)).

### Compatibility includes recursion and the transaction

**CODE —** Moriarty's child proof uses Poseidon and self-verifies through stdlib. Fenrir's recl9 uses Poseidon leaves/nodes and a Blake2b decider; its off-circuit preparation explicitly omits the pairing until verify_offcircuit. The registry verifier gadget likewise says algebraic verification does not verify the PCS proof. [sherlock_child.rs:48](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_child.rs:48), [sherlock_child.rs:374](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_child.rs:374), [recl9/main.rs:6](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/main.rs:6), [recl9/inner.rs:72](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/inner.rs:72), [verifier_gadget.rs:393](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/verifier_gadget.rs:393).

**CODE —** The ledger's VerifierKey::verify calls stdlib with its TranscriptHash; contract proof verification obtains the registered VK and verifier parameters. Default strictness enables real contract verification, while a build without proof-verifying contains a no-op branch. A local test must establish its build features and mode. The node includes a transaction validation test using default strictness. [ledger proofs.rs:64](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:64), [proofs.rs:544](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:544), [structure.rs:424](/home/charles/repos/midnight-ledger/ledger/src/structure.rs:424), [verify.rs:413](/home/charles/repos/midnight-ledger/ledger/src/verify.rs:413), [node transaction.rs:485](/home/charles/repos/midnight-node/ledger/src/versions/common/api/transaction.rs:485).

**CODE —** Sherlock's acceptance requirement binds circuit/VK identities, predecessor/result roots and heights, and ordered batch commitment; a host-provided digest or success flag is insufficient. Its source study also warns that the precise Fenrir decider lineage reaching ledger-9 remains a source-pinning dependency. [experiment.md:13](/home/charles/repos/Moriarty-factory/docs/sherlock/stagenet-recursive-rollup-experiment.md:13), [experiment.md:23](/home/charles/repos/Moriarty-factory/docs/sherlock/stagenet-recursive-rollup-experiment.md:23).

### Reuse and licensing

**SOURCE —** Axiom's halo2-gpu exposes a CUDA prover, requires CUDA for ordinary builds, and describes BN254 benchmark parameters. Borrow its execution ideas; importing its prover would not establish BLS12-381/Midnight compatibility. [repository](https://github.com/axiom-crypto/halo2-gpu) ([capture](../raw/10/halo2-gpu.md)).

**SOURCE —** cuZK documents BLS12-381 experiments on V100/CUDA 11.5, and its paper describes transfer reduction and overlap. Its reported Groth16 and MSM results are not k20 Sherlock or RTX 5090 measurements. [repository](https://github.com/Tao-Lu-123/cuZK) ([capture](../raw/10/cuzk.md)); [paper abstract](https://eprint.iacr.org/2022/1321) ([capture](../raw/10/cuzk-paper.md)).

**SOURCE —** sppark provides CUDA/C++ field, MSM and NTT templates including BLS12-381; its README explicitly cautions about example FFI safety. ICICLE's FAQ calls the CUDA backend source available under ICICLE-SA, while its installation page still calls GPU backends closed-source and requires licensing, with production use directed to sales. Do not infer backend redistribution rights from the frontend license. [sppark](https://github.com/supranational/sppark) ([capture](../raw/10/sppark.md)); [ICICLE FAQ](https://dev.ingonyama.com/start/integration-%26-support/faq_and_troubleshooting) ([capture](../raw/10/icicle-licensing.md)); [backend installation](https://dev.ingonyama.com/start/architecture/install_gpu_backend) ([capture](../raw/10/icicle-backend-license.md)).

## Proposed design

### Modules and APIs

**PROPOSAL —** Prefer a small fork of pinned 0.8.3 with opt-in cuda and gpu-crosscheck features, a default CpuBackend, and a separate CUDA implementation crate. Keep an MSM-only adapter as the first milestone; add NTT and quotient execution after equality tests. A complete external prover replacement has the largest audit surface and is deferred.

**PROPOSAL —** Put contracts/plans in proofs/src/backend/mod.rs, original arithmetic delegates in backend/cpu.rs, conversion code in backend/codec.rs and optional device ownership/FFI in the CUDA crate. Start with synchronous host-visible results. The following is schematic API design, not implemented or compiled Rust; supporting types and bounds must be defined during implementation.

~~~rust
trait ProverBackend<F: PrimeField>: Send + Sync {
    fn msm<C: CurveAffine<ScalarExt = F>>(
        &self, bases: &[C::Curve], scalars: &[F]
    ) -> Result<C::Curve, BackendError>;
    fn ntt(&self, values: &mut [F], plan: &NttPlan<F>)
        -> Result<(), BackendError>;
    fn quotient(&self, plan: &QuotientPlan<F>, columns: &ColumnViews<F>)
        -> Result<Vec<F>, BackendError>;
}
~~~

**PROPOSAL —** Use static dispatch: the generic MSM method makes this trait unsuitable for a dyn object. CpuBackend supports the existing generic path; CudaBackend advertises BLS12-381 G1 and its scalar field only, with explicit CPU delegation for unsupported types before launch. NttPlan fixes length, root, direction, normalization, ordering, coset and padding. QuotientPlan freezes every identity, rotation, boundary selector and challenge coefficient in the reference numerator, not just custom gates.

**PROPOSAL —** Add create_proof_with_backend and backend-aware keygen entry points; preserve existing entry points as CPU wrappers. Thread context through compute_trace, parse_advices, finalise_proof and argument commitment helpers. Add fallible commit_with_backend, commit_lagrange_with_backend and multi_open_with_backend methods to PolynomialCommitmentScheme with CPU-delegating defaults; KZG overrides invoke the backend. Existing commit methods remain CPU reference operations. Propagate errors through the new path.

**CODE —** The existing PCS commitment methods have no context or error return, and multi_open commits opening polynomials internally. Therefore the new path must change those internal call sites too. [commitment.rs:43](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/commitment.rs:43), [kzg/mod.rs:163](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:163).

**PROPOSAL —** Route domain conversions through backend-aware variants, hook compute_nu_poly/Evaluator::evaluate_numerator, and keep vanishing division, splitting and blinding on CPU initially. Later fuse division and inverse NTT only after intermediate comparisons pass. Expose stdlib prove_with_backend/setup_vk_with_backend/setup_pk_with_backend through utils::plonk_api::BlstPLONK, where current wrappers fix KZG and call create_proof. Update the mirrored benchmark path alongside prover changes.

**CODE —** These wrapper and benchmark coupling points exist at [stdlib lib.rs:1984](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1984), [utils/plonk_api.rs:103](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/utils/plonk_api.rs:103) and [plonk/prover.rs:346](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:346).

### Data flow, representation and failure behavior

**PROPOSAL —** Execute: CPU synthesis and blinding → canonical staging → GPU arithmetic → synchronization and checked host conversion → ordered CPU transcript writes → CPU opening orchestration → independent CPU verification. Retain SRS/NTT plans across proofs by a cache key containing SRS digest, field, basis, domain, encoding, device and backend revision. Never identify an SRS solely by k.

**CODE —** Midnight calls the BLS12-381 scalar field Fq; it stores four little-endian Montgomery limbs, while its public representation is 32 bytes. Naming agreement with another library is insufficient. [fq.rs:22](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/fq.rs:22), [fq.rs:464](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/fq.rs:464).

**PROPOSAL —** Begin with canonical scalar bytes and checked point encodings; explicitly convert Montgomery domains inside the adapter. Normalize returned projective points before canonical serialization. Validate lengths, subgroup membership, infinity and field bounds; validate SRS imports once and bind subsequent reuse to their digest. Never generate a replacement SRS during GPU initialization. Add raw-limb transport only with documented layouts and round-trip tests; never transmute unrelated Rust/CUDA objects.

**PROPOSAL —** Own device buffers until their completion event; isolate proof workspaces, bound the staging pool and preserve polynomial indices across concurrent MSM completion. Unsupported capability may choose CPU before launch. Kernel failure, mismatch or synchronization failure aborts the proof; discard partial transcript and device state. Any automatic retry starts a complete proof with fresh randomness, rather than concealing a faulty kernel.

**CODE —** NVIDIA's local CUDA Rust project models ownership across launches but labels itself early-stage research. Treat it as an experimental adapter option, not evidence that arithmetic kernels are correct. [README.md:23](/home/charles/repos/nvidia-cuda-rust/README.md:23).

### Exact proofs and upstreaming

**CODE —** The reference prover samples a blinding polynomial using ChaCha streams seeded according to Rayon thread count. Equal top-level seeds alone do not guarantee equality across thread counts. [vanishing/prover.rs:51](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/vanishing/prover.rs:51).

**PROPOSAL —** For golden fixtures fix seed, Rayon width, package/source hashes, feature graph, circuit, SRS, keys, instances and transcript. Preserve host RNG call order and transcript ordering. Correct finite-field/group arithmetic permits different reduction schedules while preserving canonical output; projective memory bytes need not match. Production retains fresh cryptographic randomness, and independently verifier-accepted proofs are the compatibility requirement.

**PROPOSAL —** This backend changes no proof format and requires no verifier, SRS or VK migration. Switching to Groth16, another field/PCS, another transcript, different quotient commitments or a different recursive relation is a protocol/circuit change: it needs compatible verifier support, new keys/contract registration and potentially a new setup. It belongs outside acceleration acceptance. The checkout's single-h-commitment feature must not be enabled as an incidental GPU optimization.

**CODE —** That feature explicitly changes quotient commitment structure and SRS degree requirements. [checkout Cargo.toml:88](/home/charles/repos/midnight-zk/proofs/Cargo.toml:88).

**SOURCE —** Midnight's contribution guide requests early discussion of large subsystems, signed commits, crate changelogs, tests and Apache headers. [guide](https://github.com/midnightntwrk/midnight-zk/blob/main/CONTRIBUTING.md) ([capture](../raw/10/midnight-contributing.md)).

**PROPOSAL —** Upstream in reviewable increments: CPU-equivalent hooks and tests; optional MSM adapter; NTT; quotient execution. Keep CUDA compilation/linking absent from default and verifier-only builds, retain dependency notices and pin kernel revisions. Prefer a reviewed permissive-source adapter for distribution; evaluate any separately licensed binary independently.

## Expected speedup

**ESTIMATE —** Let m, n and q be measured CPU-time fractions for MSM, NTT and quotient evaluation; let h cover transfers, conversions and verification overhead relative to baseline. Predicted total ratio is R = (1−m−n−q) + m/s_m + n/s_n + q/s_q + h. Assume m=.45, n=.25, q=.15, other=.15. These are planning assumptions, not a profile.

| Label | Assumed acceleration | Arithmetic | Illustrative total speedup |
| --- | --- | --- | --- |
| ESTIMATE | MSM only, 8×; h=.03 | .55+.45/8+.03=.63625 | 1/.63625=1.57× |
| ESTIMATE | MSM 8×, NTT 6×, quotient 4×; h=.05 | .15+.05625+.04167+.0375+.05=.33542 | 2.98× |
| ESTIMATE | Same kernels, h=.20 | .15+.05625+.04167+.0375+.20=.48542 | 2.06× |

**ESTIMATE —** A hypothetical completed 500-second baseline maps to 318.1, 167.7 or 242.7 seconds respectively. The brief's >500-second attempt is a censored observation, not a measured completed baseline. For a separate hypothetical 120-second VK build with 40% accelerated 8× and 5% overhead: 120×(.60+.40/8+.05)=84 seconds, or 1.43×. Keygen needs its own profile.

**ESTIMATE —** At k20, a 32-byte field vector occupies 2²⁰×32=32 MiB. Assuming extended k22, each vector is 128 MiB; 40 simultaneously live vectors occupy 5 GiB before SRS, buckets, scratch or duplicate staging. This is a residency example, not a measured VRAM peak or evidence that 12 GiB host RSS predicts device requirements.

**PROPOSAL —** Measure cold initialization/keygen, warm proving, transfers and independent verification separately; report end-to-end latency, throughput, CPU RSS and device allocation high-water. Do not extrapolate V100/Groth16 speedups or GPU kernel timings into Sherlock performance.

## Risks and soundness concerns

**PROPOSAL —** Treat arithmetic bugs primarily as correctness/availability failures under an unchanged independent verifier; accepting a proof still depends on the original cryptographic assumptions and verifier correctness. A faulty prover does not acquire permission to weaken verification. Verifier acceptance also cannot certify intended circuit semantics or zero-knowledge: broken/reused blinding can leak information in otherwise accepted proofs.

| Label | Failure | Required defense |
| --- | --- | --- |
| PROPOSAL | Wrong modulus, Montgomery factor, carry or reduction | Compare CPU canonical values for 0, 1, modulus−1, carry chains, maximal limbs and randomized vectors; document every lazy-reduction bound. |
| PROPOSAL | MSM exceptional cases or bucket overflow | Test zero scalars, identity, P+(−P), equal points, concentrated buckets, partial windows and 2ʲ±1 lengths. |
| PROPOSAL | NTT direction, order or coset mismatch | Compare each transform to CPU; also direct polynomial evaluation at small sizes. Round-trip alone can hide matching forward/inverse mistakes. |
| PROPOSAL | Races, stale buffers, aliasing or early free | Stress stream interleavings, allocation reuse and boundary launch sizes; compare results repeatedly and use explicit completion ownership. |
| PROPOSAL | Quotient identity omitted or wrong rotation | Compare every numerator row and quotient coefficient, including lookup/trash/permutation boundaries and negative rotations. |
| PROPOSAL | Shared faulty reference, bad RNG or secret leakage | Separate CPU-only verifier/process and frozen reference build; keep RNG on host; allow detailed traces only for synthetic witnesses. |

**SOURCE —** Compute Sanitizer provides memcheck, racecheck, initcheck and synccheck. Racecheck targets shared-memory hazards, so a clean report is not proof of global-memory race freedom or modular arithmetic correctness. [manual](https://docs.nvidia.com/compute-sanitizer/ComputeSanitizer/index.html) ([capture](../raw/10/nvidia-sanitizer.md)).

**SOURCE —** WSL2 has limited pinned memory, incomplete managed-memory support and restricted NVML reporting; NVIDIA says to use the Windows driver rather than installing a Linux display driver inside WSL. [WSL guide](https://docs.nvidia.com/cuda/wsl-user-guide/index.html) ([capture](../raw/10/nvidia-wsl.md)).

**SOURCE —** NVIDIA lists RTX 5090 at compute capability 12.0. Its compatibility guide explains that incompatible cubins need a compatible PTX fallback or rebuilding; “Blackwell” alone does not specify a usable binary target. [GPU table](https://developer.nvidia.com/cuda/gpus) ([capture](../raw/10/nvidia-capabilities.md)); [compatibility guide](https://docs.nvidia.com/cuda/blackwell-compatibility-guide/index.html) ([capture](../raw/10/nvidia-blackwell.md)).

**PROPOSAL —** Build sm_120 plus suitable PTX, validate actual driver/device/module initialization, and bound pinned staging instead of relying on managed-memory oversubscription. Record allocator high-water independently of NVML. Treat unavailable sanitizer coverage as missing evidence, never a passing check.

## Validation plan

**PROPOSAL —** First freeze a separately built CPU reference and artifact manifest. Run arithmetic differential/fuzz cases, then small real proofs with both transcript families and fixed randomness. Require equality of canonical intermediate values and final bytes within each fixed configuration. Vary seeds, thread counts and scheduling across configurations; require CPU verification everywhere without demanding cross-configuration byte equality.

**CODE —** MockProver synthesizes and checks circuit assignments; it is not GPU commitment/NTT execution. Moriarty already has a bounded witness-mutation harness with explicit under-constraint exclusions. Use it for relation checks alongside real cryptographic tests. [dev/mod.rs:804](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/dev/mod.rs:804), [soundness_mutation.rs:1](/home/charles/repos/Moriarty-factory/crates/mori-backend/tests/soundness_mutation.rs:1).

**PROPOSAL —** Exercise CPU/GPU combinations at child, parent and decider stages. Check the final pairing/accumulator invariant, not prepare alone. Feed exact resulting bytes to an independent verifier and the actual ledger-9 call path with real verification enabled. Reject flipped/truncated/trailing proof bytes, wrong public inputs, wrong VK, malformed points, tampered inner proofs, wrong predecessor/height, reordered batches and replayed state. Inject arithmetic/kernel errors to confirm the harness detects them.

**CODE —** The stdlib wrapper already checks transcript exhaustion and finalizes the verification guard; checkout aggregation additionally checks VK identity and the final accumulated pairing. Preserve both obligations. [plonk_api.rs:144](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/utils/plonk_api.rs:144), [ivc/verifier.rs:49](/home/charles/repos/midnight-zk/aggregation/src/ivc/verifier.rs:49).

**PROPOSAL —** The following are proposed factory records, not implemented checks. Each uses the contract-mandated unimplemented command until its runner exists. All records share source wiki-llm/loki/nodes/loki-10-integration-verification.md#validation-plan; illustrative runner names below are future commands.

~~~yaml
version: 1
predicates:
  - {id: F101, set: F, title: "CPU feature boundary and unchanged artifacts", tier: T0, check: "scripts/qa/unimplemented.sh F101", heavy: false, depends: [], source: "wiki-llm/loki/nodes/loki-10-integration-verification.md#validation-plan", holdout: false}
  - {id: F102, set: F, title: "GPU arithmetic differential corpus", tier: T1, check: "scripts/qa/unimplemented.sh F102", heavy: false, depends: [F101], source: "wiki-llm/loki/nodes/loki-10-integration-verification.md#validation-plan", holdout: false}
  - {id: F103, set: F, title: "Small proofs equal and independent verifier rejects mutations", tier: T1, check: "scripts/qa/unimplemented.sh F103", heavy: false, depends: [F102], source: "wiki-llm/loki/nodes/loki-10-integration-verification.md#validation-plan", holdout: false}
  - {id: F104, set: F, title: "Sanitizers and scheduling stress", tier: T2, check: "scripts/qa/unimplemented.sh F104", heavy: true, budget: "gpu-sanitizers", depends: [F103], source: "wiki-llm/loki/nodes/loki-10-integration-verification.md#validation-plan", holdout: false}
  - {id: F105, set: F, title: "k20 mixed-backend recursive chain closes accumulator", tier: T2, check: "scripts/qa/unimplemented.sh F105", heavy: true, budget: "gpu-k20-chain", depends: [F104, A3, A4], source: "wiki-llm/loki/nodes/loki-10-integration-verification.md#validation-plan", holdout: false}
  - {id: F106, set: F, title: "Exact GPU artifacts accepted by real ledger-9 with negatives", tier: T2, check: "scripts/qa/unimplemented.sh F106", heavy: true, budget: "gpu-ledger9", depends: [F105, A6, A7], source: "wiki-llm/loki/nodes/loki-10-integration-verification.md#validation-plan", holdout: false}
  - {id: F107, set: F, title: "Finalized stagenet GPU proof and independent effects readback", tier: T3, check: "scripts/qa/unimplemented.sh F107", heavy: false, depends: [F106, A8, A10], source: "wiki-llm/loki/nodes/loki-10-integration-verification.md#validation-plan", holdout: false}
  - {id: F108, set: F, title: "Measured end-to-end GPU improvement", tier: T2, check: "scripts/qa/unimplemented.sh F108", heavy: true, budget: "gpu-performance", depends: [F105], source: "wiki-llm/loki/nodes/loki-10-integration-verification.md#validation-plan", holdout: false}
~~~

**PROPOSAL —** Future runners are scripts/qa/check-gpu-{cpu-boundary,arithmetic,proofs,sanitizers,k20,ledger9,stagenet,performance}.sh respectively. F101 checks default builds need no CUDA and preserves VK/proof fixtures; F102–F103 run bounded corpora under 20 minutes; F104 runs all four sanitizers with nonzero error exit codes. F108 requires at least 1.5× speedup in median warm end-to-end time across five paired runs, including transfers and independent verification, with a successful CPU baseline and separately reported cold initialization. This is a proposed promotion target, not a measured result.

**CODE —** Factory schema, exits and tiers are defined in [CONTRACT.md:33](/home/charles/repos/Moriarty-factory/models/doctorwatson/factory/CONTRACT.md:33); heavy checks measure then cap at twice the first measurement and retries require hypotheses. T3 includes the one-hour nonblocking cold review. [CONTRACT.md:78](/home/charles/repos/Moriarty-factory/models/doctorwatson/factory/CONTRACT.md:78), [CONTRACT.md:94](/home/charles/repos/Moriarty-factory/models/doctorwatson/factory/CONTRACT.md:94), [CONTRACT.md:101](/home/charles/repos/Moriarty-factory/models/doctorwatson/factory/CONTRACT.md:101).

**PROPOSAL —** Register budget keys before implementation checks; run scripts/doctorwatson check F105 rather than bypassing heavy, and include GPU initialization/residency details in its trace. After a failure, read the trace and record a one-line hypothesis before retrying; after two failures of the same kind, shrink the reproducer. Keep evaluator-owned holdouts unchanged. Future Rust changes also run cargo fmt --all --check, cargo clippy --workspace --all-targets --locked -- -D warnings and cargo test --workspace --locked under the authorized factory workflow. No builds, benchmarks, deployments or implementation changes were performed for this research node.

## Open questions

- **OPEN QUESTION —** Which exact ledger-9 SDK, contract VK, SRS and decider lineage define the final acceptance fixture?
- **OPEN QUESTION —** What are completed CPU stage timings and peak device workspace requirements for the current parent/decider circuits?
- **OPEN QUESTION —** Can a pinned sppark adapter provide the required BLS12-381 ABI and sm_120 coverage without unsafe layout assumptions?
- **OPEN QUESTION —** Which ICICLE backend revision/license permits intended production deployment, redistribution, offline operation and source audit?
- **OPEN QUESTION —** Which operations dominate after MSM offload, and does WSL staging erase NTT gains?
- **OPEN QUESTION —** Will Midnight accept the generic backend hooks upstream, or prefer an internal BLS-specific execution context?

## Links

**PROPOSAL —** Coordinate profiling with [[loki-01-prover-profile]], kernels with [[loki-02-gpu-msm]] and [[loki-03-gpu-ntt]], and reuse decisions with [[loki-04-halo2-gpu-precedents]].

**PROPOSAL —** Preserve accumulator obligations from [[loki-05-recursion-accumulation]] and circuit semantics from [[loki-06-recursive-circuit-cost]]. Treat [[loki-07-folding-alternatives]] as a separate protocol workstream. Integrate residency/scheduling from [[loki-08-recursion-tree-scheduling]] and host arithmetic from [[loki-09-cpu-witness-field]].

**SOURCE —** Thirteen primary pages were captured with Scrapling and read; their URLs are in frontmatter and paired capture links accompany the claims. Captures reside exclusively in raw/10/. Web capture dates are 2026-10-24; upstream pages are mutable, so implementation must pin revisions separately.
