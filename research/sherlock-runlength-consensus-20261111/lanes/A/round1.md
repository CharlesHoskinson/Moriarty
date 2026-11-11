# Lane A — Round 1 Independent Study: Prior Art & Deep Web Lead (`sherlock-runlength-consensus-20261111`)

Date: 2026-11-11
Scope: Independent Round 1 technical study answering Q1–Q5 of `research/sherlock-runlength-consensus-20261111/BRIEF.md:25-43`, leading on Q4 (production and 2025–2026 zkVM prior art on run-length bounds, cross-segment memory persistence, and on-chain public statements). All repository citations are verified 1-based `path:line` references in `/home/hoskinson/Projects/Moriarty`. Every unmeasured projection is marked `ESTIMATE`.

---

## Executive Summary of Lane A Positions (Q1–Q5)

| Question | Lane A Position | Core Technical Basis |
| :--- | :--- | :--- |
| **Q1 (`GP-1`, `GP-001`, meaning of `2^64` & "unbounded")** | Clarify in `general-purpose.md` GP-1 and EARS `GP-001` that fixed-size leaf proofs (`S` steps, `k = 13..16`) fold linearly (`O(n_steps)` prover work, `O(1)` final proof size) under one fixed key set with **no `2^64`-row circuit**, and that **`2^64` (`SRC-004`) is solely a 64-bit step-counter and address non-wraparound guard in the field**, never a physical target or circuit domain. Never narrow GP-1..GP-4. | In `crates/mori-recursion/src/ivc.rs:359-376`, `constrain_heights` takes ~30–50 rows in a `k = 20` node (`research/sherlock-speed-replan-20261108/prover.md:181-186`) to prevent `h = prev_h + steps` from wrapping modulo $r_{\text{BLS12-381}} \approx 2^{255}$ back to `0`. Every global-clock zkVM in Q4 (SP1 v6 `2^48` clk, Ziren `~2^39.7`, Pico `2^39`, Airbender `2^38`, zkWasm `2^32-1`) uses an identical fixed-width range guard while proving small fixed-size shards. |
| **Q2 (D3 ceiling: keep (a), widen (b), non-per-leaf (c), or better (d))** | **Remove D3's per-leaf segment burn and 17-bit ceiling** by adopting **Q2(d) `M3-Dense`** (*Write-Back Tail-Buffered Dense Segment Memory* + 61-bit top tree `TOP = 61`, or 29-bit `TOP = 29` variant + Carbon cons-linked slices). Reject Q2(a) (GP-2 run-chaining as a workaround for D3) and pure Q2(b) without dense buffering. | Q2(a) conflicts with `SVM-010`, `SRC-006`, `GP-002`, and `SRC-023`. Pure Q2(b) adds **+0 node/decider columns** (`+4` to `+12` node rows, `+0` decider rows, `ESTIMATE` from E2 `experiments/e2-e4.md:25-35`) yet still burns 8 addresses on 0-append leaves and leaves `len`/`fp`/`mem_root` partition-dependent. `M3-Dense` (`W = 15` MEM rows + 1-word `tail_d` in `pre_d`/`post_d`) burns **0 segments on 0-append leaves**, makes physical `pre_d`/`post_d` **100% partition-independent**, eliminates cross-leaf padding holes (`SI-12..SI-14`), keeps node at **`~824k–830k` rows (`k = 20`, `ESTIMATE`)** and decider unchanged at **`929,166` ordinary / `1,032,467` table rows (`k = 20`)**, and removes every run-length and cell ceiling below `2^64`. |
| **Q3 (Inter-run continuations, canonical state digest, `input_d`, GP-2, C6)** | Use **intra-run `SEG + SEG -> SEG` continuations** (hidden inside recursion) for unbounded single runs. If inter-run continuations (`k -> k+1`) are ever used (e.g., voluntary multi-tx suspension in profile `/2`), run `k` must halt at a deterministic logical checkpoint (`fuel == 0` / `Yield`) emitting an address-free `sherlock-value/1` machine-state value `ms_v` (`ms_d = vd(ms_v)`), consumed via `SRC-021` `input_d` + `GP-002` in-circuit claim verification and aggregated via `N` (`SET` DAG) into one `C/D` proof. | `SRC-006` (`requirements.md:31`) and `GP-002` (`requirements.md:57`) forbid sealing `Running` states or accepting segment/state digests as GP-2 claims. Under `SVM-012` (`requirements.md:20`), heap values already store creation-time `vd`, so checkpointing folds only live control + `env` + `frame` + `task` chains ($O(\text{stack} + \text{env})$ steps). However, using GP-2 run-chaining to work around physical capacity either leaks checkpoint claims in `SET` or requires rewriting guest programs into fuel-sliced trampolines (violating `GP-1`). |
| **Q4 (Prior art across 17 production & 2025–2026 zkVMs)** | Every SNARK-wrapped production zkVM (**RISC Zero, SP1 v4/v6.1, OpenVM v1/v2.1, Ziren, Brevis Pico v3.1**) **hides final memory digests, segment counts, and cycle counts on-chain**, publishing only `(programId/vkey, outputDigest)` (+ fixed `vkRoot`/`exitCode=0`/`nonce`). Boundary-Merkle zkVMs (`RISC Zero`, `OpenVM`, `Nexus 1.0/2.0`) achieve unbounded run length and full partition independence; offline-memory zkVMs (`SP1`, `Ziren`, `Pico`, `Airbender`, `Valida`) require verifier challenges unavailable inside Sherlock's leaf (`verifier_gadget.rs:16-17`) and impose hard global clock bounds (`2^32..2^45`). | Because `midnight-circuits` 7.3.0 supports **zero user challenge phases** (`verifier_gadget.rs:16-17`, cited at `study.md:254`) and **unit rotations `-1, 0, +1`** (`verifier_gadget.rs:609`, `design.md:122`), Sherlock's leaf must use deterministic Merkle-authenticated boundary memory (the RISC Zero / OpenVM / Nexus-PCD class), where widening tree depth and eliminating empty-leaf segment consumption gives full partition independence and `< 2^64` run length. |
| **Q5 (Practical run length on this host & spec tested target)** | On this 20-core ARMv9 / 120 GiB GB10 host (`CPU` today, `S = 16`), marginal proving time is **`~4.0–4.5 s / owner step` (`ESTIMATE`)** sequentially (`~2.7 s/step` `ESTIMATE` with 2–3 concurrent binary `N` workers) at **`O(1)` (`~17.5–19 GB`) peak process RSS**. Keep `<= 320` steps (`fact(3)` 293, `list123` 303) in `< 2 h` for `SRC-011`/`SRC-015(c)`, and add a **GP-1 multi-segment continuation target of `>= 1,024` steps (`>= 64` leaves) and `>= 256` allocated cells (`>= 32` segments) in `< 2 h`** (`~75 min` `ESTIMATE`). | Measured `k = 20` node step is `59.62–63.61 s` (`16.58–17.35 GB` RSS, `qa/sherlock/runs.jsonl:648-649`); decider is `70.35–77.90 s` (`runs.jsonl:617,649`); `k = 13` leaf is `2.55 s` (`prover.md:11`); `k = 20` leaf is `66.2 s` (`11.14 GB`, `runs.jsonl:709`). RAM does not grow with run length; wall-clock time bounds 2 h runs to `~1,600` steps and 24 h runs to `~20,000` steps (`ESTIMATE`). |

---

## 1. Question 4 (Lead Deep Study): How Production & 2025–2026 Frontier zkVMs Bound and Extend Run Length and Memory

### 1.1 Taxonomy of the Four zkVM Memory-Continuation Paradigms

Across 17 production and 2025–2026 research zkVMs audited from primary Rust/C++/Solidity source code, specifications, security audits, and cryptographic papers, run-length scaling and cross-segment memory persistence fall into **four distinct cryptographic paradigms**:

