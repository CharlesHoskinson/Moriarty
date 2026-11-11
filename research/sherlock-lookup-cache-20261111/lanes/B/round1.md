# Round 1 — Lane B: Memory Caching and Offline Memory Checking

Run: `sherlock-lookup-cache-20261111`  
Lane: **B** (Memory Caching & Offline Memory Checking Lead)  
Date: 2026-11-11

Conventions: repository paths are relative to `/home/hoskinson/Projects/Moriarty`. `MP:` denotes `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src`, and `MC:` denotes `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.3.0/src`. Every computed or projected figure is marked `ESTIMATE`.

---

## 0. Baseline Accounting: Where Poseidon Rows Go Today and in a Complete Uncached Leaf

Before comparing cache candidates, two baselines must be separated: (1) the **measured E8 spike leaf** (`S = 16`), which implements M3 segment append and boundary state digests but omits `PAGE` older-cell openings, `PROG` instruction openings, and C6 value digests (`research/sherlock-lookup-cache-20261111/BRIEF.md:9-15`; `crates/mori-chip/src/rfcore.rs:1204-1243`), and (2) the **complete uncached Merkle leaf** (`Baseline-Full`, `ESTIMATE`), which includes those unbuilt Merkle walks at the measured cost of **70 rows per Poseidon permutation** (`crates/mori-chip/src/poseidon.rs:58-59`).

### 0.1 Measured E8 Spike Leaf (`S = 16`, `k = 13`)

In `narrow,1,pos_m3+r1+a1+pc` (`research/sherlock-processor-analogy-20261110/experiments-4.md:52-64,155-160`), the leaf uses **6,511 `cost_model` region rows** (`k = 13`, usable domain `8,186` rows; fixed `RANGE` table `4,096` rows at `crates/mori-chip/src/range.rs:34`), **12 advice columns** (`crates/mori-chip/src/lib.rs:42`), **13 equality-enabled columns** (12 advice + 1 public-input instance), **20 fixed columns** (17 selectors/constants + 3 lookup table columns), **4 PLONK lookups** (2 bus lanes in `crates/mori-chip/src/bus.rs:50`, 1 running-sum `RANGE` in `crates/mori-chip/src/range.rs:98`, 1 `DEC` in `crates/mori-chip/src/rfcore.rs:941`), max gate degree **6**, and rotations `{-1, 0, +1}`.

| Leaf Region | Formula in `crates/mori-chip/src/` | Perms | Rows (`S = 16`) | Share |
|---|---|---:|---:|---:|
| Core (`Arch::Narrow`, `h = 13`) | `13 * (S + 1)` (`rfcore.rs:216,1204`) | 0 | 221 | 3.39% |
| ALU (`range1` running-sum) | `12 * S` (`range.rs:33,138`; `rfcore.rs:1205-1208`) | 0 | 192 | 2.95% |
| `SegMemConfig` (`C = 8`) | `SEG = 8` (`mem.rs:122`; `rfcore.rs:1224-1227`) | 0 | 8 | 0.12% |
| **Non-HASH subtotal** | | **0** | **421** | **6.47%** |
| M3 segment record sponges | `8 * ceil(4/2)` (`H(act, a, b, c)`, `m3.rs:54,306-310`) | 16 | 1,120 | 17.20% |
| M3 segment subtree compressions | `(8 - 1) * 1` (`m3.rs:54,316-326`) | 7 | 490 | 7.53% |
| M3 shared-path top walks (`TOP = 17`) | `2 * 17` (old `0 -> old_root`, new `seg_root -> new_root`, `m3.rs:47,54,332-414`) | 34 | 2,380 | 36.55% |
| Leaf boundary state digests (`pre_d`, `post_d`) | `2 * ceil(30/2)` (29 reg words + `mem_root`, `rf.rs:34`; `m3.rs:54,420-447`) | 30 | 2,100 | 32.25% |
| **HASH subtotal** | `87 * 70` (`poseidon.rs:59,113-115`) | **87** | **6,090** | **93.53%** |
| **Total E8 Spike Leaf** | (`experiments-4.md:57,157`) | **87** | **6,511** | **100.00%** |

Measured recursion baselines (`experiments-4.md:55-56,62-63`; `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:213`):
- **Node (`ChipNode`)**: **823,466 rows** (`k = 20`, gate `950,000`, headroom **126,534 rows**). Linear fit: `~570,957 + 7,298 * advice + 8,189 * equality + 6,004 * fixed + 22,600 * lookup` (`research/sherlock-processor-analogy-20261110/experiments-2.md:125,230-232`).
- **Decider (`RunDeciderCircuit`)**: **929,166 circuit rows** (`table_rows = 1,032,467` of `1,048,576`, remaining `table_rows` margin **16,109**). Leaf VK changes do not change decider rows unless Node's `ZkStdLibArch`, public-input count, or decider constraints change (`research/sherlock-processor-analogy-20261110/experiments.md:114-119`).

### 0.2 Unbuilt Poseidon Costs (`Baseline-Full`, `ESTIMATE`)

When the missing uncached Merkle operations are priced at the measured 70 rows/perm (`poseidon.rs:59`) instead of the pre-E7 25 rows/perm placeholder (`research/sherlock-processor-analogy-20261110/study.md:171`):

1. **`prog_root` in boundary state digest**: 31 state words instead of 30 raises `ceil(31/2)` from 15 to 16 perms per side (`research/sherlock-processor-analogy-20261110/experiments-3.md:134`): **+2 perms = +140 rows/leaf** (`2,240` rows total).
2. **`PAGE` older-cell Merkle read (depth 20)**: 1 record sponge `H(1, a, b, c)` (`2` perms) + 20 binary compressions (`20` perms, `experiments-3.md:205-206`; `experiments-2.md:281`) = **22 perms = 1,540 rows per cell read** (`ESTIMATE`; rises to **34 perms = 2,380 rows** at depth 32 if the `2^20`-cell ceiling is lifted via deeper trees).
3. **`PAGE` `PROG` fetch (depth 8)**: 1 instruction sponge (`2` perms) + 8 binary compressions (`8` perms) = **10 perms = 700 rows per fetch** (`ESTIMATE`; rises to **18 perms = 1,260 rows** at depth 16).
4. **C6 value digests (`sherlock-value/1`)**: `[V, 1, tag, arity, child_d...]` uses 4 to 7 inputs (`review.md:104-142`) = **2 to 4 perms = 140–280 rows per heap value allocation** (`ESTIMATE`). At E0's ~0.22 allocations/step (`BRIEF.md:14-15`), `S = 16` averages ~3.5 allocations (~11 perms); budgeting a fixed `P_c6 = 12` perms (`840` rows) at `S = 16` and `P_c6 = 24` perms (`1,680` rows) at `S = 64` handles ~4–6 and ~8–12 allocations per leaf before early leaf close (`act = 0` prefix padding, `rfcore.rs:14-16`).

At `S = 16` (with 16 uncached `PROG` fetches and 8 older-cell reads) and `S = 64` (`C = 32`, `TOP = 15`, 64 uncached `PROG` fetches, 32 older-cell reads):
- **`Baseline-Full` (`S = 16`, uncached)**: `~520` (Core+ALU+glue) + `2,240` (state) + `3,990` (M3) + `11,200` (`PROG`) + `12,320` (`PAGE` cells) + `840` (`C6`) = **31,110 rows (`k = 15`, 441 Poseidon perms)** (`ESTIMATE`).
- **`Baseline-Full` (`S = 64`, uncached)**: `~1,963` (Core+ALU+glue) + `2,240` (state) + `8,750` (M3 `C=32`) + `44,800` (`PROG`) + `49,280` (`PAGE` cells) + `1,680` (`C6`) = **108,713 rows (`k = 17`, 1,525 Poseidon perms)** (`ESTIMATE`).

---

