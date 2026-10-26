# Prior study: loki07-prior
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/nodes/loki-07-folding-alternatives.md
> Collected: 2026-10-26
> Published: 2026-10-24

---
title: Folding alternatives and the Midnight final-proof boundary
project: loki
node: 7
author: gpt-6.1-sol
date: 2026-10-24
tags: [folding, ivc, recursion, gpu, midnight, decider, curves]
sources: [
  "https://eprint.iacr.org/2021/370",
  "https://eprint.iacr.org/2022/1758",
  "https://eprint.iacr.org/2023/573",
  "https://eprint.iacr.org/2023/620",
  "https://eprint.iacr.org/2023/1106",
  "https://eprint.iacr.org/2024/1220",
  "https://eprint.iacr.org/2023/1192",
  "https://eprint.iacr.org/2024/257",
  "https://github.com/microsoft/Nova",
  "https://github.com/ethereum/sonobe",
  "https://eprint.iacr.org/2021/1152",
  "https://github.com/arkworks-rs/algebra/blob/master/curves/README.md",
  "https://arxiv.org/html/1803.02067v3",
  "https://github.com/ingonyama-zk/open-icicle",
  "https://eprint.iacr.org/2023/969",
  "https://github.com/NethermindEth/latticefold"
]
---

## Question

**PROPOSAL:** Decide whether folding Sherlock's repeated interpreter chunks can reduce proving cost while preserving Midnight acceptance. Treat the installed proof system, transcript, SRS and VK as hard constraints; separately describe a future redesign requiring new keys. Compare complete execution-to-ledger paths, including the final decider.

## Summary

- **PROPOSAL:** Prioritize GPU acceleration of the existing tree. None of the surveyed folding systems supplies a replacement proof under Sherlock's current fixed VK; a new folding wrapper is outside that compatibility constraint.
- **CODE:** Sherlock already accumulates child verification obligations and settles the final pairing. Folding would replace its step-proof architecture, rather than merely remove repeated pairings. See [inner.rs:143](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/inner.rs:143) and [decider.rs:186](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:186).
- **SOURCE:** CycleFold still requires a suitable cycle; its benefit is a much smaller secondary circuit. BLS12-381's embedded Jubjub/Bandersnatch curves do not automatically supply that cycle. [CycleFold](https://eprint.iacr.org/2023/1192) ([capture](../raw/07/cyclefold-paper.md)); [curve implementations](https://github.com/arkworks-rs/algebra/blob/master/curves/README.md) ([capture](../raw/07/ark-curves.md)).
- **SOURCE:** HyperNova reduces the principal commitment work to one witness-sized MSM; Mova's published advantages depend on small witness elements. Neither claim measures a Midnight wrapper. [HyperNova](https://eprint.iacr.org/2023/573) ([capture](../raw/07/hypernova-paper.md)); [Mova](https://eprint.iacr.org/2024/1220) ([capture](../raw/07/mova-paper.md)).
- **PROPOSAL:** If a future deployment permits new keys, test bounded Nova/CycleFold strands, GPU commitments, one fully checked wrapper per strand, then an outer Midnight tree. Admit this hybrid only after its complete cost beats the accelerated tree.

## Findings

### What the ledger actually accepts

**CODE:** Ledger contract verification selects the operation's registered VK and calls its verifier with the ledger-produced public inputs; the transient-crypto verifier uses `midnight_zk_stdlib::verify` and Blake2b. A valid Nova, Groth16, Spartan or unrelated KZG proof cannot be substituted at this byte boundary. References: [structure.rs:435](/home/charles/repos/midnight-ledger/ledger/src/structure.rs:435), [proofs.rs:64](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:64), [proofs.rs:545](/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:545). The local node explicitly includes ledger-9 and transient-crypto 3.0.0 dependencies: [Cargo.toml:96](/home/charles/repos/midnight-node/Cargo.toml:96).