1. **Boundary-Merkle Continuation + Local Intra-Segment Memory (`RISC Zero rv32im-v2`, `OpenVM v1/v2.1`, `Nexus 1.0/2.0 PCD`, and Sherlock §2.7)**:
   - Each segment proves a fixed-height trace using either segment-local permutation/lookup arguments whose clock resets every segment (`RISC Zero`, `OpenVM`) or direct Merkle-authenticated bus/step proofs (`Nexus 1.0/2.0`, Sherlock §2.7).
   - At segment boundaries, persistent architectural state (registers + RAM) is committed into a collision-resistant **Poseidon/Poseidon2 Merkle root** (`SystemState.merkle_root` in RISC Zero; `VmPvs.{initial_root, final_root}` in OpenVM; `z[33] = root` in Nexus; `pre_d`/`post_d` in Sherlock).
   - **Run-length ceiling**: **Arithmetically unbounded** unless an explicit step counter is carried (RISC Zero `join.zkr` has **zero** cycle/segment counters; OpenVM's 8-bit `recursion_depth` cap allows $4 \times 3^{257} \approx 1.67 \times 10^{123}$ segments; Sherlock carries a 64-bit `h < 2^64` counter in `SRC-004`).
   - **Sealing & Partition Independence**: Every segment proof is **immediately self-contained and cryptographically sealed** ($O(\text{touched pages/cells} \cdot \log |M|)$ boundary work, $O(0)$ untouched-memory scan), giving **100% partition independence** when intermediate roots are telescoped (`L.post == R.pre`) and erased at terminal wrap.

2. **Global Offline Multiset / Elliptic-Curve Sum Continuation (`SP1 v4/v6.1`, `Ziren`, `Brevis Pico v2/v3.1`, `Valida v1.0`, `ZKsync Airbender`, `Polygon ZisK`, `Lambda VM`, `Spice / Nebula / Protostar`)**:
   - **Zero Merkle hashing** occurs at execution shard boundaries. Instead, intra-shard offline memory checking (`BEGKN92` / `Spice` / `LogUp` / `LogUp-GKR`) connects to a cross-shard global accumulator: either Poseidon2-to-elliptic-curve point sums on **`SepticCurve<F>`** over $\mathbb{F}_{p^7}$ (`SP1`, `Ziren`, `Pico`), two-pass subtree pre-commitments (`Airbender`), or a multi-segment `persistent_mem_bus` (`Valida`, `ZisK`, `Lambda VM`), closed by dedicated `MemoryGlobalInit` / `MemoryGlobalFinalize` (`IS`/`FS`) shards scanning all uniquely touched addresses in strictly ascending order (`prev_addr < addr`).
   - **Run-length ceiling**: **Hard in-circuit arithmetic ceiling ($2^{32}$ to $2^{45}$ instructions)** dictated by the range-checked bit-width of the **strictly increasing global timestamp** (`clk < 2^48` / `CLK_INC = 8` $\implies 2^{45}$ insns in SP1 v6; `(u16, u26)` $\implies 2^{39.7}$ in Ziren; `(u16, u25)` $\implies 2^{39}$ in Pico; `(u19, u19)` $\implies 2^{36}$ cycles in Airbender).
   - **Theorem-level sealing limitation**: Until the global `MemoryGlobalFinalize` (`FS`) sweep over all uniquely touched addresses closes $\mathsf{IS} \cup \mathsf{WS} = \mathsf{RS} \cup \mathsf{FS}$, **zero intermediate shards are sound on their own** (a shard can read a forged `(addr, val_fake, ts_fake)` into `RS` that remains undetected until `FS` enforces conservation of exactly 1 final write per unique address). Moreover, every variant requires **verifier randomness (Fiat–Shamir lookup/multiset challenges)** or extension-curve EC additions not natively available in a zero-challenge-phase leaf.

3. **Full-Snapshot Column-Commitment Chaining (`zkWasm`, `Sharded Twist` ePrint `2025/105` §2.9.3)**:
   - Each slice materializes the **entire** linear memory array (`img_col` and `post_img_col` in zkWasm; `Val_0` and `Val_final` in Sharded Twist) and links slices in the recursive aggregator via direct commitment equality ($C_{\text{post}}^{(s)} == C_{\text{pre}}^{(s+1)} \in \mathbb{G}_1$).
   - **Trade-off**: Zero in-circuit Merkle hashes and $O(1)$ intra-slice access, **at the cost of $O(|M|_{\max})$ prover work on EVERY slice**, restricting zkWasm linear memory to `1–31 MiB` ($k = 18..22$) and capping total run length at `eid: u32` ($2^{32} - 1$ steps).

4. **Monolithic Streaming / Write-Once Relocation (`Jolt Twist+Shout`, `Cairo Stone & Stwo`, `Miden VM`, `Nexus 3.0 Stwo`, `PetraVM / Binius64`)**:
   - Proves an entire invocation in a **single proof** without mutable-RAM continuations: either streaming a monolithic multilinear sumcheck (`Jolt` up to $2^{24}\text{–}2^{31}$ cycles; `Lambda VM` prove-and-retire), bounded by field two-adicity / circle-group order (`Miden` $2^{29}$ cycles; `Stwo` / `Nexus 3.0` $2^{28}\text{–}2^{30}$ rows), or using write-once relocated memory (`Cairo`) where cross-run state is passed explicitly as application-level Sparse/Patricia-Merkle state roots across recursive PIE trees (`SHARP`, `Miden` Batch/Block kernels).

---

### 1.2 Master Comparative Matrix Across 17 Production & Frontier Systems

| # | System & Version | Field / PCS | (i) Per-Segment Ceiling | (i) Total Run Ceiling | (ii) Cross-Segment Memory Persistence | Partition Independent? | (iii) Published On-Chain (Memory & Length) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **1** | **RISC Zero** `rv32im-v2` (`v3.1`) | BabyBear FRI + BN254 Groth16 | `po2` $\in [14, 24]$ lift; **on-chain `[14, 22]`** ($\le 4.19\text{M}$ cycles/seg) | **UNBOUNDED** in circuit (`join.zkr` has 0 counters); host `u32` segs, PoVW `u64` | Intra-seg permutation + **depth-22 Poseidon2 binary Merkle tree** (`1 KiB` pages; regs in kernel RAM) | **YES** | `(seal, imageId, journalDigest)` $\to$ 5 Groth16 scalars. **Length: NO. Final RAM: NO** (`post = ZERO_DIGEST`). |
| **2** | **SP1** `v4 Turbo` / **`v6.1 Hypercube`** | v4: BabyBear FRI<br>v6: KoalaBear MLE + Jagged PCS + Groth16/Plonk | v4: $2^{22}$ cycles<br>v6: `HEIGHT = 2^22`/chip, `402.65M` cells | v4: `(u16, u24)` $\implies \mathbf{2^{38}}$ cycles<br>**v6: `u48` `clk` (`/8`) $\implies \mathbf{2^{45}}$ insns** | Intra-shard LogUp-GKR + cross-shard `MemoryLocal` + `MemoryGlobalInit/Finalize` on **`SepticCurve` ($\mathbb{F}_{p^7}$)** | **YES** | `(programVKey, publicValues, proofBytes)` $\to$ 5 scalars (`vkey, pvDigest, exitCode=0, vkRoot, nonce`). **Length: NO. Final RAM: NO.** |
| **3** | **OpenVM** `v1.0` / **`v2.1`** | BabyBear WHIR + Stacked MLE + Halo2 BN254 KZG | `timestamp < 2^29` ($\lesssim 2^{27}$ insns); stacked height $2^{24}$ | `MAX_RECURSION_DEPTH = 256` (`u8`, arity 3) $\implies \mathbf{4\cdot 3^{257} \approx 10^{123}}$ segs (**UNBOUNDED**) | Intra-seg offline bus + **depth-29 Poseidon2 Merkle tree** (8-cell chunks; regs `AS=1`, RAM `AS=2`, PVs `AS=3`) | **YES** | `(publicValues, proofData, appExeCommit, appVmCommit)` $\to$ `14 + L` KZG scalars. **Length: NO. Final RAM: NO** (`final_root` pruned). |
| **4** | **Ziren** (ex-`zkMIPS`) | KoalaBear LogUp-GKR + WHIR + Groth16 | `460M` cells, $2^{22}$ rows/chip, **`clk < 2^26` (`/5`) $\implies 13.42\text{M}$ insns** | `shard < 2^16` (`u16`) $\implies \mathbf{8.80 \times 10^{11}}$ ($\approx 2^{39.7}$) insns | Intra-shard LogUp-GKR + `MemoryLocal` + `MemoryGlobalInit/Final` on **`SepticCurve`** + 2-PC MIPS delay-slot link | **YES** | `(programVKey, publicValues, proofBytes)` $\to$ 3 scalars (`vkey, pvDigest, VK_ROOT`). **Length: NO. Final RAM: NO.** |
| **5** | **Brevis Pico** `v2` / **`v3.1`** | KoalaBear ($\mathbb{F}_{p^5}$) MLE + WHIR + Groth16-BSB22 | `MAX_LOG_CHUNK_SIZE = 23` ($8.39\text{M}$ rows); `clk < 2^25` | `chunk < 2^16` (`u16`) $\implies \mathbf{5.50 \times 10^{11}}$ ($\approx 2^{39}$) cycles | Regional LogUp-GKR + `has_initial` fresh-zero opt + `MemoryLocal` + `MemoryInitFinalize` + `GlobalChip` | **YES** (whitelist EVM mode) | `(riscvVkey, publicValues, uint256[12] proof)` $\to$ **2 scalars** (`vkey, pvDigest`; `vk_root` in R1CS). **Length: NO. Final RAM: NO.** |
| **6** | **Jolt** (`0.1` $\to$ **`Twist+Shout`**) | BN254 `Fr` or `AkitaField` ($2^{128}$); Dory / HyperKZG / Akita | Monolithic: default $2^{24}$ cycles (`Akita` up to **`2^31`**, `Dory` $\le 2^{65}$ evals) | Monolithic streaming sumcheck (no continuation shards on `main`) | Intra-run **Twist** increments $\widetilde{\mathsf{Inc}}(j)$ + $\widetilde{\mathsf{LT}}$ virtual $\widetilde{\mathsf{Val}}$ sumcheck (zero Merkle/grand products) | **N/A** (monolithic; `trace_length` public) | No Solidity contract on `main` yet; `JoltStatement` exposes **`trace_length` ($T$)**, **`ram_K` ($K$)**, `preprocessing_digest`, I/O. |
| **7** | **Cairo** (`Stone` & **`Stwo`**) | Stone: Stark252<br>Stwo: $M_{31}$ Circle STARK | Stone: `log_n_steps < 50` ($\sim 2^{24..27}$ RAM)<br>Stwo: $2^{25}$/table, $N B^2 \le 2^{30}$ | **UNBOUNDED** across PIEs via **SHARP** (Stwo recursive tree + Stone L1 root) | Write-once relocated memory (`2^29` words in Stwo); cross-PIE persistence via explicit Starknet Patricia-Merkle roots | **Leaves hidden**; root `log_n_steps` public | L1 `CpuVerifier` + `MemoryPageFactRegistry`: root **`log_n_steps`**, segment stop ptrs, **Continuous Page `(startAddr, size, keccak, prod)`**, `(progHash, outFact)`. |
| **8** | **Miden VM** `v1.0-rc1` | Goldilocks Lifted STARK (`LOG_BLOWUP = 3`) | **`MAX_CYCLES = 2^29`** ($2^{29}\cdot 2^3 = 2^{32}$ two-adicity + `u32` `clk`) | Unbounded at rollup layer via **MASM + ACE** recursive STARKs (Tx $\to$ Batch $\to$ Block) | Intra-run sorted Memory Chiplet; cross-tx **depth-64 Sparse Merkle Trees (`SMT`)** + Note MMRs | **YES** (`log_heights` hidden in `ExecutionClaim`) | `ExecutionClaim` = 40 felts (`program_hash, kernel_root, stack_in, stack_out`); L1 AggLayer bridge posts state/exit roots (**0 lengths/pages**). |
| **9** | **Valida** `v1.0.0` | BabyBear Plonky3 FRI (`log_blowup = 1`) | FFT $\le 2^{26}$; default `--max-segment-size = 2^20` **mem ops** ($\le 2^{21}$ buffer) | `3 + segment_number < p` ($\le 2.01 \times 10^9$ segs; $O(S)$ proof size, no multi-seg recursion yet) | Global cross-segment LogUp `persistent_mem_bus` (`PersistentSend`/`Receive` + `is_final` pruning); RAM `< 0x3C000000` (960 MiB) | **NO** (`segment_proofs.len()` & `pc/fp` public) | No Solidity verifier in public repos; native verifier exposes all `SegmentProof`s, `segment_number`, `(pc, fp)`, and `stdout`. |
| **10** | **Nexus** `1.0/2.0` $\to$ **`3.0`** | 2.0: BN254/Grumpkin Nova/HyperNova<br>3.0: $M_{31}$ Stwo | 2.0: `K = 16` (Nova) or `1` (HyperNova)<br>3.0: **Monolithic** `log_size <= 28..29` | 2.0: `i < p` (`usize` host) PCD<br>3.0: `< 2^{28..29}` (**continuations unimplemented** in `v0.3.6`) | 2.0: **height-27 Poseidon Merkle** (81 perms/insn)<br>3.0: two-pass linear RAM + global LogUp + boundary tables | 2.0: **YES** (PCD)<br>3.0: **NO** | No Solidity verifier in repo. 2.0 `PCDNode` exposes `(i, j, z_i, z_j)`; 3.0 `view` exposes code, initial RAM, I/O, `log_sizes`. |
| **11** | **zkWasm** (Delphinus) | BN254 Halo2 KZG + `halo2aggregator-s` | $k \in \{18,20,22\}$: `57,056` / `253,664` / `1,040,096` steps; `1 / 7 / 31 MiB` RAM | **`eid: u32` $\implies \mathbf{2^{32}-1}$ steps**; `--cont c` prebuilds $2^c$ VK chain hashes | **KZG Lagrange column chaining** (`C_post_img == C_img` in $\mathbb{G}_1$, 3 `Fr` limbs; pays $O(2^k)$ rows every slice) | **Parallel leaves**, `final_hash` binds round topology | `AggregatorVerifier.sol`: 1 Keccak-hashed scalar over `target_instance` + `[vk_chain_hash, img_init_G1_3limbs, ctx_out_G1_3limbs]`. |
| **12** | **ZKsync Airbender** | Mersenne-31 FRI + recursive STARK | Per-chunk trace (`~10^6` cycles); delegation circuits separate | `(u19, u19)` **`2^38` timestamps (`/4`) $\implies \mathbf{2^{36}}$ cycles** ($6.87\times 10^{10}$) | Two-pass **memory-subtree pre-commitment** (~5–10% extra work) + lazy `u32` sorted init/teardown permutation | **YES** after recursion | Recursive verifier checks cycle/row bounds $< |F|$ and publishes program + I/O state transition on ZK Stack L1. |
| **13** | **Polygon ZisK & Lambda VM** | Goldilocks / BabyBear STARKs | Fixed-height per-chip slices (`ZisK`) or epochs / 2-pass chunks (`Lambda VM`) | Bounded by global continuation timestamp width (`ZisK` / `Lambda` epochs) or host compute (`Prove-and-retire`) | **ZisK**: decoupled per-chip slicing + continuation bus.<br>**Lambda VM**: cross-epoch `local-to-global` bus OR 2-pass **Prove-and-Retire** monolithic proof | **ZisK**: YES at root.<br>**Lambda**: epochs O(E); P&R = 1 proof | ZisK root / SNARK wrapper publishes `(programVk, publicOutputs)` without internal slice counts. |
| **14** | **Twist & Shout** (`ePrint 2025/105`) | Multilinear Sumcheck + Dory / Nova | Monolithic $2^{24..32}$; paper §2.9.3 shards at $2^{19..20}$ | Monolithic $2^{32}$ (Dory); Sharded + Nova (§2.9.3): unbounded / field-bounded | One-hot $d$-dim `ra`/`wa` + write increments $\widetilde{\mathsf{Inc}}$. **Sharded Twist costs $O(|M|)$ per shard** to bind $S'$ to $\mathsf{Inc}$! | Sharded: **YES** (at $O(|M|)$/shard cost) | Research / `jolt-verifier` (`trace_length`, `ram_K` public unless SNARK-wrapped). |
| **15** | **Neo / SuperNeo / LatticeFold+ / Symphony** | Module-SIS Ajtai over $\mathcal{R}_q$ / Goldilocks ($2^{64}$) | Arbitrary CCS step; Symphony folds $\ell_{\text{np}} \ge 2^{10}$ statements/shot | Norm reset via `split_b` every fold (**no norm ceiling**); 1 Goldilocks counter wraps at $\approx 2^{64}$ (2 limbs: $2^{128}$) | Pay-per-bit matrix Ajtai (`Neo`), coeff embedding (`SuperNeo`), double commits (`LF+`), ROM CP-SNARK (`Symphony`) | **YES** (high-arity tree folding) | PQ research / Rust prototypes (`NethermindEth/latticefold`, `sonobe` `SuperNeo`); tens-of-KiB proofs, no EVM verifier yet. |
| **16** | **Binius / PetraVM / Binius64 / Ligerito** | Binary towers $\mathbb{F}_{2^{32..128}}$; BaseFold / Ligerito / Bolt | Monolithic: Ligerito/Bolt tested to $N = 2^{30}$ (4 GiB poly in 0.84–80 s) | PetraVM: $2^{32}-1$ PC (`VROM_MULT = 8`!). **Binius64: 30-bit `ValueIndex` ($2^{30}$ `u64` words/seg)** | PetraVM: write-once `VromTable` (RAM unimplemented before Jan 2026 archive); Ligerito §6.5 partial-eval batching | Monolithic in repos | PetraVM `Statement { boundaries, table_sizes }`; Binius64 `InOut` `[u64]`. No Solidity contracts. |
| **17** | **Spice / Nebula / Protostar / Sonobe** | Commitment-Carrying IVC (BN254/Grumpkin) | 1 insn or basic-block switchboard per IVC step ($O(1)$ per RAM op) | Timestamp range check (`2^32..2^64`); step `i` in `Fr` | Homomorphic advice commitments + running `(RS, WS)` closed by **1 end-of-run $O(|M|)$ `FS` scan** | **NO intermediate sealing** before `FS` | **Sonobe `DeciderEth`** publishes **`(i, z_0, z_i, U_i, u_i, proof)`** — explicit step count `i` & states on-chain! |

---

### 1.3 System-by-System Primary-Source Audit

#### 1. RISC Zero (`rv32im-v1` & `rv32im-v2`, Monorepo `v3.1.0`)
- **Primary Sources**: `risc0/risc0` @ `d7708ddbde9b9521c9ffae6c12b8889bd11d0243` (`v3.1.0`, `rv32im-v2`) and `v1.2.0` (`rv32im-v1`); `risc0/risc0-ethereum` @ `6a9bca7e7b27d1af853620a806a216f7560b62d1`:
  - `https://github.com/risc0/risc0/blob/d7708ddbde9b9521c9ffae6c12b8889bd11d0243/risc0/zkp/src/lib.rs`
  - `https://github.com/risc0/risc0/blob/d7708ddbde9b9521c9ffae6c12b8889bd11d0243/risc0/circuit/rv32im/src/execute/executor.rs`
  - `https://github.com/risc0/risc0/blob/d7708ddbde9b9521c9ffae6c12b8889bd11d0243/risc0/binfmt/src/image.rs`
  - `https://github.com/risc0/risc0/blob/d7708ddbde9b9521c9ffae6c12b8889bd11d0243/risc0/circuit/rv32im/src/execute/platform.rs`
  - `https://github.com/risc0/risc0/blob/d7708ddbde9b9521c9ffae6c12b8889bd11d0243/risc0/zkvm/src/claim/receipt.rs`
  - `https://github.com/risc0/risc0/blob/d7708ddbde9b9521c9ffae6c12b8889bd11d0243/risc0/binfmt/src/povw.rs`
  - `https://github.com/risc0/risc0/blob/d7708ddbde9b9521c9ffae6c12b8889bd11d0243/website/api/zkvm/zkvm-specification.md`
  - `https://github.com/risc0/risc0-ethereum/blob/6a9bca7e7b27d1af853620a806a216f7560b62d1/contracts/src/IRiscZeroVerifier.sol`
  - `https://github.com/risc0/risc0-ethereum/blob/6a9bca7e7b27d1af853620a806a216f7560b62d1/contracts/src/StructHash.sol`
  - `https://github.com/risc0/risc0-ethereum/blob/6a9bca7e7b27d1af853620a806a216f7560b62d1/contracts/src/groth16/RiscZeroGroth16Verifier.sol`
  - `https://github.com/risc0/risc0-ethereum/blob/6a9bca7e7b27d1af853620a806a216f7560b62d1/contracts/src/groth16/ControlID.sol`
- **(i) Per-Segment & Total Run Ceilings**:
  - Over BabyBear ($p = 15\cdot 2^{27}+1$, `INV_RATE = 4`), raw ZKP bounds are `MIN_CYCLES_PO2 = 13` ($8{,}192$) and `MAX_CYCLES_PO2 = 24` ($16{,}777{,}216$), with recursion `lift_rv32im_v2_{14..=24}.zkr` predicates starting at `MIN_LIFT_PO2 = 14` (`RECURSION_PO2 = 18`) and default `DEFAULT_SEGMENT_LIMIT_PO2 = 20` ($1{,}048{,}576$ cycles).
  - **On-chain `ALLOWED_CONTROL_ROOT` ceiling**: `ControlID.sol` whitelists `lift` predicates only for **`po2` $\in [14, 22]$** ($\le 4{,}194{,}304$ total cycles/segment); `po2 = 23, 24` require a custom verifier control root. Each segment reserves `ZK_CYCLES (1024) + RESERVED_CYCLES (4113) = 5,137` fixed rows plus `PAGE_IN`/`PAGE_OUT` hashing cycles and splits `MAX_INSN_CYCLES = 25,000` cycles early (`executor.rs:64-101`).
  - **Total run ceiling**: **Arithmetically UNBOUNDED in-circuit** — `ReceiptClaim { pre, post, exit_code, input, output }` carries **no cycle counter and no segment index**, and `join.zkr` folds arbitrary `(pre -> mid, SystemSplit)` + `(mid -> post, exit)` trees. Host software uses `segment_counter: u32` ($2^{32}$ segments $\approx 4.5 \times 10^{15}$ cycles at `po2 = 20`), optional `session_limit: Option<u64>`, and optional Proof-of-Verifiable-Work (`povw.rs`) `Work.value: u64` ($< 2^{64}$ cycles).
- **(ii) Cross-Segment Memory Persistence & Partition Independence**:
  - Intra-segment memory uses a local BabyBear permutation over `(addr, cycle, value)` with segment-local cycles. Across segments, `rv32im-v2` maps the full **4 GiB** (`MEMORY_BYTES = 1 << 32`) address space into $2^{22}$ **`1 KiB` (`PAGE_BYTES = 1024`)** pages hashed in a **depth-22 binary Poseidon2 Merkle tree** (`POSEIDON_PAGE_ROUNDS = 32` perms/page). All 32 machine registers (`0xffff_0000`), 32 user registers (`0xffff_0080`), `SUSPEND_PC` (`0xffff_0210`), and `SUSPEND_MODE` (`0xffff_0214`) reside **inside kernel RAM pages**, so `SystemState` is `{ pc: 0, merkle_root }` and `merkle_root` alone captures 100% of machine state.
  - **Partition independent**: `join.zkr` erases intermediate `merkle_root`s, and on `Halted(0)` `lift.zkr` zeros `post.merkle_root = Digest::ZERO` (`receipt.rs:191-194`). Cross-run composition uses `Output.assumptions` discharged in recursion by `resolve.zkr`.
- **(iii) On-Chain Published Data**:
  - `IRiscZeroVerifier.verify(bytes seal, bytes32 imageId, bytes32 journalDigest)` reconstructs `ReceiptClaim` with `pre = imageId`, `post = SYSTEM_STATE_ZERO_DIGEST`, `input = 0`, `assumptions = 0`, `exitCode = Halted(0)`, passing 5 BN254 Groth16 scalars `[CONTROL_ROOT_0, CONTROL_ROOT_1, claim0, claim1, BN254_CONTROL_ID]`. **Neither run length nor final memory root is ever published on-chain.**

#### 2. SP1 by Succinct (`v4.0 Turbo` & `v6.1.0 Hypercube`)
- **Primary Sources**: `succinctlabs/sp1` @ `749b0c63f226063c471a6238a94101f930c14a13` (`v6.x Hypercube`) and `4a1dcea0749021ce6e2596bce5bb45f2def7a95c` (`v4.0 Turbo`); `succinctlabs/sp1-contracts` @ `d3629729c3216eb51bd4859d027a8eb729399fa4`; Jagged PCS paper `https://eprint.iacr.org/2025/917`:
  - `https://github.com/succinctlabs/sp1/blob/749b0c63f226063c471a6238a94101f930c14a13/crates/prover/CIRCUITS.md`
  - `https://github.com/succinctlabs/sp1/blob/749b0c63f226063c471a6238a94101f930c14a13/crates/primitives/src/consts.rs`
  - `https://github.com/succinctlabs/sp1/blob/749b0c63f226063c471a6238a94101f930c14a13/crates/core/executor/src/opts.rs`
  - `https://github.com/succinctlabs/sp1/blob/749b0c63f226063c471a6238a94101f930c14a13/crates/core/machine/src/adapter/bump.rs`
  - `https://github.com/succinctlabs/sp1/blob/749b0c63f226063c471a6238a94101f930c14a13/crates/core/machine/src/memory/global.rs`
  - `https://github.com/succinctlabs/sp1/blob/749b0c63f226063c471a6238a94101f930c14a13/crates/core/machine/src/memory/local.rs`
  - `https://github.com/succinctlabs/sp1/blob/749b0c63f226063c471a6238a94101f930c14a13/crates/core/machine/src/global/mod.rs`
  - `https://github.com/succinctlabs/sp1/blob/749b0c63f226063c471a6238a94101f930c14a13/crates/recursion/circuit/src/machine/complete.rs`
  - `https://github.com/succinctlabs/sp1/blob/749b0c63f226063c471a6238a94101f930c14a13/audits/sp1-v4.md`
  - `https://github.com/succinctlabs/sp1-contracts/blob/d3629729c3216eb51bd4859d027a8eb729399fa4/contracts/src/ISP1Verifier.sol`
  - `https://github.com/succinctlabs/sp1-contracts/blob/d3629729c3216eb51bd4859d027a8eb729399fa4/contracts/src/v6.1.0/SP1VerifierGroth16.sol`
- **(i) Per-Shard & Total Run Ceilings**:
  - **SP1 v4 Turbo** (RV32IM, BabyBear FRI): `MAX_CPU_LOG_DEGREE = 22` ($4.19\text{M}$ cycles/shard, 24-bit `clk` step 4) and `MAX_LOG_NUMBER_OF_SHARDS = 16` (`U16Range` on `shard`), capping total runs at $(2^{16}-1)\cdot 2^{22} \approx \mathbf{2.75 \times 10^{11}}$ (**$2^{38}$**) cycles and RAM at `0x7800_0000` (1.875 GiB).
  - **SP1 v6.1 Hypercube** (RV64IM, KoalaBear MLE Zerocheck + LogUp-GKR + Jagged PCS, no monolithic CPU table): Per-shard caps are `HEIGHT_THRESHOLD = 1 << 22` rows/chip and `ELEMENT_THRESHOLD = 3 * 2^27 = 402,653,184` trace cells (`MAX_LOG_NUMBER_OF_SHARDS = 24`). Across the whole run, Hypercube carries **one continuous 48-bit global `clk`** (`clk_low` 24-bit + `clk_high` 24-bit range-checked in `StateBumpChip`, `bump.rs:706-738`) advancing by `CLK_INC = 8` per instruction, imposing an exact in-circuit total ceiling of **$2^{48} / 8 = \mathbf{2^{45} \approx 3.52 \times 10^{13}}$ instructions** over a 48-bit RV64 address space (`(1 << 48) - 1` B).
- **(ii) Cross-Shard Memory Persistence & Partition Independence**:
  - **Zero Merkle trees**: Intra-shard LogUp-GKR checks `(clk_high, clk_low, addr, val)` ordering; at shard edges, `MemoryLocalChip` receives `(init_clk, addr, init_val)` and sends `(final_clk, addr, final_val)` mapped via Poseidon2 onto **`SepticCurve<KoalaBear>`** ($\mathbb{F}_{p^7}$) and summed into `global_cumulative_sum: SepticDigest`. Static ELF words are pre-summed into `vk.initial_global_cumulative_sum`, while `MemoryGlobalInit` and `MemoryGlobalFinalize` (`global.rs`) sweep all dynamically touched addresses in strictly ascending 48-bit order (`prev_addr < addr`). Abelian EC cancellation (`+P - P = \mathcal{O}`) makes `programVKey` and public values **100% partition-independent**.
- **(iii) On-Chain Published Data**:
  - `ISP1Verifier.verifyProof(bytes32 programVKey, bytes publicValues, bytes proofBytes)`. In v4/v5, Groth16 took 2 scalars `[vkey, pvDigest]`; in **v6.1** (`v6.1.0/SP1VerifierGroth16.sol`), it checks `vkRoot == VK_ROOT() && exitCode == 0` in Solidity and passes **5 scalars** `[programVKey, publicValuesDigest, exitCode(0), vkRoot, nonce]`. `assert_complete` (`complete.rs:480-548`) enforces `global_cumulative_sum == 0` and `next_pc == HALT_PC` privately; **neither cycle count nor final memory state is published on-chain**.

#### 3. OpenVM by Axiom & Scroll (`v1.0.0` & `v2.1.0`)
- **Primary Sources**: `openvm-org/openvm` @ `05f3b73e7294fbb6d38155abafc98f1fe2bcfb73` (`v2.1.0`) and `v1.0.0`; `openvm-org/stark-backend` @ `v2.1.0`:
  - `https://github.com/openvm-org/openvm/blob/05f3b73e7294fbb6d38155abafc98f1fe2bcfb73/docs/vocs/docs/pages/specs/architecture/continuations.mdx`
  - `https://github.com/openvm-org/openvm/blob/05f3b73e7294fbb6d38155abafc98f1fe2bcfb73/docs/vocs/docs/pages/specs/architecture/memory.mdx`
  - `https://github.com/openvm-org/openvm/blob/05f3b73e7294fbb6d38155abafc98f1fe2bcfb73/crates/vm/src/arch/config.rs`
  - `https://github.com/openvm-org/openvm/blob/05f3b73e7294fbb6d38155abafc98f1fe2bcfb73/crates/vm/src/system/connector/mod.rs`
  - `https://github.com/openvm-org/openvm/blob/05f3b73e7294fbb6d38155abafc98f1fe2bcfb73/crates/verify/src/pvs.rs`
  - `https://github.com/openvm-org/openvm/blob/05f3b73e7294fbb6d38155abafc98f1fe2bcfb73/crates/continuations/src/circuit/root/mod.rs`
  - `https://github.com/openvm-org/openvm/blob/05f3b73e7294fbb6d38155abafc98f1fe2bcfb73/crates/continuations/src/circuit/root/verifier/air.rs`
  - `https://github.com/openvm-org/openvm/blob/05f3b73e7294fbb6d38155abafc98f1fe2bcfb73/crates/static-verifier/src/stages/full_pipeline/public_values.rs`
  - `https://github.com/openvm-org/openvm/blob/05f3b73e7294fbb6d38155abafc98f1fe2bcfb73/crates/sdk/contracts/src/IOpenVmHalo2Verifier.sol`
  - `https://github.com/openvm-org/stark-backend/blob/v2.1.0/crates/stark-sdk/src/config/mod.rs`
- **(i) Per-Segment & Total Run Ceilings**:
  - Over BabyBear (WHIR + Stacked MLE PCS in v2.1), `SystemConfig::new` enforces `timestamp_max_bits <= 29` (`VmConnectorAir` range-checks `final.timestamp < 2^29` via `17 + 12`-bit limbs, capping segments at $< 2^{29}$ memory accesses $\lesssim 1.34 \times 10^8$ instructions) alongside `MAX_APP_LOG_STACKED_HEIGHT = 24` (`w_stack = 2048`) and host worker `DEFAULT_MAX_MEMORY = 15 GiB`.
  - **Total run ceiling**: Aggregation (`app -> leaf(4) -> internal-for-leaf(3) -> internal-recursive(3) -> root -> static-verifier`) enforces `recursion_depth - 1 < 256` (`LOG_MAX_RECURSION_DEPTH = 8`, `air.rs:381-397`), supporting **$4 \times 3^{257} \approx 1.67 \times 10^{123}$ segments** (**cryptographically UNBOUNDED**; host counter `instret_start: u64`).
- **(ii) Cross-Segment Memory Persistence & Partition Independence**:
  - Unified `(address_space, pointer)` model with **zero separate register bus**: RV32 registers are `AS = 1` (128B), guest RAM is `AS = 2` (`2^29` B = 512 MiB default), and user public values are `AS = 3` (`32..8192` B). Intra-segment BEGKN92 offline bus resets `timestamp = 1` every segment; at boundaries, `PersistentBoundaryChip` + `MemoryMerkleChip` update a **height-29 Poseidon2 binary Merkle tree** (`3 + 29 - 3 = 29` over **8-cell chunks**!). Aggregation chains `(final_pc, final_root) == (initial_pc, initial_root)` (`exit_code = 42` suspend, `0` terminate). **100% partition-independent.**
- **(iii) On-Chain Published Data**:
  - `RootCircuit` (`UserPvsInMemoryAir`) verifies a Merkle inclusion path proving `publicValues` (`AS = 3`) sits inside `final_root`, checks `is_terminate == 1 && exit_code == 0`, and **discards `final_root` and `recursion_depth`**. On-chain `IOpenVmHalo2Verifier.verify(publicValues, proofData, appExeCommit, appVmCommit)` checks `14 + L` BN254 KZG scalars (`12` KZG accumulator limbs + `appExeCommit` + `appVmCommit` + `L` byte-felts). **Neither run length nor final memory root is published on-chain.**

#### 4. Ziren / zkMIPS by Project ZKM (`HEAD`)
- **Primary Sources**: `ProjectZKM/Ziren` @ `a38b73404a2cdcaf6f27bcab047920fcf45efd7c` (redirect target of `ProjectZKM/zkMIPS`), `https://docs.zkm.io`:
  - `https://github.com/ProjectZKM/Ziren/blob/a38b73404a2cdcaf6f27bcab047920fcf45efd7c/crates/pcs/src/opts.rs`
  - `https://github.com/ProjectZKM/Ziren/blob/a38b73404a2cdcaf6f27bcab047920fcf45efd7c/crates/core/machine/src/air/memory.rs`
  - `https://github.com/ProjectZKM/Ziren/blob/a38b73404a2cdcaf6f27bcab047920fcf45efd7c/crates/core/machine/src/mips/mod.rs`
  - `https://github.com/ProjectZKM/Ziren/blob/a38b73404a2cdcaf6f27bcab047920fcf45efd7c/crates/core/machine/src/memory/local.rs`
  - `https://github.com/ProjectZKM/Ziren/blob/a38b73404a2cdcaf6f27bcab047920fcf45efd7c/crates/pcs/src/air/public_values.rs`
  - `https://github.com/ProjectZKM/Ziren/blob/a38b73404a2cdcaf6f27bcab047920fcf45efd7c/crates/recursion/circuit/src/machine/complete.rs`
  - `https://github.com/ProjectZKM/Ziren/blob/a38b73404a2cdcaf6f27bcab047920fcf45efd7c/crates/recursion/gnark-ffi/assets/IZKMVerifier.sol`
  - `https://github.com/ProjectZKM/Ziren/blob/a38b73404a2cdcaf6f27bcab047920fcf45efd7c/crates/recursion/gnark-ffi/assets/ZKMVerifierGroth16.txt`
- **(i) Per-Shard & Total Run Ceilings**:
  - MIPS32r2 over KoalaBear (LogUp-GKR + Stacked/Jagged WHIR PCS). Three shard fences (`opts.rs:554-593`): `ELEMENT_THRESHOLD = 460_000_000` cells, `CORE_MAX_LOG_ROW_COUNT = 22` ($4.19\text{M}$ rows/chip), and `TIMESTAMP_BITS = 26` (`16 + 10` bits, `memory.rs:182-192`) with per-shard `clk += 5` per MIPS instruction $\implies$ **$\lfloor(2^{26}-1)/5\rfloor = 13{,}421{,}772$ instructions/shard**. With `MAX_LOG_NUMBER_OF_SHARDS = 16` (`u16` range-checked `shard`), total run ceiling is **$65{,}535 \times 13{,}421{,}772 \approx \mathbf{8.80 \times 10^{11}}$ ($\approx 2^{39.7}$) instructions** over 4 GiB RAM.
- **(ii) Cross-Shard Memory Persistence & Partition Independence**:
  - Offline elliptic-curve multiset sum (`SepticDigest` via `MemoryLocal` + sorted `MemoryGlobalInit/Final`), plus **dual-PC chaining** `(start_pc, start_next_pc) == (next_pc, next_next_pc)` (`public_values.rs:1517-1527`) so shards can split cleanly inside MIPS branch delay slots! Dynamic `RAMP_SHARDS = 8` varies shard sizes without affecting `programVKey` or outputs (**100% partition-independent**).
- **(iii) On-Chain Published Data**:
  - `IZKMVerifier.verifyProof(programVKey, publicValues, proofBytes)` passes **3 BN254 Groth16 scalars** `[programVKey, publicValuesDigest, VK_ROOT()]` (`ZKMVerifierGroth16.txt:2076-2081`). `assert_complete` checks `global_cumulative_sum == 0`, `next_pc == 0`, `exit_code == 0` privately; **zero length or memory digests on-chain**.

#### 5. Brevis Pico (`v2.x` & `v3.1.1`)
- **Primary Sources**: `brevis-network/pico` @ `7f9b670ec8b0ec82a5bd680b97fe085cd98be71e` (`v3.1.1`), Sherlock Audit `https://audits.sherlock.xyz/contests/1045`:
  - `https://github.com/brevis-network/pico/blob/7f9b670ec8b0ec82a5bd680b97fe085cd98be71e/docs/soundness/README.md`
  - `https://github.com/brevis-network/pico/blob/7f9b670ec8b0ec82a5bd680b97fe085cd98be71e/vm/src/primitives/consts.rs`
  - `https://github.com/brevis-network/pico/blob/7f9b670ec8b0ec82a5bd680b97fe085cd98be71e/vm/src/primitives/layout.rs`
  - `https://github.com/brevis-network/pico/blob/7f9b670ec8b0ec82a5bd680b97fe085cd98be71e/vm/src/chips/chips/riscv_cpu/chunk_clk/constraints.rs`
  - `https://github.com/brevis-network/pico/blob/7f9b670ec8b0ec82a5bd680b97fe085cd98be71e/vm/src/chips/chips/riscv_memory/local/constraints.rs`
  - `https://github.com/brevis-network/pico/blob/7f9b670ec8b0ec82a5bd680b97fe085cd98be71e/vm/src/chips/chips/riscv_memory/initialize_finalize/constraints.rs`
  - `https://github.com/brevis-network/pico/blob/7f9b670ec8b0ec82a5bd680b97fe085cd98be71e/vm/src/compiler/recursion/circuit/wrap_mle.rs`
  - `https://github.com/brevis-network/pico/blob/7f9b670ec8b0ec82a5bd680b97fe085cd98be71e/contracts/src/PicoVerifier.sol`
- **(i) Per-Chunk & Total Run Ceilings**:
  - RV64IM over KoalaBear ($\mathbb{F}_{p^5}$ 128-bit provable RBR soundness, LogUp-GKR + WHIR `log_blowup = 1`). KoalaBear 2-adicity $24$ caps chunk height at `MAX_LOG_CHUNK_SIZE = 23` ($8{,}388{,}608$ rows), enforced alongside a 25-bit `range_check_u25_16_9` on per-chunk `clk` (step $\ge 4$). `MAX_LOG_NUMBER_OF_CHUNKS = 16` (`U16Range` on `chunk`) gives an arithmetic run ceiling of **$65{,}535 \times 2^{23} \approx \mathbf{5.50 \times 10^{11}}$ ($\approx 2^{39}$) cycles** and 1.875 GiB RAM (`0x7800_0000` with a 64 KiB `0x777f` stack guard gap).
- **(ii) Cross-Chunk Memory Persistence & Partition Independence**:
  - Two-scope (`Regional` + `Global`) offline multiset lookup with a **`has_initial` fresh-zero optimization** (`local/constraints.rs:1160-1190`): first touches to zeroed heap/stack set `has_initial = 0`, constrain `initial_value == 0` locally, and skip `MemoryInitialize` rows while still being audited once in ascending-address `MemoryFinalize`! Partition-independent in EVM whitelist mode (`h = 23` fixed).
- **(iii) On-Chain Published Data**:
  - `PicoVerifier.verifyPicoProof(bytes32 riscvVkey, bytes publicValues, uint256[12] proof)` (Groth16 + BSB22 commitment) passes **only 2 BN254 scalars** `[riscvVkey, publicValuesDigest]`, baking `AssertVkRoot` and `flag_complete == 1` as R1CS constants (`wrap_mle.rs:635-650`). **Zero cycle count or memory digests on-chain.**

#### 6. Jolt by a16z crypto (`0.1 Lasso/Spice` $\to$ `2025–2026 Twist+Shout`)
- **Primary Sources**: `a16z/jolt` (`main`), Original Jolt `https://eprint.iacr.org/2023/1217`, Twist & Shout `https://eprint.iacr.org/2025/105`, Dory `https://eprint.iacr.org/2020/1274`:
  - `https://github.com/a16z/jolt/blob/main/common/src/constants.rs`
  - `https://github.com/a16z/jolt/blob/main/crates/jolt-verifier/src/verifier.rs`
  - `https://github.com/a16z/jolt/blob/main/crates/jolt-dory/src/types.rs`
  - `https://github.com/a16z/jolt/blob/main/crates/jolt-hyperkzg/src/types.rs`
  - `https://github.com/a16z/jolt/blob/main/crates/jolt-akita/schedules/README.md`
  - `https://github.com/a16z/jolt/blob/main/book/src/how/twist-shout.md`
  - `https://github.com/a16z/jolt/blob/main/book/src/roadmap/streaming.md`
  - `https://github.com/a16z/jolt/blob/main/book/src/roadmap/on-chain-verifier.md`
- **(i) Step & RAM Ceilings**:
  - RV64IMAC over BN254 `Fr` or 128-bit `AkitaField` ($p = 2^{128} - 1780711455$). Sumcheck over $\{0,1\}^{\log T}$ uses no length-$T$ FFT, so two-adicity never bounds $T$. Default `DEFAULT_MAX_TRACE_LENGTH = 2^24` ($16.77\text{M}$ cycles); `HyperKZG` is bounded by trusted SRS length $K^{1/d} \cdot T \le \text{srs\_capacity}$; **`Dory`** uses $O(\sqrt{K^{1/d} T})$ generators (`MAX_SERIALIZED_PROOF_ROUNDS = 64`); **`Akita`** schedules support up to **`2^31` cycles**. `ram_K` is a dynamic power-of-two 8-byte word count (`DEFAULT_HEAP_SIZE = 32 MiB`). Jolt deliberately pursues **monolithic streaming sumcheck** (`streaming.md`) rather than continuation sharding.
- **(ii) Memory Checking**:
  - **Twist** commits sparse cycle-indexed write increments `RamInc(j)` and verifies $\widetilde{\mathsf{Val}}(r_{\text{addr}}, r_{\text{cycle}})$ via prefix-sum sumcheck against $\widetilde{\mathsf{LT}}(j', r_{\text{cycle}})$ with zero Merkle paths or grand products.
- **(iii) Public / On-Chain Data**:
  - No Solidity verifier on `main` yet (`on-chain-verifier.md` WIP). Crucially, `build_statement` (`verifier.rs:371-423`) **explicitly absorbs both padded `ram_K` (word 9) and `trace_length` (word 10)** into `JoltStatement` alongside `preprocessing_digest` and public I/O.

#### 7. Cairo / StarkWare (`Stone` & Circle STARK `Stwo` / SHARP)
- **Primary Sources**: Cairo paper `https://eprint.iacr.org/2021/1063`, Circle STARKs `https://eprint.iacr.org/2024/278`, SHARP docs `https://docs.starknet.io/learn/protocol/sharp`, `starkware-libs/{starkex-contracts,stone-prover,stwo,proving}`:
  - `https://github.com/starkware-libs/starkex-contracts/blob/master/evm-verifier/solidity/contracts/cpu/CpuVerifier.sol`
  - `https://github.com/starkware-libs/starkex-contracts/blob/master/evm-verifier/solidity/contracts/cpu/MemoryPageFactRegistry.sol`
  - `https://github.com/starkware-libs/starkex-contracts/blob/master/evm-verifier/solidity/contracts/gps/GpsStatementVerifier.sol`
  - `https://github.com/starkware-libs/stone-prover/blob/main/src/starkware/air/components/memory/memory.inl`
  - `https://github.com/starkware-libs/stwo/blob/dev/crates/stwo/src/core/circle.rs`
  - `https://github.com/starkware-libs/proving/blob/main/crates/common/src/memory.rs`
  - `https://github.com/starkware-libs/proving/blob/main/crates/stwo_run_and_prove_recursive_tree/src/lib.rs`
- **(i) Per-Run & Total Ceilings**:
  - **Stone** ($P = 2^{251}+17\cdot 2^{192}+1$): Solidity `CpuVerifier.sol` checks `log_n_steps < 50` (16 rows/step; prover RAM bounds single proofs to $\sim 2^{24}\text{–}2^{27}$ steps; continuous sorted memory requires `max_addr - min_addr` padded without gaps; `rc_builtin` is 128-bit).
  - **Stwo-Cairo** ($M_{31} = 2^{31}-1$): Circle group order $2^{31}$ fixes `MAX_CIRCLE_DOMAIN_LOG_SIZE = 30` ($N \cdot B^2 \le 2^{30} \implies N \le 2^{28}$ rows at $\beta=1$); flat-AIR opcodes cap at `MAX_SEQUENCE_LOG_SIZE = 25` ($2^{25}$ rows/component) and LogUp memory at `LOG_MEMORY_ADDRESS_BOUND = 29` ($2^{29}$ words split across 16 tables).
  - **Total ceiling**: **Unbounded** across **Cairo PIEs** aggregated by **SHARP** (Stwo proves leaf PIEs + binary `multiverifier` recursion tree; Stone wraps the root for L1).
- **(ii) Cross-Run Memory Persistence**:
  - Write-once memory is local to each PIE; multi-block Starknet OS runs pass explicit **Poseidon/Pedersen Patricia-Merkle `state_root`s** in task `output` segments verified across OS merger programs.
- **(iii) On-Chain L1 Published Data (Direct Contrast to SNARK Wrappers)**:
  - Because Ethereum verifies the unwrapped root **Stone STARK** directly, `CpuVerifier.sol` + `MemoryPageFactRegistry.sol` **publish on L1**: root **`log_n_steps`** (`publicInput[0]`), relocated segment `[begin_addr, stop_ptr]` pairs, Page 0 `(addr, val)` pairs, and **Continuous Memory Page headers `(startAddr, pageSize, keccak256(values), prod)`** (binding L1 calldata/blobs directly into the STARK memory grand product at challenges `(z, alpha)`) before `GpsStatementVerifier` registers `keccak256(programHash, programOutputFact)`.

#### 8. Miden VM (`0xMiden/miden-vm` `v1.0-rc1` & `miden-base` `v0.14-alpha`)
- **Primary Sources**:
  - `https://github.com/0xMiden/miden-vm/blob/next/processor/src/execution_options.rs`
  - `https://github.com/0xMiden/miden-vm/blob/next/crates/air/generated/src/vm/config.rs`
  - `https://github.com/0xMiden/miden-vm/blob/next/docs/src/design/chiplets/memory.md`
  - `https://github.com/0xMiden/miden-vm/blob/next/core/src/program/claim.rs`
  - `https://github.com/0xMiden/miden-vm/blob/next/crates/air/source/src/vm/statement.rs`
  - `https://github.com/0xMiden/miden-vm/blob/next/verifier/src/recursive/mod.rs`
  - `https://github.com/0xMiden/miden-base/blob/main/crates/miden-protocol/src/transaction/verifier.rs`
  - `https://github.com/0xMiden/miden-base/blob/main/crates/miden-agglayer/SPEC.md`
- **(i)–(iii) Ceilings, Memory & On-Chain Data**:
  - Single VM proof over Goldilocks ($p = 2^{64}-2^{32}+1$, two-adicity $32$) at `LOG_BLOWUP = 3` is hard-capped at **`MAX_CYCLES = 2^29`** ($536{,}870{,}912$ cycles: $2^{29}\cdot 2^3 = 2^{32}$ subgroup saturation + `u32` `clk` 16-bit limb $\Delta < 2^{32}$), element-addressable `[0, 2^32)` (`DEFAULT_MAX_MEMORY_ELEMENTS = 2^28`).
  - No mid-execution flat-RAM continuations; state persists across transactions/batches/blocks via **depth-64 Sparse Merkle Trees (`SMT`)** + Note MMRs verified recursively in MASM using the hardware **ACE chiplet** (`exec.vm::verify_proof`).
  - `ExecutionClaim` (`claim.rs`) is a fixed **40-felt** tuple `(program_hash, kernel_root, stack_inputs[16], stack_outputs[16])` — per-AIR `log_trace_heights` stay inside `StarkProofData` (`statement.rs`), and L1 settlement via Polygon AggLayer (`SPEC.md`) posts only rollup/exit roots (**zero lengths or memory pages**).

#### 9. Valida by Lita (`v1.0.0`, `arXiv:2505.08114`)
- **Primary Sources**: ISA Spec `https://arxiv.org/abs/2505.08114`, `lita-xyz/valida-vm` (`main` / `c36d3177c1708c70d641cfda8512ac42b1e6d9bf`):
  - `https://github.com/lita-xyz/valida-vm/blob/main/basic/src/bin/args/mod.rs`
  - `https://github.com/lita-xyz/valida-vm/blob/main/bytes/src/lib.rs`
  - `https://github.com/lita-xyz/valida-vm/blob/main/basic-api/src/machine/basic.rs`
  - `https://github.com/lita-xyz/valida-vm/blob/main/persistence/src/lib.rs`
  - `https://github.com/lita-xyz/valida-vm/blob/main/memory/src/lib.rs`
  - `https://github.com/lita-xyz/valida-vm/blob/main/memory/src/stark.rs`
  - `https://github.com/lita-xyz/valida-vm/blob/main/machine/src/proof.rs`
- **(i)–(iii) Ceilings, Memory & Public Statement**:
  - Registerless stack-only Harvard VM over BabyBear (`log_blowup = 1` $\implies$ FFT cap $2^{26}$ rows; default `--max-segment-size = 2^20` **memory ops**, up to $2^{21}$ control-flow split buffer). RAM addresses and `clk` diffs are restricted to half-BabyBear **`[0, 0x3C000000)`** ($\approx 960\text{ MiB}$, `Under0x3C` in `bytes/src/lib.rs`) to prevent modular wraparound.
  - `MultiSegmentBasicMachine` links up to $p - 4 \approx 2.01 \times 10^9$ segments via a global cross-segment LogUp bus (`persistent_mem_bus` carrying `(addr, val[0..4], 3 + segment_number)`, pruning unread final states via `is_final = 1`). Without multi-segment recursion, `MultiSegmentMachineProof` grows as $O(S)$ and **exposes every segment's `segment_number`, `(pc_init, pc_final, fp_init, fp_final)`, and chip `log_degree`**; no Solidity verifier exists in public repos.

#### 10. Nexus zkVM (`1.0/2.0` Nova/HyperNova IVC $\to$ `3.0` Stwo Circle STARK)
- **Primary Sources**: Whitepaper `https://nexus-xyz.github.io/assets/nexus_whitepaper.pdf`, `nexus-xyz/nexus-zkvm` (`releases/0.2.4` & `main` `v0.3.6`), 3.0 Spec `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/specification/zkvm-spec-3.0.pdf`:
  - `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/releases/0.2.4/vm/src/circuit/riscv.rs`
  - `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/releases/0.2.4/vm/src/circuit/step.rs`
  - `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/releases/0.2.4/vm/src/memory/trie.rs`
  - `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/releases/0.2.4/nova/src/circuits/nova/pcd/mod.rs`
  - `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/sdk/src/stwo/seq.rs`
  - `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/prover2/machine/src/verify.rs`
- **(i)–(iii) Era Comparison & Lessons for Sherlock**:
  - **Nexus 1.0/2.0 (`0.2.4`)**: Nova (`K = 16` instrs/step) / HyperNova (`K = 1`) + CycleFold (BN254/Grumpkin) over state vector `z = [pc, x0..x31, root]` (`ARITY = 34`). Used a **height-27 Poseidon Merkle tree** (`32 B` `CacheLine` leaves in `[Fr; 2]`), giving full parallel binary-tree PCD partition independence (`PCDNode` `[i, j)`), **at a crushing cost of $3 \times 27 = 81$ Poseidon compressions per single RV32 instruction** (`step.rs`) because it lacked a local intra-leaf memory bus (contrast Sherlock §2.7's `MEM` + `D7` cached top frame, which amortizes one segment update across `S = 16` steps!).
  - **Nexus 3.0 (`v0.3.6`)**: Migrated to Stwo ($M_{31}$ Circle STARK + LogUp memory) for 10–16 kHz laptop speed, **but `Stwo::prove` (`sdk/src/stwo/seq.rs`) calls `k_trace(..., 1)` — continuations are not implemented**, capping runs at a single monolithic Stwo domain (`log_size <= 28..29`, practically host-RAM bound at ~13.3 GB for $2^{20}$ cycles). Neither era ships a Solidity verifier in-repo.

#### 11. zkWasm by Delphinus Lab (Halo2 KZG Slices & `AggregatorVerifier.sol`)
- **Primary Sources**: `DelphinusLab/zkWasm`, `DelphinusLab/continuation-batcher`, `DelphinusLab/halo2aggregator-s`:
  - `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/crates/zkwasm/src/circuits/etable/mod.rs`
  - `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/crates/zkwasm/src/circuits/mtable/allocator.rs`
  - `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/crates/zkwasm/src/circuits/utils/image_table.rs`
  - `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/crates/zkwasm/src/circuits/post_image_table/continuation.rs`
  - `https://github.com/DelphinusLab/continuation-batcher/blob/main/sol/contracts/AggregatorVerifier.sol`
  - `https://github.com/DelphinusLab/halo2aggregator-s/blob/main/src/circuit_verifier/mod.rs`
- **(i)–(iii) Ceilings, KZG Column Chaining & On-Chain Data**:
  - PSE Halo2 KZG (BN254) at $k \in \{18, 20, 22\}$ proves $\lfloor(2^k - 33{,}920)/4\rfloor =$ **`57,056` / `253,664` / `1,040,096` Wasm steps/slice** and $\lfloor(2^k - 123{,}008)/8{,}192\rfloor =$ **`16` / `112` / `496` 64-KiB pages (`1 / 7 / 31 MiB`)**, with a **hard total run ceiling of $2^{32} - 1$ steps** from `eid: AllocatedU32StateCell` (`allocator.rs`) and $2^{\text{cont}}$ precomputed VK-chain hashes.
  - Instead of Poseidon hashing Wasm RAM, `OngoingCircuit` copies/updates `img_col` into `post_img_col` across all $2^k$ rows and `halo2aggregator-s` checks $\mathbb{G}_1$ KZG commitment equality $C_{\text{post\_img}}^{(s)} == C_{\text{img}}^{(s+1)}$ (3 `Fr` limbs). (External L2 KV state uses a height-32 Poseidon Merkle host precompile.)
  - On-chain `AggregatorVerifier.sol` compresses `target_instance` (guest public inputs) and `verify_instance` (`[vk_chain_hash, img_init_G1_3limbs, ctx_out_G1_3limbs]`) via `Keccak256 mod r` into **1 Halo2 instance scalar** (`final_root` is omitted, though `vk_chain_hash` binds the continuation round count).

#### 12. ZKsync Airbender (`matter-labs/zksync-airbender`, 2025–2026)
- **Primary Sources**:
  - `https://docs.zksync.io/zk-stack/components/zksync-airbender/deepdive`
  - `https://raw.githubusercontent.com/matter-labs/zksync-airbender/main/docs/philosophy_and_logic.md`
  - `https://raw.githubusercontent.com/matter-labs/zksync-airbender/main/docs/subarguments_used.md`
- **(i)–(iii) Two-Pass Subtree Pre-Commitment & `2^36` Cycle Ceiling**:
  - RV32IM over Mersenne-31 FRI. To parallelize shards under an offline `(address, ts, value)` read/write permutation without EC points, the prover first generates only the memory/delegation columns across all chunks, commits them into a separate **memory Merkle subtree** (~5–10% overhead), draws global challenges, discards the witness, and proves chunks + specialized delegation circuits (`u256`, `blake2s`) in parallel (`subarguments_used.md`).
  - Registers and RAM share `(is_reg: bool, addr: u32)` with lazy sorted `u32` init/teardown. Timestamps are pairs of **19-bit integers (`2^38` timestamps)** consuming 4 timestamps/cycle, giving a hard single-invocation ceiling of **`2^36 = 68,719,476,736` cycles** checked by the recursive verifier (alongside delegation row sum $< |F|$).

#### 13. Polygon ZisK & Aligned Layer's Lambda VM (2025–2026)
- **Primary Sources**: `https://0xpolygonhermez.github.io/zisk-docs/`, `https://blog.alignedlayer.com/lambda-vm-continuations/`:
  - **ZisK** decouples segmentation from execution boundaries: each chip slices its own trace independently at fixed height (padding only the final slice per chip) and reconciles cross-slice state on a global continuation bus closed at the aggregation root.
  - **Lambda VM** built and compared two designs (`lambda-vm-continuations`): (1) **Epochs** with a `local-to-global` cross-epoch memory bus (noting two critical soundness bugs they caught: epoch pointers must enforce strict `prev_epoch < cur_epoch` ordering to prevent self-cancelling forged rows, and the real-vs-padding selector must gate *both* local and cross-epoch interactions), and (2) **Prove-and-Retire**, a 2-pass streaming monolithic prover that commits & drops table chunks in pass 1, draws one global challenge, and re-executes in pass 2 to produce the exact monolithic proof with $O(1)$ chunk RAM and zero epoch-table duplication.

#### 14–17. 2025–2026 Cryptographic Frontier: Sharded Twist (`2025/105`), Lattice Folding (`Neo / SuperNeo / LatticeFold+ / Symphony`), Binary Towers (`Binius64 / Ligerito / Bolt`), and Commitment-Carrying IVC (`Spice / Nebula / Protostar / Sonobe`)
- **Primary Sources**:
  - Twist & Shout: `https://people.cs.georgetown.edu/jthaler/TwistShout.pdf` (§2.9.3, pp. 26–28), `https://eprint.iacr.org/2025/105`
  - LatticeFold / LatticeFold+ / Neo / SuperNeo / Symphony: `https://eprint.iacr.org/2024/257`, `https://eprint.iacr.org/2025/247`, `https://github.com/NethermindEth/latticefold`, `https://eprint.iacr.org/2025/294`, `https://eprint.iacr.org/2026/242`, `https://eprint.iacr.org/2025/1905`
  - PetraVM / Binius64 / Ligerito / Bolt: `https://petraprover.github.io/PetraVM/specification.html`, `https://github.com/binius-zk/binius64/blob/main/crates/core/src/constraint_system/value_index.rs`, `https://angeris.github.io/papers/ligerito.pdf` (`https://eprint.iacr.org/2025/1187`), `https://eprint.iacr.org/2026/310` (`https://github.com/bcc-research/bolt-rs`)
  - Spice / Nebula / Protostar / Sonobe: `https://eprint.iacr.org/2018/907`, **`https://eprint.iacr.org/2024/1605`** (Arun & Setty, IEEE S&P 2026; note `2023/1257` is unrelated VOLE work), `https://eprint.iacr.org/2023/620`, `https://github.com/privacy-scaling-explorations/sonobe/blob/main/crates/ivc/src/lib.rs`
- **Four Decisive Frontier Findings**:
  1. **Sharded Twist's $O(|M|)$-per-shard barrier (`TwistShout.pdf` §2.9.3, p. 27)**: Folding Twist shards via Nova requires committing to end-of-shard memory $S'$ and proving consistency with increments $\widetilde{\mathsf{Inc}}$, which costs **$O(|M|)$ time per shard** (unlike Merkle trees, which cost $O(\text{writes} \cdot \log |M|)$).
  2. **Nebula's $O(|M|)$-once advantage vs non-sealable intermediates (`2024/1605`)**: Commitment-carrying IVC pays $O(1)$ per RAM op and $O(|M|)$ **only once** at the end of the run, **but intermediate folds cannot seal standalone valid states** before the terminal `FS` scan, and Sonobe's `DeciderEth` (`nova-verifier.sol`) explicitly exposes `(i, z_0, z_i)` on-chain unless wrapped in a C6-style outcome decider.
  3. **Post-quantum lattice folding (`Neo`/`SuperNeo`/`LatticeFold+`/`Symphony`)**: Resets Ajtai witness norms $\|z\|_\infty < b < \beta_{\text{SIS}}$ at every fold via `split_b` decomposition so norm growth **never bounds run length**; over 64-bit Goldilocks, a single-scalar step counter naturally wraps at $q \approx 2^{64}$ (`u64`), matching Sherlock's `2^64` counter guard.
  4. **Binius64 & Ligerito**: `Binius64` packs a 2-bit segment tag + **30-bit word index (`2^30` 64-bit words = 8 GiB)** in `ValueIndex(u32)` (`value_index.rs`); `Ligerito` (`2025/1187`) and `Bolt` (`2026/310`) prove $N = 2^{20}\text{–}2^{30}$ polynomials in 0.08–80 s (840 ms commit in Bolt) with 145–420 KiB proofs and native Round-1 partial-evaluation batching (`ligerito.pdf` §6.5).

---

### 1.4 Direct Implications of Prior Art for Sherlock (`Q1`–`Q3`)

1. **Why Sherlock Must Use Boundary-Merkle Memory (Not Offline Multiset / Twist / Column Chaining)**:
   - `midnight-circuits` 7.3.0's in-circuit verifier supports **zero user challenge phases** (`verifier_gadget.rs:16-17`, cited at `study.md:254`) and **unit rotations `-1, 0, +1`** (`verifier_gadget.rs:609`, `design.md:122`). Offline LogUp/Spice/Nebula buses require verifier challenges (or expensive non-native extension-curve arithmetic in every leaf), and zkWasm/Twist snapshot chaining requires $O(|M|)$ rows every leaf.
   - Boundary-Merkle memory (`RISC Zero`, `OpenVM`, Sherlock §2.7) needs **zero user challenges**, seals every leaf immediately, supports arbitrary GP-4 DAG/tree shapes (`N`), and keeps prover memory $O(1)$ in run length.
2. **Why Sherlock's D3 Hit an Artificial Ceiling That RISC Zero and OpenVM Avoid**:
   - OpenVM also groups persistent memory into **8-cell Merkle chunks** (`CHUNK = 8`, height 29, `memory/controller/mod.rs`), yet OpenVM has **unbounded** run length because (a) steps that do not allocate new chunks do not advance the memory frontier, and (b) its chunk tree is 29 levels deep (`2^29` bytes) rather than burning one 8-cell slot unconditionally on every 16-step leaf! Fixing D3's unconditional per-leaf burn + 17-bit top tree (`Q2(d)`) aligns Sherlock directly with the proven RISC Zero / OpenVM architecture.
3. **Universal Validation of C6 (`SRC-006`) + Sherlock's Intentional `n_steps` Choice**:
   - Every SNARK-wrapped zkVM (`RISC Zero`, `SP1`, `OpenVM`, `Ziren`, `Pico`) strips `final_root` and shard counts before hitting the chain, validating C6's ban on `fin_d`, `post_d`, and `mem_root` (`SRC-006`). Publishing **logical** `n_steps` beside `claim_d` (while keeping `n_steps` out of `claim_d`, `SRC-007` & `dissent.md` D-3) matches RISC Zero's `WorkClaim` separation — safe *only* because Sherlock counts deterministic logical owner steps (`SRC-003`, `SRC-023`), never partition-dependent paging cycles!

---

## 2. Question 1: GP-1 & GP-001 Exact Replacement Text, What `2^64` Actually Is, and What "Unbounded" Honestly Means

### 2.1 Resolving the `2^64` Confusion (`BRIEF.md:5-7,14`)

The owner asked (`research/sherlock-runlength-consensus-20261111/BRIEF.md:6-7`):
> - *"2^64 is physically impossible"*
> - *"I'm not understanding what 2^64 is, is it a limit or a something we calculate? We cannot calculate a 2^64 circuit"*

Both observations are 100% right:
1. **No circuit ever has $2^{64}$ rows, gates, or SRS powers.** Every Sherlock circuit has a small, fixed power-of-two domain:
   - **Leaf**: `k = 13..16` ($8{,}192$ to $65{,}536$ rows, proving up to `S = 16` steps; `openspec/changes/sherlock-vm-consensus-20261107/design.md:105,127-135`).
   - **Recursive Node**: `k = 20` ($1{,}048{,}576$ rows; `823,466` rows over the E8 chip leaf, `research/sherlock-processor-analogy-20261110/experiments-4.md:53-63`).
   - **Run Decider**: `k = 20` ($1{,}048{,}576$ rows; `929,166` ordinary rows, `1,032,467` table rows; `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:213`).
2. **Proving work is strictly proportional to the actual steps executed ($N$), never $2^{64}$.** A 300-step program (`list123` = 303 steps, `requirements.md:40`) proves $\lceil 303/16 \rceil = 19$ fixed-size leaves, 19 fixed-size node folds, and 1 decider proof.
3. **What `2^64` actually is (`SRC-004`, `requirements.md:29`, `crates/mori-recursion/src/ivc.rs:359-376`)**:
   - Inside the node circuit, the step counter `h` is a field element modulo the 255-bit BLS12-381 scalar prime $r \approx 2^{255}$.
   - At genesis, the node detects the start of a run via `is_genesis = is_zero(prev_h)` (`crates/mori-recursion/src/ivc.rs:362`). Without an integer range check on `prev_h` and `h = prev_h + steps`, a malicious prover could pick `prev_h = r - 4` and `steps = 4` so `h ≡ 0 (mod r)`, wrapping the step counter back to genesis or forging `n_steps`!
   - `constrain_heights` (`crates/mori-recursion/src/ivc.rs:363-376`) prevents modular wraparound using three 64-bit (`u64`) range checks (`prev_h < 2^64`, `steps - 1 < S_MAX`, `h = prev_h + steps < 2^64`), costing **~30–50 rows** inside the 823,466-row node (`research/sherlock-speed-replan-20261108/prover.md:181-186`).
   - As Section 1 showed, **every global-counter zkVM in production uses the exact same pattern**: SP1 v6 range-checks a 48-bit `clk` ($2^{45}$ insns), Ziren checks `(u16, u26)` ($2^{39.7}$ insns), Brevis Pico checks `(u16, u25)` ($2^{39}$ cycles), ZKsync Airbender checks `(u19, u19)` ($2^{36}$ cycles), and zkWasm checks `eid: u32` ($2^{32}-1$ steps). None builds a $2^{45}$-row circuit; `2^64` is simply a `u64` non-overflow guard chosen wide enough that physical time runs out billions of years before the counter ever can.

### 2.2 What "Unbounded" Can Honestly Mean Here

Writing *"proves ... every run length below 2^64"* (`general-purpose.md:14`, `requirements.md:56`) conflated a **soundness non-wraparound guard** with a **physical completeness claim**. In honest technical terms, "unbounded run length and memory" in a universal recursive zkVM means **three distinct, exact properties**:

1. **Key- and Circuit-Unboundedness (Architectural Universality)**: One fixed key set and one verifier contract verify runs of any program on any input for any outcome without re-keying, new ceremonies, or capacity ladders (`general-purpose.md:9-13`). Circuit domain sizes (`k_leaf`, `k_node`, `k_decider`), final proof bytes (~6.4 KB, `review.md:204`), and on-chain verification cost are **$O(1)$ constants independent of run length and memory size**.
2. **No Narrower Sub-Counter Bottleneck Below the 64-Bit Counter Guard (`< 2^64`)**: Wherever the recursion or memory commitment carries an integer step height (`h`) or cell/segment counter (`len`), that counter is range-checked to 64 bits (`< 2^64`, `SRC-004`) strictly to prevent field wraparound modulo $r$ — and **no sub-circuit (such as D3's 17-bit segment walker, `crates/mori-chip/src/m3.rs:51,113-139`) may impose a narrower artificial ceiling** that forces an honest prover to abort early (`SVM-010`, `dissent.md:228-232`). Furthermore, GP-2 composition can chain checkpointed runs if multi-transaction execution is ever desired.
3. **Linear Prover Scaling Bounded Practically Only by Host Time and Storage**: Prover compute grows linearly $O(\text{n\_steps})$ in the **actual** steps executed (`ceil(n_steps / S)` leaves and folds), with $O(1)$ streaming node RAM and $O(\text{allocated cells})$ host witness storage. Practical run length on any machine is bounded solely by the operator's wall-clock budget and host storage — quantified honestly by a recorded **tested qualification target** (`SAS-001`, Section 5), never disguised as a `2^64` physical capability nor frozen into the circuit as a hard cap.

### 2.3 Exact Replacement Text (Zero Narrowing of GP-1 to GP-4)

#### A. Replacement for `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md` GP-1 (run-length bullet `:14` and acceptance line `:16`)

Keep lines `7-13` of `general-purpose.md` verbatim (`"One fixed key set proves the execution of every program of the Sherlock language, on every input, for every run length and every outcome..."`), and replace lines `14-16` with:

```markdown
- Run length scales linearly without re-keying or fixed-capacity circuits. Fixed-size leaf proofs (`S` steps) fold recursively into one constant-size final proof (`O(n_steps)` prover work, `O(1)` final proof size and on-chain verification); no circuit is sized to the total run length. Neither the leaf nor the memory commitment imposes a step, segment or cell ceiling narrower than the 64-bit non-wraparound counter guard (`< 2^64`, enforced in-circuit by `SRC-004` solely to prevent modular field overflow, far beyond physical reach); practically, run length on a given host is bounded only by prover wall-clock time and host storage, and GP-2 composition also chains runs across checkpoints when multi-run execution is chosen.

Acceptance: one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from, including one program whose state and step count span multiple leaves and multiple memory segments beyond one leaf's carriers (at or above the declared multi-segment qualification target), one run with input, and one run of each outcome kind.
```

#### B. Replacement for `GP-001` in `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56` (EARS Table Row)

- **Requirement cell (`requirements.md:56`)**:
```markdown
GP-1 universal VM. One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every run length and every outcome (`Halted(v)` for every value type, as a digest of a canonical value encoding; `Rejected(code)`; `Exhausted(kind)`) by folding fixed-size leaf proofs linearly in the executed step count into one constant-size proof; no circuit SHALL scale its domain with total run length, and neither the leaf nor the memory commitment SHALL impose a step, segment or cell ceiling narrower than the 64-bit non-wraparound counter guard (`< 2^64`, `SRC-004`); the public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants; capacity SHALL NOT depend on the key (state larger than one leaf's carriers runs against committed memory); a new program SHALL need no new key, contract or ceremony. When a supported run exceeds one leaf's step, storage or hashing capacity, the prover SHALL continue through authenticated memory and recursive decomposition under the same key set, bounded in circuit only by the 64-bit counter non-wraparound guard (`SRC-004`) and in practice only by prover time and host storage.
```

- **Acceptance check cell (`requirements.md:56`)**:
```markdown
One key set and one deployed verifier contract accept proofs of at least three programs the key set was not built from, including one multi-leaf, multi-segment program exceeding one leaf's step and carrier capacity at or above the declared qualification target (`>= 1,024` steps and `>= 256` memory cells), one run with input, and one run of each outcome kind (row A33).
```

#### C. Mirror Replacement for `openspec/changes/sherlock-vm-consensus-20261107/specs/sherlock-general-purpose/spec.md:7-15`

```markdown
### Requirement: GP-001 Universal VM
One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every run length and every outcome (`Halted(v)` for every value type as a digest of a canonical value encoding, `Rejected(code)`, `Exhausted(kind)`) by folding fixed-size leaf proofs linearly in the executed step count into one constant-size proof. No circuit SHALL scale its domain with total run length, and neither the leaf nor the memory commitment SHALL impose a step, segment or cell ceiling narrower than the 64-bit non-wraparound counter guard (`< 2^64`, `SRC-004`). The public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants. Capacity SHALL NOT depend on the key: a program whose state does not fit one leaf's carriers runs against committed memory. A new program SHALL need no new key, contract or ceremony.

When a supported run exceeds one leaf's step, storage or hashing capacity, the prover SHALL continue through authenticated memory and recursive decomposition under the same key set, bounded in circuit only by the 64-bit counter non-wraparound guard (`SRC-004`) and in practice only by prover time and host storage.

#### Scenario: Programs the keys were not built from
- **WHEN** one key set and one deployed verifier contract receive proofs of at least three programs the key set was not built from, including one multi-leaf, multi-segment program exceeding one leaf's step and carrier capacity (`>= 1,024` steps and `>= 256` memory cells), one run with input, and one run of each outcome kind.
- **THEN** every proof is accepted with no new key and no new contract (row A33).
```

---

## 3. Question 2: Should the Per-Run Ceiling Stay at D3, Be Raised, or Be Removed? Quantitative Comparison of (a), (b), (c), and (d)

### 3.1 Baseline Cost-Model Equations & Measured Anchors

All `ESTIMATE` values below use the repository's measured anchors and fitted equations:
- **E8 baseline (`S = 16, C = 8`, `TOP = 17`, `design.md:125-135`, `experiments-4.md:53-63`)**:
  - Leaf CS: `12` advice cols, `13` equality cols, `20` fixed cols, `4` lookups, max degree `6`, `22` gates $\to$ **`6,511` leaf rows (`k = 13`)**, **`823,466` node rows (`k = 20`)**, **`929,166` decider ordinary rows (`k = 20`)** and **`1,032,467` decider table rows** (`16,109` margin below $2^{20} = 1{,}048{,}576$, `review.md:213`).
- **Poseidon HASH chip (`crates/mori-chip/src/poseidon.rs:59,113-115`)**: **`70` leaf rows per permutation** (rate 2, width 3); `M3` today uses `8*2 + 7 + 2*17 + 2*15 = 87` perms = **`6,090` rows** (`crates/mori-chip/src/m3.rs:54`), and each `PAGE` cell read uses `2` (record) `+ 3 + TOP` perms = `(5 + TOP) * 70` rows (`1,540` rows at `TOP = 17`, `experiments-3.md:205-206`).
- **Node row sensitivity (`design.md:134`, `experiments-2.md:125,230`, `experiments/e2-e4.md:25-35`)**:
  - $\text{node\_rows} \approx 570{,}957 + 7{,}298 N_{\text{adv}} + 8{,}189 N_{\text{eq}} + 6{,}004 N_{\text{fix}} + 22{,}600 \Delta N_{\text{lookup}} + 6{,}460 \Delta N_{\text{filled\_sel}}$.
  - **Crucial measured invariance (E2, `experiments/e2-e4.md:25-35`)**: Raising `k_leaf` on a fixed CS shape adds **only `+4` node rows per `+1` `k_leaf`** and **`+0` decider rows**!

### 3.2 Summary Comparison Matrix Across Candidates (a), (b), (c1–c4), and Recommended (d)

*(Full leaf rows below include core + TRAP + DEC + `vd` + 4 `PAGE` cell openings + PROG openings at `S = 16`, against baseline full-leaf projection `16k–24k` rows `k = 14..15`, `design.md:117`.)*

| Candidate | Step Ceiling | Cell Ceiling | M3 / Leaf Rows (`ESTIMATE`) | `k_leaf` (`ESTIMATE`) | Node Rows (`ESTIMATE`) | Decider Rows (`ESTIMATE`) | Physical Partition Independent? | `SI-12..SI-14` (`D-1`) Impact | Verdict |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **(a) Keep D3 (`TOP=17`) + GP-2 run chain** | `< 2^17` leaves (`< 2^17..2^21` steps/run) | `< 2^20` cells/run | `6,090` M3 (`18k–24k` full) | `14–15` | `823,466` (`+0`) | `929,166` / `1,032,467` | **NO** (`len` pads `+8` per leaf; checkpoint claims leak slice boundaries) | **Broken** (padding gaps + `C=8` slice split) | **REJECT** (violates GP-1, SVM-010, C6) |
| **(b1) Widen D3 to 32-bit addr (`TOP=29`)** | **`< 2^29` leaves** (`< 2^29` steps at `S=1`!) | `< 2^32` (`4.29B` cells) | `7,770` M3 (`+1,680` M3, `+840`/PAGE; `23k–29k` full) | `15` | `~823,470` (`+4`) | `929,166` (`+0`) | **NO** (0-append leaves still burn 8 addrs & mutate `mem_root`!) | **Broken** (leaf-close padding gaps remain) | **INCOMPLETE** (`< 2^29 < 2^64` at `S=1`) |
| **(b2) Widen D3 to 64-bit addr (`TOP=61`)** | `< 2^61` leaves (`< 2^61..2^65`) | `< 2^64` cells | `12,250` M3 (`+6,160` M3, `+3,080`/PAGE; `36k–46k` full) | `16` | `~823,474` (`+8`) | `929,166` (`+0`) | **NO** (0-append leaves still burn 8 addrs; `len`/`fp` depend on partition) | **Broken** (leaf-close padding gaps remain) | **SUBOPTIMAL** (keeps padding holes & 1-step `2^61` cap) |
| **(c1) D3 + Zero-append skip (`m=0 -> no-op`)** | `< 2^64` on 0-alloc loops; `< 2^17..2^61` if 1 alloc/leaf | `< 2^{TOP+3}` | `+0` to `+70` over (b) | `15–16` | `~823,500–829,500` | `929,166` (`+0`) | **NO** (`m=1` leaf still pads 7 dead cells!) | **Broken** (`1..7` cell leaves still inject holes) | **PARTIAL FIX ONLY** |
| **(c2) Per-append unbatched Merkle (`0..8` walks)** | `< 2^64` (`SRC-004`) | `< 2^32` or `< 2^64` | **Huge**: `8 * 2 * 64 * 70 = 71,680` walk rows alone! | `17–18` (fails `k<=16` gate!) | `~823,500–830,000` | `929,166` (`+0`) | **YES** (dense `len`) | Fixes leaf gaps; `Call` interleave still needs cons-slices | **REJECT** ($8\times$ Merkle walk blowup breaks `k_leaf <= 16`) |
| **(c3) Growing-depth tree / MMR (`depth = ceil(log2 len)`)** | `< 2^64` | `< 2^64` | Dynamic loop up to `64` in fixed Halo2 circuit = same max rows as `64` + early-stop mux gates! | `16` | `~830,000–836,000` (`+1` sel col) | `929,166` (`+0`) | **YES** (if dense) | Same as dense | **INFERIOR TO FIXED DEPTH** (Halo2 fixed height pays max rows anyway + extra mux bug surface) |
| **(c4) Height-keyed addresses (`addr = (h, sub_idx)`)** | `< 2^64` (`h < 2^64`) | Sparse `2^66` keyspace | `+6,580` M3 + `+3,290`/PAGE (`TOP=66`) | `16` | `~823,500–830,000` | `929,166` (`+0`) | **YES** (`h` is logical step index!) | **Breaks `SI-12` completely** (every step jumps `h` by `+1`, zero cross-step contiguity) | **VIABLE ONLY WITH CONS-SLICES**, strictly worse depth than `(d)` |
| **(d) RECOMMENDED: `M3-Dense` (`W=15` tail-buffered dense segments + `TOP=61` [or `29`] + Carbon cons-slices)** | **`< 2^64` (`SRC-004`) under EVERY partition** | **`< 2^64` (`TOP=61`)** *(or `< 2^32` at `TOP=29`)* | `TOP=61`: **`13,860–14,350`** M3 (`40k–50k` full)<br>`TOP=29`: **`9,380–9,870`** M3 (`25k–31k` full) | `16` (`TOP=61`)<br>`15` (`TOP=29`) | **`~823,474–829,940`** (`k=20`, >120k under gate!) | **`929,166` ord / `1,032,467` tbl (`+0` rows!)** | **100% YES** (`len`, `fp`, `mem_root`, `tail_d` depend ONLY on executed steps!) | **SOLVED** (zero padding holes + cons-linked slices remove all `addr + k`) | **ADOPT** |

---

### 3.3 Detailed Technical Evaluation of Each Option

#### Option (a): Keep D3 (`2^17` leaves, `2^20` cells) and Chain Runs via GP-2 Continuations — **REJECT**
- **Mechanism**: Leave `m3.rs` unchanged (`TOP = 17`, `len_post = base + 8`, `< 131,072` leaves = `< 2,097,152` steps at `S = 16` or `< 131,072` steps at `S = 1`) and prove longer executions as multiple runs chained through GP-2 claims.
- **Why It Fails Four Adopted Requirements**:
  1. **`SVM-010` (`requirements.md:18`)**: Forbids encoding physical capacity exhaustion as `Rejected` or `Exhausted`, and forbids any machine rule from reading `len` or a segment index. Thus a normal Sherlock program **cannot detect** when D3 hits $2^{17}$ segments to checkpoint itself!
  2. **`SRC-006` & `GP-002` (`requirements.md:31,57`)**: `SRC-006` requires the decider to **refuse `Running`**, and `GP-002` explicitly rules: *"A consumed claim SHALL be a sealed claim `(prog_d', input_d', outcome_d')` opened from a terminal state in circuit; a segment statement or a state digest SHALL NOT be accepted as a claim of another run."*
  3. **`GP-1` (`general-purpose.md:9-15`) & `SRC-023` (`requirements.md:48`)**: Forcing the caller to slice a single deterministic computation into fuel-bounded sub-runs either changes the program into a state-stepping trampoline (violating `GP-1` "every program of the Sherlock language") or exposes fuel-slice checkpoint claims inside the aggregated `SET` (violating `C6` and `SRC-023` partition independence).
  4. **Severe Runtime Overhead**: Even with canonical state serialization (Section 4), every run boundary must fold live `env`/`frame`/`task` chains at halt and lazily unfold them in the next run, whereas intra-run `SEG + SEG -> SEG` continuations carry `post_d == pre_d` in $O(1)$ constraints with zero overhead.

#### Option (b): Widen D3's Segment Index (`TOP = 29` for 32-bit or `TOP = 61` for 64-bit) — **CHEAP IN NODE/DECIDER, BUT INSUFFICIENT ALONE**
- **Exact Cost Math (`m3.rs:51-54,104-144`, `poseidon.rs:59`, `experiments/e2-e4.md:25-35`)**:
  - In `WalkConfig::configure` (`m3.rs:104-144`), each top-tree level `l` already constrains boolean `b_l` and `acc_in = 2 * acc_out + b_l` using existing advice columns `a[2..11]` and 1 coded fixed column `wk.col`. **Zero `RANGE` table lookups and zero extra columns are needed to widen `TOP`** (`8 * sum_{l=0}^{60} b_l 2^l < 2^64 \ll r_{\text{BLS12-381}} \approx 2^{255}$)! (64-bit `addr < len` comparisons in `ALU` already fit inside the existing 12-row 128-bit `RANGE` running-sum column `range.rs:93-118` with zero added lookups.)
  - **`TOP = 29` (`seg < 2^29`, `base < 2^32` = 4.29B cells)**: Adds `+12` levels $\implies +24$ walk perms = **`+1,680` M3 leaf rows** (`6,090 -> 7,770`) and **`+840` rows per `PAGE` cell read** (`1,540 -> 2,380`). Full leaf stays at `k = 15` (`~23k–29k` rows `ESTIMATE`); Node grows by **`+4` rows** (`~823,470` `ESTIMATE`); Decider grows by **`+0` rows** (`929,166`).
  - **`TOP = 61` (`seg < 2^61`, `base < 2^64` cells)**: Adds `+44` levels $\implies +88$ walk perms = **`+6,160` M3 leaf rows** (`6,090 -> 12,250`) and **`+3,080` rows per `PAGE` cell read** (`1,540 -> 4,620`). With 4–5 `PAGE` reads + core + `vd`, full leaf is `~36k–46k` rows = **`k = 16` (`ESTIMATE`, within `k_leaf <= 16` gate)**; Node grows by **`+8` to `+12` rows** (`~823,478` `ESTIMATE`); Decider **`+0` rows**.
- **Why Pure Option (b) Must Not Be Adopted Alone**:
  1. **Zero-append leaves still burn segments (`len_post = base + 8`, `m3.rs:133`)**: Under `TOP = 29`, a 1-step-per-leaf partition (`SRC-023` acceptance test!) hits `2^29` steps and aborts — violating `GP-001` (`< 2^64`); even under `TOP = 61`, a 1-step-per-leaf pure-compute loop caps at `2^61 < 2^64` steps!
  2. **Physical state remains partition-dependent**: Closing a leaf after 1 step vs 16 steps shifts `len` by `+8`, altering every subsequent `fp`, `tp`, `Data(addr)`, `Env(addr)`, `mem_root`, and `post_d`. While C6 hides `post_d` at the terminal seal, any internal proof debugging, Quint refinement (`SVM-011`), or future checkpointing remains polluted by partition-dependent dead holes.
  3. **Leaves `SI-12` (`dissent.md` D-1) completely unsolved**.

#### Option (c): Memory Designs Without Fixed Per-Leaf Segment Allocation (`c1`–`c4`)
- **(c1) Zero-Append Skip (`if m == 0 { post.mem_root = pre.mem_root; len_post = base }`)**: Lets pure register loops run to `< 2^64` steps without burning segments, costing only 1 mux gate (`~+70` leaf rows `ESTIMATE`). **Flaw**: Any leaf that appends just **1 cell** (e.g., 1 frame push) still pads 7 dead cells (`len_post = base + 8`), leaving physical state partition-dependent and `SI-12` broken.
- **(c2) Per-Append Unbatched Cell Merkle Updates (`0..8` independent depth-64 walks per leaf)**: Eliminates segments completely (`len` advances by exact `+1`), **but** supporting up to 8 appends/leaf requires $8 \times 2 \times 64 = 1{,}024$ walk permutations = **`71,680` leaf rows (`ESTIMATE`)** for memory writes alone — blowing past the `k_leaf <= 16` ($65{,}536$-row) gate (`design.md:105`) to `k = 17..18`! (Batching 8 adjacent cells in a segment subtree cuts Merkle updates $8\times$; throwing away segment batching is pure waste.)
- **(c3) Dynamically Growing Merkle Depth / Merkle Mountain Range (`depth = ceil(log2 len)` or MMR peaks)**: In a static PLONK/Halo2 circuit (`design.md:90`: *"Chip heights are constants of the circuit"*), the circuit must still lay out all `61` (or `64`) walk levels for the worst case! Stopping early at dynamic depth `d` saves **zero circuit rows**, adds dynamic mux selectors (`~+6,460` node rows if a new selector column is needed, `ESTIMATE`), and introduces canonicality/prefix-extension soundness hazards if tree height is not bound into every root. An MMR over 61 peaks would require carrying 61 peak digests in `pre_d`/`post_d` (`+30` perms/side = `+4,200` leaf rows `ESTIMATE`) instead of 1 `mem_root`!
- **(c4) Height-Keyed Address Space (`addr = 4 * h + sub_op`, `0 <= sub_op < 4`, `h < 2^64`)**: Keys every cell by the logical step height `h` at which it was created (`TOP = 63` or `66`). **Pros**: Automatically partition-independent (`h` is a logical step invariant!) and never runs out before `h = 2^64`. **Cons**: (1) Sparse keyspace precludes 8-cell cross-step segment amortization unless steps consecutive in `h` share a 32-slot `(h >> 3)` bucket — which on low-allocation code (`0.22` allocs/step, `design.md:119`) leaves ~94% of bucket slots empty and forces an `M3` walk on *every* leaf with $\ge 1$ append (instead of once every ~36 steps!); (2) breaks `SI-12` contiguity across microsteps completely.

---

### 3.4 Option (d) — Recommended Design: `M3-Dense` (Write-Back Tail-Buffered Dense Segment Memory + `TOP = 61` + Carbon Cons-Linked Slices)

We can get **all** the $8\times$ write-amortization of D3's 8-cell segments, **zero** segment burn on 0-append or small-append leaves, **exact dense addresses (`len` = exact cell count)**, **100% physical partition independence**, and **`+0` decider rows / `k = 20` node** with one clean refinement to `m3.rs`: **carry the partial open 8-cell segment (`0..7` cells) in a 1-word digest `tail_d` inside `pre_d`/`post_d`, flushing to `mem_root` ONLY when an 8-cell segment actually fills!**

#### 1. Exact State Invariant & Layout (`M3-Dense`)
- **State digest (`31` words = `16` Poseidon perms = `1,120` rows/side, `+70` rows/side over M3's 30 words)**:
  $$\texttt{state\_d} = \mathsf{Poseidon}(w_0, \dots, w_{28}, \texttt{mem\_root}, \texttt{tail\_d}).$$
- **Genesis (`len = 8`)**: Segment `0` (`0..7`, containing `NIL = 0`) is reserved (`design.md:96`). At `h = 0`, `len = 8` (`seg = 1`, `off = 0`), `mem_root = empty_top(TOP)`, and `tail_d = EMPTY_SEG_ROOT` (the constant depth-3 root over 8 `R_PAD = H(0,0,0,0)` records).
- **Canonical Decomposition at Any Step Height `h`**:
  Write exact dense cell count `len = 8 * seg + off` with **`seg = len >> 3` (`1 <= seg < 2^61`)** and **`off = len & 7` $\in \{0, \dots, 7\}$**:
  - `mem_root` is the depth-`TOP` (`TOP = 61`, depth-64 cell tree; or `TOP = 29` if 32-bit `4.29B`-cell cap is preferred) binary Merkle root over all **completed, 100%-full** 8-cell segments `0 .. seg - 1` (slots `>= seg` are `0`).
  - `tail_d` is the depth-3 binary Merkle root over `[r_0, ..., r_6, R_PAD]` where `r_k = H(1, cell_{8*seg + k})` for `0 <= k < off` and `r_k = R_PAD` for `off <= k < 7`.
- **Leaf `MEM` Region (`W = 15` rows at fixed offsets `k = 0..14`, `addr_k = 8 * seg + k`)**:
  - Holds up to `7` carried-in cells of open segment `seg` (`k = 0 .. off - 1`) plus up to **`C = 8` newly appended cells** (`k = off .. off + m - 1`, `0 <= m <= 8`, `total = off + m <= 15`). Because every Sherlock microstep appends `<= 3 <= 8` cells (`design.md:119`), at least one step always fits (`design.md:90`); if adding another step would exceed `m = 8`, the prover closes the leaf cleanly.
  - **Bus Provider Split (`D2` & `D6` preserved verbatim)**:
    - `PAGE` serves `CELL` strictly for `addr < 8 * seg` (completed segments in `pre.mem_root`).
    - `MEM` serves `addr_k = 8 * seg + k` (`k = 0..14`) whenever `post_act[k] = [k < total] == 1`. Step reads check `addr < running_len` (`design.md:103`), which simultaneously **(a)** blocks reading cells appended by later slots in the same leaf and **(b)** serves reads of recently created cells in `[8 * seg, pre.len)` **directly from `MEM` with zero `PAGE` Merkle openings**!
- **Bijective Soundness & Leaf-Close Flush (`flush = [total >= 8]`)**:
  - Every `MEM` row `k = 0..14` hashes `rec[k] = H(1, cell[k])` **once** (`15 * 2 = 30` perms):
    1. *Carry-in check (`7` perms)*: `seg_root([pre_act[0]?rec[0]:R_PAD, ..., pre_act[6]?rec[6]:R_PAD, R_PAD]) == pre.tail_d` pins `cell[0 .. off - 1]` to `pre.tail_d`.
    2. *New appends*: The `m` enabled append micro-ops emit bus lookups at consecutive `running_len` values `8*seg + off .. 8*seg + off + m - 1`. Since `MEM` keys `8*seg + k` are strictly distinct (`D6`), `lookup_any` uniquely pins `cell[off .. off + m - 1]`!
    3. *Flush mux*:
       - If **`flush == 0` (`0 <= total <= 7`, including every 0-append leaf!)**: `post.mem_root == pre.mem_root` (while `old_walk` still proves slot `seg` is `0` in `pre.mem_root` and `seg < 2^61`), `post.tail_d` hashes `cells[0 .. total - 1]`, and `post.len = len_end = 8 * seg + total`. **Zero segments burned; zero padding holes!**
       - If **`flush == 1` (`8 <= total <= 15`)**: `post_act[0..7]` are all `1` (8 real written records, **zero padding cells ever enter `mem_root`**!). `new_walk` writes `full_seg_root = seg_root(rec[0..7])` (`7` perms) into slot `seg` producing `post.mem_root`, `post.tail_d` hashes the spillover `cells[8 .. total - 1]` (`7` perms) for new open segment `seg + 1` (requiring `seg + 1 < 2^61` when `total > 8`), and `post.len = len_end = 8 * seg + total`.

#### 2. Exact Row & Cost `ESTIMATE`s for `M3-Dense`
- **M3-Dense HASH permutations per leaf**:
  - `30` (15 records) `+ 7` (`pre.tail_d`) `+ 7` (`full_seg_root`) `+ 7` (`post.tail_d`) `+ 2 * TOP` (walks) `+ 32` (`pre_d`, `post_d` of 31 words) = **`83 + 2 * TOP` permutations**:
    - At **`TOP = 61` (64-bit cell addresses `< 2^64`, `< 2^64` steps — zero sub-`2^64` ceiling anywhere)**: `205` perms = **`14,350` M3 leaf rows (`ESTIMATE`)** (`+8,260` over E8's `6,090`). With core + TRAP + DEC + `vd` + 4–5 `PAGE` openings (`66 * 70 = 4,620` rows each), total leaf is **`~40,000–50,000` rows = `k = 16` (`ESTIMATE`)**, satisfying `k_leaf <= 16` (`design.md:105`)!
    - *(Optional faster-leaf variant `TOP = 29`: `141` perms = **`9,870` M3 leaf rows (`ESTIMATE`)**, `2,380` rows/PAGE, full leaf **`~25,000–31,000` rows = `k = 15` (`ESTIMATE`)**. Because `M3-Dense` burns **0 cells** on non-allocating steps, pure-compute runs STILL reach `< 2^64` steps even at `TOP = 29`, while memory capacity is `2^32 = 4.29 billion` cells = ~137 GB of 3-field cells — impossible to exhaust in years of proving! However, `TOP = 61` fits `k = 16` and eliminates even the theoretical `2^32`-cell memory bound.)*
- **Node & Decider Cost (`ESTIMATE`)**:
  - `M3-Dense` uses **zero new advice columns** (`12`), **zero new equality columns** (`13`), **zero new lookup arguments** (`4`), and at most **0 to 1 additional coded fixed column** (`20 -> 20..21`, `~0` to `+6,460` node rows) plus `+8` to `+12` node rows for `k_leaf = 15..16` (`experiments/e2-e4.md:25-35`).
  - **Node**: **`~823,474 – 829,940` rows (`k = 20`, `ESTIMATE`)** — over **120,000 rows of headroom** below the `950,000` gate!
  - **Decider**: **`929,166` ordinary rows / `1,032,467` table rows (`+0` rows, `k = 20`)** — leaves the tight `16,109` table-row margin (`review.md:213`) completely untouched because `tail_d` stays inside `cur_d` and terminal runs discard `mem_root` and `tail_d` upon opening the terminal outcome (`SRC-006`)!

#### 3. Soundness, `SI-12..SI-14` (`D-1`), `SRC-023`, and `GP-2..GP-4` Under `M3-Dense`
- **`SRC-023` (Partition Independence)**: **Strongest possible guarantee** — not only are published `(prog_d, input_d, outcome_d, n_steps)` partition-independent, **the physical state `(w_0..w_28, mem_root, tail_d)` itself is 100% deterministic and identical at step $h$ across every possible leaf partition** (`S = 16`, `S = 1`, or arbitrary mixed splits), because `len`, `mem_root`, and `tail_d` depend solely on the sequence of cells appended by steps `1..h`! Furthermore, on typical Sherlock code (`0.22` appends/step, `design.md:119`), an 8-cell segment fills only once every ~36 steps (~2.25 leaves), while up to 7 recent cells stay hot in `MEM` across leaf boundaries without `PAGE` openings.
- **`SI-12..SI-14` (`dissent.md` D-1)**:
  1. Eliminating leaf-close padding makes physical addresses strictly dense (`8, 9, 10, ...`), so consecutive `XAdtHelperBuildField` microsteps (`owner_runtime.rs:2990`) no longer suffer cross-leaf padding gaps even when a constructor has `> 8` fields!
  2. However, in §2.7's unified `MEM` space (`DATA + FIELDS + ENV + FRAMES + SLOTS`), evaluating arguments $e_1, e_2, \dots, e_m$ of a multi-arg `Call` interleaves frame/env/data appends between argument-slot writes. Therefore, we **pair `M3-Dense` with Carbon's cons-linked slice encoding (`dissent.md:199`, Task 4.18)**: inline `<= 2` child refs directly (`Leaf`, `Cons`, `Branch`), and represent variable-length argument lists and ADT constructor field lists as immutable write-once linked cells `(TAG_SLICE, elem_ref, prev_link)`. Then **no rule ever computes `addr + k`** (enforcing `SVM-010` structurally), `SI-12` becomes exact linked-list ownership, and `SI-13`/`SI-14` (`child_addr < parent_addr`, `prev_env < own_addr`) hold strictly on dense monotone `len`! (Avoids Astra's `logical_id -> m3_addr` translation table, which would cost `+2,380` to `+4,620` extra `PAGE` rows per translated read, `review.md:54`.)
- **`GP-2` to `GP-4`**: Keeps `SEG` boundary digests fixed-size (1 scalar `pre_d`, 1 scalar `post_d`), requiring zero end-of-run `FS` memory sweeps (`Nebula`/`SP1`) and enabling arbitrary parallel binary-tree / DAG folding in `N` (`GP-4`), clean terminal sealing for `GP-2`, and untouched node budget for `GP-3` foreign key registry checks.

---

## 4. Question 3: Continuations Across Runs (`k -> k+1`), Canonical Machine-State Digest, `input_d`, GP-2 Aggregation, and C6 Interaction

**Architectural Distinction First**: With `M3-Dense` (Section 3.4), a single long computation does **not** need to split into multiple GP-2 runs; it folds `SEG + SEG -> SEG` **inside** one run's recursion tree (`SHKIVC06` / `N`), where intermediate `(pre_d, post_d)` stay private and vanish at the decider. Nevertheless, if a multi-run continuation chain (`run_0 -> run_1 -> ... -> run_{K-1}`) *is* constructed (either to evaluate Option Q2(a) or to support voluntary multi-transaction suspension/resumption in a future `/2` profile), here is the exact cryptographic protocol required by `SRC-006`, `SRC-021..023`, `SVM-010..012`, and `GP-002`:

### 4.1 Canonical, Partition-Independent Full Machine-State Digest (`ms_d`)
Raw physical digests (`post_d`, `fin_d`, `mem_root`) **cannot** cross run boundaries: `SRC-006` (`requirements.md:31`) forbids exposing them, `GP-002` (`requirements.md:57`) forbids consuming a segment/state digest as a claim, and physical pointers (`fp`, `tp`, `len`, `Data(addr)`) depend on layout/allocation history (`SRC-023`).

Instead, suspension must occur at a **deterministic logical trigger** (specifically `fuel == 0` under a suspension-enabled profile — **never** a physical leaf/address cap per `SVM-010`, `requirements.md:18`), transitioning to a terminal checkpoint outcome (either a fourth terminal tag `Suspended(ms_d)` with `outcome_d = H(O, 1, 3, ms_d)` or `Halted(Left(ms_v))` under `sherlock-value/1`, `SRC-022`), where `ms_d = vd(ms_v)` is the **address-free canonical `sherlock-value/1` structural digest (`sherlock-state/1`)** of the full reachable machine state:
1. **Control & Logical Registers**: `ctl_d = H(SHK_CTL, mode_tag, expr_or_rule_id, cur_val_vd, stack_depth, task_depth, helper_logical_state)` (`expr_id` is a static AST index in `prog_root`; physical `len`, `fp`, `tp` are excluded).
2. **Heap Values (`O(1)` at Suspend via `SVM-012`!)**: By `SVM-012` (`requirements.md:20`), every heap data value created during run `k` **already carries its canonical address-free `vd` bound into its write-once record**! Thus run `k` does **not** have to traverse the reachable heap DAG at suspension time — every value reference in registers, environments, frames, and tasks already exposes its authenticated `vd` in $O(1)$!
3. **Live Environment Chain (`env_vd`)**: Folded deterministically from the active `env` pointer (`NIL -> vd_nil`; `EnvCell(binder_id, val_ref, prev_env) -> H(SHK_ENV, binder_id, vd(val_ref), env_vd(prev_env))`).
4. **Live Frame Stack (`frames_vd`) & Task Stack (`tasks_vd`)**: Folded top-down from cached `top` through `fp` to `NIL` (`H(SHK_FRM, frame_tag, ret_expr_id, saved_env_vd, args_slice_vd, below_frames_vd)`), and identically for `task`/`tp`.
5. **Canonical Machine State Digest**:
   $$\texttt{ms\_d} = \mathsf{vd}(\texttt{ms\_v}) = H(\texttt{SHK\_MS},\; 1,\; \texttt{prog\_d},\; \texttt{ctl\_d},\; \texttt{env\_vd},\; \texttt{frames\_vd},\; \texttt{tasks\_vd}).$$
Because `ms_d` hashes only AST IDs, logical depths, and `sherlock-value/1` digests, **two runs of `prog_d` with identical logical inputs and fuel slice produce bit-identical `ms_d` regardless of leaf partitioning, physical memory addresses, garbage cells, or subvalue sharing (`SRC-023`)!**

### 4.2 What `input_d` Is for Run `k + 1` (Preventing State-Injection Forgery)
Under `SRC-021` (`requirements.md:46`), `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)`.
- **Red-Team Trap**: If run `k + 1` simply took `val_d = ms_d` and resumed from `ms_v` without authenticating *how* `ms_d` was produced, an attacker could fabricate an arbitrary mid-program state `ms_fake` (e.g., jumping straight to a privileged return expression with a forged accumulator) and prove a valid run `k + 1` from `ms_fake`!
- **Sound Provenance Binding via GP-2**: Run `k + 1` takes structured continuation input `val_d = vd(Resume(claim_{k}, ms_v))` where `claim_k = (prog_d, input_d^{(k)}, outcome_d^{(k)})`. During run `k + 1`'s `Init` / `ENTRY` check (`SRC-021`, `GP-002`):
  1. Run `k + 1` consumes `claim_k` as an **in-circuit verified GP-2 dependency** (`deps_d` records `claim_d^{(k)} = Poseidon(SHK_RUN, 1, prog_d, input_d^{(k)}, outcome_d^{(k)})`), asserting `claim_k.prog_d == self.prog_d` and `claim_k.outcome_d == H(O, 1, TAG_SUSPENDED, ms_d)`!
  2. Run `k + 1` sets its registers to `ctl_d` and **lazily unfolds** `env_vd`, `frames_vd`, `tasks_vd`, and referenced heap `vd` values into fresh dense `MEM` cells as execution pops frames or reads variables (using the exact `SRC-021` lazy-unfold mechanism so `Init` never scans the whole state in one leaf, `requirements.md:46`, `design.md:253-260`).

### 4.3 Proving the Entire Chain Once On-Chain (`GP-2` Aggregation)
In the GP architecture (`research/sherlock-gp-architecture-20261110/architecture.md:75-120`):
1. Each run $j \in \{0, \dots, K-1\}$ seals via run-lift/seal into a `RUN` claim $C_j = (\texttt{prog\_d}, \texttt{input\_d}^{(j)}, \texttt{outcome\_d}^{(j)})$ with unsatisfied dependency set $D_0 = \emptyset$ and $D_j = \{C_{j-1}\}$ for $j \ge 1$.
2. Binary aggregation nodes `N` (`SET + SET -> SET`) merge adjacent sub-chains, discharging each internal assumption $C_{j-1} \in D_j$ against the verified claim $C_{j-1}$ produced by the left child so the root `SET` has **zero unresolved dependencies ($D_{\text{root}} = \emptyset$)**.
3. Closing circuit `C` resolves deferred fixed-base scalars and `RunDecider` `D` emits **one single ~6.4 KB Blake2b proof** verified **once** on-chain!

### 4.4 How This Interacts With C6 Outcome-Only Publication (Why Intra-Run `M3-Dense` Wins)
C6 (`SRC-006`, `SRC-007`) forbids publishing physical state digests so that (a) replay protection `claim_d = H(SHK_RUN, 1, prog_d, input_d, outcome_d)` is invariant to proving partitions (`SRC-023`), and (b) private intermediate execution states are not leaked on-chain. Comparing GP-2 run-chaining against C6 exposes a sharp dilemma:
- **Case 1 — Batch `SET` exposes all $K$ claims $\{C_0, \dots, C_{K-1}\}$ (`GP-002` part (b))**: Although `ms_d` is canonical (no physical addresses), publishing $C_0 \dots C_{K-2}$ on-chain **publishes intermediate logical machine-state digests `ms_d`**, makes the on-chain claim set depend on the prover's fuel-slice schedule `(fuel_0, ..., fuel_{K-1})`, and adds $K$ claims to calldata/replay storage.
- **Case 2 — A coordinator/wrapper run `W` consumes $C_0 \dots C_{K-1}$ privately (`GP-002` part (a)) and publishes only `C_W = (prog_W, input_d^{(0)}, final_outcome_d)`**: This preserves single-claim C6 privacy on-chain, **but** requires wrapping/modifying the guest semantics into a fuel-sliced checkpoint/resume protocol and replaces the run's `prog_d` and `n_steps` with `prog_W`'s!
- **Conclusion**: Inter-run GP-2 continuations are the right design **only** when suspension is an intentional **logical** multi-transaction feature (e.g., async `Await` in profile `/2`), whereas **intra-run `SEG + SEG -> SEG` continuations over `M3-Dense`** keep intermediate `(pre_d, post_d)` 100% hidden inside recursion and satisfy `C6`, `GP-1`, `GP-2`, and `SRC-023` natively for single runs of any length.

---

## 5. Question 5: Practical Run Length on This Hardware and Recommended Spec Tested Targets

### 5.1 Host Hardware & Measured Prover Throughput (`qa/sherlock/runs.jsonl`, `speed-levers.json`, `e2-e4.md`)
- **Host (`research/sherlock-speed-replan-20261108/prover.md:15-16`, `qa/sherlock/runs.jsonl:713`)**: `aarch64` NVIDIA GB10 / RTX Spark N1X — **20 ARMv9 cores** (10x Cortex-X925 + 10x Cortex-A725, no SMT), **120.3 GiB** (`126.1 GB`) unified LPDDR5X @ 273 GB/s, integrated Blackwell GPU (`sm_121`, 6,144 CUDA cores, unbuilt past smoke F23), **~3,057 GiB** free NVMe. Kernel cgroup caps (`SRC-016`, `requirements.md:41`): **`48 GiB`/process**, **`100 GiB` aggregate** (`60 GiB` tested), `20 GiB` free reserve, `<= 4 GiB` swap.
- **Measured CPU Component Baselines (`midnight-proofs` 0.8.3, 20 threads)**:
  1. **Node step (`k = 20`, `652,437` rows, `resident_pk`)**: **`59.62 – 63.61 s`** (`61.07 s` solo in `A16-node-step`, **`16.58 – 17.35 GB` RSS**; `qa/sherlock/runs.jsonl:648-649`). At `823,466–830,000` rows on the same $2^{20}$ FFT/MSM domain (`d = 0.884`, `wiki-llm/recursion-perf/profiles/c0-first-profile.md:27-55`), one solo `k = 20` node step projects to **`~62 – 68 s` (`ESTIMATE`)** at **`~17.5 – 19.0 GB` RSS (`ESTIMATE`)**.
  2. **Chip Leaf (`S = 16`)**: Measured `k = 13` leaf = **`2.55 s`**, `< 1 GB` RSS (`prover.md:11`); measured `k = 20` leaf (`742,327` rows, E4) = **`66.2 s`**, `11.14 GB` RSS (`runs.jsonl:709`, `experiments/e2-e4.md:7-19`). Thus a `k = 15..16` `M3-Dense` chip leaf (`25k–50k` rows, $16\times\text{–}32\times$ smaller than `k = 20`) projects to **`~4 – 9 s` solo (`ESTIMATE`)** at **`~1.0 – 2.2 GB` RSS (`ESTIMATE`)**.
  3. **Pipelined Contention (`SRC-011` makespan formula, `requirements.md:36`)**: When a `k = 15..16` leaf proves concurrently beside a `k = 20` node (`max(node_service, leaf_service)`), the leaf finishes in `< 15%` of the node's window, adding modest bandwidth contention (**`~65 – 72 s` effective per 16-step fold = `~4.1 – 4.5 s / owner step` `ESTIMATE`**, aggregate RSS **`~19 – 21.5 GB` `ESTIMATE`**, well under `48 GiB` / `60 GiB`!). *(Under binary `N` tree folding with `P = 3` concurrent `k = 20` workers at measured `f3 = 1.73x`, `runs.jsonl:616`, throughput improves to `~130 s / 3 = ~43.3 s` per node = **`~2.7 s / owner step` `ESTIMATE`** at `~48 GB` aggregate RSS; if optional `mori-gpu` `SRC-018` lands a $2.5\times\text{–}4\times$ MSM+NTT speedup, **`~1.0 – 1.8 s / step` `ESTIMATE`**.)*
  4. **Fixed Run Overhead (Setup + Decider + Verify + `validate_call`)**: SRS/PK load (`~45–90 s`, `runs.jsonl:648,709`) + `k = 20` Run Decider (**`70.35 – 77.90 s`**, `17.35 GB` RSS, `runs.jsonl:617,649`) + `SRC-010` bundle verify & `SRC-020` local `validate_call` (`~20–45 s` `ESTIMATE`) = **`~140 – 210 s` (`2.3 – 3.5 min`, `ESTIMATE`)**.

### 5.2 Practical Scaling Table on This Host (`S = 16`, CPU Today)
Crucially, **peak RAM is $O(1)$ (`~19–21.5 GB` sequential, `~48 GB` at `P = 3`, `ESTIMATE`) regardless of run length**: each node fold drops the previous witness, `GP bundle` proof files (`SRC-019`) take only `~10 KB` per 16-step chunk on disk (`100,000` leaves = `~1 GB` of `3,057 GiB` free NVMe), and `M3-Dense` host Merkle witness nodes take `< 100 MB` per million cells (`ESTIMATE`). **Wall-clock time alone bounds practical run length:**

| Workload / Horizon | Owner Steps (`S = 16`) | Leaves & Folds | Projected CPU Wall Time (`P=1` pipelined, `ESTIMATE`) | Peak RSS (`ESTIMATE`) | Practical Status on This Host |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `entry 7;` (`G-B`, `SRC-015(b)`) | `6` steps | `1` leaf + `1` node + dec | **`~3.5 – 4.7 min`** | `~18.5 GB` | Trivial smoke (`< 20 min` cap) |
| `fact(3)` / `list123` (`G-C`, `SRC-011`) | `293` / `303` (`<= 320`) | `19 – 20` leaves | **`~24 – 28 min`** | `~20 GB` | **4x faster than mandatory `< 2 h` gate** |
| **Recommended GP-1 Multi-Segment Target** | **`1,024` steps** (`>= 256` cells) | **`64` leaves** (`>= 32` segs) | **`~72 – 80 min` (`1.2 – 1.35 h`)** | `~20 GB` | **Fits comfortably inside `< 2 h` `heavy` cap** |
| **Max `< 2 h` (`7,200 s`) CPU Budget** | **`~1,550 – 1,700` steps** | `~97 – 106` leaves | **`~2.0 hours`** | `~20 GB` | Upper bound for routine CI / qualification jobs |
| Overnight Soak (`12 h`) | **`~9,600 – 10,500` steps** | `~600 – 656` leaves | **`~12.0 hours`** (`~7.5 h` at `P=3`) | `~20 GB` (`48 GB` `P=3`) | Practical overnight single-host run |
| Full-Day Soak (`24 h`) | **`~19,200 – 21,000` steps** | `~1,200 – 1,312` leaves | **`~24.0 hours`** (`~15 h` at `P=3`) | `~20 GB` (`48 GB` `P=3`) | Practical 1-day single-host ceiling on CPU |
| Old D3 Ceiling (`2^17` leaves) | `2,097,152` steps | `131,072` leaves | **`~98 – 109 days`** continuous CPU | `~21 GB` + `~1.3 GB` disk | Already 2+ orders of magnitude past routine CPU runs |
| **`SRC-004` Counter Guard (`2^64` steps)** | **`1.845 × 10^19` steps** | `1.153 × 10^18` leaves | **`~2.38 – 2.65 trillion years`** (~$180\times$ age of universe) | Physical impossibility | Pure `u64` non-wraparound soundness guard (`SRC-004`) |

### 5.3 What Length the Spec Should Name as Tested Targets
Never confuse a tested qualification benchmark with a circuit cap (`SAS-001`, `requirements.md:65`):
1. **Keep the Mandatory Seeded Throughput Gates Unchanged (`SRC-011`, `SRC-015(b)-(c)`)**: `entry 7;` (`6` steps `< 20 min`) and `fact(3)` / `list123` (`293` / `303 <= 320` steps `< 2 h`, `< 48 GB` per process, `< 100 GB` aggregate).
2. **Add One Explicit GP-1 Multi-Segment Continuation Tested Target (`1,024` owner steps, `>= 256` allocated cells across `>= 32` full 8-cell segments, proved in `< 2 h` under `< 48 GB` / `< 100 GB` in row `A33` / `SAS-001`)**:
   - *Why `1,024` steps and `>= 256` cells?* `list123` (`303` steps, `~65` cells) touches only ~8 segments. A `1,024`-step program (`64` leaves at `S = 16`) that (i) allocates `>= 256` cells (`>= 32` flushed segments), (ii) runs a `>= 256`-step zero-append compute loop (verifying 0 segment burn), and (iii) reads back early segment cells via `PAGE` openings proves in **`~75 min` (`< 2 h`, `ESTIMATE`)** on this exact CPU host while exercising every mechanism that makes run length and memory genuinely unbounded!
3. **Record Tested Bounds Explicitly in `SAS-001`**: Every release receipt states `counter_guard: "< 2^64 steps / < 2^64 cells (SRC-004, M3-Dense TOP=61)"` alongside `tested_bounds: "1,024 steps, >= 256 cells (< 2 h CI); optional 8,192-step soak"`.

---

## 6. What Was Not Checked

In accordance with the Round 1 rules (`BRIEF.md:42,75-79`), the following items were **not** checked or executed in this study:
1. **No circuit compilation, `cost_model` runs, MockProver tests, or heavy proving**: Because builds, tests, and proving are forbidden in Round 1 (`BRIEF.md:76`), `M3-Dense` (`W = 15`, `TOP = 29` / `61`) and Carbon cons-linked slices were not coded in `crates/mori-chip` or measured through `midnight-proofs` `cost_model`; their row and timing numbers are analytic projections marked `ESTIMATE` from the measured E2/E4/E8/A16 baselines.
2. **Pre-adoption full rule inventory (`R`) wiring in `crates/mori-chip`**: Only the E8 spike (`crates/mori-chip`, `6,511` rows) exists in code today; the exact leaf row count of the full 39-field `Init` (`SRC-021`), creation-time `vd` (`SVM-012`), and all rules in `R` inside `mori-chip` awaits Tasks 4.16–4.19.
3. **GPU (`sm_121`) proving throughput (`SRC-018`)**: `crates/mori-gpu` has only passed toolchain smoke `F23`; no BLS12-381 MSM or NTT kernels have been benchmarked end-to-end inside `midnight-proofs`, so Q5 relies strictly on measured CPU throughput and marks GPU numbers `ESTIMATE`.
4. **Closed/prebuilt external zkVM artifacts in Q4**:
   - *RISC Zero*: Precompiled `.zkr` recursion binaries (`lift`, `join`, `resolve`) are fetched from RISC Zero's artifact bucket at build time (`zkr.rs`), so constraint semantics were verified via `risc0/zkvm/src/host/recursion/`, `claim/receipt.rs`, `zkvm-specification.md`, and `ControlID.sol` rather than decompiling `.zkr` bytecode.
   - *Valida*: In `lita-xyz/valida-vm` (`v1.0.0` commit `c36d3177...`), `basic-api/src/machine/multi_segment.rs` is omitted from the public git tree (shipped only in binary releases), so cross-segment challenge ordering was verified from `persistence/src/lib.rs`, `memory/src/stark.rs`, `machine/src/proof.rs`, and `basic-api/src/commands/verify.rs`.
   - *IACR ePrint challenge-gated PDFs*: Direct `eprint.iacr.org/*.pdf` URLs behind interactive anti-bot gates were never bypassed; Twist & Shout (`2025/105`) and Ligerito (`2025/1187`) were read in full from ungated author-hosted PDFs (`people.cs.georgetown.edu/jthaler/TwistShout.pdf`, `angeris.github.io/papers/ligerito.pdf`), whereas Nebula (`2024/1605`) and Symphony (`2025/1905`) lack official public code repositories and were verified via ungated ePrint metadata, cross-citations in `TwistShout.pdf` §2.9.2–2.9.3, and `sonobe` source code.
   - *DNS-unreachable docs host*: `docs.nexus.xyz` did not resolve in DNS (`curl: (6)`), so Nexus 1.0/2.0 and 3.0 were verified directly from `nexus-xyz/nexus-zkvm` source (`releases/0.2.4` and `main` `v0.3.6`), `nexus_whitepaper.pdf`, and `specification/zkvm-spec-3.0.pdf`.
5. **Other Round 1 lanes**: `lanes/B`, `lanes/C`, `lanes/D`, and `lanes/E` were never listed or read.