## 1. Q1 — What Should the Cache Hold? Evaluation of Candidates (a)–(f)

### Q1(a) Memory Read Cache Against a Committed or Verified Table

Two distinct scopes fit PLONK/KZG without full-run offline memory checking:

#### (a1) Intra-Leaf Deduplicated Read Cache (`Leaf-Cache + M3`)
- **What it holds**: Up to `K_m` distinct older `MEM` cells (`addr < base`) and `K_p` distinct `PROG` rows touched during the leaf's `S` steps.
- **How it works**: In `midnight-proofs` `lookup_any` (`MP:plonk/lookup/prover.rs:108-114`), a single provider row on the bus (`crates/mori-chip/src/bus.rs:47-62`) can be looked up with **arbitrary multiplicity** across step slots at zero extra lookup or gate cost (already relied on in `models/sherlock/quint/chipLeafCached.qnt:43-45,719-724`). A `CACHE` provider region of `K_m + K_p` rows runs one depth-20 Merkle walk per *enabled* `MEM` cache slot and one depth-8 walk per *enabled* `PROG` cache slot; all `S` core slots hit those provider rows over the existing `bus0`/`bus1` lookups.
- **What it removes vs leaves behind**: In loops (`S = 64` touches ~6 distinct `PROG` rows per `study.md:171` and ~10 distinct older frame/heap cells, `ESTIMATE`), `(a1)` eliminates **58 of 64 `PROG` walks** (`-40,600` rows) and **22 of 32 cell walks** (`-33,880` rows), shrinking `S = 64` from `108,713` to **`~34,320` rows (`k = 16`)** (`ESTIMATE`). However, it **cannot** remove:
  1. **Cross-leaf cold misses**: a hot closure environment or frame chain popped across 1,000 leaves still pays `1,540` Poseidon rows in *every* leaf that touches it.
  2. **M3 segment close** (`3,990` rows at `C = 8`, `8,750` rows at `C = 32`).
  3. **Leaf-boundary state digests** (`2,240` rows).
  4. **`D3` segment gaps and fixed Merkle depth `TOP`**.

#### (a2) KZG `committed_instances` Read-Only / Prefix Cache (`KZG-Table + Leaf-Cache + M3`)
- **Key stack finding**: `VerifierGadget` in `midnight-circuits-7.3.0` **already supports KZG-committed instance columns** (`assigned_committed_instances: &[S::AssignedPoint]` in `MC:verifier/verifier_gadget.rs:278,298-300,435-437,625-639`), and `midnight-proofs` `lookup_any` evaluates `Expression::Instance` across all `2^k` Lagrange rows (`MP:plonk/prover.rs:36-49`; `MP:plonk/lookup/prover.rs:84-98`). Moreover, `Leaf` (`crates/mori-chip/src/leaf.rs:159`) and `RfLeaf` (`crates/mori-chip/src/rfcore.rs:833`) **already allocate `committed` as `instance_column` 0** and pass `[C::identity()]` in `ChipNode` (`crates/mori-chip/tests/e3_spike.rs:227`).
- **What it holds**: Packing `(space_2b, key_32b, word0, word1)` or packed 254-bit `PROG` / `TY` / static `INPUT` entries into `Instance(0)` (`C_ro` in `G1`, up to `2^{k_leaf} - 6` entries, e.g., `8,186` entries at `k = 13`), paired with one fixed index column `idx[r] = r` and one lookup argument `(space, key, packed_val) in (fixed_space, fixed_idx, Instance(0))`. `prog_d` binds `H(DOM_PROG, C_ro.x, C_ro.y, overflow_root)` once at genesis/seal.
- **Assessment**: Eliminates 100% of `PROG`, `TY`, and static `INPUT` Merkle walks for programs up to `8,186` rows (with `overflow_root` Merkle fallback preserving GP-1 for larger programs), saving `2,800–4,200` Leaf rows over `(a1)` (`ESTIMATE`), at a cost of **+~35,900 Node rows** (`~7,300` for `Instance(0)` variable base + `6,004` fixed index col + `22,600` lookup argument, `ESTIMATE`). However, because `C_ro` is static across the run (updating a KZG table commitment dynamically per leaf would require in-circuit non-native `BLS12-381 G1` scalar multiplications costing ~3,500 Node-equivalent rows per write), `(a2)` **cannot cache dynamically allocated `MEM` cells** without moving to Q1(b).

---

### Q1(b) Full-Run Offline Memory Checking Instead of a Merkle Tree

Sherlock's heap and spilled-frame arena `MEM` is **strictly write-once and sequentially bump-allocated** (`openspec/changes/sherlock-vm-consensus-20261107/design.md:99-101`; `crates/mori-chip/src/mem.rs:1-4`; `architecture.md:127`). Exploiting write-once semantics radically simplifies offline memory checking compared to read/write RAM.

#### Structural Theorem: Write-Once Sequential Memory Needs Neither Timestamps Nor Read-Writebacks
In general read/write BEGKN / Spice offline memory checking (used by OpenVM `https://docs.openvm.dev/specs/architecture/memory/`, Ziren `https://docs.zkm.io/design/memory-checking.html`, and Nexus `https://deepwiki.com/nexus-xyz/nexus-zkvm/4.4-memory-operations`), every read at address `a` must:
1. Witness `prev_ts`,
2. Range-check `0 <= clk - prev_ts - 1 < 2^B` (~12 rows in `RANGE`),
3. Subtract `(a, v, prev_ts)` from the bus and add `(a, v, clk)` back to the bus, and
4. Run a post-execution finalizer sweep emitting `(a, v_final, ts_final)` for every initialized address `a`.

In Sherlock, **all four overheads vanish**:
1. **Unique dense write addresses**: `len` starts at `init_len` (at `h = 0`), carries in the boundary register state (`leaf[i].post.len == leaf[i+1].pre.len`), and increments by `+1` iff a step allocates a cell at `addr_w = len` (`chipLeafCached.qnt:213-214,219-220`). Thus every write across the entire run has a **strictly increasing, unique address** `a_w in [0, W)`.
2. **Timestamp-free temporal causality (`no read-before-write`)**: Because address `a` is written ONLY at the unique step `t_w(a)` where `len` transitions `a -> a + 1`, for every earlier step `t <= t_w(a)` the core's `len_t` satisfies `len_t <= a`. Every `MEM` read in Sherlock already checks `0 <= addr_r < len_t` via ALU `OP_LT` (`crates/mori-chip/src/rfcore.rs:369,510`; `qa/gemini-quint/chipleaf-final/m3.qnt:682-683`). Therefore `addr_r < len_t` **implies `t_w(addr_r) < t`**: a step can never read a cell written in its own step or a future step/leaf, with **zero timestamps**.
3. **LogUp partial-fraction balance (`Stwo/Cairo` style, Haböck `https://eprint.iacr.org/2022/1530`)**: Each write `(space, a_w, v0, v1, v2)` emits **one** positive rational term with prover-supplied total read multiplicity `m_w`, and each read `(space, a_r, v0, v1, v2)` emits **one** negative rational term:
   $$\Sigma_{\text{run}} = \sum_{w \in \text{Writes}} \frac{m_w}{\alpha - E_\gamma(w)} - \sum_{r \in \text{Reads}} \frac{1}{\alpha - E_\gamma(r)} = 0$$
   where $E_\gamma(\text{space}, k, a, b, c) = \text{space} + \gamma k + \gamma^2 a + \gamma^3 b + \gamma^4 c$. Unused cells have `m_w = 0` and require **zero finalizer sweep** at the end of the run.