**CODE:** The Sherlock decider verifies its fixed parent key, folds the carried accumulator, ties reassigned subgroup-checked points to its encoding, and checks the BLS12-381 pairing. It hashes selected parent statement fields and binds the communication commitment to that statement: [decider.rs:148](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:148), [decider.rs:163](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:163), [decider.rs:193](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:193). The parent key is part of the relation's serialization: [decider.rs:233](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/decider.rs:233).

**PROPOSAL:** Distinguish three compatibility levels. Replacing exact MSM/NTT arithmetic can preserve all four pins. Replacing the relation with a folding verifier can preserve Midnight's outer protocol and SRS, but needs a new VK and contract registration or deployment. Submitting folding proofs directly changes the accepted proof format and requires a ledger/verifier protocol change. Only the first level meets this project's present constraint. An unchanged decider would still require a genuine proof of its original parent relation.

**CODE:** The actual Moriarty dependency graph includes proofs 0.8.3, curves 0.3.1, circuits 7.2.4 and stdlib 2.3.5: [Cargo.lock:2030](/home/charles/repos/Moriarty-factory/Cargo.lock:2030), [Cargo.lock:2271](/home/charles/repos/Moriarty-factory/Cargo.lock:2271). The checkout and registry are distinct API surfaces: checkout `msm_specific` takes affine bases, while the installed registry function takes projective bases. Compare [checkout msm.rs:174](/home/charles/repos/midnight-zk/proofs/src/poly/kzg/msm.rs:174) with [registry msm.rs:163](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:163). Designs must pin the actual source contents as well as version numbers.

### Protocol choices

