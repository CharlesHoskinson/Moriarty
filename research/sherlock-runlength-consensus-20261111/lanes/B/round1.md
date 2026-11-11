# Sherlock Run Length and Memory Ceiling — Round 1 Position (Lane B: Chip Leaf Memory Design and Cost Modeling)

Date: 2026-11-11
Study: `sherlock-runlength-consensus-20261111` (`research/sherlock-runlength-consensus-20261111/BRIEF.md:1-79`)
Scope: Independent Round 1 analysis of Q1–Q5, leading on Q2 (chip leaf memory architecture, constraint-system cost equations, `SI-12..SI-14` slice invariants, and `SRC-023` partition independence).

---

## Executive Summary of Lane B Positions

1. **Q1 (What `2^64` is, honest meaning of "unbounded", and exact GP-1 / GP-001 text)**:
   - `2^64` in `SRC-004` (`crates/mori-recursion/src/ivc.rs:79,363-376`) is the **bit-width of the integer step counter `h`**, checked in 4 field constraints per node fold so `h = prev_h + steps` cannot wrap modulo the BLS12-381 scalar prime $r \approx 2^{255}$ back to `0` (`is_genesis`). **No circuit ever has $2^{64}$ rows**: leaf domains are $2^{13}..2^{16}$ rows (`8,192..65,536`) and node/decider domains are fixed at $2^{20} = 1{,}048{,}576$ rows (`research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md:25-35`). Because proving work grows linearly $O(N)$ with executed steps $N$, executing or proving $2^{64}$ steps is physically impossible (`BRIEF.md:6-7`).
   - "Unbounded" in GP-1 honestly means **key-independent capacity and linear-work extensibility**: the fixed verifying-key set bakes in **zero** program-size, stack-depth, heap-size, or step-count table cap; any physically realizable computation is proved by folding fixed-size leaves within a run and chaining runs through GP-2 continuation claims without re-keying or redeploying contracts.
2. **Q2 (Per-run ceiling and chip leaf memory design — Lane B lead)**:
   - **Reject keeping D3 (`C = 8`, 17-bit segment index, unconditional `+8` address advance per leaf) as-is (Q2a)**: D3 burns 1 segment (`8` addresses) on every leaf close even when `used = 0` (`crates/mori-chip/src/m3.rs:27,138`), capping pure-compute loops at `131,071` leaves (`131,071` steps at `S = 1`, `2,097,136` steps at `S = 16`), breaking contiguous slice indexing `addr = start + k` across leaf boundaries (`SI-12`, `SI-23`), and making internal addresses (`len`, `fp`, `tp`, `mem_root`, `pre_d`, `post_d`) depend on leaf partition choices (`research/sherlock-c6-outcome-ledger9-review-20261110/review.md:38-40,51-55`).
   - **Widening the D3 segment index `B` alone (Q2b) is cheap in `node` (`+4` rows per `+1` in `leaf_k`, `+0` in `decider`) up to `B = 32..48`, but fails to fix `SI-12` or zero-append waste**: Each `+1` bit of `B` adds `+140` rows (`+2` Poseidon permutations) on the E8 leaf spike (`6,511 -> 8,611` rows at `B = 32`, crossing `k = 13` [`<= 8,186` usable rows] into `k = 14` [`16,384`] at **`B = 29`**), and adds `ESTIMATE +70 * (2 + N_page_cell)` rows per bit on a complete leaf with `N_page_cell` old-memory openings (`~41,791` rows at `B = 32, N_page_cell = 8`, `k = 16` vs `~64,191`–`101,711` rows at `B = 64`, `k = 16..17`). Yet D3 with `B = 64` still inserts leaf-close holes that break `SI-12` and partition-independent state digests.
   - **Recommended Synthesis Design (Q2d) — Hash-Stack Frames/Tasks + Dense Carry-Buffered Merkle Arena (`B = 32`, `C = 8` $\implies 2^{35} = 34.36 \times 10^9$ cells) + GP-2 Continuations**:
     1. Move `FRAMES` and `TASKS` out of addressable arena memory into cached-top Poseidon hash chains (`research/sherlock-gp-architecture-20261110/architecture.md:245,251,443-444`), so stack pushes/pops consume **0** arena addresses and pay **0** Merkle paths.
     2. Advance `len` strictly by actual arena allocations (`post.len = pre.len + used`, `0 <= used <= 8`, **zero leaf-close padding holes**), carry the 8 record hashes `open_r[0..7]` of the single open segment `floor(len / 8)` inside `pre_d`/`post_d`, and flush to the `B = 32` Sparse Merkle tree **only when `open_r` fills all 8 slots** (`flushed = 1`).
     3. **Result**: Pure-compute and stack-heavy loops consume **0** memory segments (unbounded up to `h < 2^64` in a single run); `2^35` arena cells (`~1.1 TB` logical cells) would take `ESTIMATE > 1,350 years` of proving on this host to fill; every `Build`, `Cons`, and `Append` slice in `mori-core` stays **100% physically contiguous (`addr = start + k`) across arbitrary leaf boundaries** (`SI-12`, `SI-23`); and **`pre_d(h)` / `post_d(h)` itself becomes bit-for-bit invariant to leaf partitioning (`SRC-023`)**! E8-equivalent leaf size is `ESTIMATE 10,439` rows (`k = 14`), full universal leaf is `ESTIMATE ~42,179` rows (`k = 16`), node is `ESTIMATE ~829,930–836,390` rows (`k = 20`), and decider is unchanged at `929,166` rows (`k = 20`).
3. **Q3 (Continuations)**: Introduce a domain-separated internal continuation claim `cont_claim = H("SHK_CONT", 1, prog_d, input_d_0, ckpt_d, n_cum)` carrying an address-free canonical checkpoint digest `ckpt_d` (`sherlock-ckpt/1`, computed in $O(1)$ leaf work by maintaining incremental `vd` [`SVM-012`], `env_cd`, and stack digests, and resumed lazily per `SRC-021`). GP-2 / GP-4 (`SEG + SET -> SET`) discharges intermediate `SHK_CONT` dependencies privately in-circuit so the terminal `RunDecider` / `SherlockClaims` publishes **only** the C6 outcome-only statement `[prog_d, input_d_0, outcome_d, n_steps]` (`SRC-006`, `SRC-007`).
4. **Q4 (Prior art)**: Production zkVMs split into **Boundary Paged Merkle** (RISC Zero, OpenVM, zkWasm host SMT), **Global Multiset / Elliptic-Curve Bus** (SP1 Turbo/Hypercube, Ziren/zkMIPS, Valida), **Continuous Relocated / Protocol Kernel Recursion** (Cairo/Stwo, Miden), and **Monolithic Streaming Sum-Check / STARK** (Jolt Twist & Shout, Nexus 3.0). Every system that achieves on-chain partition independence hides segment/shard counts and final RAM commitments inside recursion, publishing only `(program_id, public_io_digest)` (plus `n_steps` in Sherlock C6).
5. **Q5 (Practical run length on this host and spec targets)**: On this 20-core ARM64 + `120.3 GiB` unified-RAM host (`48 GiB`/proc, `60 GiB` operational / `100 GiB` nominal aggregate caps), `S = 16` chip-leaf + `k = 20` node throughput is `ESTIMATE ~4.1–4.8 s/step` serial (`~750–875 steps/h`, `~18k–21k steps/day`) and `ESTIMATE ~2.9–3.3 s/step` with 3-way tree concurrency (`~1,100–1,250 steps/h`, `~26k–30k steps/day`). Keep the mandatory `SRC-011` gate at **`320` steps in `< 2 h`**, and add a GP-1 multi-leaf/memory endurance qualification target of **`4,096` steps (`>= 256` leaves, `>= 4,096` allocated cells) in `< 6 h`** plus a **2-run `>= 1,024`-step GP-2 continuation test in `< 2 h`**.

---

## Q1. GP-1 Run-Length Semantics, Exact Replacement Text, and Meaning of "Unbounded"

### 1.1 What `2^64` Actually Is — and Why There Is No `2^64` Circuit

In the current codebase and specification, `2^64` appears in `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:15`, `requirements.md:27,57`, `specs/sherlock-general-purpose/spec.md:8`, and `specs/sherlock-recursion/spec.md:35-36` (`SRC-004`). Its actual role in `crates/mori-recursion/src/ivc.rs:77-79,359-376,497-501` is strictly a **64-bit integer overflow guard on the accumulated step counter `h`**:

1. **Fixed small circuits folded linearly**: Every leaf proof proves `1 <= steps <= S_MAX` (`S_MAX = 16`, `ivc.rs:77`) machine steps inside a fixed-size PLONK circuit (`k = 13..16`, i.e., `8,192..65,536` rows for the chip leaf; `design.md:240-264`). Each self-recursive node proof (`k = 20`, `1,048,576` domain rows, `741,708..823,466` active rows; `e2-e4.md:28`, `design.md:252`) folds one leaf proof into the running KZG accumulator, and one `RunDecider` proof (`k = 20`, `929,166` rows; `crates/mori-recursion/src/run_decider.rs:190-329`, `e2-e4.md:28`) checks the final pairing. **No circuit anywhere has $2^{64}$ rows or gates.**
2. **Why `SRC-004` checks `h < 2^64`**: All node public inputs live in the BLS12-381 scalar field $\mathbb{F}_r$ ($r \approx 2^{255}$). At `ivc.rs:498`, genesis is detected by `is_genesis = std.is_zero(l, &prev_h)`. Without a range bound on `prev_h` and `h = prev_h + steps`, a malicious prover could pick field elements `prev_h = r - 5, steps = 5` so `h = 0 (mod r)`, re-triggering `is_genesis = 1` mid-run and discarding the entire prior proof accumulator (`ivc.rs:359-362,538`). `constrain_heights` (`ivc.rs:363-376`) closes this soundness attack by checking `prev_h < 2^64`, `steps - 1 < 16`, and `h = prev_h + steps < 2^64`.
3. **Why "for every run length below `2^64`" in GP-1 was defective**:
   - Proving work is strictly $\Theta(N)$ in the number of executed machine steps $N$ (`BRIEF.md:14`). Even at $1\,\mu\text{s/step}$ (roughly $4 \times 10^6\times$ faster than today's single-host CPU prover), $2^{64} \approx 1.84 \times 10^{19}$ steps would take $\approx 584{,}942$ years. Stating that the system proves "every run length below $2^{64}$" reads as an impossible operational claim (`BRIEF.md:6-7`) rather than a counter-width soundness guard.
   - Worse, while GP-1 named `2^64` for `h`, `design.md` §2.7 (`m3.rs:50,125-139`) quietly capped physical leaves per run at `< 2^17` (`131,071` leaves after genesis segment 0) and physical cells at `< 2^20` (`review.md:40,94`).

### 1.2 What "Unbounded" Can Honestly Mean in Sherlock

In a recursive SNARK VM where every executed step must be witnessed and folded, "unbounded run length and memory" has **three exact, honest meanings** (and one thing it can never mean):

1. **Unbounded key validity (zero capacity baked into keys)**: The fixed verifying-key set `{U, R, F, N, C, D}` (or `{leaf, node, decider}` in M0) contains **no circuit constant** that caps program AST/bytecode length, call-stack depth, helper-task depth, environment length, heap node count, or step count below machine word/address non-wraparound widths. Growing a workload never triggers key regeneration, trusted setup, or contract redeployment (`general-purpose.md:13-17`).
2. **Linear-work intra-run scaling ($O(N)$ proving, $O(1)$ on-chain verification)**: Within a single run, arbitrary finite traces are decomposed into $\lceil N / \bar{s} \rceil$ fixed-size leaves over commitment-authenticated memory (program Merkle root, stack hash chains, and sparse Merkle heap) and folded in chains or balanced trees (`GP-4`), bounded only by fixed arithmetic counter/address widths (`h < 2^64` for step count in `SRC-004`; $2^{35}..2^{64}$ cells for arena memory under Q2d) chosen wide enough that no physically executable single run can wrap or exhaust them.
3. **Cross-run continuation composition (`GP-2`)**: Any computation that checkpoints across time, machines, memory compactions, or per-run budgets is proved as a sequence of runs linked in-circuit via authenticated GP-2 continuation claims and sealed into a **single** constant-size on-chain proof whose published statement `[prog_d, input_d, outcome_d, n_steps]` is identical to an unsplit run (`SRC-006`, `SRC-023`).
4. **What it never means**: Sub-linear proving without executing the steps, or materializing a circuit of size $2^{64}$. Practical length is always governed by physical prover time ($\propto N$) and host storage for the live working set.

### 1.3 Exact Replacement Text for `general-purpose.md` GP-1

Replace lines 11–18 of `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md` (`### GP-1. Universal VM`) with:

```markdown
### GP-1. Universal VM

One fixed key set proves the execution of every program of the Sherlock language, on every input, for any physically realizable run length and every outcome:

- The program and the run input are **public inputs (commitments)** to the statement, not constants of any circuit. Changing the program or the input never changes a proving key, verifying key or deployed verifier contract.
- Every outcome of the reference machine is provable: `Halted(v)` for every value type (published as a digest of a canonical value encoding, not u128-only), `Rejected(code)`, and `Exhausted(kind)` under explicit resource parameters in the run input.
- **Run length and memory scale by folding fixed-size proofs, never by growing a circuit**: each leaf proves up to `S` steps over commitment-authenticated state, nodes fold leaves in linear prover work `O(N)` with constant final proof size, and long or resumable computations also compose across runs through GP-2 continuation claims under the same key set. Per-run arithmetic widths (`h < 2^64` in `SRC-004` and the committed-memory address width) exist solely to prevent field-integer wraparound, not as circuit sizes, and exceed any physically executable single run.
- **Capacity does not depend on the key.** A program whose state does not fit one leaf's registers runs against committed memory (hash stacks and Merkle commitments opened per access). A new program or larger input needs no new key, contract or ceremony.

**Acceptance.** One key set and one deployed verifier contract accept proofs of at least three programs the key set was not built from, including one program and state larger than one leaf's registers, one multi-leaf run exceeding single-leaf step and memory budgets, one 2-run continuation chain aggregated into one proof, one run with input, and one run of each outcome kind.
```

### 1.4 Exact Replacement Text for `GP-001` (EARS)

#### (a) Summary Table Row in `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:57`

```markdown
| GP-001 | Universal VM | One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every finite run proved by folding fixed-size leaf proofs within the `SRC-004` non-wrap counter bound (`h < 2^64`) and across runs via GP-2 continuations, and for every outcome (`Halted(v)` for every value type as a digest of a canonical value encoding, `Rejected(code)`, `Exhausted(kind)`). `prog_d` and `input_d` SHALL be public-statement data, not circuit constants; capacity SHALL NOT depend on any key; proving cost SHALL grow with executed steps while circuit sizes and the final on-chain proof remain fixed; a new program SHALL need no new key, contract or ceremony. | One key set and one contract accept >= 3 unseen programs, including one larger than one leaf's registers, one multi-leaf run above single-leaf step and memory capacity, one aggregated 2-run continuation, one with input, and each outcome kind (row A33). |
```

#### (b) Full EARS Requirement in `openspec/changes/sherlock-vm-consensus-20261107/specs/sherlock-general-purpose/spec.md:7-15`

```markdown
### Requirement: GP-001 Universal VM
One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every finite run length achievable by folding fixed-size leaf proofs (bounded per run only by non-wraparound arithmetic invariants including `h < 2^64` of `SRC-004`, and extensible across runs without re-keying via `GP-002` continuation claims), and for every outcome (`Halted(v)` for every value type as a digest of a canonical value encoding, `Rejected(code)`, `Exhausted(kind)`). The public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants. Capacity SHALL NOT depend on the key: no circuit in the key set SHALL grow with run length or memory size, and a program whose state or trace exceeds one leaf's registers and step slots SHALL execute against authenticated memory commitments and recursive proof composition under the same key set. A new program or larger workload SHALL need no new key, contract or ceremony.

When a supported run exceeds one leaf's step, storage or hashing budget, or is split across continuation checkpoints, the prover SHALL continue through authenticated memory, intra-run segment folding, and/or `GP-002` continuation claims under the same key set, producing one constant-size proof whose published statement satisfies `SRC-006` and `SRC-023`.

#### Scenario: Programs and lengths the keys were not built from
- **WHEN** one key set and one deployed verifier contract receive proofs of at least three programs the key set was not built from, including one program larger than one leaf's registers, one multi-leaf run (`>= 4,096` steps and `>= 4,096` allocated cells), one 2-run continuation chain folded into a single proof, one run with input, and one run of each outcome kind.
- **THEN** every proof is accepted with no new key and no new contract, and the continuation proof yields the same `claim_d` and `n_steps` as an unsplit run of the same `(prog_d, input_d)` (row A33).
```

---

## Q2. Chip Leaf Memory Design and Per-Run Ceiling (Lane B Deep-Dive)

### 2.1 Exact Baseline Circuit Anatomy: Where Every Row Comes From

To model memory alternatives without guesswork, we extract the exact row and column equations from the implemented chip spike (`crates/mori-chip/src/{rfcore.rs,m3.rs,mem.rs,poseidon.rs,range.rs,alu.rs,bus.rs}`), `midnight-proofs 0.8.3`, and experiments E2–E8 (`research/sherlock-processor-analogy-20261110/experiments-{2,3,4}.md`, `research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md`).

#### 2.1.1 Domain Size `k` and Exact `nb_unusable_rows` Threshold
In `tools/profile/vendor/midnight-proofs-0.8.3/src/plonk/circuit.rs:2344-2389` and `src/dev/cost_model.rs:370-398`:
- `rfcore.rs` (`narrow,1,pos_m3+r1+a1+pc`) queries advice columns only at rotations `{-1, 0, +1}` (`3` distinct points; `MC:verifier/verifier_gadget.rs:609` panics on any rotation outside `-1..+1`; `research/sherlock-processor-analogy-20261110/experiments.md:31-36`) and uses `0` trashcans.
- Thus `cs.blinding_factors() = max(3, 3) + 0 + 1 + 1 = 5` (`circuit.rs:2346-2376`), `nb_unusable_rows = 5 + 1 = 6` (`cost_model.rs:370`), and `min_circuit_size = max(rows_count + 6, table_rows_count + 6, nb_instances + 6, 9)` (`cost_model.rs:373-381`).
- With `table_rows_count = 4,096` (`crates/mori-chip/src/range.rs:34`, 12-bit lookup table), the **exact usable `rows_count` threshold** for each domain $2^k$ is:
  - **`k = 13` (`8,192`)**: `rows_count <= 8,186`
  - **`k = 14` (`16,384`)**: `8,187 <= rows_count <= 16,378`
  - **`k = 15` (`32,768`)**: `16,379 <= rows_count <= 32,762`
  - **`k = 16` (`65,536`)**: `32,763 <= rows_count <= 65,530`
  - **`k = 17` (`131,072`)**: `65,531 <= rows_count <= 131,066`

#### 2.1.2 Exact Decomposition of the `6,511`-Row E8 Chip Leaf (`S = 16`, `C = 8`, `B = 17`)
`rfcore.rs:1011-1048,1119-1243` configures **12 advice columns**, **1 instance column** (`13` equality-enabled permutation columns), **20 fixed columns**, **4 `lookup_any` arguments** (`bus` lane 0, `bus` lane 1, `range` 12-bit table, `dec` table), **22 gates**, and **`max_deg = 6`** (`experiments-4.md:61-63`), laying out one contiguous region `"rf leaf"` of **`6,511` rows**:

| Region Block | Formula | E8 Parameters | Exact Rows | Source |
|---|---|---|---|---|
| **1. Core slots (`Layout::Narrow`)** | $13 \cdot (S + 1)$ | $S = 16$ (`29` digested + `3` scratch regs across 6 pair rows, 4 flag rows, 3 bus rows/slot) | **`221`** | `crates/mori-chip/src/rfcore.rs:42-45,168-220,1204` |
| **2. ALU + 1-col RANGE** | $12 \cdot N_{\text{alu}}$ | $N_{\text{alu}} = 16$ (`12` rows $\times$ 12-bit lookup $= 144$ bits, top limb capped to 4 bits $\implies 128$-bit exact) | **`192`** | `crates/mori-chip/src/range.rs:12,33-35,135-140`; `rfcore.rs:1205-1208` |
| **3. M3 Segment (`SegMemConfig`)** | $C$ | $C = \text{SEG} = 8$ (`(TAG_MEM, addr, a, b, c)` + `addr, len_end, act`) | **`8`** | `crates/mori-chip/src/mem.rs:123,150-189`; `rfcore.rs:1224-1227` |
| **4a. M3 Record Hashes** | $70 \cdot 2 C$ | $8 \times 2 = 16$ perms of `H(act, a, b, c)` (`70 = 68 + 2` rows/perm) | **`1,120`** | `crates/mori-chip/src/m3.rs:50-55`; `poseidon.rs:56-59` |
| **4b. M3 8-Cell Subtree** | $70 \cdot (C - 1)$ | $7$ binary compressions over the 8 record hashes | **`490`** | `crates/mori-chip/src/m3.rs:52-55` |
| **4c. M3 Dual Top-Tree Walks** | $70 \cdot 2 B$ | $B = \text{TOP} = 17 \implies 34$ perms (`0 -> pre.mem_root`, `seg_root -> post.mem_root`) | **`2,380`** | `crates/mori-chip/src/m3.rs:20-28,53-55,125-139` |
| **4d. Boundary State Digests** | $70 \cdot 2 \lceil \frac{N_{\text{dig}}+1}{2} \rceil$ | $2 \times \lceil 30/2 \rceil = 30$ perms (`pre_d`, `post_d` over 29 regs + `mem_root`) | **`2,100`** | `crates/mori-chip/src/m3.rs:48,54-55` |
| **Total E8 Spike** | | `87` Poseidon perms (`6,090` rows) + `421` non-hash rows | **`6,511`** | `openspec/.../design.md:251`; `experiments-4.md:62,157` |

**Critical Completeness Note (`ESTIMATE`)**: The `6,511`-row spike omits: (i) **`PAGE` Merkle openings** (`N_page_prog` program reads at fixed depth $D_{\text{prog}} \approx 16$ costing `18` perms $= 1{,}260$ rows each, plus `N_page_cell` old-memory reads at depth $D_{\text{cell}} = B + 3$ costing `B + 5` perms $= 70(B + 5)$ rows each; `experiments-3.md:205`), (ii) **`SVM-012` `vd` creation hashing** (`+24` perms $= +1{,}680$ rows for 8 allocations; `review.md:135-136`), and (iii) **39 architectural fields** vs 29 (`+10` perms $= +700$ rows; `design.md` §3.2(5)). Below, we model **both** the exact E8-spike delta and the complete universal leaf (`N_page_prog = 8, N_page_cell = 8`), because widening $B$ scales **both** M3 leaf-close walks (`2B` perms) **and** every `PAGE` old-cell read (`N_page_cell * B` perms).

#### 2.1.3 Node (`823,466` Rows, `k = 20`) and Decider (`929,166` Rows, `k = 20`) Cost Laws
1. **Column/lookup sensitivity (`experiments-2.md:125,223-230`, `experiments-3.md:174`, `experiments-4.md:92-109`)**:
   $$\text{node\_rows} \approx 570{,}957 + 7{,}298\,N_{\text{advice}} + 8{,}189\,N_{\text{perm}} + 6{,}004\,N_{\text{fixed}} + 22{,}600\,(N_{\text{lookup}} - 7)$$
   (max residual `3,917` rows; filled fixed/selector columns cost `6,352–6,611` node rows, whereas all-zero fixed columns commit to `G1::identity()` at `~0` cost).
2. **Domain-height (`leaf_k`) invariance (`e2-e4.md:25-35`, `e2-rows.jsonl:1-7`)**:
   Across `leaf_k` $= 13 \dots 20$, `node_rows` grows from `741,708` (`leaf_k = 13`) to `741,736` (`leaf_k = 20`) — **exact `+4` node rows per `+1` in `leaf_k`** (one native-field squaring for $Z_H(\xi) = \xi^{2^k} - 1$), while **`decider_rows` stays identically `929,166` (`+0` rows)** (`table_rows = 1,032,467`, `review.md:213`) because leaf fixed bases are resolved inside the node (`crates/mori-recursion/src/ivc.rs:550-559`, `inner.rs:207`) and `RunDeciderRelation` (`run_decider.rs:209-245`) verifies only `node_vk` (`nr_cols = 4, self_k = 20`).

---

### 2.2 Why D3 Breaks Run Length, `SI-12..SI-14`, and Internal `SRC-023`

1. **Unconditional segment burn on every leaf (`m3.rs:20-28,125-139`)**:
   `m3 walk first` (`l = 0`) enforces `x = 0` (old top-tree leaf at `idx` was empty), `8 * acc_in = base`, and **`len_post = base + 8`**! Even when a leaf executes 16 pure register/ALU steps with `used = 0` appends, `post_d.len` advances by `+8` and consumes segment `idx`. With segment `0` reserved at genesis (`base = 8`, `experiments-4.md:148`), a run is hard-capped at **`2^17 - 1 = 131,071` leaves** (`<= 2,097,136` steps if all leaves run `S = 16` steps; **`<= 131,071` steps** if `S = 1` or 1-step early closes). Exceeding `131,071` leaves makes `acc_out(16) = 0` (`m3.rs:28,139`) unsatisfiable (`SVM-010` Scenario: prover produces no proof; `specs/sherlock-vm/spec.md:85-89`).
2. **`SI-12`, `SI-13`, `SI-14`, and `SI-23` slice/index invariants in `mori-core`**:
   In `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:4698-4818,5065-5073`:
   - **`SI-12` (`owner_runtime.rs:4698-4741`) & `SI-21`/`SI-22`/`SI-23` (`owner_adt_runtime.rs:1585-1646`, `owner_si21_closure.rs:632-645,994-998`)**: `DATA` constructor field slices `(start, len)` in `data_fields` and live `ValSlice`s `(start, len)` in `value_slots` are indexed by **direct address addition `start + k`** across `PreImage::field_at` (`owner_runtime.rs:300-331`), `bind_pair_payload` (`families/structural.rs:637-643`), `append_cell` (`families/environment.rs:212-216`), `impl_build_cell` (`owner_adt_runtime.rs:675-679`), `Plan` (`owner_adt_runtime.rs:901-907`), `BHelperMatchField` (`crates/mori-backend/src/sherlock_machine/c6_match_helper_step.rs:307`), `BHelperBindPair` (`c6_bind_helper_step.rs:240,253`), and `BHelperValidateField` (`c6_validate_field_step.rs:1910`).
   - **`SI-13` (`owner_runtime.rs:4742-4806`) & `SI-14` (`owner_runtime.rs:4807-4818`)**: Require topological ordering `child_id < owner_id` on `DATA` refs and `prev_id < i` on `ENV` cells, plus `id < active.*` bounds (with resource caps checked against logical `alloc.*` counters in `cap_first`, `owner_runtime.rs:3129-3162`).
   - **How D3 breaks `SI-12` and `SI-23` (`review.md:40,51-55`, `dissent.md:196-206`)**:
     - *(i) Inter-leaf padding holes*: Any multi-step `Build` or `Append` slice crossing a leaf boundary with `used < 8` gets `8 - used` dead zero cells (`act = 0`) injected inside `start .. start + len - 1`, so `start + k` reads zero padding instead of the $k$-th element!
     - *(ii) Intra-leaf frame/task spills in a unified log*: Under D7 cached frames (`d7-frames.md`), if `FRAMES` or `TASKS` spill into the same physical address counter as slices, any spill during slice construction also separates slice cells (`review.md:54`). *(Crucial observation from `owner_runtime.rs`: during `Append` [`environment.rs:212-265`] and `Build` [`owner_adt_runtime.rs:647-725`], `mori-core` executes consecutive `AppendCell`/`BuildCell` microsteps with **zero** intervening `DATA`, `ENV`, `FRAMES`, or `TASKS` allocations! Thus if leaf-close holes are removed and stacks are separated, `start + k` contiguity is already preserved natively — see §2.6.)*
3. **Partition dependence (`SRC-023`)**: Because closing a leaf at step 3 vs step 16 shifts `base` by `+8` at different step heights, `len`, `fp`, `tp`, every `RefC::Data` address, `mem_root`, and `pre_d`/`post_d` diverge across partitions (`review.md:38-45`). C6 hides this at final settlement by stripping `fin_d` (`SRC-006`) and computing canonical `outcome_d` (`SRC-022`), **but** any continuation boundary between runs cannot pass raw `post_d` without violating `SRC-023`!

---

### 2.3 Option (a): Keep D3 (`2^17` Leaves, `2^20` Cells) + GP-2 Continuations

- **Mechanism**: Retain `TOP = 17, C = 8` in `m3.rs`; prove runs exceeding `131,071` leaves or `1,048,568` physical cells by splitting into sequential sub-runs linked via GP-2 claims (Q3).
- **Rows & Cost (`ESTIMATE`)**:
  - E8 spike + `SVM-012` `vd` creation (`+1,680` rows; `review.md:135-136`): `6,511 + 1,680 = 8,191` rows (`> 8,186` usable at `k = 13` $\implies$ **already `k = 14`**). With `PAGE` (`N_page_prog = 8, N_page_cell = 8` at depth 20) + 39-field state digests + linked-slice fix for `SI-12`: **`ESTIMATE ~31,291–33,500` rows (`k = 15..16`)**.
  - Node: `ESTIMATE ~823,470–830,000` rows (`k = 20`); Decider: `929,166` rows (`k = 20`).
  - Continuation overhead: Every `<= 131,071` leaves (or earlier on `S < 16`), the prover must execute an extra continuation seal, `ResumeInit`, and GP-2 `SEG + SET -> SET` join.
- **Soundness, `SI-12..14`, `SRC-023`, and `GP-2..4` Verdict — REJECT as Sole Fix**:
  1. **Unprovable live working sets `> 2^20` cells**: Even with GC compaction at continuation boundaries, if the **live** reachable heap + stack + environment at a single step exceeds `1,048,568` cells, **no continuation can serialize or resume it** inside a `2^20`-cell address space — directly violating `GP-1` ("Capacity does not depend on the key").
  2. **Pure-compute runs needlessly forced into continuations**: A register-only loop with **0** allocations burns all `131,071` segments in `131,071..2,097,136` steps, and whether a single run succeeds depends on the prover's partition `S`!
  3. **`SI-12` / `SI-23` remains broken** unless slices are rewritten as linked lists (`dissent.md` D-1) or given separate dense tables.

---

### 2.4 Option (b): Widen Segment Index `B` (`24, 28, 29, 32, 48, 64` Bits) Under D3

#### 2.4.1 Exact Range-Check & ALU Impact (`0` Extra Rows Up to `B = 125`!)
- Core registers (`len`, `fp`, `tp`, scalars) and `ALU` `LT` (`addr < base` for `D2`) **already operate on full 128-bit range-checked integers** via the 12-row $\times$ 12-bit `RANGE` table (`range.rs:12,33-35`).
- In `m3.rs:116-139`, `idx = base / 8 < 2^B` is **not** sent to `RANGE`: `m3 walk` enforces `b_l * (b_l - 1) = 0`, `acc_in(l) = 2 * acc_out(l) + b_l` ($l = 0 \dots B-1$), `acc_out(B-1) = 0`, and `8 * acc_in(0) = base`. Since $8 \cdot 2^B \le 2^{128} \ll r \approx 2^{255}$ for every $B \le 125$, `m3 walk` **exact-checks** $\text{idx} < 2^B$ and $\text{base} < 2^{B+3}$ with **0 extra `RANGE` rows, 0 extra `ALU` rows, 0 extra columns, and 0 extra lookups**!

#### 2.4.2 Exact Leaf, Node, and Decider Row Table Across `B` (`ESTIMATE` from Repo Equations)
- **E8 Spike (`N_page_cell = 0`)**: $\text{rows}_{\text{E8}}(B) = 6{,}511 + 140(B - 17) = 4{,}131 + 140B$. Crosses `k = 13` (`8,186` max usable rows) into `k = 14` (`16,384`) at **`B = 29`** (`8,191` rows, `+5` over `8,186`!).
- **Complete Universal Leaf (`N_page_prog = 8` [depth 16], `N_page_cell = 8` [depth `B + 3`], `+1,680` `vd` rows, `+700` 39-field digest rows)**:
  $$\text{rows}_{\text{full}}(B) = 31{,}291 + 70 \cdot (2 + N_{\text{page\_cell}}) \cdot (B - 17) = 31{,}291 + 700\,(B - 17)$$

| Segment Bits `B` | Cell Depth `B+3` | Max Leaves / Max Cells | E8 Spike Rows (`k`) (`ESTIMATE`) | Full Leaf Rows, `N_page_cell=8` (`k`) (`ESTIMATE`) | Node Rows (`k=20`) (`ESTIMATE` via E2 `+4/Δk`) | Decider Rows (`k=20`) (`ESTIMATE`) |
|---:|---:|---|---:|---:|---:|---:|
| **17 (D3)** | 20 | `2^17 - 1` / `2^20` (1.05M) | `6,511` (`k = 13`) | `31,291` (`k = 15`) | `823,466` (`k=13`) / `823,474` (`k=15`) | `929,166` |
| **24** | 27 | `2^24 - 1` / `2^27` (134M) | `7,491` (`k = 13`) | `36,191` (`k = 16`) | `823,466` (`k=13`) / `823,478` (`k=16`) | `929,166` |
| **28 (max `k=13` spike)** | 31 | `2^28 - 1` / `2^31` (2.15B) | `8,051` (`k = 13`) | `38,991` (`k = 16`) | `823,466` (`k=13`) / `823,478` (`k=16`) | `929,166` |
| **29 (`k=14` threshold)** | 32 | `2^29 - 1` / `2^32` (4.29B) | `8,191` (`k = 14`) | `39,691` (`k = 16`) | `823,470` (`k=14`) / `823,478` (`k=16`) | `929,166` |
| **32** | 35 | `2^32 - 1` / `2^35` (34.4B) | `8,611` (`k = 14`) | `41,791` (`k = 16`) | `823,470` (`k=14`) / `823,478` (`k=16`) | `929,166` |
| **48** | 51 | `2^48 - 1` / `2^51` ($2.25 \times 10^{15}$) | `10,851` (`k = 14`) | `52,991` (`k = 16`) | `823,470` (`k=14`) / `823,478` (`k=16`) | `929,166` |
| **61 (`2^64` cells)** | 64 | `2^61 - 1` / `2^64` ($1.84 \times 10^{19}$) | `12,671` (`k = 14`) | `62,091` (`k = 16`) | `823,470` (`k=14`) / `823,478` (`k=16`) | `929,166` |
| **64 (`2^64` S=1 leaves)** | 67 | `2^64 - 1` / `2^67` ($1.48 \times 10^{20}$) | `13,091` (`k = 14`) | `64,191` (`k = 16`; `82,951` `k=17` at `N_page_cell=12`) | `823,470` (`k=14`) / `823,482` (`k=17`) | `929,166` |

#### 2.4.3 Verdict on Option (b) Alone — Necessary (`B = 32..48`), Not Sufficient
- **Crucial finding**: Widening $B$ from `17` to `32` (or `48`) costs **almost nothing in the node (`+4` to `+12` rows out of `1,048,576`) and `+0` rows in the decider**, while raising cell capacity to $3.44 \times 10^{10}$ (or $2.25 \times 10^{15}$) cells! By contrast, pushing $B = 64$ doubles `PAGE` Merkle work vs $B = 32$ (`+22,400` to `+40,320` rows/leaf at `N_page_cell = 8..16`), pushing the full leaf into `k = 17` for a capacity ($2^{67}$ cells) that can never physically exist.
- **Why (b) alone still fails**: Even at $B = 64$, keeping D3's unconditional `post.len = base + 8` **still leaves `SI-12` and `SI-23` broken** (padding holes on leaf close), **still makes `pre_d`/`post_d` partition-dependent**, and **still hashes `2B` dummy segment insertions on pure-compute leaves**.

---

### 2.5 Option (c): Non-Fixed-Per-Leaf Memory Designs

| Design | Leaf / Node Row Cost (`ESTIMATE`) | Soundness Risk | `SI-12..14` (Slices) | `SRC-023` (Partition Indep.) | `GP-2..4` Fit |
|---|---|---|---|---|---|
| **c.1 Zero-append skip (`used > 0` gate on D3)** | Same rows as Q2b (`8,611` spike / `41,791` full at `B=32`); `+0` cols, `+0` node rows | Low (`used * inv = has_alloc` boolean check) | **Broken** (leaves with `1..7` appends still pad `8 - used` zero cells!) | **Internal `sd` still partition-dependent** | Needs canonical relocation for continuations |
| **c.2 Unaligned per-append (`M1`) or 2-segment frontier (`M2`)** (`experiments-2.md` §3.4–3.5) | `M1` (`B=32`): `+39,200` write rows (`8.75x` worse). `M2`: 2 dual walks `4B = 8,960` rows + complex prefix muxing (`+2–3` fixed cols $\implies +13k..19k$ node rows) | Medium (`M2` straddle mux off-by-one risk) | **Fixed** (dense `len += used`, zero leaf-close holes; see §2.6) | **Strong** (`sd(h)` partition-independent!) | Composes in any `GP-4` DAG |
| **c.3 Paged Sparse Merkle (`RISC Zero` / `OpenVM` dirty pages)** | $P=32$, $K_{\text{page}}=4$ pages/leaf at $D=27$: `4 * 149 * 70 = 41,720` page-hash rows alone (`k = 16..17`) | Medium (page deduplication & dirty-writeback ordering) | **Fixed** if `len += used` is dense across page boundaries | **Strong** (`mem_root` depends only on dense cells `[0..len)`) | Composes cleanly (`pre_root -> post_root`) |
| **c.4 Dynamic-depth Merkle / MMR peaks** (`architecture.md:502-509`) | Static PLONK still lays out `D_max` (`64`) walk rows (`~64k–100k` full leaf rows) + level/peak muxing (`+1–2` cols $\implies +6.5k..13k$ node rows) | **High**: dynamic height boundaries caused **OpenVM `CVE-2026-62333`** (`https://osv.dev/vulnerability/CVE-2026-62333`, below-leaf subtree forgery at `parent_height = 0`) | Needs dense `len += used` separately | `sd(h)` canonical only if deterministic growth rule (`seg == 2^d` or MMR binary peaks) | Streamed multi-leaf path openings break 1-leaf step atomicity |
| **c.5a Height-keyed `(h, sub_op)` tree** | Forces `D >= 64` (`h < 2^64`) $\implies$ `64k–100k` full leaf rows (`k = 16..17`) | Low (unique key per step) | **Broken** (1-step `Cons` uses `(h,0),(h,1)` vs multi-step `Build` `(h,0),(h+1,0)` — non-uniform stride!) | Internal addresses depend on `h`, not partition, **but** sparse keys kill locality | Poor locality |
| **c.5b Offline Challenge (`LogUp` / `Twist & Shout` / `Nebula`)** (`https://eprint.iacr.org/2025/105`, `2024/1605`) | **Blocked on pinned stack**: `MC:verifier/verifier_gadget.rs:16-17,308-310` rejects multi-phase challenge leaves (`study.md:94-97`) | High if retrofitted without upstream `midnight-proofs` support | Supports dense RAM | Requires boundary commitments or global finalization | Needs custom commitment-carrying IVC |
| **c.5c Challenge-free Jubjub Curve Multiset (`MSet-Add`)** (`https://arxiv.org/abs/1601.06502`, `architecture.md:259`) | Fast per-access (`~15–30` rows/point add), **but** requires `M / C` finalization leaves at run end to sweep all $M$ live cells to $\mathcal{O}$! | Medium (hash-to-curve + timestamp linear-use discipline) | Supports dense `len` | Telescopes across shards (`SP1`/`Ziren` style) | **Bad fit for KZG IVC**: finalization leaves double `65s` `k=20` node folds! |

---

### 2.6 Option (d): Recommended Synthesis Design — Hash-Stack Frames/Tasks + Dense Carry-Buffered Merkle Arena (`B = 32`, `C = 8`) + GP-2 Continuations

We obtain **all** advantages of `M2` dense addressing, `M3` single-subtree aligned hashing, and `RISC Zero` L1 page caching — with **half the walk rows of `M2` and zero dynamic-depth `CVE-2026-62333` surface** — by combining three orthogonal mechanisms:

#### 1. Move `FRAMES` and `TASKS` Out of Arena Memory Into Cached-Top Hash Chains
- Both `frames` (13 lanes) and `tasks` (7 lanes) in `mori-core` (`owner_runtime.rs:3566-3585`, `architecture.md:245,251,443-444`) are pure LIFO stacks (`PUSH`, `POP`, `SETTOP` top-rewrite — **never** random-indexed).
- Keep `top_frame`, `frame_h`, `top_task`, `task_h` in registers (per D7, `d7-frames.md`):
  - **`SETTOP` (top rewrite)**: Register-only (`0` hashes, `0` arena cells).
  - **`PUSH`**: Computes `frame_h' = H(TAG_FRAME, old_top_fields..., frame_h)` on the `HASH` chip (`~4` Poseidon perms $= 280$ rows, **0 Merkle paths, 0 arena cells**).
  - **`POP`**: Witnesses `parent_fields, parent_h`, checks `H(TAG_FRAME, parent_fields..., parent_h) == frame_h` (`~4` perms $= 280$ rows vs `40` perms $= 2{,}800$ rows for a depth-35 `PAGE` opening!), sets `top_frame := parent_fields, frame_h := parent_h`.
- **Consequence**: Arbitrarily deep recursion and helper-task loops consume **0 arena cells** and remove stack traffic from `PAGE` and `MEM`.

#### 2. Dense Carry-Buffered Append Arena ("Flush-on-Full Carry Segment", `C = 8`, `B = 32` $\implies 2^{35}$ Cells)
- **Dense allocation pointer**: `len` starts at `0` (or after initial input cells) and advances strictly by actual appends: `post.len = pre.len + used` (`0 <= used <= 8`). **Leaf close NEVER pads `len` to a multiple of 8.**
- **State representation in `pre_d` / `post_d`**: Let `seg_0 = floor(len / 8)` and `off_0 = len mod 8` (`0 <= off_0 <= 7`).
  - Complete segments `0 .. seg_0 - 1` sit in `tree_root` (a fixed-depth `B = 32` Sparse Merkle tree with precomputed `ZERO_HASH[0..32]`, giving **`2^32` segments = `2^35` = `34,359,738,368` cells**).
  - The **single open partial segment** `seg_0` is carried across leaf boundaries as its 8 cell-record hashes `open_r[0..7]` inside `pre_d` and `post_d` (with `open_r[j] = 0` for `j >= off_0`; adding `8` fields to `pre_d`/`post_d` costs exact **`4` perms $\times 2 = 8$ perms $= 560$ rows**!).
- **Single-flush invariant (`0` or `1` top-tree walk per leaf)**:
  Because `0 <= off_0 <= 7` and `0 <= used <= 8`, `off_0 + used` lies in `0 .. 15`, so **at most ONE segment (`seg_0`) can complete in any leaf**! Define boolean `flushed = ((off_0 + used) >= 8)`:
  - **If `flushed == 0` (including `used == 0` pure compute!)**: `post.tree_root = pre.tree_root`; the `used` new record hashes fill `post.open_r[off_0 .. off_0 + used - 1]`. **Zero segments in `tree_root` are consumed!** (The static walk rows verify a dummy `0 -> 0` path gated off by `1 - flushed`.)
  - **If `flushed == 1`**: The first `8 - off_0` new record hashes combine with `pre.open_r[0 .. off_0 - 1]` to form `r_full[0..7]`, hash via `7` compressions to `seg_root`, and execute the exact M3 dual walk at `idx = seg_0 < 2^{32}` (`0 -> pre.tree_root` and `seg_root -> post.tree_root`, `64` perms $= 4{,}480$ rows). The remaining `(off_0 + used - 8)` new record hashes become `post.open_r[0 .. off_0 + used - 9]`, with trailing zeros!
- **L1 Write-Combining Read Cache + Strict `D2` Provider Disjointness**:
  Witness the `<= 8` tuples of `pre`'s open segment (authenticated against `pre.open_r[0..7]` in `16` perms) alongside the `<= 8` new tuples in a `16`-row `MEM` provider:
  - Reads with `seg_0 * 8 <= addr < cur_len` are answered directly in-leaf by `MEM` (`0` Merkle paths — automatic 8–15 cell temporal locality cache!).
  - Reads with `addr < seg_0 * 8` are answered by `PAGE` against `pre.tree_root` (`floor(addr / 8) < seg_0`), preserving Quint invariant `D2` (`quint-check.md:332-338`) with zero overlap!

#### 3. Exact Verification of `SI-12..SI-14` and Strong Internal `SRC-023`
- **`SI-12`, `SI-13`, `SI-14`, `SI-23` hold natively**:
  - Each arena record stores `(kind_tag, vd, a, b, c)` (`kind_tag in {DATA, FIELD, ENV, SLOT}`).
  - `SI-13` (`child_addr < owner_addr`) and `SI-14` (`prev_addr < env_addr`) hold for every monotonically increasing `len`. Logical resource caps (`CapKind::{DataNodes, DataFields, EnvironmentCells, HelperTasks}`, `owner_runtime.rs:3129-3162`) remain separate registers per `SVM-010` (`specs/sherlock-vm/spec.md:83`).
  - For `SI-12` (`DATA_FIELDS`) and `SI-23` (`VALUE_SLOTS`): exhaustive inspection of `crates/mori-core/src/sherlock/` shows that runtime multi-element slices are **only** built by `Cons`/`Leaf`/`Branch` (atomic 1-step append of 1–2 fields; `owner_runtime.rs:3488-3543`), `Build` (`BuildCell` loop appending 1 field/step with **0** other allocations before `BuildField`; `owner_adt_runtime.rs:647-725`), and `Append` (`AppendCell`/`AppendTail` loop appending 1 slot/step with **0** other allocations; `environment.rs:212-265`). With `FRAMES`/`TASKS` in hash-stacks and **zero leaf-close padding holes**, every slice element sits at exact contiguous physical address **`start + k`** regardless of where leaves close! *(Defense-in-depth note: if a future machine profile ever interleaved allocations mid-slice, either Carbon's cons-linked slice cells [`dissent.md` D-1] or a 2-root split `NODE_TREE` + `SLICE_TREE` [`+1` walk $= +4{,}480$ rows] isolates slices unconditionally.)*
- **Strong Internal Partition Independence (`SRC-023`)**:
  Because `len` advances solely on semantic machine allocations, at any step height $h$, `len(h)`, every pointer inside every cell, `tree_root(h)` (segments `0 .. floor(len(h)/8) - 1`), and `open_r(h)` (cells `floor(len(h)/8)*8 .. len(h) - 1`) are **bit-for-bit identical across all valid leaf partitions** (`S = 1`, `S = 16`, or arbitrary early closes)! Thus **`pre_d(h)` and `post_d(h)` themselves are partition-independent functions of `(prog_d, input_d, h)`**.

#### 4. Exact Cost Model of Q2(d) (`ESTIMATE`)
- **E8-equivalent leaf (`S = 16, C = 8, B = 32, N_page = 0`)**:
  - Core (`221`) + ALU/RANGE (`192`) + 16-row `MEM` (`16`) = `429` rows.
  - Poseidon: `16` (`pre_open` records) + `16` (`new` records) + `7` (8-cell subtree) + `64` (dual `B=32` walk) + `40` (`pre_d`/`post_d` over 40 words) = `143` perms $\times 70 =$ `10,010` rows.
  - **Total E8-equivalent rows**: **`10,439` rows (`ESTIMATE`) $\implies$ `k = 14` (`16,384`, `5,939` rows headroom)**.
- **Complete universal leaf (`B = 32`, `N_page_prog = 8` [depth 16], `N_page_cell = 6` [depth 35], `6` stack hashes, `SVM-012` `vd` creation)**:
  - `10,439 + 10,080 (PAGE_prog) + 16,800 (PAGE_cell) + 1,680 (stacks) + 1,680 (vd) + 1,500 (core/ALU) =` **`~42,179` rows (`ESTIMATE`) $\implies$ `k = 16` (`65,536`)** (`~10,800` rows *smaller* than naive widened D3 at `N_page_cell = 12`!).
- **Node & Decider (`ESTIMATE`)**: `+0` advice columns, `+0` lookups, `+1..2` selector/fixed columns (`flushed` mux + `pre_open` selector at `~6,460` node rows/col) + `4 * (leaf_k - 13)` $\implies$ **Node `~829,930–836,400` rows (`k = 20`, `> 113,600` rows below `950,000` gate)**; **Decider `929,166` rows (`+0` rows, `k = 20`)**.
- **Combined with GP-2 Continuations (Q3)**: Pure-compute runs never touch `2^35` cells; allocating runs cannot physically fill `2^35` cells (`> 1,350` prover-years at `~4 s/step`); and multi-session/checkpointed runs compose seamlessly via Q3 continuations.

---

## Q3. Continuations Across Runs: Canonical State Digest, `input_d`, GP-2 Aggregation, and C6 Interaction

### 3.1 Why `Halted(v)` Alone Cannot Express General Continuations — and Why Raw `post_d` Cannot Be Public
1. Current `SRC-006` (`specs/sherlock-recursion/spec.md:58`) refuses `Running` states at the decider, and `GP-002` (`specs/sherlock-general-purpose/spec.md:19`) permits consuming only terminal `SHK_RUN` claims (`Halted(v)`, `Rejected(code)`, `Exhausted(kind)`). `Halted(v)` carries only a return value digest `vd(v)` — it drops active `frames`, `tasks`, `env`, `helper`, `result`, `value_slots`, `remaining_work` (fuel), `helper_work`, and `alloc.*` counters.
2. Conversely, publishing raw `post_d` as a claim violates `SRC-006` ("SHALL NOT expose `fin_d`, `init_d`, `pre_d`, `post_d`, `mem_root`... or any value that depends on a memory address"), `GP-002` ("a segment statement or a state digest SHALL NOT be accepted as a claim of another run"), and (under D3) `SRC-023` (partition independence).

### 3.2 Canonical, Partition-Independent Machine Checkpoint Digest `ckpt_d` (`sherlock-ckpt/1`)
To chain `Run_0 -> Run_1 -> ... -> Run_{K-1}` across arbitrary suspension heights (and allow optional live-set GC compaction between runs!), define an address-erased **canonical checkpoint digest** computed in **$O(1)$ leaf rows** by maintaining incremental canonical digests at creation (extending `SVM-012`):
- **Values (`vd`, existing `SRC-022` / `SVM-012`)**: Every `DATA` record already binds its address-free `sherlock-value/1` digest `vd` at creation (`specs/sherlock-vm/spec.md:103-109`).
- **Environments (`env_cd`)**: When an `ENV` cell `(binder_id, val_ref, prev_env)` is allocated, bind `env_cd = H("SHK_ENV", 1, binder_id, vd(val_ref), prev_env_cd)` (`0` for `None`) in `2` Poseidon perms.
- **Slices (`slice_cd`)**: Any live `ValSlice` (`len <= max_arity`) has `slice_cd = H("SHK_VSLICE", 1, len, vd(s_0), ..., vd(s_{len-1}))`, independent of `s.start`.
- **Stacks (`frames_cd`, `tasks_cd`)**: Under Q2d's hash-stacks, maintain `frames_cd = H("SHK_FRM", 1, site, kind, env_cd, slice_cd, parent_frames_cd)` alongside physical `frame_h` on each `PUSH`/`POP` (`+3–4` perms).
- **Checkpoint Digest**:
  $$\text{ckpt\_d} = H(\text{"SHK\_CKPT"}, 1, \text{prog\_d}, \text{control\_cd}, \text{fuel\_lo}, \text{fuel\_hi}, \text{helper\_work}, \text{stack\_limit}, \text{alloc\_d}, \text{frames\_cd}, \text{tasks\_cd}, \text{helper\_cd}, \text{result\_cd}, \text{deps\_d})$$
  - **100% Partition-Independent (`SRC-023`)**: Zero physical addresses, `len`, `mem_root`, or dead unreachable cells appear in `ckpt_d`. Carrying `alloc_d = (alloc.data_nodes, alloc.data_fields, alloc.environment_cells)` preserves exact `CapKind` exhaustion semantics (`owner_runtime.rs:3129-3162`, `SVM-010`) even if `Run_{k+1}` starts in a fresh compacted arena!
  - **Lazy $O(1)$-Step `ResumeInit` (`SRC-021` kill criterion respected)**: Just as `SRC-021` (`specs/sherlock-recursion/spec.md:235`) forbids scanning `val_d` in `Init`, `Run_{k+1}`'s `h_pre = 0` check (`ResumeInit(prog_d, ckpt_d)`) binds `ckpt_d` into registers in 1 step and authenticates any pre-populated live `DATA`/`ENV`/stack cell lazily on first read by verifying its `vd` / `env_cd` / `frames_cd` opening!

### 3.3 Exact `input_d` Threading, GP-2 Single-Proof Aggregation, and C6 Privacy

1. **What `input_d` is across the chain**:
   - `Run_0` starts via `Init(prog_d, input_d_0)` where `input_d_0 = H("SHK_IN", val_d, fuel_lo, fuel_hi, stack_limit)` (`SRC-021`), executes $n_0 \ge 1$ steps without terminating, and seals a **domain-separated continuation claim**:
     $$\text{cont\_claim}_0 = H(\text{"SHK\_CONT"}, 1, \text{prog\_d}, \text{input\_d}_0, \text{ckpt\_d}_1, n_0)$$
   - `Run_k` ($1 \le k \le K - 2$) starts via `ResumeInit(prog_d, ckpt_d_k)` (setting fuel/caps strictly from `ckpt_d_k`, **preventing fuel-reset attacks**!), consumes `cont_claim_{k-1}` via `deps_d`, checks `prog_d` and `ckpt_d_k` equality, checks $n_{\text{cum},k} = n_{\text{cum},k-1} + n_k < 2^{64}$ (`SRC-004`), and emits `cont_claim_k = H("SHK_CONT", 1, prog_d, input_d_0, ckpt_d_{k+1}, n_{\text{cum},k})`, **threading the original semantic `input_d_0` unchanged**.
   - Final `Run_{K-1}` starts via `ResumeInit(prog_d, ckpt_d_{K-1})`, consumes `cont_claim_{K-2}`, reaches a **terminal** state (`Halted(v)`, `Rejected(code)`, `Exhausted(kind)`), and seals the standard **terminal `SHK_RUN` claim**:
     $$\text{claim\_d} = H(\text{"SHK\_RUN"}, 1, \text{prog\_d}, \text{input\_d}_0, \text{outcome\_d}), \qquad n_{\text{steps}} = \sum_{j=0}^{K-1} n_j < 2^{64}$$
2. **Proving the chain once on-chain & C6 outcome-only interaction**:
   - In the GP-2 / GP-4 node `N` (`architecture.md:84,132-135`), `SEG + SET -> SET` verifies that `Run_k`'s `deps_d` equals the verified `SET` containing `cont_claim_{k-1}`, **privately discharging** `cont_claim_{k-1}` inside the recursion without exporting it to the root `SET`!
   - Domain separation (`"SHK_CONT"` vs `"SHK_RUN"`) ensures `acceptRun` (`SRC-006`) and `openClaim` (`SherlockClaims`) **refuse any undischarged `SHK_CONT` claim** (no unfinished `Running` computation can ever settle on-chain or be consumed as a program output under `GP-002`).
   - The terminal `RunDecider` proof exposes **exact `[prog_d, input_d_0, outcome_d, n_steps]`** (`6,432 B` proof). Neither `ckpt_d_1 .. ckpt_d_{K-1}`, nor chunk count $K$, nor any memory root is public — preserving C6 outcome-only publication (`SRC-006`, `SRC-007`) and making a $K$-chunk continuation proof **indistinguishable on-chain** (identical `claim_d` and `n_steps`) from a 1-run proof!

---

## Q4. Prior Art: Run-Length Bounds, Memory Arguments, Continuations, and On-Chain Statements

### 4.1 Comparative Matrix Across 10 zkVMs

| System | Segment / Shard Limit | Total Run & Memory Ceiling | Memory Argument (Intra-Shard $\to$ Cross-Shard) | Partition-Indep. On-Chain? | Published On-Chain Fields (Length / Memory Exposed?) |
|---|---|---|---|---|---|
| **RISC Zero v3.x** | `po2` $\in [13, 24]$ (`8K..16.7M` cycles; default $2^{20}$) | Unbounded segments (`CycleLimit::None`); 4 GiB (`2^32` B) in 1 KiB pages | Intra-segment BabyBear timestamped trace $\to$ **depth-22 Poseidon2 Binary Merkle `PageTable`** in `SystemState` | **Yes** (`Halted` zeroes `postStateDigest`; `join` folds splits) | `verify(seal, imageId, journalDigest)` $\to$ `ReceiptClaim` digest (`pre=imageId, post=0, Halted(0)`). **No** length or RAM root (unless `Paused`/PoVW). |
| **SP1 (v4 Turbo / v5–v6 Hypercube)** | `~2^22` insns / max chip height (Jagged PCS) | **48-bit global clock** (`24b+24b` $\implies 2^{48}$ events); **48-bit addr** (`3x u16` words in RV64 Hypercube) | Intra-shard `MemoryLocalChip` (LogUp-GKR) $\to$ **Septic-Curve Global Multiset Bus** (`GlobalChip` + `MemoryGlobalInit/Finalize`, **no Merkle tree**) | **Yes** (curve sum telescopes to $\mathcal{O}$ in recursion) | `verifyProof(programVKey, publicValues, proofBytes)` $\to$ 2 BN254 Fr inputs: `[programVKey, sha256(publicValues) & ((1<<253)-1)]`. **Zero** cycle/shard/RAM leak. |
| **Jolt v0.2+ (RV64)** | Monolithic streaming sum-check ($T$ padded to $2^k$) | Configured `max_trace_length`; RAM $K$ doublewords (`0x80000000+`) | **Twist & Shout** $d$-way one-hot $\widetilde{\textsf{ra}}(k,j)$ + write increments $\widetilde{\textsf{Inc}}(j)$ via multilinear sum-check + `BlindFold` ZK | **No** (no recursion wrapper yet; verifier runs $\log_2 T, \log_2 K$ rounds) | Fiat-Shamir transcript absorbs **`trace_length` ($T$), `ram_size` ($K$), `memory_layout`, `panic`, `preprocessing_digest`, I/O**. |
| **Nexus (1.0/2.0 $\to$ 3.0)** | 1.0/2.0: $N$ insns/IVC step; **3.0**: Stwo M31 domain ($\le 2^{30}$) | 1.0/2.0: unbounded Nova/HyperNova IVC; **3.0**: single Stwo domain | 1.0/2.0: SMT $\to$ offline folding; **3.0**: Harvard RO/Reg/RW **BEG+94 / LogUp** (`TimestampChip`, `ram_init_final`) | **No** in raw 3.0 (`log_size` mixed into `Blake2sChannel`) | 3.0 `Machine::verify` checks `Proof { log_size: Vec<u32>, claimed_sum }`, `program_info`, `ad`, `init_memory`, `exit_code`, `output_memory`. |
| **Valida (Lita)** | Default $2^{20}$ rows/chip/segment | BabyBear clock; **960 MiB** RAM (`[0, (p-1)/2) = 0x3c000000`) | Stack/frame-only (`fp`, no GPRs); ephemeral LogUp $\to$ **Persistent Memory Bus** (`PersistentSend/Receive`) | Raw: **No**; Wrapped in Valida `recursive-verifier`: **Yes** | Recursive proof binds `(vk/elf, output, pc_init, fp_init=0x3bfffff8, did_stop=true)` (hides segment count). |
| **Cairo / Stwo (SHARP)** | Stwo M31 circle domain ($< 2^{31}$ rows/proof) | Unbounded via recursive **Cairo Bootloader** tree; `< 2^31` M31 addrs/proof | Continuous write-once relocated `segment:offset -> [1..M]` (`addr_to_id`, `id_to_big` LogUp) + L1 **Public Memory Pages** | **Yes** at `FactRegistry` layer (`keccak256(progHash, outHash)`) | Root STARK checks `MemoryPageFactRegistry` page hashes/products; downstream L1 contracts consume **`factHash = keccak256(programHash, outputHash)`** (no step count). |
| **zkWasm (Delphinus)** | Halo2 slice $2^k$ ($k \in [18,22]$, `RESERVE=32,896`) | Unbounded slices via `continuation-batcher`; ~31.2 MiB direct heap ($k=22$) + 4 GiB host SMT | Sorted `(ltype, offset, eid)` `mtable` $\to$ **`img_col <-> post_img_col` KZG commitment equality** + **depth-32 Poseidon SMT** host circuits | **Yes** for slice/step count (`k` fixed in VK) | Solidity batch verifier checks initial `img_col` KZG commitment (3 limbs), stateful post root/commitment (if rollup), and `--public` I/O; slice count hidden. |
| **Miden VM** | Single STARK: **`MAX_CYCLES = 2^29`** (`MIN_TRACE_LEN` padded to $2^k$) | $2^{29}$ cycles/VM proof; unbounded across recursive Tx $\to$ Batch $\to$ Block kernels; `[0, 2^32)` `MemoryChiplet` + depth-64 `MerkleStore` SMT | `MemoryChiplet` `(ctx, word_addr, clk)` (`2x 16b` delta checks) + on-demand Hash Chiplet Merkle paths (`mtree_get/set`) | **Yes** inside recursive kernels | Canonical 40-felt `ExecutionClaim` (`Eidos(P \|\| K \|\| I \|\| O)`) + `DeferredRoot`. **Zero** cycle count or RAM fields in `ExecutionClaim`. |
| **OpenVM v2.1** | Metered cut at `max_trace_height_bits` ($\approx 2^{22}$), 15 GiB RAM, or $2^{29}$ ticks | Unbounded segments; **512 MiB** guest RAM (`POINTER_MAX_BITS = 29`, 8 addr spaces) | Intra-segment offline `MemoryBridge` (`ts <= 29b`) $\to$ **depth-29 Poseidon2 Binary `MemoryMerkleAir`** (`CHUNK = 8`) | **Yes** (static Halo2 KZG wrapper hides segments & roots) | `IOpenVmHalo2Verifier.verify(publicValues, proofData, appExeCommit, appVmCommit)`. **Zero** segment/cycle/RAM state on-chain. |
| **Ziren v2.0 (zkMIPS)** | Cut at `shard_size`, `clk < 2^26`, or chip height $< 2^{22}$ | 32-bit `shard` $\times 2^{26}$ `clk`; 32-bit MIPS RAM + regs mapped to `0..=35` | CPU-less frames + KoalaBear LogUp + Jagged WHIR $\to$ **Septic-Curve Global Bus** (`global_cumulative_sum`) | **Yes** (`compress -> shrink -> wrap_bn254` Groth16) | 3 BN254 Fr inputs: `[zkm_vkey_hash, committed_values_digest, VK_ROOT_BYTES]`. **Zero** shard/cycle/RAM leak. |

### 4.2 Detailed Citations and 2025–2026 Research Notes

1. **RISC Zero**: `MIN_CYCLES_PO2 = 13`, `MAX_CYCLES_PO2 = 24`, `DEFAULT_SEGMENT_LIMIT_PO2 = 20` (`https://raw.githubusercontent.com/risc0/risc0/main/risc0/zkp/src/lib.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/zkvm/src/host/client/env.rs`); `PAGE_BYTES = 1024`, `MERKLE_TREE_DEPTH = 22` (`https://raw.githubusercontent.com/risc0/risc0/main/risc0/binfmt/src/image.rs`); `SystemState`, `ReceiptClaim`, `join`/`resolve` (`https://dev.risczero.com/proof-system/`, `https://dev.risczero.com/api/zkvm/receipts`, `https://dev.risczero.com/api/zkvm/composition`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/binfmt/src/lib.rs`); Solidity `IRiscZeroVerifier` (`https://raw.githubusercontent.com/risc0/risc0-ethereum/main/contracts/src/IRiscZeroVerifier.sol`).
2. **SP1 (Turbo & Hypercube)**: RV64IM over KoalaBear, Jagged PCS (`https://eprint.iacr.org/2025/917`, `https://docs.succinct.xyz/docs/sp1/hypercube/`, `https://docs.succinct.xyz/docs/sp1/hypercube/shards`); 48-bit `addr: [T; 3]` & 48-bit `(clk_high, clk_low)` septic-curve global memory bus (`https://docs.succinct.xyz/assets/files/SP1_Turbo_Memory_Argument-b042ba18b58c4add20a8370f4802f077.pdf`, `https://raw.githubusercontent.com/succinctlabs/sp1/dev/crates/core/machine/src/memory/global.rs`, `https://raw.githubusercontent.com/succinctlabs/sp1/dev/crates/core/machine/src/memory/local.rs`); 2-input Solidity verifier (`https://docs.succinct.xyz/docs/sp1/verification/solidity-sdk`, `https://raw.githubusercontent.com/succinctlabs/sp1-contracts/main/contracts/src/ISP1Verifier.sol`, `https://raw.githubusercontent.com/succinctlabs/sp1-contracts/main/contracts/src/v5.0.0/SP1VerifierPlonk.sol`).
3. **Jolt**: Twist & Shout one-hot memory (`https://eprint.iacr.org/2025/105`), BlindFold ZK sum-check folding (`https://eprint.iacr.org/2025/2094`), RAM doubleword remapping, streaming roadmap, and Fiat-Shamir public statement absorbing `trace_length` and `ram_size` (`https://jolt.a16zcrypto.com/`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/how/architecture/ram.md`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/how/fiat-shamir.md`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/how/blindfold.md`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/roadmap/streaming.md`).
4. **Nexus**: 1.0/2.0 IVC whitepaper (`https://nexus-xyz.github.io/assets/nexus_whitepaper.pdf`) and 3.0 Stwo M31 architecture + `Machine::verify` mixing `log_size` (`https://github.com/nexus-xyz/nexus-zkvm`, `https://github.com/nexus-xyz/nexus-zkvm/blob/main/specification/zkvm-spec-3.0.pdf`, `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/prover/src/lib.rs`, `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/prover/src/machine.rs`).
5. **Valida**: Stack/frame ISA spec (`https://arxiv.org/html/2505.08114v1`), `[0, (p-1)/2)` RAM bound (`https://raw.githubusercontent.com/lita-xyz/valida-vm/main/memory/src/lib.rs`), persistent memory bus (`https://raw.githubusercontent.com/lita-xyz/valida-vm/main/machine/src/persistence.rs`), multi-segment vs recursive proof (`https://raw.githubusercontent.com/lita-xyz/valida-vm/main/machine/src/proof.rs`, `https://raw.githubusercontent.com/lita-xyz/valida-vm/main/recursive-verifier-api/src/lib.rs`).
6. **Cairo / Stwo**: Write-once relocated memory (`https://eprint.iacr.org/2021/1063`, `https://zksecurity.github.io/stark-book/cairo/memory.html`), Circle STARKs & `stwo-cairo` LogUp relations (`https://eprint.iacr.org/2024/278`, `https://raw.githubusercontent.com/starkware-libs/stwo-cairo/main/crates/cairo-air/src/air.rs`), and L1 `MemoryPageFactRegistry.sol` (`https://raw.githubusercontent.com/starkware-libs/starkex-contracts/master/evm-verifier/solidity/contracts/cpu/MemoryPageFactRegistry.sol`).
7. **zkWasm**: `OngoingCircuit`/`LastSliceCircuit` `img_col <-> post_img_col` continuation & page capacity (`https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/crates/zkwasm/src/circuits/post_image_table/continuation.rs`, `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/crates/zkwasm/src/circuits/utils/image_table.rs`, `https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/crates/zkwasm/src/circuits/zkwasm_circuit/mod.rs`), `continuation-batcher` (`https://raw.githubusercontent.com/DelphinusLab/continuation-batcher/main/README.md`), and depth-32 Poseidon SMT host circuits (`https://raw.githubusercontent.com/DelphinusLab/zkWasm-host-circuits/main/README.md`).
8. **Miden VM**: `MAX_CYCLES = 1 << 29` (`https://raw.githubusercontent.com/0xMiden/miden-vm/next/processor/src/execution_options.rs`), 32-bit `MemoryChiplet` + `MerkleStore` (`https://docs.miden.xyz/reference/miden-vm/design/chiplets/`, `https://raw.githubusercontent.com/0xMiden/miden-vm/next/docs/src/design/chiplets/memory.md`), and 40-felt `ExecutionClaim` (`https://raw.githubusercontent.com/0xMiden/miden-vm/next/core/src/program/claim.rs`).
9. **OpenVM**: 29-bit pointers, `CHUNK = 8`, depth-29 `MemoryMerkleAir` (`https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/system/memory/mod.rs`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/arch/config.rs`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/arch/execution_mode/metered/segment_ctx.rs`), static Halo2 Solidity verifier (`https://raw.githubusercontent.com/openvm-org/openvm/main/crates/sdk/contracts/src/IOpenVmHalo2Verifier.sol`), and dynamic-expansion soundness advisory **`CVE-2026-62333`** (`https://osv.dev/vulnerability/CVE-2026-62333`).
10. **Ziren (zkMIPS v2.0)**: Paper (`https://eprint.iacr.org/2026/2330`), register-in-RAM `0..=35` septic-curve continuations & shard cuts (`https://docs.zkm.io/design/continuation.html`, `https://docs.zkm.io/design/prover-architecture/prover-architecture.html`), and 3-input Groth16 verifier (`https://raw.githubusercontent.com/ProjectZKM/Ziren/main/crates/verifier/src/groth16/mod.rs`).
11. **2025–2026 IVC Offline Memory (`Nebula`, IEEE S&P 2026)**: Commitment-carrying IVC folding read/write transcript commitments per step and discharging grand-product memory once in the terminal decider (`https://eprint.iacr.org/2024/1605`) — shows why offline memory in IVC requires foldable vector commitments not present in `midnight-proofs 0.8.3`, whereas boundary Merkle (`OpenVM` `CHUNK = 8`, closely matching our Q2d `C = 8` carry-segment!) works directly on stock KZG verifiers.
12. **Comparison to Sherlock C6**: Sherlock's C6 statement `[prog_d, input_d, outcome_d, n_steps]` (`SRC-006`) aligns with RISC Zero, SP1, OpenVM, Miden, and Ziren in exposing **zero memory roots or physical segment counts**, while intentionally publishing semantic `n_steps` (`tools/sherlock-sdk/sherlock-run.compact:24`, omitted from `claim_d` replay key per `SRC-007`), which remains partition-independent (`SRC-023`) because `n_steps` counts logical owner steps rather than padded trace cycles.

---

## Q5. Practical Run Length on This Hardware and Recommended Tested Targets

### 5.1 Host Hardware and Measured Prover Envelope

1. **Host specs (`research/sherlock-speed-replan-20261108/prover.md:15-17`, `gpu.md:14-19`)**:
   - **CPU**: 20-core `aarch64` (`10x Cortex-X925 + 10x Cortex-A725`).
   - **Memory**: `120.3 GiB` (`126.1 GB`) unified LPDDR5X (`244 GB/s` measured R+W). Integrated `NVIDIA RTX Spark N1X` (`sm_121`, 6,144 CUDA cores) shares the same physical pool (`cudaMalloc` counts directly in `VmRSS`); GPU lever `L4` is currently `PARKED` (`qa/sherlock/decisions/speed-levers.json:112-125`).
   - **Enforced cgroup limits (`SRC-016`, `models/doctorwatson/factory/heavy.py:26-34`, `preflight.py:21-22`)**: **`48 GiB` per process** (`MemorySwapMax=0`), **`100 GiB` nominal aggregate** (`SRC-011`/`SRC-016`), and **`60 GiB` operational concurrent cap** (`72 GiB` `heavy.slice` after the 2026-11-08/09 host OOM crashes; `openspec/changes/sherlock-vm-consensus-20261107/tasks.md:36`, `wiki-llm/recursion-perf/profiles/c0-first-profile.md:57`).
2. **Measured per-circuit times and peak RSS (`qa/sherlock/runs.jsonl`, `c0-first-profile.md`, `e2-e4.md`)**:
   - **`k = 20` Node (`SHKIVC06`, `741,708` rows, `20t` solo)**: **`61.07–67.0 s`** (`runs.jsonl:617,633,648`), peak RSS **`16.55–17.58 GB`**, one-time SRS+PK load **`~42.25 s`**. Under 3-way concurrency (`3 x 6t`, `C0-t1b-concurrent-3`, `runs.jsonl:618`), each node proves in **`128.09–131.50 s`** (`f3 = 1.69–1.74` $\implies$ **`43.8 s` amortized/node**), aggregate RSS **`47.73 GB <= 60 GiB`**.
   - **Leaf**: `k = 13` M2 leaf proved in **`2.55 s`** (`prover.md:11`); `k = 20` E4 stand-in (`742,327` rows) proved in **`66.16 s`** solo at **`11.14 GB`** RSS (`e4-solo.json:12`, `runs.jsonl:709`), or **`69.11 s`** leaf + **`90.2 s`** node partially overlapped (`e4-concurrent.json`, `runs.jsonl:711`). For our `k = 14..16` hand-written chip leaf (`10,439..42,179` rows, no ZKIR `282 s` `model_pick`), prove time is **`ESTIMATE ~1.8–6.0 s`** at **`ESTIMATE ~2–4.5 GB`** RSS.
   - **`k = 20` Run Decider (`929,166` rows, `20t`)**: **`70.35–77.88 s`** (`runs.jsonl:617,649`), peak RSS **`17.35 GB`**.
   - **One-time fixed run tail (`T_fixed`)**: Node+leaf SRS/PK load (`~55–70 s` `ESTIMATE`) + Decider (`~75 s`) + independent bundle verifier `SRC-010` (`< 10 s` cap; legacy `A5` took `2.86 s` at `507 MB`, `budgets.json:35-42,354-363`) + `validate_call` `SRC-020` (`ESTIMATE ~10–25 s`) $\approx$ **`ESTIMATE 150–180 s` (~2.5–3.0 min)**.

### 5.2 Throughput Per Step and Practical Run-Length Ceilings (`ESTIMATE`)

Because leaf (`3,824 B`) and node (`6,432 B`) proofs stream to disk in the `SRC-019` bundle (`~6.4 MB` per `1,000` leaves), **process RSS remains flat (`<= 18 GB` solo, `<= 48–50 GB` 3-way concurrent) regardless of run length**. Prover wall time is the binding constraint (shown for `S = 16` full leaves and $\bar{s} = 12$ effective steps/leaf with early chip closes):

| Proving Regime (`k=16` chip leaf + `k=20` node, `~830k` rows) | Time / Leaf (`ESTIMATE`) | Time / Step (`S=16` / `s̄=12`) (`ESTIMATE`) | Steps in **2 h** (`SRC-011`) (`ESTIMATE`) | Steps in **6 h** (`ESTIMATE`) | Steps in **24 h (1 day)** (`ESTIMATE`) |
|---|---:|---:|---:|---:|---:|
| **1. Serial Chain (`1 x 20t`, CPU today)** | `~70 s` (`65s` node + `5s` leaf) | **`4.38 s`** / **`5.83 s`** | `~1,600` / `~1,200` | `~4,860` / `~3,650` | **`~19,600` / `~14,700`** |
| **2. Binary Tree (`GP-4` `R+N`, `3 x 6t` concurrent <= `60 GiB`)** | `~49 s` amortized (`~1` `k=20` proof/leaf at `44s` + leaf) | **`3.06 s`** / **`4.08 s`** | `~2,290` / `~1,720` | `~6,950` / `~5,210` | **`~28,100` / `~21,000`** |
| **3. Future CPU Horner multithread + GPU (`L4`, `ESTIMATE 2.5x`)** | `~20 s` amortized | **`1.25 s`** / **`1.67 s`** | `~5,600` / `~4,200` | `~17,000` / `~12,700` | **`~69,000` / `~51,500`** |

*(Sanity check on real benchmarks)*: `entry 7;` (`6` steps, `1` leaf) takes `ESTIMATE ~4.2 min` (`< 20 min` `A18` gate); `fact(3)` (`293` steps, `19..25` leaves) and `list123` (`303` steps, `19..26` leaves; `docs/sherlock/stagenet-gp-recursion.md:229-232`) take **`ESTIMATE ~25–33 min` serial** (`< 120 min` `SRC-011` gate).

### 5.3 Recommended Tested Targets to Name in the Specification

1. **Keep `SRC-011` Mandatory Release Throughput Gate**: **`<= 320` owner steps proved end-to-end (leaves + node fold + decider + `SRC-010` independent verifier + `SRC-020` local `validate_call`) in `< 2 hours` (`7,200 s`)**, `<= 48 GiB` per-process RSS, `<= 100 GiB` (`60 GiB` default operational) aggregate RSS, verified on `fact(3)` (`293` steps) and `list123` (`303` steps) (`specs/sherlock-recursion/spec.md:110-116`).
2. **Add `GP-001` Multi-Leaf & Committed-Memory Endurance Target (Milestone M5, row `A33b`)**: **`>= 4,096` owner steps (`>= 256` leaves at `S = 16`) and `>= 4,096` dynamically allocated cells** proved end-to-end under the frozen key set in **`< 6 hours` (`21,600 s`)** within the same RSS caps. Why `4,096`: it exceeds both single-leaf registers and hundreds of 8-cell segments (exercising top-tree flushes, `PAGE` openings, and hash-stacks thoroughly) while running comfortably in a single `~2.2–3.5 h` unattended `heavy` job.
3. **Add `SRC-023` Partition-Independence Target (row `A38`)**: One program of **`>= 64` owner steps** proved under three partitions (`S = 16`, `S = 1`, mixed early closes) yielding identical `[prog_d, input_d, outcome_d, n_steps]` in **`< 90 minutes`** total.
4. **Add `GP-002` Continuation Chain Target (row `A34c`)**: One program of **`>= 1,024` owner steps** proved both as a single run and as **2 chained `>= 512`-step continuation runs** aggregated via `GP-002`, completing in **`< 2 hours`** and producing identical `claim_d` and `n_steps`.

---

## What Was Not Checked

In compliance with the Round 1 rules (`BRIEF.md:42,75-77`), the following items were **not** executed or verified empirically in this study:

1. **No circuit compilation, `cost_model` execution, Quint run, or proof generation was run**: All row counts beyond already-recorded repo artifacts (`6,511` E8 leaf, `741,708..741,736` E2 node, `823,466` chip-key node, `929,166` decider) are algebraic `ESTIMATE` derivations from the exact row equations in `crates/mori-chip/src/{rfcore.rs,m3.rs,poseidon.rs,range.rs}` and the E2/E5/E7/E8 linear fits.
2. **Full 108-rule universal chip leaf (`PAGE` + `SVM-012` `vd` + `Init` + `ResumeInit`) is not yet built**: Effective average steps per leaf $\bar{s} \le 16$ on `fact(3)` / `list123` (how often `N_alu`, `N_page`, or `C = 8` closes a leaf early before slot 16) was not simulated against execution traces.
3. **Full simultaneous `k = 16` chip-leaf + `k = 20` node mixed contention**: `e4-concurrent.json` (`e2-e4.md:19`) overlapped `prove` only partially because the `k = 20` ZKIR stand-in spent `361.8 s` in `model_pick`; steady-state parallel pipeline throughput with a fast hand-written chip leaf is not yet measured.
4. **Binary node `N`, lift `R`, closing relation `C`, and Compact `transientHash` parity (`architecture.md` experiments 1, 2, 7)**: Row counts for the 6-key GP-2..GP-4 relations (`R, F, N, C`) and `openClaim` path verification remain unmeasured hypotheses (`architecture.md:214-224,630-638`).
5. **Midnight ledger `9.1.0.1` (`midnight-proofs =0.8.2`) vs workspace (`midnight-proofs 0.8.3`) byte parity (`tasks.md:58`, `SRC-020`)**: Not re-tested here.
6. **Other lanes' Round 1 folders (`lanes/A`, `lanes/C`, `lanes/D`, `lanes/E`)**: Never listed or read.