#### Critical Midnight Stack Constraint: `VerifierGadget` Forbids Multi-Phase Challenges
In `MC:verifier/verifier_gadget.rs:308-310`, `VerifierGadget::parse_trace` executes:
```rust
assert_eq!(cs.phases().count(), 1);
```
and squeezes `theta` immediately after reading `advice_commitments` (`MC:verifier/verifier_gadget.rs:314-320`), with no user `Challenge` vector in `VerifierTrace`. Even if `VerifierGadget` were forked to support Phase-1 challenges, an intra-leaf Fiat-Shamir challenge $\alpha_i$ would differ per leaf ($\alpha_0 \neq \alpha_1$), defeating cross-leaf LogUp cancellation. Cross-leaf offline memory checking on this stack therefore has exactly two sound realizations:

#### Candidate (b1): Two-Pass KZG Pre-Committed Write-Once LogUp (`WO-LogUp`)
This adapts Matter Labs' Airbender two-pass memory commitment pattern (`https://olegfomenko.dev/2025/09/15/memory.html`) directly to `midnight-proofs`'s existing `committed_instances` (`Instance(0)`) mechanism, keeping `cs.phases().count() == 1` in unmodified `VerifierGadget`:
1. **Simulate**: Host runs `mori simulate` (already required before proving in `design.md` §2.5, §3.1), recording dense writes `(space, a_w, v0, v1, v2)` with exact run-wide read counts `m_w`, plus leaf-deduplicated reads `(space, a_r, v0, v1, v2, mult_leaf)`.
2. **Stage-1 Pre-Commit (`< 0.2 ms/leaf`, `ESTIMATE`)**: For each leaf $i \in [0, N)$, pack its write tuples (including `m_w`!) and leaf-deduplicated read tuples (including `mult_leaf`) into a short Lagrange vector `P_i` (~150–700 scalars out of `8,186` at `k = 13`) and compute $C_i = \text{commit\_to\_instances}(\text{params}, \text{domain}, P_i) \in \mathbb{G}_1$ (`MP:plonk/prover.rs:36-49`). Hash $d_i = H(\text{DOM\_LEAF\_COM}, C_{i,x}, C_{i,y})$ up the planned aggregation tree (or left-to-right chain) to $d_{\text{root}}$, and derive run-wide $(\gamma, \alpha) = H(\text{DOM\_MEM\_CHAL}, d_{\text{root}}, \text{prog\_d}, \text{input\_d})$.
3. **Stage-2 Parallel Leaf Proving**: Pass $C_i$ as `assigned_committed_instances[0]` and `(gamma, alpha, sum_leaf)` in `pi`. Inside Leaf $i$:
   - Gate `q_com * (a[11] - Instance(0)) == 0` binds advice cells in column 11 to `P_i` (already committed in $C_i$ **before** $(\gamma, \alpha)$ were derived).
   - Intra-leaf reads/fetches hit a small local provider table via existing `bus0`/`bus1` (`lookup_any`), incrementing `mult_leaf`? Wait: `lookup_any` does not count multiplicity!
   - **Crucial Soundness Check on Intra-Leaf Deduplication**: Because PLONK `lookup_any` does NOT enforce multiplicity, if a leaf deduplicates $k$ core reads into 1 LogUp read entry `(a_r, v_r)` with weight `-1` (instead of `-k`), does that hurt soundness?!
     **Check both cases**:
     - If $m_w$ on the write side counts **number of leaves that read cell $w$** (so each leaf reading $w$ subtracts `1 / (alpha - E_gamma(w))` once from its deduplicated `CACHE` table, and `m_w` equals the count of reading leaves!), let's verify: can a leaf forge a read `(a_r, v_r) not in Writes`? If a core slot looks up `(a_r, v_r)` in Leaf $i$'s `CACHE` region, that `CACHE` row MUST be active (`r_act = 1`), so it MUST emit `-1 / (alpha - E_gamma(a_r, v_r))` into $\Sigma_{\text{run}}$! Since `(a_r, v_r)` is never written anywhere in `Writes`, the pole at $E_\gamma(a_r, v_r)$ has residue $-\text{count}_{\text{Leaves}}(a_r, v_r) \in [-N, -1] \not\equiv 0 \pmod p$ (since $N \le 2^{64} < p$)!
     - **Result**: Counting *leaf-level* hits (`-1` per active leaf-cache entry, `m_w` = number of leaves whose cache contains $w$) is **100% as sound as per-step counting**, requires **zero intra-leaf multiplicity counters**, and shrinks LogUp terms to `<= W_cap + K_m + K_p` per leaf!
4. **Node & Seal/Decider**: Each Node witnesses $C_i$ via `assign_without_subgroup_check` (matching `MC:verifier/transcript_gadget.rs:142`), checks child $(\gamma, \alpha)$ equality, folds $d_{\text{seg}}$ (`1` Poseidon perm = ~22 rows) and adds $\Sigma_{\text{seg}} = \Sigma_L + \Sigma_R$ (`1` native add). At `SEG -> SET` seal (`architecture.md:84`) or `RunDeciderCircuit`, the circuit checks $(\gamma, \alpha) == H(\text{DOM\_MEM\_CHAL}, d_{\text{root}}, \text{prog\_d}, \text{input\_d})$ and $\Sigma_{\text{root}} == 0$.
5. **Rows (`ESTIMATE`)**: Eliminates **all 57 M3 segment/walk Poseidon perms** (`-3,990` rows vs E8) and **all `PAGE`/`PROG` Merkle walks** (`-23,520` rows at `S = 16`, `-94,080` rows at `S = 64` vs `Baseline-Full`). Only `pre_d`/`post_d` (`2,100` rows) and `C6` value digests remain!

#### Candidate (b2): Single-Pass Write-Once Poseidon-to-Jubjub Multiset Hash (`EC-MSH`)
If strictly single-pass streaming proving (proving Leaf 0 before simulation reaches Leaf $N$) is desired, SP1 Hypercube (`https://docs.succinct.xyz/docs/sp1/hypercube/global`) and Ziren (`https://docs.zkm.io/design/memory-checking.html`) use elliptic-curve multiset hashes.
- **Why NOT hashless map-to-curve (Groth–Malvai–Miller–Zhang, Asiacrypt 2025, `https://eprint.iacr.org/2025/1503`)**: **El Housni and Bünz (Asiacrypt 2026, `https://eprint.iacr.org/2026/590`)** demonstrated a concrete algebraic forgery against Groth et al.'s original $x$-increment parameters outside the EC-GGM, and showed that witness generation is practical only on low 2-adicity fields while their $y$-increment fix targets $j = 0$ curves. Midnight's native scalar field `BLS12-381 Fq` has **2-adicity 32**, and Midnight's embedded curve **Jubjub** (`MC:ecc/native/edwards_chip.rs:14-16,49`) is a twisted Edwards curve with $j \neq 0$. Hashless map-to-Jubjub must therefore be **rejected on security and 2-adicity grounds**.
- **Sound construction on Jubjub (`Poseidon-to-Jubjub`)**: Hash `(space, key, a, b, c)` + 8-bit tweak `t` via Poseidon (`3` perms = `210` rows for 6 inputs, or `4` perms = `280` rows chaining tweak) to an Edwards coordinate, verify on-curve + cofactor-8 clearing (`3` doublings = `~18` rows), and add `[m_w] P` (`~260` rows for 64-bit `m_w` scalar mul; `m_w < 2^64` prevents wrap-around mod Jubjub subgroup order $r_{\text{JJ}} \approx 2^{251.8}$) on writes and `-P` (`~286` rows total) once per leaf-deduplicated read.
- **Assessment (`ESTIMATE`)**: Costs `~540` rows/write and `~286` rows per cold leaf read (5.4x cheaper than a 1,540-row depth-20 Merkle walk, `0` cross-leaf challenges, `~7,809` rows at `S = 16` and `~17,529` rows at `S = 64`), but **2.2x–4.6x more leaf rows than `(b1)` / `(f1)`**.

---