| Candidate | Evidence | Sherlock assessment |
| --- | --- | --- |
| Nova | **SOURCE:** Relaxed-R1CS folding avoids per-step SNARKs and FFTs; original prover work includes two linear-size multiexponentiations. Microsoft's implementation provides three cycles and Spartan compression. [Paper](https://eprint.iacr.org/2021/370) ([capture](../raw/07/nova-paper.md)); [implementation](https://github.com/microsoft/Nova) ([capture](../raw/07/nova-implementation.md)). | **PROPOSAL:** Best initial control implementation for repeated fixed chunks. GPU MSMs help, but sparse matrix work and an eventual wrapper remain. |
| SuperNova | **SOURCE:** Non-uniform IVC charges primarily for the selected instruction circuit and reduces to Nova for one instruction. [Paper](https://eprint.iacr.org/2022/1758) ([capture](../raw/07/supernova-paper.md)). | **PROPOSAL:** Consider only if separate opcode circuits beat a uniform chunk dispatcher. Identical chunks alone supply no reason to add non-uniform machinery. |
| HyperNova | **SOURCE:** CCS accommodates R1CS, Plonkish and AIR relations; the folding scheme supports multiple instances and one witness-sized MSM. The current paper also describes CycleFold and blinding. [Paper](https://eprint.iacr.org/2023/573) ([capture](../raw/07/hypernova-paper.md)). | **PROPOSAL:** Strong follow-up for custom gates and parallel chunk folding. Include sumcheck, matrix evaluation, hashing and witness generation in costs. An existing Halo2 circuit is not automatically an exported CCS circuit. |
| ProtoStar | **SOURCE:** Generic accumulation for special-sound protocols; its Plonk instantiation supports high-degree gates and vector lookups without prover FFTs. [Paper](https://eprint.iacr.org/2023/620) ([capture](../raw/07/protostar-paper.md)). | **PROPOSAL:** Attractive for preserving gate expressiveness, but it changes the inner argument. It does not fold existing Midnight proof bytes through a drop-in PCS adapter. |
| ProtoGalaxy | **SOURCE:** Extends ProtoStar-style folding to multiple instances with low marginal verifier work. [Paper](https://eprint.iacr.org/2023/1106) ([capture](../raw/07/protogalaxy-paper.md)). | **PROPOSAL:** Candidate for a parallel folding tree after a correct uniform implementation exists; keep its batching degree and verifier cost explicit. |
| Mova | **SOURCE:** Removes error/cross-term commitments without sumcheck; reports approximately 5–10× over Nova and 1.05–1.3× over HyperNova for the evaluated R1CS settings, assuming small witness elements. [Paper](https://eprint.iacr.org/2024/1220) ([capture](../raw/07/mova-paper.md)). | **PROPOSAL:** Do not transfer these ratios to Sherlock. Measure witness bit distributions, including hash intermediates and randomized accumulated values, before selecting it. |
| CycleFold | **SOURCE:** Uses a second curve for a roughly 1,000–1,500-multiplication scalar-multiplication circuit, rather than duplicating the full recursive verifier; both running instances can be discharged over the first curve's scalar field. [Paper](https://eprint.iacr.org/2023/1192) ([capture](../raw/07/cyclefold-paper.md)). | **PROPOSAL:** An IVC compiler/curve optimization, not a new ledger-compatible decider. Check the secondary instance too. |
| LatticeFold | **SOURCE:** Module-SIS/Ajtai folding supports R1CS/CCS and small fields; low-norm enforcement is central. Nethermind labels its implementation a proof-of-concept, without careful review, and LatticeFold+ work in progress. [Paper](https://eprint.iacr.org/2024/257) ([capture](../raw/07/latticefold-paper.md)); [implementation](https://github.com/NethermindEth/latticefold) ([capture](../raw/07/latticefold-implementation.md)). | **PROPOSAL:** Ring arithmetic is a plausible GPU target, but field translation, norm checks and a Midnight wrapper make it a separate research program. A final BLS/KZG proof forfeits an end-to-end post-quantum claim. |

**SOURCE:** Sonobe is a library, not another proof system. Its captured current support matrix marks Nova/CycleFold/Pedersen stable, HyperNova/ProtoGalaxy/Mova pending merge, KZG needing revamp, and LegoGroth16 pending merge. The package is explicitly `0.1.0-alpha.1` with unstable APIs; GPU acceleration is described as future work. These are branch-local maturity claims, not evidence that every historical Sonobe backend is currently supported. [Sonobe](https://github.com/ethereum/sonobe) ([capture](../raw/07/sonobe.md)).

**PROPOSAL:** Rank implementation risk separately from protocol elegance: use Nova as the baseline; assess Sonobe's exact pinned branch before adopting CycleFold; defer HyperNova/Mova/ProtoGalaxy until their needed implementations and deciders are reviewed. Treat LatticeFold as exploratory. No reviewed source establishes a ready Midnight wrapper for any candidate.

**PROPOSAL:** Repeated chunks make folding worth investigating because it can replace many inner SNARK productions with cheaper relation folding. Reusing an identical circuit or VK is also available to the existing tree; charge neither benefit exclusively to folding. The recommendation for this release follows compatibility and implementation risk. The performance crossover remains unmeasured.

### Curves: embedding is not cycling

**CODE:** Midnight's `Fq` is the BLS12-381 scalar field, despite naming conventions elsewhere that call this `Fr`; its modulus is 255 bits. Jubjub's base aliases that field and has a different scalar modulus. References: [fq.rs:1](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/fq.rs:1), [jubjub/curve.rs:49](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/jubjub/curve.rs:49). BLS12-381's base modulus is 381 bits: [fp.rs:1](/home/charles/repos/midnight-zk/curves/src/bls12_381/fp.rs:1).

**SOURCE:** A conventional two-curve cycle exchanges base and scalar fields. Hasse bounds a curve's point count near its base-field size. Bandersnatch, like Jubjub, instead sits over the BLS scalar field; its paper reports faster individual scalar multiplication but explicitly no MSM or R1CS improvement over Jubjub. [Cycles, §2](https://arxiv.org/html/1803.02067v3) ([capture](../raw/07/cycles-full.md)); [Bandersnatch](https://eprint.iacr.org/2021/1152) ([capture](../raw/07/bandersnatch-paper.md)).

**ESTIMATE:** Let BLS base/scalar moduli be $p,r$, with $p>2^{380}$, $r<2^{255}$. A reverse partner over $\mathbb{F}_r$ would need a subgroup of order $p$. Hasse gives total order at most $r+1+2\sqrt r<2^{256}<2^{380}<p$. Thus such a conventional BLS12-381 two-cycle partner cannot exist. This rules out the suggested BLS/Jubjub and BLS/Bandersnatch pairings; it does not rule out folding with non-native arithmetic or a different inner cycle.

**SOURCE:** Microsoft's implemented cycles are Pallas/Vesta, BN254/Grumpkin and secp/secq. Its pairing-based HyperKZG/Mercury compression is an alternative to Pedersen/IPA, with separate setup requirements. Sharing the word KZG does not identify a transcript or proof format. [Nova implementation](https://github.com/microsoft/Nova) ([capture](../raw/07/nova-implementation.md)).

**PROPOSAL:** An unrelated inner cycle can feed a BLS12-381 wrapper, but budget two field crossings: Sherlock's native scalar arithmetic/Poseidon inside the inner step relation, and the inner verifier inside Midnight's scalar-field circuit. Exact modular emulation and canonical encodings are required. Merely mapping field elements modulo BN254 changes the statement.

**PROPOSAL:** A single-curve construction over BLS G1 could retain Sherlock's native scalar field and emulate its own 381-bit group arithmetic. This avoids inventing a curve cycle but sacrifices CycleFold's cheap native secondary arithmetic and needs a separately justified implementation. Jubjub or Bandersnatch alone instead makes their own scalar field the folding relation's field; native group coordinates in the outer wrapper do not make Sherlock's original field arithmetic native inside that relation.

### Existing accumulation and GPU opportunity

**CODE:** The upstream checkout already has `IvcContext`, `IvcState`, `IvcIO` and an IVC prover. Its `prove_step` prepares the previous PLONK proof, accumulates its obligation, then makes another stdlib PLONK proof: [ivc/mod.rs:55](/home/charles/repos/midnight-zk/aggregation/src/ivc/mod.rs:55), [ivc/prover.rs:97](/home/charles/repos/midnight-zk/aggregation/src/ivc/prover.rs:97), [ivc/prover.rs:133](/home/charles/repos/midnight-zk/aggregation/src/ivc/prover.rs:133). This is proof accumulation, not Nova's relaxed-R1CS prover. Its application `decider` also has off-circuit duties: [ivc/verifier.rs:55](/home/charles/repos/midnight-zk/aggregation/src/ivc/verifier.rs:55).

**SOURCE:** The captured open-ICICLE distribution supports BN254/BLS12-381 MSM, NTT and ECNTT; Grumpkin and sumcheck are explicitly outside its present scope. Older broader ICICLE matrices cannot establish support in this distribution. [open-ICICLE](https://github.com/ingonyama-zk/open-icicle) ([capture](../raw/07/open-icicle.md)).

**PROPOSAL:** GPU-friendly folding means resident commitment bases, vector combinations, sparse matrix operations and sufficiently large MSM batches. Sequential Fiat–Shamir rounds still create dependencies. For BN254/CycleFold, initially keep the tiny Grumpkin side on CPU; batch independent strands to occupy the GPU. HyperNova changes the balance toward matrix/sumcheck kernels; Mova reduces commitment work and therefore may reduce the benefit of MSM acceleration. LatticeFold needs ring-specific kernels, not BLS MSM substitution.

## Proposed design

### Current release: accelerate the fixed relation

**PROPOSAL:** Introduce a prover-internal `ArithmeticBackend` with `msm_bls381(scalars, bases, basis_id)` and `ntt_bls_scalar(values, domain, direction)` operations and CPU fallback. Cache bases by SRS digest, degree, coefficient/Lagrange basis and encoding. Keep transcript writes, challenge ordering, randomness consumption, constraints and serialized output unchanged.

**CODE:** Concrete installed hooks are `PolynomialCommitmentScheme::{commit, commit_lagrange}` at [commitment.rs:43](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/commitment.rs:43), implemented by KZG at [kzg/mod.rs:74](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74), and their shared `msm_specific` dispatch. The installed BLS path already invokes blst Pippenger: [msm.rs:176](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/msm.rs:176). Domain transformations call `best_fft`: [domain.rs:224](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/domain.rs:224). Preserve `multi_open`, which also invokes commitments, rather than importing another library's opening argument.

**PROPOSAL:** Use the existing `create_proof` entry point and backend dispatch underneath it. Avoid a new PCS type initially: VK/PK types are generic over the PCS, so an in-place arithmetic dispatch reduces serialization/type migration. See [prover.rs:348](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/plonk/prover.rs:348). Use [[loki-02-gpu-msm]] and [[loki-03-gpu-ntt]] for kernel/backend choices.

**CODE:** Local CUDA Rust supplies host runtime and kernel tooling but describes itself as early-stage research: [README.md:8](/home/charles/repos/nvidia-cuda-rust/README.md:8), [README.md:33](/home/charles/repos/nvidia-cuda-rust/README.md:33). The separate Halo2 checkout has a similar PCS trait but different `multi_open` interface: [commitment.rs:49](/home/charles/repos/halo2/src/poly/commitment.rs:49). Neither source proves drop-in compatibility.

### Conditional hybrid: bounded folding strands plus a Midnight tree

**PROPOSAL:** This design requires new wrapper, parent and terminal VKs and corresponding contract authorization. It is a future experiment, not a solution satisfying the fixed-VK requirement.

**PROPOSAL:** The direct “fold then wrap once” variant sets the strand length to the entire execution and puts final folding verification, endpoint checks and the exact ledger-call template into one Midnight relation. It produces a normal BLS/KZG Blake2b proof under a new VK. There is no reason to impose Sherlock's old deferred KZG pairing on a Nova accumulator: those are different relations. Retain a KZG decider only for genuine Midnight recursive proofs still present in a mixed design. The strand/tree variant below trades extra wrappers for parallelism and bounded recovery.

1. **PROPOSAL:** `sherlock_fold::StepAdapter` defines a fixed-capacity `advance(z, chunk_witness) -> z_next` relation. Carry program/context commitments, complete state commitments, fuel/work, ordered trace, step count and terminal status. Prove state openings, instruction selection, padding, Init and finalization in the relation. Start with one uniform bounded chunk; only later compare SuperNova dispatch.
2. **PROPOSAL:** `FoldSession::{start, push_chunk, finish}` uses a pinned Nova/CycleFold implementation on BN254/Grumpkin. Preserve original BLS scalar operations through checked emulation where needed. `finish` returns `FoldReceipt { parameters_digest, shape_digest, z0, zn, count, primary, secondary, witnesses }`. GPU commitments use resident BN254 bases; secondary work initially stays on CPU.
3. **PROPOSAL:** `MidnightFoldWrap: Relation` checks the entire final folded relation. The first correctness prototype takes accumulated witnesses, checks primary and secondary satisfiability and commitment openings, and checks every IVC consistency/hash binding, final incoming strict instance and last-fold check required by the selected construction. This costs work proportional to the step relation, though independent of strand length. An accumulator digest or an off-circuit `verify()` result is insufficient. A later compressed-proof verifier may replace this linear check, with its foreign-field/IPA or pairing cost measured separately.
4. **PROPOSAL:** Invoke `setup_vk/setup_pk/prove` for that new relation, use Poseidon for recursively consumed wrapper proofs, and feed wrappers into a new parent that enforces adjacent endpoints, one program/context, ordered counts and trace composition. Finish with the existing style of subgroup/pairing/statement-binding decider and Blake2b outer transcript. The ledger receives only Midnight proof bytes under the newly authorized key.

**CODE:** The wrapper extension point is `Relation::{format_instance, circuit, used_chips}`, not `VerifierGadget::prepare`, which parses PLONK/KZG proofs: [stdlib lib.rs:1795](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1795), [inner.rs:124](/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/inner.rs:124). Standard proving APIs are at [stdlib lib.rs:1946](/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1946). These are proposed modules; no adapter was implemented or compiled.

**PROPOSAL:** Concretely, the raw Nova check enforces `Az ∘ Bz = uCz + E`, opens the witness/error commitments against the pinned bases, and verifies the recursive input/output hash bindings. Apply the selected construction's corresponding check to its secondary instance too. This is a large final satisfiability circuit, not the small folding verifier used between steps. Replacing it with Spartan verification changes the circuit and key again; compression is a measured design choice.

## Expected speedup

**CODE:** The inspected k20 budget has no recorded timing, and the dated evidence describes a fixed A102→A80 parent, not arbitrary repeated interpreter chunks: [budgets.json:17](/home/charles/repos/Moriarty-factory/qa/sherlock/budgets.json:17), [recursion-evidence-2026-10-22.md:7](/home/charles/repos/Moriarty-factory/docs/sherlock/reference/recursion-evidence-2026-10-22.md:7). The Push emitter also explicitly lists unproved caller assumptions and incomplete dispatch: [c6_push_step.rs:31](/home/charles/repos/Moriarty-factory/crates/mori-backend/src/sherlock_machine/c6_push_step.rs:31).

**ESTIMATE:** Use the owner's >500-second attempt only as a scenario scale, not a completed baseline. For an illustrative 500-second workload, assume 70% accelerable arithmetic, 8× arithmetic acceleration and 10 seconds overhead. Then $T=500(0.30+0.70/8)+10=203.75$ seconds, or $500/203.75=2.45\times$. If the accelerable fraction is only 40%, $T=335$ seconds and speedup is 1.49×. These are Amdahl scenarios, not RTX 5090 measurements.

**ESTIMATE:** For future strands let accelerated tree cost be $T_t=N a+D_t$, and hybrid cost be $T_h=N f+(N/b)W+D_h$. Here $a$ includes amortized leaf/tree work, $f$ includes folding, emulation and witness work, $b$ is chunks per wrapper, and $W$ is wrapper cost. If $a=5s,f=0.8s,W=120s,D_t=D_h$, folding wins only when $b>120/(5-0.8)=28.6$. At $N=1024,b=64,D=120s$: tree $=5240s$, hybrid $=819.2+1920+120=2859.2s$, a 1.83× work reduction. At $W=300s$, hybrid $=5739.2s$, losing. These are deliberately hypothetical; parallel wall time needs the one-GPU schedule in [[loki-08-recursion-tree-scheduling]].

**ESTIMATE:** The reported 120-second VK build contributes $120/P$ seconds per proof if reused over $P$ proofs: 1.2 seconds at $P=100$. At k20, one 32-byte scalar vector is $2^{20}\cdot32=32$ MiB; two affine BLS G1 basis arrays cost $2\cdot2^{20}\cdot96=192$ MiB before scratch, columns and copies. The reported 12-GiB CPU RSS does not establish GPU residency or wrapper size. No numerical folding speedup is established for the current two-child parent.

## Risks and soundness concerns

- **SOURCE:** The original two-cycle Nova implementation had a soundness vulnerability; the revisiting paper supplies a repaired construction and notes proof malleability. This is evidence to pin a proven construction, not a claim that current Nova remains broken. [Revisiting Nova](https://eprint.iacr.org/2023/969) ([capture](../raw/07/nova-cycle-security.md)).
- **PROPOSAL:** Bind parameters, circuit shape, program, genesis, count, endpoint, order and terminal mode inside the proof. Check both CycleFold instances and the final accumulated relation; blinding or compression must follow the chosen construction.
- **PROPOSAL:** Preserve exact BLS modular arithmetic, range bounds and Poseidon parameters across field emulation. Host witness rejection does not establish circuit rejection. Folding an incompletely constrained interpreter preserves its gaps.
- **PROPOSAL:** Keep untrusted GPU arithmetic outside the trusted acceptance boundary: verifier-check every produced proof, handle device failures explicitly, and preserve canonical point/subgroup checks. Do not replace integer modular kernels with approximate floating-point arithmetic.
- **PROPOSAL:** Treat raw folding receipts as private until their hiding properties are established. A zero-knowledge outer proof does not make separately published accumulated witnesses private.

## Validation plan

**PROPOSAL:** These are future predicates, not checks executed during this research task:

1. **PROPOSAL:** `GPU_ARITH_EQ`: compare CPU/GPU MSM and NTT outputs on zeros, identities, sparse/full-width scalars, maximal canonical values, varied sizes and domain round trips. Compare normalized group elements, not projective coordinates.
2. **PROPOSAL:** `FIXED_VK_ACCEPT`: with identical SRS/VK/circuit/instance and controlled RNG, compare full CPU/GPU proofs and transcript checkpoints. Require byte identity when randomness/order match; also require acceptance by the unchanged independent reference verifier. Repeat with fresh randomness, where byte identity is unnecessary.
3. **PROPOSAL:** `FOLD_STEP_EQ`: compare interpreter transitions, existing circuit witnesses and folded transitions on a bounded corpus. Mutate dispatch, range carries, inactive lanes, fuel, context and memory/state openings. Require in-circuit failure for invalid transitions.
4. **PROPOSAL:** `WRAP_COMPLETE`: compare native folding verification with wrapper satisfaction; mutate each primary/secondary accumulator, commitment, witness, parameter digest, count and endpoint. A valid outer proof must be unobtainable for each invalid receipt.
5. **PROPOSAL:** `TREE_BINDING` and `LEDGER9_ACCEPT`: reject omitted/reordered/duplicated strands and wrong predecessor roots; verify the exact contract-call template and communication commitment using ledger-9, then perform the authorized stagenet integration. Check reference and accelerated versions under the same registered key for the current design; a hybrid requires its separately registered new key.
6. **PROPOSAL:** `TOTAL_COST_WIN`: measure setup, witness, fold/leaf, wrapper, tree, terminal, transfers, RSS and GPU memory separately, with cold/warm reuse and identical semantics. Run any later heavy work through the factory's measured-cap route; this study ran no builds or benchmarks.

## Open questions

- **OPEN QUESTION:** What are actual chunk counts, constraint shapes, full-width field/hash frequencies and reusable program/state capacities for representative financial executions?
- **OPEN QUESTION:** Is a new terminal VK ever permissible? Under the current fixed-VK constraint, folding substitution remains excluded.
- **OPEN QUESTION:** Can a fully checked native-witness wrapper fit a competitive degree after both field crossings? What does compressed-proof verification cost instead?
- **OPEN QUESTION:** Which exact Sonobe/Nova commit, GPU backend and driver combination is reproducible on this Blackwell/WSL2 host? Neither CUDA version nor a generic benchmark establishes that.
- **OPEN QUESTION:** How much time remains in witness synthesis, foreign ECC constraints and final-decider proving after BLS MSM/NTT acceleration? That residual determines folding's opportunity.

## Links

**PROPOSAL:** Read [[loki-01-prover-profile]] for measured fractions; [[loki-02-gpu-msm]] and [[loki-03-gpu-ntt]] for kernels; [[loki-04-halo2-gpu-precedents]] for backend precedents; [[loki-05-recursion-accumulation]] and [[loki-06-recursive-circuit-cost]] for the existing deferred pairing; [[loki-08-recursion-tree-scheduling]] for parallel wall time; [[loki-09-cpu-witness-field]] for the non-GPU remainder; and [[loki-10-integration-verification]] for ledger predicates.

**SOURCE:** Primary web evidence is linked beside each claim with its Scrapling capture under `raw/07/`. Paper landing pages support protocol-level statements; implementation README captures support only their stated branch capabilities and maturity, not an independent audit or host performance result.