### Q1(c) Instruction, Decode, Program, Type, and Constructor Tables
1. **`DEC` (Fixed Decoder ROM, `crates/mori-chip/src/dec.rs:20-45`)**: Keep as a fixed PLONK lookup (`TableColumn`). Because each `TableColumn` holds a 254-bit `Fq` element, all decoded microcode flags (`rule_id`, `alu_op`, `reg_sel`, `bus_tags`, `trap_action` — ~40–60 bits) pack into **one `rule` `TableColumn`** and unpack linearly in `CORE` (`rule == sum 2^i b_i` with boolean `b_i`), adding **0 Leaf rows and 0 Node rows** beyond today's `DEC` lookup.
2. **`PROG` and Type/Constructor Tables (`ty_d`)**: Unify directly into `(b1)`/`(f1)`'s Write-Once LogUp bus under distinct domain tags `AS_PROG` and `AS_TYPE`! To satisfy **GP-1** ("Capacity does not depend on the key", `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:14-18`) for arbitrarily large programs without any fixed Merkle depth:
   - Define `prog_d` as a sequential hash chain `h_{k+1} = H(h_k, chunk_k)` over instruction/type chunks.
   - Prepend $M_P = \lceil |P| / B \rceil$ `INIT_PROG` loader leaves (same leaf VK `U`, mode bit `mode = INIT_PROG` before `h = 0`) that hash sequential chunks toward `prog_d`, commit `(AS_PROG, pc, op, a, b, m_pc)` into $C_i$, and emit $+\frac{m_{\text{pc}}}{\alpha - E_\gamma(\dots)}$ into $\Sigma_{\text{run}}$.
   - A 64-instruction program needs **2 tiny loader leaves**; a $10^6$-instruction program needs $31,250$ loader leaves under the **exact same VK**, while execution leaves pay **0 Poseidon permutations** for `PROG` and `TY` lookups!

---

### Q1(d) ALU and RANGE as Decomposed Lookup Tables (Jolt Style) vs Current Gates
- **Replacing 128-bit `ADD`/`LT` gates with Jolt/Lasso subword tables is a net loss on this PLONK/KZG stack**:
  - Today's `ALU + RANGE` (`range1` running-sum over one `4,096`-row 12-bit `TableColumn`, `crates/mori-chip/src/range.rs:33-34,98-145`) costs **12 Leaf rows/op** (`192` rows at `S = 16`, just **2.95%** of E8!).
  - An 8-bit `(byte_x, byte_y)` table has `65,536` rows, forcing `k_leaf >= 17` (**16x larger Leaf FFT/MSM**). A 4-bit nibble table (`256` rows/op) needs 32 lookups per 128-bit op: adding parallel PLONK lookup arguments costs **~22,600 Node rows per lookup** (`experiments-2.md:231`), so 6 extra lookups (`+135,600` Node rows) blow past the 950,000 Node gate (`823,466 + 135,600 = 959,066`), while serializing through 1 lookup takes 32 rows/op > 12 rows/op! And Jolt's `Shout` (`https://eprint.iacr.org/2025/105`) relies on multilinear sumcheck + virtual polynomials not supported by `midnight-proofs`.
- **Where a table DOES win at ZERO Node cost (Multiplexed `RANGE + 5-bit Bitwise` Table)**:
  - `k = 13` has `8,186` usable rows, of which `RANGE` uses `4,096`, leaving **`4,090` free fixed-table rows**.
  - Pack `3,072` rows of 5-bit `XOR`, `AND`, `OR` (`3 * 32 * 32 = 3,072`, `4,096 + 3,072 = 7,168 <= 8,186`) into the **existing single `RANGE` `TableColumn`** as `op_tag * 2^15 + x_5b * 2^10 + y_5b * 2^5 + z_5b` (`op_tag = 0` for 12-bit range `x_12b`, `1..3` for 5-bit bitwise).
  - Because the lookup input expression is linear in `(op_tag, x, y, z)`, gate degree is unchanged, **Node cost is +0 rows** (0 new lookups, 0 new fixed cols), and GP-1 gains 26-row 128-bit `XOR`/`AND`/`OR` alongside 12-row `ADD`/`SUB`/`LT`.

---

### Q1(e) Hash Memoization for Repeated Poseidon Calls
1. **Already memoized structurally**: Under C6 `sherlock-value/1` (`review.md:104-142`), a heap value's digest `value_d` is computed **once upon allocation** and stored in its write-once `MEM` cell / register handle; reading or nesting an existing value never re-hashes its children.
2. **Intra-leaf memoization (free today)**: In `crates/mori-chip/src/poseidon.rs:24-26` and `hashchip.rs:36-73`, `HASH` serves `lookup_any` provider rows; multiple identical 2-input (or fixed-arity) Poseidon calls within one leaf already hit **one** `HASH` run at zero extra rows.
3. **Run-wide `AS_HASH` memoization on `WO-LogUp`**: Adding `AS_HASH` tuples `(AS_HASH, arity_domain, w0, w1, w2, out_d)` (or chaining >3 words via 2-to-1 pair digests) to `(b1)`/`(f1)`'s LogUp sum lets repeated allocations of identical constructors (e.g., `None`, `Nil`, boxed small ints/tuples in loops) pay **~2 rows** per hit instead of **140–280 Poseidon rows**, at **+0 Node columns**. **Mandatory soundness rule**: `arity_domain` MUST bind the exact input word count `L` + domain tag so `[a, b]` cannot collide with `[a, b, 0]`.

---

### Q1(f) Better / Hybrid Design: Zero-Merkle Leaf (`ZM-LogUp`, Candidate `f1`)

Even after `(b1)` removes M3 (`3,990` rows) and `PAGE`/`PROG` walks (`23,520` rows), **`pre_d` and `post_d` leaf-boundary state digests still consume 30 Poseidon permutations = 2,100 rows per leaf** (`80%` of the remaining E8-comparable leaf!).

**Key observation (`ZM-LogUp`)**: In `(b1)`, Stage 1 *already* commits a KZG Lagrange vector `P_i` per leaf into `Instance(0)` ($C_i \in \mathbb{G}_1$) **before** $(\gamma, \alpha)$ are squeezed from $d_{\text{root}}(C_0, \dots, C_{N-1})$.
1. Place Leaf $i$'s 29 pre-state words $\vec{u}^{(i)}$ in `P_i[0..29]` and 29 post-state words $\vec{v}^{(i)}$ in `P_i[29..58]` (`58` scalars in `Instance(0)`, copy-constrained to Core slot `0` and slot `S` via `a[11]`).
2. Replace the two 15-permutation Poseidon sponges (`2,100` rows) with **two 29-step Horner evaluations** over $\gamma$:
   $$\text{pre\_fp}_i = \sum_{j=0}^{28} \gamma^j u^{(i)}_j, \qquad \text{post\_fp}_i = \sum_{j=0}^{28} \gamma^j v^{(i)}_j$$
   taking **58 rows** (or **0 extra vertical rows** by running in idle advice columns `6..11` alongside Core/ALU/HASH, since `poseidon.rs:17-19` uses only columns `0..5` on absorb rows and `0..3` on round rows!).
3. Combine `(b1)` Write-Once Dense LogUp (`AS_MEM`, `AS_PROG`, `AS_TYPE`, `AS_HASH`) + `(a1)` Intra-Leaf `lookup_any` Deduplication + Horner Boundary Compression + `(d)` Multiplexed `RANGE + 5b Bitwise` + **one `vsel` code column** (`crates/mori-chip/src/vsel.rs:38-77`, codes `1..4` for `q_com`, `q_horner`, `q_logup_w`, `q_logup_r`, max degree `3 + 3 = 6`, adding only **1 Fixed column = +6,004 Node rows**!).

**Result (`ESTIMATE`)**:
- **Zero Poseidon permutations** in an E8-comparable leaf (`599` rows vs `6,511`, **-90.8%**).
- In a full leaf, **only C6 value-allocation digests use Poseidon**: `1,439` rows at `S = 16` (**-95.4%** vs `31,110` `Baseline-Full`), **`3,831` rows at `S = 64`** (`k = 13`), and **`7,531` rows at `S = 128` — still fitting inside `k = 13` (`8,186` usable rows)**! Raising `S` from `16` to `64`–`128` cuts the number of expensive ~60-second `k = 20` Node proofs (`architecture.md:27`) by **4x to 8x**.

---

## 2. Q2 — Quantitative Row Estimates, Soundness, Challenges, and Recursion Fit

### 2.1 Master Quantitative Comparison Table (`ESTIMATE` Except Measured Baselines)

All Node estimates apply the measured linear fit (`experiments-2.md:125,230-232`) to the 823,466-row E8 baseline (`experiments-4.md:55`), plus `~7,300` rows when `Instance(0)` is queried at `Rotation::cur()` (1 variable base + 1 `read_scalar`; `common_point` is already paid in `MC:verifier/verifier_gadget.rs:298-300`), and `~22` rows per 2-input `Poseidon` in Node/Decider (`experiments-2.md:159`).

| Design Candidate | Leaf `S=16` vs E8 (no PAGE/C6) | Leaf `S=16` Full (+PAGE/PROG/C6) | Leaf `S=64` Full (`k_leaf`) | Node Rows (`k=20`, gate 950k) | Decider Rows (`k=20`, gate 1M; `table_rows` margin 16,109) | Extra Challenges / Passes |
|---|---:|---:|---:|---:|---:|---|
| **E8 Spike (Measured)** (`experiments-4.md:55-57`) | **6,511** (`k=13`) | n/a (incomplete) | n/a | **823,466** (measured) | **929,166** (`table` 1,032,467) | None (1-pass) |
| **`Baseline-Full` (Uncached Merkle)** (`ESTIMATE`) | 6,511 (`k=13`) | **31,110** (`k=15`) | **108,713** (`k=17`) | ~829,500 (+1 fixed) | 929,166 (+0) | None (1-pass) |
| **(a1) `Leaf-Cache + M3`** (intra-leaf dedup) (`ESTIMATE`) | 6,511 (`0` saved) | **16,570** (`-14,540`, `k=15`) | **34,320** (`-74,393`, `k=16`) | ~829,500 (`+6,004`) | 929,166 (+0) | None (1-pass) |
| **(a2) `KZG-Table PROG + Leaf-Cache + M3`** (`ESTIMATE`) | 6,511 (`0` saved) | **13,630** (`-17,480`, `k=14`) | **30,020** (`-78,693`, `k=15`) | ~859,400 (`+35,900`) | 929,166 to ~929,320 | None (1-pass) |
| **(b1) `WO-LogUp`** (2-pass write-once LogUp, keeps `pre_d`/`post_d`) (`ESTIMATE`) | **2,641** (`-3,870`, `k=13`) | **3,481** (`-27,629`, `k=13`) | **5,873** (`-102,840`, `k=13`) | ~837,200 (`+13,700`) | 929,166 (GP seal) / ~929,420 (IVC) | $(\gamma, \alpha)$ via Stage-1 `C_i` (2-pass per run) |
| **(b2) `EC-MSH`** (1-pass Poseidon-to-Jubjub write-once) (`ESTIMATE`) | **4,809** (`-1,702`, `k=13`) | **7,809** (`-23,301`, `k=13`) | **17,529** (`-91,184`, `k=15`) | ~836,000 (`+12,500`) | 929,166 (GP seal) / ~929,300 (IVC) | None (1-pass streaming) |
| **(c+d+e) `Tables + Memo Only`** (on `(a1)`, muxed 5b bitwise) (`ESTIMATE`) | 6,511 (`0` saved) | **16,290** (`-14,820`, `k=14`) | **33,200** (`-75,513`, `k=16`) | ~829,500 (`+6,004`) | 929,166 (+0) | None (1-pass) |
| **(f1) `ZM-LogUp` (Recommended Lead)** (`WO-LogUp` + Horner state + dedup + muxed 5b) (`ESTIMATE`) | **599** (`-5,912`, `k=13`*) | **1,439** (`-29,671`, `k=13`) | **3,831** (`-104,882`, `k=13`; **`S=128`: 7,531, `k=13`**) | **~837,200** (`+13,700`, **112.8k headroom**) | **929,166** (GP seal) / **~929,420** (IVC; `table` +0) | $(\gamma, \alpha)$ via Stage-1 `C_i` (2-pass per run) |

*\*Note on `k_leaf` floor*: `RANGE` currently fixes a `4,096`-row 12-bit table (`range.rs:34`), holding `k_leaf >= 13` (`8,192`), which makes growing `S` from `16` to **`64` or `128` inside the same `k = 13` domain** far higher leverage than shrinking `S = 16` below `4,096` rows.

### 2.2 Soundness Arguments and Attack Defenses

1. **Quantifier order for `m_w` and boundary registers in `(b1)` / `(f1)`**:
   - *Trap*: If write multiplicities $m_w \in \mathbb{F}$ were unconstrained advice chosen *after* $\alpha$, a cheater could set $m_{w_0} = (\alpha - E_\gamma(w_0)) \sum_r (\alpha - E_\gamma(r))^{-1}$ to forge arbitrary reads. Similarly, Horner state fingerprints $\sum \gamma^j u_j$ are collision-resistant only if $\vec{u}, \vec{v}$ are fixed *before* $\gamma$.
   - *Defense*: Every leaf's $\vec{u}^{(i)}, \vec{v}^{(i)}$, active write tuples $(w_{\text{act}}, \text{space}, a_w, \vec{v}_w, m_w)$, and active leaf-cache read tuples $(r_{\text{act}}, \text{space}, a_r, \vec{v}_r)$ are placed in `Instance(0)` ($C_i \in \mathbb{G}_1$), which is hashed into $d_{\text{root}}$ **before** $(\gamma, \alpha) = H(\text{DOM\_MEM\_CHAL}, d_{\text{root}}, \text{prog\_d}, \text{input\_d})$ is evaluated. By KZG evaluation binding on `bls_midnight_2p13` (opened at `x` in `MC:verifier/verifier_gadget.rs:627-635` after absorbing $C_i$ at line `298-300`), the polynomials $P_0, \dots, P_{N-1}$ are fixed prior to sampling $(\gamma, \alpha)$.
2. **Rational-function soundness bound (Haböck `https://eprint.iacr.org/2022/1530`)**:
   - All write keys $(\text{space}, a_w)$ across the run are **pairwise distinct** (`a_w` strictly increments via `len` in `AS_MEM`; `pc` strictly increments in `INIT_PROG` loader leaves for `AS_PROG`).
   - Tuple compression $E_\gamma(\text{space}, k, a, b, c)$ collides between any two distinct tuples with probability $\le 4 / |\mathbb{F}|$ per pair ($\le \frac{4(W+R)^2}{2|\mathbb{F}|}$ overall, or compress against a distinct $\gamma_1, \gamma_2$).
   - Conditioned on no tuple collision, any forged read $r^* \notin \text{Writes}$ introduces a pole $z^* = E_\gamma(r^*)$ absent from `Writes`. Since each leaf contributes at most $-1$ per cache slot and total leaves $N < 2^{64} \ll p \approx 2^{254.9}$, the residue at $z^*$ is $-c(r^*) \bmod p \neq 0$. Clearing denominators over $D \le W + R$ poles gives a nonzero numerator of degree $< W + R$, which vanishes at $\alpha$ with probability $\le \frac{W + R}{|\mathbb{F}|} < 2^{-185}$ for runs up to $2^{64}$ steps (`ESTIMATE`).
   - Horner boundary check: $\Delta(\Gamma) = \sum_{j=0}^{28} (v^{(i)}_j - u^{(i+1)}_j)\Gamma^j$ vanishes at $\gamma$ for $\vec{v}^{(i)} \neq \vec{u}^{(i+1)}$ with probability $\le 28 / |\mathbb{F}| \approx 2^{-250}$ per boundary (`ESTIMATE`).
3. **Pole / division-by-zero defense**:
   - Active LogUp rows enforce `act * (1 - act) == 0`, `act * (inv * (alpha - enc) - 1) == 0`, and `(1 - act) * inv == 0`, contributing `act * mult * inv` (writes) or `-act * inv` (reads) to the running sum. Disabled padding rows contribute `0` identically (`D8` zero-padding discipline, `chipLeafCached.qnt:134-135`).
4. **Temporal causality (`no read-before-write`)**:
   - Enforced deterministically by `addr_w == len_t`, `len_{t+1} == len_t + w_act`, genesis `len_0 == init_len`, cross-leaf `post.len == next_pre.len`, and per-read ALU check `0 <= addr_r < len_t` (`lt == 1`).

### 2.3 Compatibility with Midnight PLONK/KZG Recursion (`midnight-proofs` 0.8.3, `midnight-circuits` 7.3.0)
- **Zero upstream crate changes**: `(a1)`, `(a2)`, `(b1)`, `(b2)`, and `(f1)` all keep `cs.phases().count() == 1`, `num_instance_columns == 2` (1 committed + 1 public scalar), max gate degree `<= 6`, and rotations `{-1, 0, +1}` (`MC:verifier/verifier_gadget.rs:285-288,310,604-610`).
- **Decider `table_rows` safety (`16,109` margin)**: Neither `(b1)`, `(b2)`, nor `(f1)` enables any new chip in `ZkStdLibArch` (`bls12_381 + poseidon` unchanged in Node and Decider), so the fixed lookup tables of `RunDeciderCircuit` (`1,032,467` `table_rows`, `review.md:213`) add **0 table rows**.

---

## 3. Q3 — Interaction with C6 Outcome-Only, SRC-023, Run-Length Ceiling, D-1 Slices, and GP-1..GP-4

### 3.1 C6 Outcome-Only Publication and SRC-023 Partition Independence
- **Hidden defect in M3 today**: In `crates/mori-chip/src/m3.rs:132,437-440` (`qa/gemini-quint/chipleaf-final/m3.qnt:456,622`), closing a leaf forces `post.len = base + C` (`base + 8`). Consequently, **internal heap and frame addresses today depend on where leaves close** (closing after 1 append jumps `len` from `9` to `16`, whereas staying in the same leaf allocates `9`). C6 `sherlock-value/1` (`review.md:12-33,104-121`) avoids leaking those addresses on-chain only by recursively hashing values and omitting `post_d` from `RunDeciderRelation`.
- **How `ZM-LogUp` (`f1`) and `WO-LogUp` (`b1`) strengthen SRC-023**:
  1. **Internal address partition independence**: Removing `C = 8` segment subtrees deletes `len = base + C`; `len` increments strictly by `1` per allocation regardless of `S` or early leaf closes (`steps < S`). Every cell address and C6 `value_d` is invariant across all leaf partitions.
  2. **Public output partition independence**: $(d_{\text{com}}, \gamma, \alpha, \Sigma_{\text{mem}})$ and $(\text{pre\_fp}, \text{post\_fp})$ live **strictly inside `SEG`**. At the `SEG -> SET` seal (`research/sherlock-gp-architecture-20261110/architecture.md:84,99`) or `RunDeciderCircuit` (`review.md:19-28`), the circuit checks $(\gamma, \alpha) == H(\text{DOM\_MEM\_CHAL}, d_{\text{root}}, \text{prog\_d}, \text{input\_d})$ and $\Sigma_{\text{root}} == 0$, then **drops** all four internal fields, publishing only `[prog_d, input_d, outcome_kind, outcome_value_d, final_step_count]` (or `claim`). **No public value depends on leaf boundaries or intra-run tree shape.**

### 3.2 Run-Length and Memory Ceiling (`research/sherlock-runlength-consensus-20261111/`)
- **Direct touchpoint**: Today's `TOP = 17` (`crates/mori-chip/src/m3.rs:47`; `openspec/changes/sherlock-vm-consensus-20261107/dissent.md:183-218`) bakes a hard `2^17`-leaf (`2^20`-cell) ceiling into `leaf_vk`, violating GP-1 (`architecture.md:186-189`). Raising `TOP` to `32` or `64` in M3 would add `+30` to `+94` perms (`+2,100` to `+6,580` rows) to M3 *and* `+12` to `+44` perms (`+840` to `+3,080` rows) to *every* `PAGE` read!
- **`ZM-LogUp` (`f1`), `WO-LogUp` (`b1`), and `EC-MSH` (`b2`) eliminate the Merkle ceiling completely**: There is **no tree depth in `leaf_vk`**. Memory addresses and `h` use the existing 64-bit / 128-bit ALU range checks (`ivc.rs` `constrain_heights`; `aggregation.qnt:53-56`), scaling seamlessly to `2^64` steps and cells under one fixed VK. Pairing `AS_PROG` and `AS_INPUT` with sequential `INIT` loader leaves removes fixed Merkle depths from program and input loading as well.

### 3.3 `SI-12` to `SI-14` Contiguous Field Slices (`dissent.md` D-1)
- In `openspec/changes/sherlock-vm-consensus-20261107/dissent.md:17-55` (D-1), `SI-12` to `SI-14` index constructor fields at `start + 0, start + 1, ..., start + k - 1`. Under M3 (`C = 8`), any multi-step slice allocation crossing a leaf boundary hits an unallocated `[base + used, base + 8)` gap, breaking `start + idx` contiguity.
- Under `ZM-LogUp` / `WO-LogUp` / `EC-MSH`, dense bump allocation (`new_addr == running_len`, carried unchanged across leaf boundaries) guarantees that $k$ consecutive field appends receive physical addresses **`start, start + 1, ..., start + k - 1` even when split across arbitrarily many leaf boundaries**, satisfying `SI-12`–`SI-14` natively with zero address-translation tables.

### 3.4 General-Purpose Requirements GP-1 to GP-4 (`general-purpose.md` & `architecture.md`)
- **GP-1 (Universal VM)**: Satisfied without narrowing; one `{U, R, F, N, C, D}` key set supports arbitrary loops, recursion, dynamic heap/frame growth, and `2^64` steps/cells.
- **GP-2 (Cross-Run Composition)**: Each run seals its own `(d_root, gamma, alpha, sum_mem == 0)` at `SEG -> SET` (`architecture.md:84-86`). Sealed `SET` claims carry **no** open memory accumulators or challenges, so independent runs proved at different times/machines compose (`SET + SET -> SET` or `SEG + SET -> SET`) with zero cross-run challenge coordination.
- **GP-3 (Foreign Proofs)**: Foreign proofs enter via `A_c -> F -> SET` (`architecture.md:70-77`), orthogonal to `SEG`; `U`'s single VK stays a constant registry member.
- **GP-4 (Any Shape — Chains, Binary Trees, Uneven Trees, DAGs)**:
  - Claim-level DAG fan-out (reusing one proof in two parents) occurs on `SET` proofs, which are already sealed.
  - Intra-run `SEG + SEG -> SEG` supports chains, balanced trees, and uneven trees. To make `SEG` proofs **100% associative** (allowing post-hoc re-bracketing of already-proved leaves without re-deriving $\gamma, \alpha$), `d_com` can chain left-to-right via 2 public scalars `(com_pre, com_post)` where `R`/`N` checks `com_mid` equality (`left.com_post == right.com_pre`), so $d_{\text{root}}$ depends only on the sequence `(C_0, ..., C_{N-1})` and not on the aggregation tree shape!

---

## 4. Q4 — Production zkVM Prior Art and 2025–2026 Literature

| System / Paper | Intra-Shard Lookup / Cache | Cross-Shard Memory Consistency | Write-Once / Read-Only Specialization | Takeaway for Sherlock on Midnight PLONK/KZG |
|---|---|---|---|---|
| **Jolt** (`Lasso` `https://eprint.iacr.org/2023/1216`; **`Twist and Shout`** Setty–Thaler–Zhu 2025/2026 `https://eprint.iacr.org/2025/105`; summary `https://georgwiese.github.io/crypto-summaries/Papers/Twist-and-Shout`) | `Shout`: one-hot tensor address $ra \in \{0,1\}^{T \times K}$ + multilinear sumcheck $rv = ra \cdot \text{Val}$ (`8x` fewer nonzero commitments than Lasso, `3x` faster Jolt prover). | `Twist`: one-hot `ra`, `wa`, increments `Inc(j) = wv(j) - Val(k*, j)` + `LT` sumcheck; exploits temporal locality ($O(i)$ work for reuse $2^i$ cycles ago). | `Shout` drops `wa`, `Inc`, and `LT` completely for read-only tables. | Sumcheck + $d$-split one-hot commitments require a multilinear sumcheck verifier not in `midnight-proofs`; confirms separating read-only/write-once from R/W RAM. |
| **SP1** (v4 Turbo & **SP1 Hypercube**: `https://docs.succinct.xyz/docs/sp1/hypercube/global`; LogUp-GKR `https://eprint.iacr.org/2023/1284`; `https://l2beat.com/zk-catalog/sp1hypercube`) | `MemoryLocal` deduplicates all intra-shard accesses per address via LogUp-GKR over multilinear hypercube. | Single-pass BEGKN multiset hash: **Poseidon2-to-elliptic-curve** + EC point addition across shards into `Global` table. | Static ELF precomputed in `initial_global_cumulative_sum` when fixed, or initialized once. | Direct blueprint for `(a1)` intra-leaf deduplication + `(b2)` `Poseidon-to-Jubjub` single-pass `EC-MSH`. |
| **RISC Zero** (`https://deepwiki.com/risc0/risc0/2-zkvm-core-system`; `https://dev.risczero.com/faq`; `https://olegfomenko.dev/2025/09/15/memory.html`) | Per-segment permutation / grand-product memory argument with per-segment Fiat-Shamir challenges. | **Demand-paged 1 KB Merkle tree** (`MemoryImage` / `PagedMemory`); dirty pages re-hashed at segment boundary. | `ZERO_CACHE` for uninitialized memory; `ImageID` Merkle root binds ELF. | Explains the exact engineering trade-off: per-segment Merkle paging enables streaming before simulation ends, at high Poseidon paging cost (which kills BLS12-381 narrow leaves at 70 rows/perm). |
| **OpenVM** (`https://docs.openvm.dev/specs/architecture/memory/`; **CVE-2026-62333** `https://github.com/openvm-org/openvm/security/advisories/GHSA-396x-v8w4-9x82`) | Timestamped BEGKN on `MEMORY_BUS`: every R/W checks `prev_ts < ts` (`<= 29` bits) and sends/receives `(as, ptr, data, ts)`. | `PersistentBoundaryChip` (`MemoryMerkleAir`) bridges continuation segments via Merkle roots. | Separate `address_space` tags for program, registers, native/byte RAM. | **CVE-2026-62333 (Oct 2026)** (`MemoryMerkleAir` below-leaf rows forged roots without bus ops) highlights the subtle attack surface of Merkle continuation chips that `ZM-LogUp` eliminates entirely. |
| **Ziren (ZKM V2.0)** (`https://docs.zkm.io/design/memory-checking.html`; `https://docs.zkm.io/design/lookup-arguments.html`) | `MemoryLocal` + `MemoryBump` via LogUp-GKR within each MIPS32r2 shard; 26-bit `(shard, clk)` timestamps. | Cross-shard septic-extension KoalaBear EC multiset hash using Groth et al. hashless $x$-tweak map (`ePrint 2025/1503`). | Precomputes ELF `initial_global_cumulative_sum`. | Validates `MemoryLocal` shard deduplication before global accumulation, but its hashless map-to-curve faces the `ePrint 2026/590` algebraic critique below. |
| **Stwo / Cairo** (`https://zksecurity.github.io/stwo-book/cairo-air/main-components/index.html`; `https://deepwiki.com/starkware-libs/stwo-cairo/2.1-memory-management`) | **Non-deterministic write-once memory**: `MemoryAddressToId` + `MemoryIdToValue` (`Small` 72b vs `Big` 252b) + `VerifyInstruction` PC cache, all joined by Circle-STARK **LogUp with `mult` witnesses**. | Relocated contiguous write-once segments; component `claimed_sum` scalars summed to zero by verifier. | **Entire VM is write-once**: zero timestamps, zero read-writebacks, explicit `mult` per unique cell and `pc`! | **Closest semantic match to Sherlock**: proves that write-once memory + `mult`-weighted LogUp + deduplicated instruction fetch needs zero timestamps and zero Merkle trees. |
| **Valida** (`https://deepwiki.com/valida-xyz/valida/3.2-memory-chip`; `https://github.com/valida-xyz/valida`) | Registerless CPU operating directly on RAM; sorted `(addr, clk)` `MemoryChip` trace with `diff_inv` + LogUp bus and `counter_mult` range check. | Monolithic / STARK bus balance with `StaticDataChip` init. | `is_static_initial` flag for static data. | Sorting `(addr, clk)` across shards requires cross-shard permutation; LogUp rational sums avoid global sorting. |
| **Nexus** (`https://deepwiki.com/nexus-xyz/nexus-zkvm/4.4-memory-operations`) | BEGKN timestamped `LoadStoreChip` (`RamValPrev`, `RamTsPrev`) + `Range256Chip` verified via LogUp (`Init + Writes - Reads - Final = 0`). | Continuation boundary chips (`program_memory_boundary`). | Separate read-only `ProgramMemory` LogUp component. | Shows the 3x column overhead (`ValPrev`, `TsPrev`, `FinalCounter`) paid by mutable RAM that Sherlock's write-once `MEM` avoids. |
| **Matter Labs Airbender** (2025, `https://olegfomenko.dev/2025/09/15/memory.html`) | Sum/grand-product memory permutation using external challenges. | **Two-pass pre-commit**: commit chunked memory columns first (~5–10% overhead), draw global Fiat-Shamir challenges, prove chunks in parallel, verify transcript in recursion. | Read-only code/ROM shares challenge bus. | **Exact production precedent for `ZM-LogUp` Stage-1 `C_i` pre-commitment** + parallel leaf proving + recursive challenge check! |
| **Groth, Malvai, Miller, Zhang (Asiacrypt 2025)** (`https://eprint.iacr.org/2025/1503`) & **El Housni, Bünz (Asiacrypt 2026)** (`https://eprint.iacr.org/2026/590`) | Hashless map-to-curve relations (`23x` fewer constraints in EC-GGM) **vs** 2026 cryptanalysis exhibiting concrete algebraic forgery outside EC-GGM and high 2-adicity witness limits. | Proposed $y$-increment fix for $j = 0$ curves (`ePrint 2026/590`). | Directly governs EC multiset hash design. | Crucial security gate: rules out naive hashless try-and-increment on `BLS12-381 Fq` (2-adicity 32) / Jubjub ($j \neq 0$). |

---

## 5. Q5 — Concrete Recommendation and Verification Experiment

### 5.1 What to Prototype First in `crates/mori-chip`
Prototype **`ZM-LogUp` (`Candidate f1`)** incrementally in two tight steps inside `crates/mori-chip` (with `WO-LogUp` `(b1)` as Step 1a and Horner boundary state compression as Step 1b):

1. **New module `crates/mori-chip/src/zmmem.rs` (replacing `m3.rs` + unbuilt `PAGE`)**:
   - **Dense write-once `MEM`**: Remove `C = 8` `len = base + C` alignment (`m3.rs:132,437-440`); carry exact `running_len` in `LEN`.
   - **Intra-leaf `CACHE` provider region**: `K_m` older-cell read rows (`CHIP_MEM`, enforcing `addr < base` via ALU/gate so intra-leaf vs older reads partition cleanly, `D2` of `chipLeafCached.qnt:313-314`) + `K_p` instruction rows (`CHIP_PROG`), served over existing `bus0`/`bus1` (`0` new PLONK lookups).
   - **Stage-1 `Instance(0)` binding + LogUp + Horner (`1` `vsel` code column)**:
     - Code `1` (`q_com`): `a[11] - Instance(0) == 0` binding `[pre_regs(29), post_regs(29), writes(W_cap * 6), cache_reads((K_m + K_p) * 5)]`.
     - Code `2` (`q_horner`): 29-step Horner evaluation of `pre_fp` and `post_fp` in $\gamma$.
     - Codes `3, 4` (`q_logup_w`, `q_logup_r`): tuple compression $E_\gamma(\text{space}, \text{key}, a, b, c)$, guarded inverse `act * (inv * (alpha - enc) - 1) == 0` + `(1 - act) * inv == 0`, and running sum `sum_next = sum + term` outputting `sum_leaf` to `pi`.
   - **Multiplexed `RANGE` table (`range.rs`)**: Append `3,072` 5-bit `XOR`/`AND`/`OR` rows to `4,096` 12-bit range rows (`7,168` total in `k = 13`, `0` new columns).
2. **`ChipNode` mirror (`crates/mori-chip/tests/e3_spike.rs`)**:
   - Replace `assign_fixed(C::identity())` (`e3_spike.rs:227`) with `assign_without_subgroup_check(c_i)` (`MC:verifier/transcript_gadget.rs:142`), absorb `(C_i.x, C_i.y)` into associative chain `(com_pre -> com_post)`, enforce child `(gamma, alpha)` equality, and add `sum_acc = sum_prev + sum_leaf`.

### 5.2 Confirmation Experiment (Rows, Quint Model, and `MockProver` Negative Tests)

1. **`cost_model` Row Confirmation Gates**:
   - **Leaf (`S = 16`, E8-equivalent, 0 `C6` perms)**: confirm region rows drop from `6,511` to **`~550–700` rows** (`k = 13`, `0` Poseidon permutations) (`ESTIMATE`).
   - **Leaf (`S = 64`, with `K_m = 16`, `K_p = 8`, `P_c6 = 24` Poseidon perms)**: confirm region rows `<= 4,500` (`k = 13`) and at `S = 128` (`P_c6 = 48` perms) `<= 8,100` (`k = 13`) (`ESTIMATE`).
   - **Node (`ChipNode`, `k = 20`)**: confirm `cost_model` rows land in **`835,000–848,000`** (well under the `950,000` gate) and Decider `table_rows` remain **`1,032,467`** (`+0`) (`ESTIMATE`).
2. **Quint Soundness Model (`zmLogup.qnt`, forked from `qa/gemini-quint/chipleaf-final/m3.qnt` + `models/sherlock/quint/chipLeafCached.qnt`)**:
   - Model 2-leaf and 3-leaf chains (`ChainW`) over `PRIME = 1000003` (`m3.qnt:21`) with committed polynomial vectors `P_1, P_2`, derived `(gamma, alpha) = h2(d_root, prog_d)`, Horner boundary fingerprints, dense `len`, and `sum_1 + sum_2 == 0`, checking `invRefine` (`m3.qnt:792`), `invFrames` (`chipLeafCached.qnt:648`), and **partition independence** (`partId in {0, 1, 2}` producing identical memory addresses and outcomes, `aggregation.qnt:434`).
   - Require **all 7 seeded mutants to be caught with concrete counterexample traces**:
     1. `mult_uncommitted`: `m_w` excluded from `P_i` (chosen after `alpha`) -> refutes `invRefine` via forged read.
     2. `regs_uncommitted`: `pre`/`post` registers excluded from `P_i` before `gamma` -> refutes `invChainRefine` via register-state forgery satisfying `post_fp == pre_fp`.
     3. `no_read_bound`: drops ALU `addr_r < len_t` (`chipLeafCached.qnt:188`) -> refutes causality via read-before-write of a cell written in Leaf 2 during Leaf 1!
     4. `pole_unconstrained`: drops `act * (inv * (alpha - enc) - 1) == 0` -> refutes `invBus` via arbitrary `inv`.
     5. `pad_nonzero`: drops `(1 - act) * inv == 0` -> refutes `invBus` by canceling a forged read with an inactive write row.
     6. `space_tag_omitted`: omits `space` from $E_\gamma$ -> refutes `invBus` via cross-space `AS_PROG` vs `AS_MEM` confusion.
     7. `seal_sum_unchecked`: omits `sum_root == 0` at seal/decider -> refutes `invRefine`.
3. **`MockProver` Negative Tests in `crates/mori-chip`**:
   - Forged older-cell value `(addr_r, wrong_val)` with adjusted `sum_leaf` -> caught by Node/Seal `sum_root == 0`.
   - Forged `m_w` altered relative to `Instance(0)` `P_i` -> caught by `q_com` (`a[11] - Instance(0) == 0`).
   - Read at `addr_r == current_len` (future cell written in next slot) -> caught by ALU `OP_LT`.
   -Inactive `CACHE` padding row (`act = 0`) carrying nonzero `inv` -> caught by `(1 - act) * inv == 0`.
   - Boundary register mismatch between Leaf 1 `post` and Leaf 2 `pre` under fixed `(C_1, C_2)` -> caught by `post_fp_1 == pre_fp_2`.
   - Duplicate `HASH` memo key with different input arity `[a, b]` vs `[a, b, 0]` -> caught by `arity_domain` prefix in `AS_HASH`.

---

## 6. What Was Not Checked

1. **No builds, tests, `cost_model` runs, or `quint` runs were executed** (strictly forbidden by the study rules; all new row figures are `ESTIMATE` from the measured linear fits in `experiments-2.md` and `experiments-4.md`).
2. **Exact CPU wall-clock time of Stage-1 `commit_to_instances`** (`MP:plonk/prover.rs:36-49`) on sparse ~700-nonzero vectors at `k = 13` was estimated from MSM size rather than benchmarked (`midnight-proofs` calls `CS::commit_lagrange` over the full `2^13` vector unless sparse-lagrange skipping is used; even a dense `2^13 = 8,192` G1 MSM on BLS12-381 is tiny compared to `k = 20` Node proving, `ESTIMATE`).
3. **Full 108-rule C6 core width (`E0(c)`)**: Core slot height (`h = 13` in `Arch::Narrow`) and register count (`29` words in `rf.rs:34`) follow the E8 spike; if the full 108-rule C6 core widens `NDIG` from 29 to ~36 words, Horner boundary compression grows by +14 rows instead of +70–140 Poseidon rows (`ESTIMATE`).
4. **Binary PDFs of `ePrint 2025/105`, `ePrint 2025/1503`, and `ePrint 2026/590`**: Following tool guidance, HTML ePrint landing pages, technical summaries (`georgwiese.github.io`, `cronokirby.com`), and production docs (`docs.succinct.xyz`, `docs.openvm.dev`, `docs.zkm.io`, `zksecurity.github.io/stwo-book`, `olegfomenko.dev`) were read rather than binary PDFs; `docs.nexus.xyz` failed DNS resolution in this environment so `deepwiki.com/nexus-xyz/nexus-zkvm/4.4-memory-operations` (citing exact `nexus-zkvm` source lines) was used instead.
5. **Parallel study `research/sherlock-runlength-consensus-20261111/`** and other lanes (`lanes/A,C,D,E`) were not read per Round 1 independence rules.
