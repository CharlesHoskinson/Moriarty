# Lane D — Round 1: Processor Lookup Cache, Recursion Fit, `VerifierGadget` Mechanics, and System Interactions

Study: `sherlock-lookup-cache-20261111` (Round 1, independent analysis).
Repository state inspected: commit `9ffc2fe76f523453632cf3df03ca761095bc7c13` (`Cargo.toml:27-45` pins `midnight-ledger` tag `ledger-9.1.0.1` rev `a2b30128ca4b660b73a068e29b56b48ede8c9a74`, `midnight-zkir` tag `zkir-3.2.0` rev `446c7fecc6bd41dab99ebe89ab404e9d077d11a9`, `midnight-curves = "=0.3.1"`, `midnight-proofs = "=0.8.3"`, `midnight-circuits = "=7.3.0"`, `midnight-zk-stdlib = "=2.4.0"`).
Scope rule enforced throughout: general-purpose recursion means all of `GP-1` through `GP-4` (`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:1-58`, `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56-59`), without narrowing any clause.

---

## Executive Summary

In Midnight's BLS12-381 PLONK/KZG recursion (`midnight-proofs = 0.8.3`, `midnight-circuits = 7.3.0`), the cost function of a processor lookup cache is inverted relative to standalone STARK zkVMs:

1. **Leaf height is nearly free in the recursive verifier; leaf width and extra lookup arguments are expensive.** Doubling the leaf domain (`+1` in `k_leaf`) adds **4 rows** to the `k = 20` node (`research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md:25-35`) because `VerifierGadget` computes $x^{2^k}$ by `k` squarings (`midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:582-584`). By contrast, adding **one** `lookup_any` argument to the leaf adds **3 variable G1 commitments** (`permuted_input`, `permuted_table`, `product` in `midnight-circuits-7.3.0/src/verifier/lookup.rs:60-76`) to the node's non-native MSM (`collapse`), costing **+22,547 to +22,672 node rows** (`research/sherlock-processor-analogy-20261110/experiments-2.md:230-232`); adding one advice, equality, or fixed column adds **+7,298**, **+8,189**, or **+6,004** node rows (`openspec/changes/sherlock-vm-consensus-20261107/design.md:134`). Under `GP-4`'s `RunLift` circuit `R`, which verifies **two** leaf proofs `U` in one `k = 20` circuit (`research/sherlock-gp-architecture-20261110/architecture.md:72,206-219`), every variable G1 commitment in `U` is paid **twice** (`~45,200` rows in `R` per added leaf lookup, `ESTIMATE`).
2. **Cross-phase lookup challenges are hard-blocked in stock `VerifierGadget`, and two-pass workarounds violate streaming and `GP-1`/`GP-4` composition.** `VerifierGadget::parse_trace` asserts `cs.phases().count() == 1` (`verifier_gadget.rs:310`) and `eval_expression` panics on `Expression::Challenge(_)` (`midnight-circuits-7.3.0/src/verifier/expressions/mod.rs:49`). While a two-pass protocol using KZG-committed `Instance(0)` (`verifier_gadget.rs:298-301,435-436,628-634`) and `Instance(1)` challenge carry can emulate LogUp on stock `VerifierGadget`, it destroys single-pass streaming, conflicts with `GP-001` stop-and-resume (`requirements.md:56`), breaks independent subtree proving in `GP-4`, and requires an $O(N_{\text{cells}})$ teardown sweep.
3. **Sherlock's write-once memory (`design.md:82`) gives intra-leaf L1 page caching without challenges, timestamps, or multiplicities.** Because allocated cells (`DATA`, `FIELDS`, `ENV`, `SLOTS`, spilled `FRAMES`/`TASKS`) are immutable once written, functional set inclusion (`lookup_any` with zero user challenges) is already sound (`research/sherlock-processor-analogy-20261110/quint-check.md:16-30`). Opening aligned 8-cell pages once per leaf into an intra-leaf `PAGE` cache (`TAG_CELL` against `pre_d.mem_root`, `TAG_PROG` against `prog_root`) and replacing D3's unconditional `base + 8` leaf-close jump (`crates/mori-chip/src/m3.rs:23,31`) with a **dirty-gated, dense-index paged frontier** (`post.len = pre.len + used`, 61-bit page tree over 8-cell pages = $2^{64}$ cells) cuts per-step Merkle hashing by **3x–8x** (`ESTIMATE`), adds **zero** lookup arguments and **zero** cross-proof challenges, resolves `SI-12` (`dissent.md:198-202`) and the $2^{17}$-leaf D3 ceiling (`dissent.md:228-232`), and restores partition independence on `pre_d`/`post_d` themselves so Lean's `statement_independent_of_S` (`clean/fork/Clean/Sherlock/Aggregation.lean:1154-1165`) applies directly.

| Candidate | Verdict | Leaf $\Delta$Rows (`S=16`, `ESTIMATE`) | Node $\Delta$Rows (`ESTIMATE`) | Decider $\Delta$Rows (`ESTIMATE`) | Extra Challenges | `GP-1..GP-4` & `SRC-023` Status |
|---|---|---|---|---|---|---|
| **(a)** Intra-leaf `PAGE` read cache (8-cell pages, shared bus) | **ADOPT** | `-5,000` to `-18,000` vs uncached per-read Merkle walks | **`0` to `+6,460`** (0 new lookups; $\le 1$ packed code col) | **`0` circuit / `0` table** | None (`phases = 1`) | Preserves all; pair with dense paged `MEM` (`(f)`) |
| **(b1)** Offline RAM via `SecondPhase` LogUp / Spice | **REJECT (Blocked)** | N/A | Panics at `verifier_gadget.rs:310` & `expressions/mod.rs:49` | N/A | 1–2 `Challenge`s | Incompatible with stock `midnight-circuits-7.3.0` |
| **(b2)** Two-pass offline RAM via `Instance(0)` + `Instance(1)` | **REJECT** | `-3,500` to `-4,500` vs M3 (plus sweep leaves) | **`+18,000` to `+31,000`** (`+1` var G1, `+1..2` cols, `+3` `IVC_STATE`) | **`+4,000` to `+8,000` circuit** (`0` table if tags unchanged) | Run-wide two-pass Fiat-Shamir $\gamma$ | Breaks streaming, `GP-001` stop/resume & `GP-4` parallel subtrees |
| **(b3)** Single-pass Jubjub / ECMH memory checking | **REJECT** | **`+4,000` to `+15,000`** with stock `HashToCurveGadget` (+ sweep leaves) | **`+8,000` to `+16,000`** (`+4` `IVC_STATE` Jubjub coords + gates) | **`+600` to `+1,400` circuit** | None | Needs $O(N_{\text{cells}})$ halt sweep; violates `GP-1` constant halt |
| **(c1)** Fixed `DEC` dispatch ROM (`dec.rs` over ~108 rules) | **ADOPT** | `0` (108 rows hidden inside `k >= 13` domain) | **`0` to `+6,004`** (`+1` fixed `TableColumn` for control word) | **`0` / `0`** | None | Key is program-independent (`SVM-001`, `GP-1`) |
| **(c2)** Fixed `PROG` / `TYPE` / `CTOR` lookup tables | **REJECT** | N/A | N/A | N/A | None | **Violates `GP-1` & `SVM-001`** (bakes program into VK) |
| **(c3)** Intra-leaf `PROG_CACHE` (`TAG_PROG` 8-row pages in `PAGE`) | **ADOPT** | `-4,000` to `-14,000` vs per-step `prog_root` Merkle walk | **`0`** (reuses `PAGE` walker & 2-lane bus) | **`0` / `0`** | None | Full `GP-1`/`GP-2` fit; authenticates `ty_d` & const `vd` |
| **(d1)** Shared single-lookup limb `RANGE`/`ALU` (`12b` -> `16b` at `k=16`) | **ADOPT** | `-64` to `-192` ALU rows at `k=16` (`12 -> 8` limb cells/op) | **`0`** (keeps 1 `RANGE` lookup) | **`0` / `0`** | None | Full fit; pair with write-only induction range checks (`D7.5`) |
| **(d2)** Jolt-style multi-table cross-product `ALU` lookups | **REJECT** | `-100` to `-300` gate rows, **but `+65,536+` table rows** | **`+22,600` per added lookup** (`+6,004` per `TableColumn`) | **`0` / `0`** | None | Blows `k = 20` node and `GP-4` `RunLift` `R` |
| **(e)** Dedicated Poseidon memoization lookup table | **REJECT** | Near `0` net after `(a)+(c)+D7+SVM-012`; `+70` per unique hash | **`+22,600` to `+29,900`** if new lookup or 6th bus col | **`0` / `0`** | None | Redundant once page caching, LCA sharing & cached `vd` exist |
| **(f)** **Hybrid `DIPM`**: Dense-Index 64-bit Paged Merkle (`PAGE`+`MEM`+`PROG_CACHE`) + 2-row Core Bus Multiplexing | **RECOMMENDED FIRST PROTOTYPE** | Full leaf **`~18,000`–`42,000` rows** (`k = 15..16` at `S = 16`, `D = 61` w/ LCA sharing) | **`~823,466` to `844,224` rows** (105k+ below `950,000` gate) | **`929,166` circuit / `1,032,467` table** (`0` growth) | None (`phases = 1`) | Solves `D3` `2^17` cap, `SI-12`, `SRC-023`, `Aggregation.lean`, `GP-1..GP-4` |

---

## 1. Question 1 (`Q1`): What Should the Cache Hold? Evaluation of `(a)`–`(f)`

### 1.1 Architectural Baseline: Why the Leaf Is Currently a Hashing Machine

In experiment E8 (`narrow,1,pos_m3+r1+a1+pc`, `research/sherlock-processor-analogy-20261110/experiments-4.md:51-79`), the spike leaf at `S = 16` uses **6,511 rows** (`k = 13`), of which **6,090 rows (93.5%)** are 87 Poseidon permutations at 70 rows per permutation (`crates/mori-chip/src/m3.rs:53-54`, `design.md:10-11,134`):
- `16` permutations: 8 segment cell records `H(act, a, b, c)` at 2 permutations each (`m3.rs:9-10,60-62`);
- `7` permutations: binary segment tree over the 8 records (`m3.rs:11,64-70`);
- `34` permutations: two depth-17 Merkle walks (`old` from `0` to `pre.mem_root`, `new` from `seg_root` to `post.mem_root` sharing siblings and direction bits, `m3.rs:12-28`);
- `30` permutations: `pre_d` and `post_d` state digests over 29 register words plus `mem_root` at 15 permutations each (`m3.rs:29-32,89-93`).

Crucially, E8 does **not** yet include `PAGE` (reads of cells allocated in earlier leaves) or `PROG` reads (`design.md:12,136`). Without a lookup cache, adding uncached Merkle walks for `PROG` fetch (1 per step) plus 1–3 external cell reads per step (`ENV` lookup, `Constructor` field load, cross-leaf `FRAME`/`TASK` pop, `sherlock-value/1` child `vd` read) at depth 20–64 would add **2 to 4 Merkle walks $\times$ 20..64 compressions $\times$ 70 rows = 2,800 to 17,920 rows per step** (`ESTIMATE`), inflating `S = 16` from `6,511` rows to **51,000–293,000 rows** (`k = 16..19`, `ESTIMATE`) and blocking `S = 32` or `S = 64`.

At the same time, the existing chip bus (`crates/mori-chip/src/bus.rs:47-67`) already provides two 5-word `lookup_any` lanes `(chip, key, a, b, c)` over shared advice columns `cols.a[0..10]` gated by fixed selectors `q_req` and `q_prov`. Because Sherlock's memory is **write-once** (`design.md:82`), every provider chip (`DEC`, `PAGE`, `MEM`, `ALU`) is a mathematical **function** of `(chip, key)` (or `(chip, key, a, b)` for `ALU`). As confirmed by the 21-mutant Quint check (`research/sherlock-processor-analogy-20261110/quint-check.md:16-30,108`), set inclusion (`lookup_any`) is sound **without multiplicities, timestamps, or verifier challenges** whenever D1–D8 hold (`design.md:94-103`).

---

### 1.2 Candidate `(a)`: Intra-Leaf Memory Read Cache (`PAGE` + `MEM` Deduplicated Bus)

**Mechanism.** Instead of authenticating a Merkle path per memory read, `PAGE` holds a fixed-height region of `K_page` cache slots inside the leaf. Each active `PAGE` slot authenticates an external cell (or an aligned **8-cell page** of cells) below `base = pre_d.len` against `pre_d.mem_root` **once** via the `HASH` chip, and exposes provider tuples `(CHIP_MEM, addr, a, b, c)` with `q_prov = 1` on `cols.a[0..5]` (`bus.rs:50-61`). Every step in the `S = 16` window that reads `addr < base` issues a standard bus request `(CHIP_MEM, addr, a, b, c)`; steps reading cells allocated within the current leaf (`base <= addr < len_cur`) hit `MEM` on the **exact same bus** with `CHIP_MEM` (`quint-check.md:94-95`, D2 disjointness `addr < base` vs `addr >= base`).

**Cell Granularity vs 8-Cell Page Granularity (L1 Cache Line Effect).**
- **Single-cell `PAGE` slot:** Authenticating 1 cell at tree depth $D$ costs `2` record permutations + $D$ compressions (`22` perms = `1,540` rows at $D = 20$; `66` perms = `4,620` rows at $D = 64$, `ESTIMATE`), yielding **1** cached cell on the bus.
- **8-cell aligned page `PAGE` slot (cache line $P = 8$, matching M3's `SEG = 8` in `m3.rs:7,123`):** Authenticating one aligned 8-cell subtree costs `16` record permutations + `7` subtree compressions + $(D - 3)$ path compressions (`40` perms = `2,800` rows at $D = 20$; `84` perms = `5,880` rows at $D = 64$, `ESTIMATE`), and places **all 8 cells** `8p .. 8p + 7` on the bus (`8` provider rows). Even better, if the leaf stores each 8-cell page's **record digests** `r_0..r_7` and opens only requested records (or stores `H(act, a, b, c)` inline), an unrequested neighbor cell in the same 8-cell page needs **1 witness hash** (`0` extra permutations if its precomputed `r_j` sibling is given for single-cell read, or all 8 cells opened for spatial locality!). Opening all 8 cells amortizes to **5.0 perms/cell at $D = 20$** and **10.5 perms/cell at $D = 64$** (`ESTIMATE`) — spatial locality across constructor field slices (`SI-12`), adjacent environment bindings, and consecutive spilled frames/tasks (`D7`) gets **up to 8x hit amplification** per Merkle walk.
- **LCA Prefix Sharing Across Sorted `PAGE` Openings:** If `K_page = 4` page openings in a leaf are ordered by page index `p_0 <= p_1 <= p_2 <= p_3`, two pages in the same $2^{20}$ arena of a 61-level page tree share their top **44** Merkle levels! Verifying the shared upper prefix once from the lowest common ancestor (or chaining sparse subtree roots) cuts a 61-level walk for subsequent nearby pages down to **17 levels** (`17 * 70 = 1,190` rows instead of `4,270` rows, `ESTIMATE`).

**Verdict on `(a)`:** **ADOPT** at 8-cell page granularity on the existing 2-lane functional bus. Zero new lookup arguments, zero cross-proof challenges, zero node/decider row blowup.

---

### 1.3 Candidate `(b)`: Full-Run Offline Memory Checking Instead of a Merkle Tree

Offline memory checking (Blum et al.; Spice; Lasso/Twist & Shout; LogUp RAM) replaces per-leaf Merkle paths with running multiset fingerprints over `(addr, val, timestamp_or_multiplicity)` carried across the IVC chain and balanced at run end. On Midnight's pinned stack (`midnight-proofs = 0.8.3`, `midnight-circuits = 7.3.0`), there are exactly three ways to instantiate `(b)`. All three fail recursion or protocol requirements:

1. **Sub-variant `(b1)` — In-Proof Multi-Phase Challenge (`SecondPhase` Advice Columns):**
   - *How it works:* Commit Phase-0 memory trace columns, squeeze challenge $(\beta, \gamma)$ from the Poseidon transcript, and commit Phase-1 running LogUp sum / grand-product advice columns.
   - *Why it fails:* `VerifierGadget::parse_trace` hardcodes `assert_eq!(cs.phases().count(), 1)` (`midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:310`), and `eval_expression` panics with `Expression::Challenge(_) => panic!("We do not suport multi-phase yet")` (`midnight-circuits-7.3.0/src/verifier/expressions/mod.rs:49`). Upgrading `VerifierGadget` requires forking `midnight-circuits`, violating the stock-crate rule (`openspec/changes/sherlock-vm-consensus-20261107/requirements.md:43`, `research/sherlock-speed-replan-20261108/prover.md:55-60`).

2. **Sub-variant `(b2)` — Two-Pass `Instance(0)` KZG Pre-Commitment + `Instance(1)` Challenge Carry:**
   - *How it works (detailed mechanics in §2.1.4):* Pass 1 executes the whole run of $L$ leaves, packs each leaf's memory operations into the Lagrange basis of `Instance(0)` (`committed_instance_column`), computes KZG commitment $C_i \in \mathbb{G}_1$ (`midnight-proofs-0.8.3/src/plonk/prover.rs:36-49`), hashes $d_i = \text{Poseidon}(d_{i-1}, \text{enc}(C_i))$ across all $i \in [0, L)$, and derives $\gamma = \text{Poseidon}(\text{dom}, d_{L-1})$. Pass 2 supplies $\gamma$ in `Instance(1)` so Phase-0 gates check rational sums $\sigma_i = \sigma_{i-1} + \sum_j m_{i,j}/(\gamma - \text{enc}(op_{i,j}))$, while the node folds $d_i$, asserts $\gamma$ constancy, and checks $\sigma_{\text{final}} = 0$ at seal.
   - *Why it fails:*
     - **Destroys streaming and bounded disk/RAM for long runs (`GP-1`):** Leaf 0 cannot be proved until step $N$ finishes Pass 1 (`N` up to $2^{64}$ under `GP-1`, `general-purpose.md:14`). The prover must buffer or re-execute the entire witness trace of the run before generating its first leaf proof.
     - **Violates `GP-001` stop-and-resume (`requirements.md:56`):** `GP-001` requires that proving one run may stop and resume under the same key set with the same public statement. Under `(b2)`, stopping at step $k < N$ leaves zero Pass-2 leaf proofs (because $\gamma$ depends on steps $k..N$), whereas sealing `[0, k)` with its own $\gamma_1$ requires dumping and re-ingesting the entire live memory table at the checkpoint boundary.
     - **Conflicts with `GP-4` parallel tree/DAG proving (`general-purpose.md:44-50`):** Subtrees cannot prove independently without a global barrier after Pass 1.
     - **Requires $O(N_{\text{distinct}})$ teardown sweep leaves:** Even for write-once memory where each write emits $-\mu(\text{addr})/(\gamma - \text{tuple})$ and each read emits $+1/(\gamma - \text{tuple})$, the prover must know the **total future read count $\mu(\text{addr})$ across the entire run** at the leaf that allocates `addr` (impossible in Pass 1 without full-run lookahead!) OR emit a finalization sweep over every allocated cell at halt, making halt cost $O(N_{\text{cells}})$ instead of $O(1)$.
     - **Node row penalty:** Turning leaf `Instance(0)` from fixed `C::identity()` (`crates/mori-recursion/src/inner.rs:181`) into a variable witness commitment $C_i$ plus an extra advice/lookup binding and `+3` `IVC_STATE` lanes adds **`+18,000` to `+31,000` node rows** (`ESTIMATE`, §2.2).

3. **Sub-variant `(b3)` — Single-Pass Elliptic-Curve Multiset Hash (Jubjub ECMH):**
   - *How it works:* Map each `(addr, val, clk_or_count)` tuple to a prime-order embedded-curve point $H_{\mathcal{G}}(t) \in \mathbb{J}(\mathbb{F}_q)$ on Jubjub (`midnight-circuits-7.3.0/src/ecc/hash_to_curve/htc_gadget.rs:99-111`) and accumulate read/write multisets via native Edwards addition (`9` advice columns in `midnight-circuits-7.3.0/src/ecc/native/edwards_chip.rs`). No cross-leaf Fiat-Shamir challenge is needed; security reduces to Jubjub discrete log.
   - *Why it fails:*
     - **Prohibitive in-circuit `HashToCurveGadget` cost on stock `midnight-circuits-7.3.0`:** Each `hash_to_curve` executes Poseidon squeeze + **two** Shallue-van de Woestijne `map_to_curve` evaluations (`midnight-circuits-7.3.0/src/ecc/hash_to_curve/mtc.rs:55-260`, each performing `is_square`, two `sgn0` full-field bit decompositions, `inv0`, `sqrt`, rational curve evaluation, isogeny, and cofactor clearing) + Edwards addition = **`~600–1,200` leaf rows per curve hash** (`ESTIMATE`). Two curve hashes (read + write) for just 8 memory operations already cost **`9,600–19,200` rows** (`ESTIMATE`) — **more than M3's entire 6,090-row Poseidon budget**!
     - **Even with a custom hashless elliptic-curve relation** (Groth, Malvai, Miller, Zhang, `https://eprint.iacr.org/2025/1503`, supplying a deterministic nonce byte so `(H(t || byte), y)` lands on Jubjub in ~1 Poseidon + 3 field muls $\approx$ `80–90` rows `ESTIMATE`), write-once memory **still** requires either predicting total future read counts $\mu(\text{addr})$ at allocation time (impossible single-pass!) or timestamping every read `(addr, val, prev_clk) -> (addr, val, clk)` **and** sweeping all $N_{\text{cells}}$ ever allocated at halt to cancel final `(addr, val, last_clk)` states! Sweeping $2^{20}..2^{30}$ cells at halt violates `GP-1` (`requirements.md:56`).

**Verdict on `(b)`:** **REJECT** as a replacement for `mem_root`.

---

### 1.4 Candidate `(c)`: Instruction, Decode, Program, Type, and Constructor Tables

Candidate `(c)` mixes two fundamentally different classes of data that must be separated under `GP-1` (`general-purpose.md:9-14`):

1. **Static Machine Rule Decode (`DEC` — ISA-Fixed, Program-Independent):**
   - `DEC` maps `(trap_flags, ctl_tag, top_frame_tag, helper_tag, val_tags)` to `(rule_id, micro_op_control_word)` across the fixed ~108-rule inventory `R` of `sherlock-helper-microstep/1` (`requirements.md:9`, `SVM-001`).
   - In `crates/mori-chip/src/dec.rs:20-45`, `DEC` is a `meta.lookup` into fixed `TableColumn`s. Because `108` rows $\ll 2^{13} = 8,192$, `DEC` adds **0** leaf trace rows (fixed tables overlap active advice rows in Halo2/`midnight-proofs`, `midnight-proofs-0.8.3/src/dev/cost_model.rs:372-377`). Expanding `DEC` from 2 `TableColumn`s `(class, rule)` to 3 `TableColumn`s `(class, rule_id, packed_uword)` (packing up to 254 bits of micro-op control flags into one BLS12-381 scalar!) costs **+1 fixed column = `+6,004` node rows** (`ESTIMATE`), or **0 node rows** if `rule_id` and `uword` are packed into one scalar `rule_id + 2^16 * uword` unpacked by core gates!
   - **Verdict:** **ADOPT** (`DEC` remains a fixed ROM; pack `(rule_id, uword)` or add at most 1 fixed column).

2. **Program (`PROG`), Type (`TYPE`), and Constructor (`CTOR`) Tables as Fixed Lookup Tables:**
   - Making `PROG`, `TYPE`, or `CTOR` a `Fixed` / `TableColumn` changes the fixed commitments in `MidnightVK`, baking the program into the leaf, node, and decider keys. That directly violates `GP-1` (`general-purpose.md:9-11`), `SVM-001` (`requirements.md:9`), `design.md:84` (*"No program row is a fixed table"*), and fails Quint's `invShape` (`prog_in_key` mutant, `quint-check.md:107,154`).
   - **Verdict:** **REJECT as Fixed Tables.**

3. **Intra-Leaf `PROG_CACHE` (`TAG_PROG` Pages inside `PAGE` Committed Under `prog_root`):**
   - Census E0 (`design.md:119`) measured that across `fact(3)` (293 steps), `fold` (196 steps), and `list123` (303 steps), **any window of 16 steps reads at most 6 distinct `PROG` control-word rows**.
   - Storing the admitted program, its type-table canonical digests `ty_d(T)` (`SRC-022`, `requirements.md:47`), constructor arities/ordinals, and constant value digests `vd(const)` inside `prog_root` (`design.md:84,193`) and opening **`K_prog = 2` to `4` aligned 8-row program pages** (16 to 32 `PROG`/`TYPE`/`CTOR` entries!) per leaf in `PAGE` (`CHIP_PROG` on the **existing** 2-lane `BusConfig`) replaces 16–32 per-step `prog_root` Merkle walks (cf. E1's `731,523`-row ZKIR leaf, `design.md:124`) with **2 to 4 page walks per leaf** (`~2,800–5,600` rows total at depth 16–20, `ESTIMATE`) and **zero** extra lookup arguments!
   - **Verdict:** **ADOPT (`PROG_CACHE` as `CHIP_PROG` slots in `PAGE`).**

---

### 1.5 Candidate `(d)`: Decomposed `ALU` and `RANGE` Lookup Tables vs Gates

Profile `sherlock-helper-microstep/1` (`design.md:29`, `SVM-009`) operates on `UInt128` and `Bool` with checked `add`, `sub`, `mul`, `div`, `mod`, `eq`, `lt` and boolean logic (no arbitrary 64-bit bitwise `XOR`/`AND` hot loop as in SHA-256/Keccak RV32IM guests).

- **Why Jolt/Lasso-style cross-product `ALU` lookup tables lose here:** A 2-operand `8 x 8`-bit table `(op, a_byte, b_byte, out)` has $k_{\text{ops}} \cdot 2^{16}$ rows (`262,144` rows for 4 ops -> forces `k_leaf >= 19`, increasing leaf prove time from `~2.6 s` to `~35–47 s` `ESTIMATE`!). Worse, adding new `meta.lookup` arguments for `ALU` subtables costs **`+22,600` node rows per lookup** and **`+6,004` node rows per `TableColumn`** (`design.md:134`).
- **What wins in `mori-chip`:**
  1. **Keep the single running-sum limb `RANGE` lookup (`crates/mori-chip/src/range.rs:93-118`, 1 lookup, 1 `TableColumn`), and upgrade limb width to match `k_leaf` for free:** At `k_leaf = 13`, `TABLE_ROWS = 4,096` (12-bit limbs, 12 cells per `u128`). Once full-leaf HASH + CORE rows place `k_leaf` at `16` (`65,536` domain rows, gate `k_leaf <= 16` in `design.md:105`), widening `LIMB_BITS` from `12` to **`15` (`32,768` rows) or `16` (`65,536` minus blinding rows -> `15`-bit cleanly fits `32,768 <= 65,530`, or `16`-bit if `k_leaf = 17`, or `12`-bit at `k = 13..15`)** shrinks every `u128` decomposition from `12` rows to `9` rows (`15`-bit: $8 \times 15 + 8 = 128$ bits, `9` cells + 1 pad = `10` rows, `-17%` ALU rows `ESTIMATE`) at **zero node row cost** (`0` new columns, `0` new lookups).
  2. **Express `DIV`/`MOD` and `MUL` as non-deterministic witness + range-checked limb gates:** Rather than lookup tables, witness $(q, r)$ with $a = b \cdot q + r$ over limb carry chains (`4 x 32`-bit or `9 x 15`-bit schoolbook product with carry limbs checked in the **same** single `RANGE` lookup) and $0 \le r < b$ via one `LT` decomposition.
  3. **Enforce `D7.5` Induction Range Principle (`design.md:86`):** Range-check scalars **only** when newly written by a rule or initialized at `h_pre = 0`, never when carried across slots or read from authenticated `PAGE`/`MEM`.

**Verdict on `(d)`:** **ADOPT** single shared limb `RANGE` lookup + limb-carry arithmetic gates + `D7.5` write-only range checks; **REJECT** multi-table Jolt-style `ALU` lookups.

---

### 1.6 Candidate `(e)`: Intra-Leaf Poseidon Hash Memoization

**Mechanism evaluated:** A leaf-local memoization table `(CHIP_HASH_MEMO, in0, in1, in2, out)` allowing repeated Poseidon calls with identical inputs inside a 16-step leaf to look up `out` instead of running another 70-row permutation.

**Why a dedicated hash-memoization table fails the cost-benefit test:**
1. **Almost zero duplicate permutations remain once `(a)`, `(c3)`, `D7`, and `SVM-012` exist:**
   - Duplicate Merkle cell/instruction path hashes are **already** eliminated by `(a)` `PAGE` deduplication and `(c3)` `PROG_CACHE` (each page opened once per leaf), plus **LCA upper-prefix sharing** across sorted page walks (each shared internal node hashed once!).
   - Duplicate `sherlock-value/1` constructor sub-tree hashes are **already** eliminated by `SVM-012` (`requirements.md:20`), which computes `vd` **once** at value allocation and stores `vd` inside the write-once `DATA` record so parent constructors hash the cached scalar `vd(child)` instead of re-traversing child subtrees!
   - `pre_d` and `post_d` (`2 x 15 = 30` permutations) hash `h` / `pc` / `fuel` / `mem_root`, which change across any active leaf (`steps >= 1`).
2. **High tuple/column cost:** A rate-2 Poseidon state is 3 field elements `(s0, s1, cap) -> (s0', s1', cap')` (`6` elements). Exposing existing `HASH` rows on the **existing** 5-column bus (`crates/mori-chip/src/hashchip.rs` already emits `(CHIP_HASH, id, len, in_or_out, tag)`) costs `0` node rows when CORE needs a digest, whereas adding a *new* 6-column memoization bus or extra `lookup_any` argument costs `+7,298` to `+22,600` node rows (`ESTIMATE`) to save near-zero permutations.

**Verdict on `(e)`:** **REJECT** any new memoization lookup/column; capture all duplicate-hash elimination structurally via `(a)` page deduplication + LCA prefix sharing, `(c3)` `PROG_CACHE`, `D7`, and `SVM-012` cached `vd`.

---

### 1.7 Candidate `(f)`: Better Hybrid Design — Dense-Index 64-Bit Paged-Merkle Write-Once Architecture (`DIPM`) + Multi-Row Bus Multiplexing

Combining the best elements above with two structural upgrades produces **`DIPM`**, which simultaneously solves the lookup-cache problem (`Q1`–`Q2`) and all open protocol conflicts (`Q3`):

1. **64-Bit Dense-Index Paged Memory (`PAGE` + `MEM`, Page Size $P = 8$ Cells, Tree Depth $D = 61$ Pages = $2^{64}$ Cells):**
   - Replace D3's leaf-padded `len_post = base + 8` (`m3.rs:23,31`) with **exact dense logical `len`**: `post_d.len = pre_d.len + used`, where `used in 0..=N_MEM` (`N_MEM = 8`, or `16`) is the exact number of cells appended by enabled steps in this leaf.
   - Address `addr` (`0 <= addr < 2^64`) splits into `page_idx = addr >> 3` (`61` bits) and `slot_idx = addr & 7` (`3` bits).
   - **Dirty-gated frontier write-back (`K_frontier = 2` aligned 8-cell pages at `p_f = pre.len >> 3` and `p_f + 1`):**
     - If `used == 0` (`dirty = 0`): `post.mem_root == pre.mem_root` and `post.len == pre.len` (enforced by a mux gate `dirty * (new_root - post_root) + (1 - dirty) * (pre_root - post_root) = 0`). Zero-append compute leaves consume **zero** memory addresses (satisfying `GP-001`, `requirements.md:56`)!
     - If `used > 0` (`dirty = 1`) and `pre.len & 7 != 0`: the tail page `p_f` is opened from `pre.mem_root` in its pre-existing prefix `0 .. (pre.len & 7) - 1` (with unwritten tail slots `(pre.len & 7) .. 7` constrained to empty tag `0` in `pre.mem_root`), merged with the newly appended cells in `[pre.len, pre.len + used)` (spilling into empty page `p_f + 1` if `(pre.len & 7) + used > 8`), and written back along the **same** 61-bit sibling path(s) to produce `post.mem_root`!
     - **Soundness upgrade on D2:** `PAGE` read-only slots (`K_page_ro = 4` pages) serve `(CHIP_MEM, addr, a, b, c)` **only** for cells with `act_old == 1` and `addr < pre.len` (`LT` check), while `MEM` serves `pre.len <= addr < len_cur` (`act_new == 1`). Unwritten tail slots inside `p_f` have `act_old == 0`, putting `(0, 0, 0, 0, 0)` on the provider bus (`D8`), so `PAGE` and `MEM` remain strictly disjoint even inside a partially filled tail page!
2. **Separate Tag-Partitioned Dense Arenas (or Per-Table Roots) for `SI-12`:**
   - Either maintain dense `len_data` and `len_fields` (two 8-cell frontier pages in `MEM`, or partitioned high-tag subspaces `[table_id : 2 bits || dense_idx : 62 bits]` = $2^{62}$ cells per table!) so `DATA_FIELDS` indices are **100% dense contiguous integers `0, 1, 2, ...`** with **zero** interleaving from `FRAME`/`ENV` appends and **zero** leaf-close holes (`§3.3`).
3. **Intra-Leaf `PROG_CACHE` (`K_prog = 2..4` 8-Row Pages under `prog_root`):**
   - Serves instruction fetch, `ty_d(T)` for `Constructor` `vd` folding (`SRC-022`), and constant `vd` on the same 5-column bus.
4. **Zero-Lookup-Cost Core Bus Bandwidth Expansion (Time-Multiplexed Request Rows):**
   - In `Arch::Narrow` (`crates/mori-chip/src/rfcore.rs:169-200`), each step slot is 13 rows tall (`rows 0..12`), and rows `6` and `10` currently hold copy-replicas of row `2`'s `(L0, L1)`. Turning row `6` (plus 1 pair/replica row if needed) into a **second `q_req = 1` request row `(L2, L3)`** doubles per-step bus bandwidth from **2 to 4 bus requests/step** using the **exact same 2 `lookup_any` arguments** (`+0` node rows, `+16..32` leaf rows at `S = 16`, `ESTIMATE`)!

---

## 2. Question 2 (`Q2` — Lead Deep-Dive): Recursion Fit, `VerifierGadget` Audit, Node/Decider Costs, and End-to-End Wall-Clock

### 2.1 Line-by-Line Audit of `midnight-circuits-7.3.0` `VerifierGadget` and `crates/mori-recursion`

#### 2.1.1 Single-Phase Challenge Restriction (`cs.phases().count() == 1`)
Off-circuit, `midnight-proofs-0.8.3` supports multi-phase Halo2 circuits: `create_proof` loops over `cs.phases()`, commits advice columns belonging to `phase`, and squeezes `Challenge` values mapped into `challenges: HashMap<usize, F>` (`midnight-proofs-0.8.3/src/plonk/prover.rs:105`, `plonk/verifier.rs:85-99`).

In-circuit, `VerifierGadget` (`midnight-circuits-7.3.0/src/verifier/`) blocks multi-phase challenges at **two** hard panics:
1. `VerifierGadget::parse_trace` (`verifier_gadget.rs:308-320`):
   - Line `310`: `assert_eq!(cs.phases().count(), 1);`
   - Lines `314-316`: reads **all** `0..cs.num_advice_columns()` commitments in one unphased loop (`transcript.read_point(layouter)`) before squeezing `theta` (`line 319`), `beta` (`line 327`), `gamma` (`line 328`), and `trash_challenge` (`line 341`). Zero user `Challenge` squeezes exist anywhere in `parse_trace`.
2. `eval_expression` (`midnight-circuits-7.3.0/src/verifier/expressions/mod.rs:33-69`):
   - Line `49`: `Expression::Challenge(_) => panic!("We do not suport multi-phase yet"),`

Therefore, **any** leaf design that invokes `meta.challenge_usable_after(FirstPhase)` or allocates `SecondPhase` advice columns (standard in-proof LogUp, Plonkup dynamic challenges, Spice grand products, or GLC rational sums) panics during node `configure`/`synthesize` under pinned `midnight-circuits = 7.3.0`.

#### 2.1.2 Unit Rotation Restriction (`{-1, 0, +1}`)
In `VerifierGadget::verify_algebraic_constraints` (`verifier_gadget.rs:596-611`), evaluation points are precomputed only for `x_prev = x * omega^{-1}`, `x`, and `x_next = x * omega` (plus `x_last` for permutation chunking):
- Lines `604-610`: `match rotation.0 { -1 => &x_prev, 0 => &x, 1 => &x_next, _ => panic!("We do not support other rotations") }`.
Any leaf gate or `lookup_any` expression querying `Rotation(r)` with `|r| > 1` panics in node synthesis (guarded by `spike_leaf_vk_rotations_are_unit` in `crates/mori-chip/tests/e3_spike.rs`).

#### 2.1.3 How `assigned_committed_instances` (`Instance(0)`) Works in Fiat-Shamir and KZG
Every `ZkStdLib` circuit (`midnight-zk-stdlib-2.4.0/src/lib.rs:547-548`) and `mori-chip` leaf (`crates/mori-chip/src/leaf.rs:160-161`) allocates **two** `Column<Instance>` columns: `Instance(0)` (`committed_instances`) and `Instance(1)` (plain scalar `instance`). Currently, `crates/mori-recursion/src/inner.rs:106,181`, `ivc.rs:530-535`, and `run_decider.rs:221-227` bind `Instance(0)` to fixed `[C::identity()]`.

Line-by-line trace of what happens if `Instance(0)` carries a non-trivial Lagrange polynomial commitment $C_i = \text{commit\_lagrange}(v_i) \in \mathbb{G}_1$:
1. **Count check (`verifier_gadget.rs:285-288`):** `assert_eq!(cs.num_instance_columns(), assigned_committed_instances.len() + assigned_instances.len())`.
2. **Fiat-Shamir ordering (`verifier_gadget.rs:296-316`):**
   - Absorbs `assigned_vk.transcript_repr` (`line 296`).
   - Absorbs every `com` in `assigned_committed_instances` via `transcript.common_point(layouter, com)` (`lines 298-300`), which encodes `AssignedForeignPoint` into **5 native scalars** (`point_pi_len() == 5`, `inner.rs:47-49`, `transcript_gadget.rs:100-112`) and absorbs them into the Poseidon sponge.
   - Absorbs `assigned_instances` (`Instance(1)`) length and scalars via `common_scalar` (`lines 302-306`).
   - **Crucial consequence:** Both `Instance(0)` and `Instance(1)` are absorbed **before** `advice_commitments` (`lines 314-316`). Hence, within a *single* proof, an `Instance(1)` scalar cannot depend on that same proof's advice columns, **but** an `Instance(1)` scalar in Pass 2 **can** depend on `Instance(0)` commitments computed in Pass 1!
3. **Evaluation & KZG opening (`verifier_gadget.rs:432-448, 625-639, 671-678`):**
   - If the leaf CS queries `Instance(0)` at `rot` (`meta.query_instance(committed, Rotation::cur())`), `column.index() < nb_committed_instances` (`line 435`) is true: instead of computing an $O(|\pi|)$ Lagrange inner product (`lines 438-445`), `VerifierGadget` reads the evaluation scalar directly from the proof transcript (`transcript.read_scalar`, `line 436`) and emits a KZG `VerifierQuery` on `&assigned_committed_instances[0]` at `get_point(&rot)` (`lines 628-634`), folding `assigned_committed_instances[0]` into `kzg::multi_prepare` (`lines 671-678`).
4. **Subgroup subtlety (`transcript_gadget.rs:142` vs `AssignmentInstructions::assign`):** Points read from a proof via `read_point` use `S::assign_without_subgroup_check` (`transcript_gadget.rs:142`), whereas assigning `C_i` as a witness `AssignedForeignPoint` in the node via `AssignmentInstructions::assign` (`run_decider.rs:259`) enforces subgroup membership unless `assign_without_subgroup_check` is exposed and justified under KZG binding.

#### 2.1.4 Exact Cost of `lookup` / `lookup_any` in `VerifierGadget`
`midnight-proofs-0.8.3` implements Halo2's Plookup-style permuted lookup (`verifier/lookup.rs:39-161`, `verifier/expressions/lookup.rs:31-149`), **not** LogUp:
- Each `meta.lookup` or `meta.lookup_any` adds **3 variable G1 points** (`permuted_input_commitment`, `permuted_table_commitment` at `lookup.rs:60-61`; `product_commitment` at `lookup.rs:75`), **5 scalar evaluations** (`lookup.rs:90-94`), **5 vanishing polynomial identities** `id_1..id_5` (`expressions/lookup.rs:55-148`), and **5 `VerifierQuery` openings** across `x`, `x_next`, `x_prev` (`lookup.rs:120-160`).
- Because 3 variable G1 bases enter `next.collapse(l, bls, scalar)` (`ivc.rs:563`) at `~7,300` node rows per variable G1 MSM base (`design.md:134`), **each leaf lookup argument costs `+22,547` to `+22,672` node rows** (`experiments-2.md:230-232`). By contrast, widening an *existing* lookup's tuple by 1 expression adds only 2 Horner `mul_add` steps in `compress_expressions` (`expressions/lookup.rs:99-116`), costing **`~10–30` node rows** (`ESTIMATE`) if it reuses existing columns!

---

### 2.2 Exact Marginal Row Model in the `k = 20` Node and `k = 20` Decider

#### 2.2.1 `k = 20` Node (`SHKIVC06`, Baseline `823,466` Rows, Gate `950,000`, Usable `1,048,570`)
From measured node runs (`design.md:129-135`, `experiments-2.md:208-238`, `experiments-4.md:51-79`, `experiments/e2-e4.md:25-35`):
- **Leaf advice column (`meta.advice_column()`):** **`+7,298` node rows** (`1` variable G1 commitment read + `1` non-native MSM base in `collapse` + query evals).
- **Leaf equality column (`meta.enable_equality()`):** **`+8,189` node rows** (`1` fixed permutation commitment resolved via `leaf_acc.resolve_fixed_bases` at `inner.rs:207` into a variable base of `next.collapse` at `ivc.rs:563`, plus permutation product chunking every `degree - 2 = 4` columns, `midnight-proofs-0.8.3/src/dev/cost_model.rs:194-195,323`).
- **Leaf fixed / `TableColumn` / unpacked `Selector`:** **`+6,004` to `+6,462` node rows** (`1` distinct fixed G1 commitment resolved into `next.collapse`; `experiments-4.md:84-106`). Packing disjoint selectors as integer codes $c \in \{1..n\}$ in a shared fixed column (`crates/mori-chip/src/vsel.rs`, `n + d_body <= 6`) adds **`0` node rows**!
- **Leaf `lookup` / `lookup_any`:** **`~22,600` node rows** (`3` variable G1 commitments).
- **Leaf `k_leaf` (`+1` doubling of leaf rows):** **`+4` node rows** (`experiments/e2-e4.md:25-35`: `k = 13 -> 741,708` to `k = 20 -> 741,736`).
- **Variable leaf `Instance(0)` commitment (if used by `(b2)`):** **`+7,300` to `+11,500` node rows** (`ESTIMATE`: replacing constant `C::identity()` with a witnessed foreign G1 point adds foreign-point assignment/on-curve checks + `5`-scalar `common_point` Poseidon absorption + `1` non-trivial variable base in `collapse`).
- **Extra `IVC_STATE` scalar in `ivc.rs:68,502,523`:** **`+150` to `+350` node rows per scalar** (`ESTIMATE`: `1` `common_scalar` absorption + `1` Lagrange polynomial evaluation & inner product term at `verifier_gadget.rs:423-445` + genesis `select` check at `ivc.rs:500-504`). Note that `self` fixed bases in `SHKIVC06` (`46` fixed/perm bases -> `58` accumulator carry scalars, `qa/sherlock/manifests/node.json`) only grow if the **node's own** fixed/equality column count changes!

#### 2.2.2 `k = 20` Run Decider (`929,166` Circuit Rows, `1,032,467` Table Rows, Usable `1,048,570`)
1. **Zero sensitivity to leaf CS changes:** Because `inner.rs:207` resolves all leaf fixed and permutation bases inside the node before `ivc.rs:563` collapses `next` into two G1 points (`12` public-input scalars), **changing leaf columns, lookups, gates, or `k_leaf` costs `0` circuit rows and `0` table rows in `RunDeciderRelation`** (`design.md:117`, `experiments/e2-e4.md:35`).
2. **Exact proof of why `table_rows = 1,032,467` (`16,103` usable rows from `k = 21`!):**
   - `MidnightCircuit::synthesize` sets `max_bit_len = (self.k - 1) as usize = 19` (`midnight-zk-stdlib-2.4.0/src/lib.rs:1865`) and loads `Pow2RangeChip::load_table` (`lib.rs:1890`, `midnight-circuits-7.3.0/src/field/decomposition/pow2range.rs:215-236`).
   - `Pow2RangeChip::load_table` concatenates $2^b$ rows for `b = 0` (`line 221`) plus every distinct bit-length `b in 1..=19` present in `self.queried_tags` (`line 222`).
   - Binary decomposition of `1,032,467`:
     $$1{,}032{,}467 = 2^0 + 2^1 + 2^4 + 2^8 + 2^{14} + 2^{15} + 2^{16} + 2^{17} + 2^{18} + 2^{19}$$
     (`1 + 2 + 16 + 256 + 16,384 + 32,768 + 65,536 + 131,072 + 262,144 + 524,288 = 1,032,467`).
   - With `blinding_factors = 5` (`nb_unusable_rows = 6`, `cost_model.rs:347-379`), usable rows at `k = 20` are $2^{20} - 6 = 1{,}048{,}570$, leaving **only `16,103` table rows of margin**!
   - **Critical decider trap for accumulator/carry edits:** The unqueried `Pow2Range` tags in `RunDeciderRelation` are `{2, 3, 5, 6, 7, 9, 10, 11, 12, 13}`, whose sum is $16{,}108 > 16{,}103$! Any decider addition (e.g., range-checking new carry fields with non-power-of-two remainder bit lengths that touch `{13, 12, 11, 10}` = `+15,360` table rows, plus `{9, 7}` = `+640` -> **`1,048,467`**, or all remaining tags = `1,048,575 > 1,048,570`!) **silently forces `min_k = 21` via `table_rows` even when `circuit_rows` is 100,000 rows below 1,000,000**! Any decider scalar decomposition must stick strictly to already-loaded tags (`8`-bit bytes via `assigned_to_le_bytes`, `1`/`4`/`16`-bit chunks, or `19`-bit limbs).

---

### 2.3 End-to-End Prover Wall-Clock Trade-Offs (`S`, Leaf Size, Parallelism, and RSS)

Measured timings on the 20-core / 120 GB ARM64 host (`qa/sherlock/decisions/speed-levers.json:17-45,90-110`, `research/sherlock-speed-replan-20261108/prover.md:10-42`, `experiments/e2-e4.md:11-20`):
- **`k = 20` node step:** `61.07 s` solo (16 threads) / `64.95 s` (20 threads), `16.6–17.6 GB` RSS (`d = 0.884` domain-proportional work: MSM + NTT + quotient H).
- **`k = 20` run decider:** `70.35 s`, `17.35 GB` RSS.
- **Leaf prove time vs `k_leaf`:** `k = 13` (`8,192` domain): **`2.55 s`** (`prover.md:11`); `k = 20` (`1,048,576` domain, `742,327` rows in E4): **`66.2 s`**, `11.1 GB` RSS (`experiments/e2-e4.md:11-18`). Scaling the `d = 0.884` domain-proportional component across powers of two yields (`ESTIMATE`):
  - `k = 14` (`16,384`): `~3.1 s`, `< 0.5 GB` RSS (`ESTIMATE`)
  - `k = 15` (`32,768`): `~4.1 s`, `< 0.8 GB` RSS (`ESTIMATE`)
  - `k = 16` (`65,536`): `~6.2 s`, `~1.2 GB` RSS (`ESTIMATE`)
  - `k = 17` (`131,072`): `~10.5 s`, `~2.0 GB` RSS (`ESTIMATE`)
  - `k = 18` (`262,144`): `~19.0 s`, `~3.8 GB` RSS (`ESTIMATE`)
  - `k = 19` (`524,288`): `~35.5 s`, `~6.8 GB` RSS (`ESTIMATE`)
- **Memory-bus contention (`273 GB/s` unified LPDDR5X, `prover.md:65-70`):** Running 3 concurrent `k = 20` node provers at 6 threads slows each step to `~128–131.5 s` (`f3 = 1.69–1.74x`, `47.7 GB` aggregate RSS, `qa/sherlock/budgets.json:294-310`); overlapping one `k = 20` leaf with one `k = 20` node in E4 stretched `90.2 s` (`experiments/e2-e4.md:19`).

**Makespan Law for an $M = 303$-Step Run (`list123`, `SRC-011` Cap = 320 Steps):**
With linear IVC (`SHKIVC06`), node step $i$ requires node proof $i - 1$, so the $\lceil M/S \rceil$ node steps are **strictly serial**, while leaf proofs can run ahead in a burst or pipeline:
$$T_{\text{makespan}}(M, S) \approx T_{\text{setup}} + \lceil M/S \rceil \cdot t_{\text{node}} + t_{\text{leaf},0}(S) + t_{\text{decider}} + T_{\text{verify}}$$
(when leaves run ahead in a batch of $P$ workers instead of contending with the serial node, total wall is $\lceil M/S \rceil \cdot t_{\text{leaf}}(S)/P + \lceil M/S \rceil \cdot t_{\text{node}} + t_{\text{decider}}$ with **zero** memory-bus slowdown on `t_node`!).

| Regime (`M = 303` steps) | Leaf `k` (`ESTIMATE`) | Leaves $\lceil 303/S \rceil$ | Leaf Batch Time ($P=3$, `ESTIMATE`) | Serial Node Chain (`62 s`/step, `ESTIMATE`) | + Decider (`70 s`) + Setup/Verify (`~65 s`) | Net Wall-Clock (`ESTIMATE`) |
|---|---|---|---|---|---|---|
| Uncached per-read Merkle, `S = 16` | `k = 18..19` (`19–36 s`) | `19` | `~140–240 s` | `1,178 s` | `+135 s` | **`~24.2–25.9 min`** |
| **Cached `DIPM` (`(f)`), `S = 16`** (`S_MAX = 16` today) | **`k = 15..16` (`4.1–6.2 s`)** | **`19`** | **`~28–42 s`** | **`1,178 s`** | **`+135 s`** | **`~22.3–22.6 min`** |
| **Cached `DIPM` (`(f)`), `S = 32`** (if `S_MAX` raised to 32) | **`k = 16..17` (`6.2–10.5 s`)** | **`10`** | **`~25–40 s`** | **`620 s`** | **`+135 s`** | **`~13.0–13.3 min` (`-42%`!)** |
| **Cached `DIPM` (`(f)`), `S = 64`** (if `S_MAX` raised to 64) | **`k = 17..18` (`10.5–19 s`)** | **`5`** | **`~22–38 s`** | **`310 s`** | **`+135 s`** | **`~7.8–8.1 min` (`-65%`!)** |
| **GP-4 Binary Tree (`R` lifts 2 leaves + `N` tree, `P = 3`), `S = 32`** | **`k = 16..17`** | `10` (`5` `R` + `4` `N`) | `~35 s` | `3` tree levels $\approx$ `~330 s` | `+200 s` (`C + D`) | **`~9.4 min` (`ESTIMATE`)** |
| Bad trade: Leaf adds `+4` lookups -> Node hits `k = 21` (`~128 s`), `S = 16` | `k = 14` (`3.1 s`) | `19` | `~20 s` | **`2,432 s`** (`k = 21`!) | `+140 s` | **`~43.2 min` (`+92%` slower!)** |

**Three quantitative conclusions from the makespan table:**
1. **At fixed `S = 16`, shrinking a `k = 16` leaf to `k = 14` saves `< 25 s` out of `~1,340 s` (`< 2%`), whereas pushing the node to `k = 21` costs `+1,250 s` (`+92%`)!** Never trade node columns/lookups to shave leaf rows once `k_leaf <= 16`.
2. **The real wall-clock payoff of `PAGE` + `PROG_CACHE` (`DIPM`) is enabling larger `S` (`S = 32` or `S = 64` once `S_MAX` in `SRC-014` is raised) and keeping `U` narrow enough for `GP-4`'s 2-leaf `RunLift` `R`!** Raising `S` from `16` to `32` or `64` cuts the `k = 20` node count in half or by `3.8x`, slashing `list123` wall-clock from **`~22.5 min` to `~13.1 min` (`S = 32`) or `~8.0 min` (`S = 64`)** (`ESTIMATE`), because `DIPM` keeps per-step hashing sublinear in `S` (amortizing page openings across 32 or 64 steps!).
3. **Batching `k = 15..16` leaves ahead of the node chain avoids LPDDR5X bandwidth contention and keeps streaming RSS under `< 20 GB`:** 3 concurrent `k = 16` leaf provers use `< 4 GB` total RSS (`ESTIMATE`) and finish 19 leaves in `~40 s` before the single 20-thread `k = 20` node prover (`17.6 GB` RSS) runs unimpeded at full `61–65 s`/step.

---

## 3. Question 3 (`Q3` — Lead Deep-Dive): System & Protocol Interactions

### 3.1 C6 Outcome-Only Publication (`SRC-006`, `SRC-007`), `SRC-023`, and Lean `Aggregation.lean`

Under `SRC-006` and `SRC-007` (`requirements.md:31-32`, `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:148-156`), the decider exposes `[prog_d, input_d, outcome_d, n_steps]`, replay protection keys on `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`, and `outcome_d` uses the address-free value digest `sherlock-value/1` (`SRC-022`, `requirements.md:47`). Internal state digests (`pre_d`, `post_d`, `cur_d`, `mem_root`, `len`) stay inside recursion.

**Subtle Formal Gap Audited in `clean/fork/Clean/Sherlock/Aggregation.lean`:**
Inspecting `Aggregation.lean` (`lines 184-214, 960-976, 1136-1165`) reveals how partition independence is currently proved in Lean:
- `Premises` (`Aggregation.lean:184-205`) declares a single `State : Type`, deterministic `Step : Prog → State → State → Prop` (`det` at `line 1137`), and `digest_inj : ∀ s t, Carrier s → Carrier t → digest s = digest t → s = t` (`line 201`), used by `sound_link` (`line 964`) to turn `a.post = b.pre` (digest equality in the node) into `sa_post = sb_pre` (state equality).
- `result_unique` (`lines 1136-1150`) and `statement_independent_of_S` (`lines 1154-1165`) conclude:
  `root₁.hi = root₂.hi ∧ root₁.post = root₂.post ∧ r₁ = r₂`.

Notice what breaks under D3 (`m3.rs:23,31`): closing a leaf sets `post_d.len = base + 8` even if the leaf appended fewer than 8 cells! Consequently, under D3:
1. Two valid partitions of the same run (`S₁ = 1` vs `S₂ = 16`) produce **different** physical states (`len`, `fp`, stored links, `mem_root`) and therefore **different** `root₁.post ≠ root₂.post`! `Aggregation.lean`'s `statement_independent_of_S` **cannot be instantiated directly over raw D3 physical states** without inserting a non-machine "leaf-close padding transition" and refactoring `Aggregation.lean` to prove quotient equivalence (`proj : State_phys → State_log`, `SVM-011` at `requirements.md:19`).
2. **How `DIPM` (`(f)`) resolves this cleanly:**
   - In `DIPM`, `PAGE` and `PROG_CACHE` live **purely in ephemeral leaf witness rows** — their cache contents and hit counts never enter `pre_d`, `post_d`, or `mem_root`.
   - Even better, because `DIPM` advances `len` by **1 per allocating step and 0 at leaf close** (`post.len = pre.len + used`) and updates the canonical 61-bit page-Merkle `mem_root` with the exact active prefix `[0, post.len)` (gated on `dirty = (used > 0)`), **`pre_d` and `post_d` themselves become 100% invariant to leaf boundaries**!
   - Therefore, proving the same run under partitions `[16]`, `[8, 8]`, or `[1, 5, 10]`, or stopping and resuming (`GP-001`), yields the **exact same `mem_root`, exact same `len`, exact same `fp`/`tp` addresses, and exact same `post_d`**, so `Aggregation.lean`'s `digest_inj`, `result_unique`, and `statement_independent_of_S` (`lines 201, 1136-1165`) instantiate **literally unchanged** while C6 outcome-only (`SRC-006`, `SRC-022`) still keeps internal digests private!

---

### 3.2 Run-Length & Memory Ceiling (`D3` `2^17` Cap vs `GP-1` / `GP-001` `2^64` Bound)

`GP-001` (`requirements.md:56`) and `general-purpose.md:14-16` impose four binding rules:
1. Run length bounded only by `h < 2^64` (`SRC-004`).
2. *"No leaf count, memory address counter, segment index or other circuit constant SHALL make a run of fewer than 2^64 steps unprovable."*
3. *"A step that allocates no memory SHALL consume no memory address."*
4. *"The proving of one run MAY stop and resume under the same key set; the resumed run SHALL give the same `prog_d`, `input_d`, `outcome_d` and `n_steps` as the uninterrupted run."*

D3 (`m3.rs:14-28`, `dissent.md:228-232`) violates (2) and (3): its 17-bit segment walker caps a run at $< 2^{17} = 131,072$ leaves and $2^{20}$ cells, and consumes 8 addresses even on a 0-append leaf.

**How `DIPM` (`(f)`) satisfies all four clauses simultaneously:**
- **64-bit cell address space (`D = 61` page tree $\times$ `8` cells/page = $2^{64}$ cells per table, or `D = 61` over 4 tag-partitioned $2^{62}$-cell arenas):** No step below $2^{64}$ can exhaust the address counter (`1` step appends $\le 3$ cells in E0 census, `design.md:119`; if a step can ever append up to 4 cells, $2^{64}$ total cells or a 64-bit page tree = $2^{67}$ cells guarantees zero exhaustion below $2^{64}$ steps!).
- **Dirty-gated write-back (`dirty = (used > 0)`):** Steps that allocate no memory increment `len` by `0`; leaves with `used == 0` leave `mem_root` and `len` untouched (`0` addresses consumed).
- **Why `PAGE` caching is what makes 61-level Merkle trees cheap:** A naive 61-level Merkle walk per cell read would cost `61 * 70 = 4,270` rows **per read**. With 8-cell `PAGE` caching + **LCA prefix sharing** (sorting page requests within the leaf so pages sharing their upper 41–45 zero-padded address bits share one upper walk to their LCA!), a leaf whose working set lives below $2^{20}$ cells pays the 44-level upper spine **once per leaf** (`44 * 70 = 3,080` rows `ESTIMATE`, or `2 x 44 = 88` compressions = `6,160` rows for old+new frontier spine) and only **17 levels per additional local page** (`1,190` rows `ESTIMATE`)!
- **Multi-leaf continuation microsteps (`GP-001`):** Any single architectural operation whose data spans more than one leaf's chip capacity (`e.g.`, constructing or unfolding a massive ADT) decomposes into 1-field-per-step microsteps (`BuildPhase::Copy` in `owner_runtime.rs:2985-3020` already copies 1 field per microstep!), closing leaves freely between microsteps.

---

### 3.3 `SI-12..SI-14` Field Slices (`dissent.md` `D-1` and `crates/mori-core`)

In `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs`:
- `BuildPhase::Copy` (`lines 2989-3018`) checks `pre.helper.field_start + pre.helper.src_cursor == pre.img.active.data_fields`, appends one field per microstep into `img.data_fields`, and seals `SliceC::normalized(pre.helper.field_start, src.len)`.
- `SI-12` (`lines 4725-4741`) and `SI-13` (`lines 4744-4755`) check contiguous slice indexing `start .. start + len` inside `img.data_fields` and child-before-parent ordering `child_id < owner_id`.

Under D3 (`C = 8`, unified `MEM` log), `SI-12` fails for **three** reasons (`dissent.md:198-202`, `review.md:51-55`): (i) leaf close mid-`Build` jumps `len` to `base + 8`, punching zero-padding holes inside `[field_start, field_start + len)`; (ii) a constructor with `> 8` fields (`GP-001` acceptance test explicitly requires *"builds a constructor of more than 8 fields across a leaf boundary"*, `requirements.md:56`!) spans multiple segments; and (iii) interleaved frame/env spills in a single `MEM` log break contiguous `data_fields` offsets.

**Comparison of the three `SI-12` solutions under `PAGE` caching:**

| Option | How it works with `PAGE` | Pros & Cons |
|---|---|---|
| **Option S1 (Recommended in `DIPM`): Separate Dense `FIELDS` Subspace (`[TAG_FIELDS \|\| dense_idx]`) with Dense Paged `MEM`** | Partition the 64-bit tree by a 3-bit arena prefix (`001 = DATA`, `010 = FIELDS`, `011 = ENV`, `100 = SLOTS`, `101 = FRAMES/TASKS`, each $2^{61}$ dense indices = $2^{58}$ 8-cell pages). `len_fields` advances **only** on `FIELDS` appends and **never** jumps on leaf close! | **`SI-12`, `SI-13`, `SI-14` hold verbatim (`start + k` works across any number of leaves and for any arity > 8!)**; spatial locality in `PAGE` is **maximal** (an 8-cell `FIELDS` page fetches 8 consecutive constructor fields in 1 Merkle opening!); 100% partition-independent (`post_d` invariant to `S`). Cost: tracking `len` per arena + `1` extra dirty-gated frontier walk only in leaves that append to multiple arenas (`MEM` appends average `0.20/step`, `design.md:119`, so `K_frontier = 2` covers almost all 16-step windows!). |
| **Option S2 (`dissent.md:201` Linked Slices):** `(TAG_SLICE, elem_ref, prev_link)` cons cells in unified dense `MEM` | `BuildPhase::Copy` prepends/links `(elem_ref, prev_link)`; arity $\le 2$ inlined in `DATA` record. No rule ever computes `start + k`. | Needs only **one** frontier pointer `len` in unified `MEM`! However, random $O(1)$ field projection `proj(ctor, i)` becomes an $O(\text{arity})$ linked-list walk (requiring multiple steps or multiple `PAGE` pointer-chasing hops for arity > 8) and diverges from `owner_runtime.rs:2990,4725` slice representation. |
| **Option S3 (`dissent.md:199` Logical-to-Physical Map):** Authenticate `logical_id -> phys_addr` in an index tree | Rules compute `logical_id = start + k`, then look up `phys_addr` in an authenticated map before reading `MEM[phys_addr]`. | Doubles external read indirection (`2` Merkle trees + `2` bus lookups per field read) and leaves physical `mem_root` partition-dependent. Strictly dominated by **S1**. |

---

### 3.4 `GP-1..GP-4` Universal Recursion (`{U, R, F, N, C, D}` in `architecture.md`)

In `research/sherlock-gp-architecture-20261110/architecture.md:67-108,206-248`, the universal fixed-key recursion comprises 6 circuits: `U` (universal step leaf), `R` (`RunLift`: verifies **1 or 2 `U` proofs** -> `SEG` or 1-segment `SET`), `F` (`ForeignLift`: verifies foreign `Compact`/PLONK proof against `reg_root` + adapter `A_c` -> `SET`), `N` (binary `k = 20` node over `{N, R, F}`), `C` (closing fixed-base resolution), and `D` (pairing decider -> `SherlockClaims`).

**Critical Quantitative Constraint from `R` (`RunLift`) on Leaf Lookups:**
- `SHKIVC06` verifies **1** `self` proof + **1** `leaf` proof (`823,466` rows).
- `R` (`architecture.md:206-219`, estimated at `~600k–800k` rows) verifies **2** `U` proofs in one `k = 20` circuit (`R` resolves `U`'s fixed/permutation bases once, **but** runs `VerifierGadget::prepare` **twice**)!
- Therefore, in `R`:
  - Every **fixed/equality** column added to `U` is resolved **once** (`~6,000–8,200` rows in `R`, `ESTIMATE`), plus modest scalar combination overhead.
  - Every **advice** column added to `U` contributes **2** variable G1 commitments to `R`'s `collapse` MSM (**`~14,600` rows in `R`**, `ESTIMATE`)!
  - Every **`lookup_any`** argument added to `U` contributes **6** variable G1 commitments (`2 proofs x 3 commitments`) to `R`'s `collapse` MSM (**`~45,200` rows in `R`**, `ESTIMATE`)!
- Keeping `U` at **4 lookups** (`2` bus lanes + `1` `RANGE` + `1` `DEC`) and **12–13 advice columns** keeps `R` safely inside `k = 20` (`< 850,000` rows `ESTIMATE`), whereas adding 4–5 new lookup tables in `U` would add `~180,000–226,000` rows to `R` and blow `R` past `k = 20` into `k = 21`!

**Compositional Isolation Across `GP-2`, `GP-3`, and `GP-4`:**
- **Associative `SEG + SEG -> SEG` (`GP-4` trees):** Because `DIPM`'s `PAGE` and `PROG_CACHE` flush/authenticate strictly against canonical `pre_d.mem_root` and `post_d.mem_root` with zero cross-leaf challenges, `N` checks `left.post_d == right.pre_d` (`architecture.md:83`) identically regardless of tree bracketing.
- **DAG Fan-Out & Cross-Run Consumption (`GP-2` / `GP-4`):** When Producer Run A seals into `SET` (`claim_A = (1, prog_A, in_A, out_A)`) and feeds **both** Consumer B and Consumer C in a DAG (`general-purpose.md:50`, `architecture.md:359-360`), `out_A` embeds `vd(v)` (`SRC-022`). Consumers B and C authenticate `v` against `vd(v)` inside their own `Init`/unfold leaves (`SRC-021`) without ever touching Run A's `mem_root` or cache witnesses.
- **Foreign Proofs (`GP-3`):** Foreign `Compact` ZKIR v3 circuits lift through `F` into `SET` without sharing `U`'s memory bus; keeping `N`, `C`, and `D` free of memory-checking accumulators keeps foreign aggregation completely orthogonal.

---

## 4. Question 4 (`Q4`): Production zkVM Prior Art & 2024–2026 Literature Survey

All URLs below were fetched and verified live during this study.

### 4.1 Summary Matrix of Production zkVMs

| System | RAM Consistency Mechanism | Program / Decode & ALU Lookups | Multi-Segment / Recursion Boundary | Lesson for Sherlock on Midnight KZG |
|---|---|---|---|---|
| **Jolt** (`a16z`) | **Twist** (read/write increments + Val evaluation polynomial via sumcheck, replacing Lasso/Spice) | **Shout** (one-hot address read-only lookup; prefix-suffix sumcheck for ALU & bytecode) | Monolithic sumcheck / streaming prefix-suffix passes (`O(K^{1/c})` space); HyperKZG / Dory | Shout/Twist rely on interactive sumcheck rounds, not single-phase Halo2 `VerifierGadget` |
| **SP1** (`Succinct` v3–v6 Hypercube) | Intra-shard timestamped multiset (`MemoryLocalChip`) lifted via **LogUp-GKR** (`v4`) or **SepticDigest** (`Fp^7` EC sum, `v5/v6`) | `InstructionDecodeChip` + `ByteChip` (`2^16` table: U8Range, AND/OR/XOR, SLL, LTU, MSB) | Cross-shard global sum of `SepticDigest` EC points checked at recursion root; Jagged PCS (`ePrint 2025/917`) | Septic EC sum works fast in BabyBear/KoalaBear STARKs **only** via hashless non-det curve points + global init/finalize shards |
| **RISC Zero** (`rv32im` v2) | Intra-segment offline RAM; cross-segment **Poseidon2 Merkle `PagedMemory`** (`PageIn` / `PageOut` dirty pages) | Fixed byte/range/decode tables inside STARK segment | **Zero cross-segment challenges!** Segment public IO is `(pre_image_id, post_image_id)` Merkle roots | **Closest production analogue to Sherlock `DIPM`:** page-in touched pages once per segment, page-out dirty pages, fold roots |
| **OpenVM** (`Axiom` v2.1) | Intra-segment Spice offline checker (`OfflineMemory`) bridged to `PersistentBoundaryAir` / `MemoryMerkleAir` | Static ROM cached once with multiplicity column; bitwise/range LogUp buses | **Hybrid:** fast intra-segment cache + **Poseidon2 Merkle root `(initial_root, final_root)` at segment boundaries** | Directly validates hybrid **intra-leaf cache + boundary Merkle root** for continuations & tree recursion |
| **Ziren** (`ProjectZKM`) | Timestamped offline RAM via **LogUp-GKR** inside shard + **hashless septic ECMH** (`ePrint 2025/1503`) across shards | KoalaBear LogUp-GKR lookup buses for decode, range, bitwise | Global septic EC sum over `(addr, ts, val)` across shards + `MemoryInit`/`MemoryFinalize` shards | Confirms hashless ECMH needs $O(N_{\text{touched}})$ init/finalize sweep shards |
| **Stwo / Cairo** (`StarkWare`) | **Write-once continuous memory** split into `addr -> id` + `id -> value` (`Small` <120b vs `Big` 252b) via LogUp | **`verify_instruction` (`inst_cache`):** deduplicates repeated PC decodes by multiplicity `(pc, mult)`! | Monolithic / segment LogUp sum; public memory injected into boundary LogUp sum | **Write-once semantics** (`no timestamps!`) + **PC decode deduplication** directly mirror Sherlock `PAGE` & `PROG_CACHE` |
| **Valida** (`Lita`) | Registerless stack-frame RAM: single-segment sorted `MemoryChip`; multi-segment **two-pass LogUp** (`PersistentMem`) | Bus interactions over BabyBear STARK chips | Two-pass challenge across all segments (`prove_Helper` commits all segments before Pass-2 challenges!) | Demonstrates the exact two-pass latency & full-run buffering penalty of `(b2)` |
| **Nexus** (`v1` -> `Stwo`) | Migrated **from** Nova per-step Merkle paths **to** Stwo typed offline LogUp (`RamInitFinal` over `u32` words + `program_io`) | Instruction ROM + range/bitwise LogUp relations (`RelationEntry`) | Run-wide LogUp sum (`claimed_sum == 0` across execution + `RamInitFinal`) | Shows per-read Merkle is too slow (`O(D)` per read), while run-wide LogUp requires a multi-phase STARK |
| **ZKsync Airbender** | **Two-pass RAM:** Pass 1 commits **only** memory columns across all chunks -> derives $(\beta,\gamma)$ -> Pass 2 lazy-init shuffles (`ePrint 2023/1115`) | **Induction range-check avoidance** (registers stay range-checked by invariant!) + CSRRW delegation | Two-pass memory-subtree commitment root hashes challenges shared across parallel chunk provers | **Induction range-check principle** is already `D7.5` (`design.md:86`); two-pass subtree needs `SecondPhase` support |

### 4.2 Detailed System Notes & Verified Citations

1. **Jolt — From Lasso/Spice to Twist & Shout (`2023–2026`):**
   - Early Jolt (`https://eprint.iacr.org/2023/1217`) used **Lasso** (`https://eprint.iacr.org/2023/1216`) for decomposable instruction lookups and **Spice** (`https://eprint.iacr.org/2018/907`, offline grand-product multiset hashing with read/write counters) for RAM/registers.
   - Current Jolt replaces Lasso/Spice with **Twist and Shout** (Setty, Thaler, Wahby/Zhu, `https://eprint.iacr.org/2025/105`; Jolt architecture docs: `https://raw.githubusercontent.com/a16z/jolt/main/book/src/how/twist-shout.md`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/architecture/ram.md`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/architecture/registers.md`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/architecture/instruction_execution.md`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/architecture/bytecode.md`) and streaming **Prefix-Suffix Shout** (Nair, Thaler, Zhu, `https://eprint.iacr.org/2025/611`). Shout commits one-hot read addresses $ra \in \{0,1\}^K$ (factored into $d$ digits of size $K^{1/d}$) and proves $\text{rv}(r_{\text{cycle}}) = \sum_k ra(k, r_{\text{cycle}}) \cdot \text{Val}(k)$ by sumcheck with **zero multiplicities or grand products**; Twist commits write increments $\text{Inc}(j) = wv(j) - \text{Val}(k, j)$ and reconstructs $\text{Val}$ over boolean time hypercubes. Prefix-Suffix sumcheck eliminates multi-gigabyte witness materialization, achieving $O(K^{1/c})$ streaming space — **but** requires $O(\log T)$ interactive sumcheck rounds unavailable in Halo2's fixed-degree univariate `VerifierGadget`.

2. **SP1 (`Succinct`, v3–v6 Hypercube):**
   - Within each shard, `MemoryLocalChip` (`https://raw.githubusercontent.com/succinctlabs/sp1/main/crates/core/machine/src/memory/local.rs`) records initial `(prev_shard, prev_clk, initial_val)` and final `(last_shard, last_clk, final_val)` states of every touched address inside the shard, absorbing all intra-shard hits locally and emitting **only two global boundary events per distinct address touched in the shard**.
   - Cross-shard consistency uses either **LogUp-GKR** (`https://raw.githubusercontent.com/succinctlabs/sp1/main/crates/hypercube/src/logup_gkr/verifier.rs`) or an elliptic-curve multiset sum **`SepticDigest`** over the degree-7 extension curve $y^2 = x^3 + 2x + 26z^5$ over $\mathbb{F}_{p^7}$ (`https://raw.githubusercontent.com/succinctlabs/sp1/main/crates/hypercube/src/septic_curve.rs`, `https://raw.githubusercontent.com/succinctlabs/sp1/main/crates/hypercube/src/septic_digest.rs`), paired with `MemoryGlobalChip` init/finalize shards (`https://raw.githubusercontent.com/succinctlabs/sp1/main/crates/core/machine/src/memory/global.rs`), `InstructionDecodeChip` (`https://raw.githubusercontent.com/succinctlabs/sp1/main/crates/core/machine/src/program/instruction_decode.rs`), the $2^{16}$-row `ByteChip` (`https://raw.githubusercontent.com/succinctlabs/sp1/main/crates/core/machine/src/bytes/mod.rs`), and **Jagged PCS** (`https://eprint.iacr.org/2025/917`, EUROCRYPT 2026) to avoid padding unused table heights.
   - *Key takeaway for Sherlock:* SP1's `MemoryLocalChip` (collapse $R$ intra-shard accesses to 1 boundary entry per distinct address) is the exact read-write analogue of Sherlock's intra-leaf `PAGE` cache — except Sherlock's write-once memory avoids timestamps and closes against `mem_root` instead of requiring `MemoryGlobalChip` to sweep all memory at halt.

3. **RISC Zero (`rv32im` v2 Continuations & `PagedMemory`):**
   - RISC Zero (`https://dev.risczero.com/terminology`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/binfmt/src/image.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/circuit/rv32im/src/execute/pager.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/circuit/rv32im/src/execute/platform.rs`) commits full VM memory in a fixed-depth binary **Poseidon2 Merkle tree of 1 KiB pages** (`PAGE_WORDS = 256`, depth ~22).
   - Within a segment, CPU reads/writes hit a fast local offline RAM table; at segment start, `PageIn` verifies Merkle paths only for the **distinct pages touched in that segment** against `pre_image_id`, and at segment end, `PageOut` re-hashes **only dirty pages** to compute `post_image_id`!
   - *Key takeaway for Sherlock:* RISC Zero proves in production that **zero-cross-segment-challenge paged Merkle caching** gives self-contained, streaming, stop-and-resumable, tree-aggregatable segment proofs (`pre_root -> post_root`) while amortizing Merkle hashing over page-level spatial and temporal locality.

4. **OpenVM (`Axiom` v2.1.0):**
   - OpenVM (`https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/system/memory/mod.rs`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/system/memory/offline_checker/bridge.rs`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/system/memory/persistent.rs`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/system/memory/merkle/mod.rs`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/system/program/mod.rs`) explicitly splits single-segment mode (`VolatileBoundaryAir`, pure offline Spice) from multi-segment continuation mode (`PersistentBoundaryAir` + `MemoryMerkleAir`), which **bridges intra-segment cached accesses to a Poseidon2 Merkle tree of 8-field-element chunks** at segment boundaries! Static program ROM is loaded once with a multiplicity counter (`ProgramExecutionBus`).

5. **Ziren (`ProjectZKM`, formerly zkMIPS) & Hashless Septic ECMH (`ASIACRYPT 2025`):**
   - Ziren (`https://raw.githubusercontent.com/ProjectZKM/Ziren/main/docs/src/design/memory-checking.md`, `https://raw.githubusercontent.com/ProjectZKM/Ziren/main/docs/src/design/lookup-arguments.md`) combines intra-shard **LogUp-GKR** (Papini & Haböck, `https://eprint.iacr.org/2023/1284`) with Groth, Malvai, Miller, and Zhang's **hashless elliptic-curve multiset hash** (`https://eprint.iacr.org/2025/1503`) for single-pass cross-shard RAM: instead of an expensive deterministic `hash_to_curve` (`SVDW`), the prover supplies an 8-bit nonce `r` and curve $y$-coordinate as witness so $x = x_0(op) + r$ lands on the curve, verified by a degree-3 curve equation (`~3` muls) plus Poseidon binding. Even with hashless ECMH, Ziren still requires `MemoryInit` and `MemoryFinalize` chips over every touched address at run boundaries.

6. **Stwo / Cairo (`StarkWare`):**
   - Cairo (`https://eprint.iacr.org/2021/1063`) and `stwo-cairo` over Circle STARKs (`https://eprint.iacr.org/2024/278`; `https://raw.githubusercontent.com/starkware-libs/stwo-cairo/main/crates/adapter/src/memory.rs`, `https://raw.githubusercontent.com/starkware-libs/stwo-cairo/main/crates/cairo-air/src/components/memory_address_to_id.rs`, `https://raw.githubusercontent.com/starkware-libs/stwo-cairo/main/crates/cairo-air/src/components/memory_id_to_big.rs`, `https://raw.githubusercontent.com/starkware-libs/stwo-cairo/main/crates/cairo-air/src/components/verify_instruction.rs`) use **write-once continuous memory** (`no timestamps!`), decoupling `Address -> ValueID` from `ValueID -> Value` (splitting small values `< 120` bits from 252-bit values and deduplicating identical constants/values across addresses!) and caching decoded instructions in `verify_instruction` (`inst_cache` keyed by `pc` so a hot loop of $10^5$ iterations decodes each PC **once**!).

7. **Valida (`Lita`), Nexus (`Stwo`), and ZKsync Airbender:**
   - **Valida** (`https://raw.githubusercontent.com/lita-xyz/valida-vm/main/memory/src/stark.rs`, `https://raw.githubusercontent.com/lita-xyz/valida-vm/main/machine/src/persistence.rs`, `https://raw.githubusercontent.com/lita-xyz/valida-vm/main/machine/src/proof.rs`) is a registerless stack-frame VM (closest ISA cousin to Sherlock's frame-linked machine, except Valida uses mutable frame slots while Sherlock `D7` uses write-once linked spills + cached `top` register!). In multi-segment mode, Valida must commit **all** segments in Pass 1 before squeezing cross-segment persistent LogUp challenges.
   - **Nexus** (`https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/prover/src/machine.rs`, `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/prover/src/extensions/ram_init_final.rs`, `https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/prover2/machine/src/lib.rs`) abandoned Nova per-step Merkle memory checking for Stwo LogUp (`RamInitFinal`).
   - **ZKsync Airbender** (`https://raw.githubusercontent.com/matter-labs/zksync-airbender/main/docs/philosophy_and_logic.md`, `https://raw.githubusercontent.com/matter-labs/zksync-airbender/main/docs/subarguments_used.md`) uses Yang & Heath's lazy-init RAM shuffles (`https://eprint.iacr.org/2023/1115`), commits **only** the memory column subtree across chunks in Pass 1 so Pass 2 chunks prove in parallel, and pioneers **induction range-check avoidance** (registers initialized in range and written only via range-checked results never need read-side range checks — identical to `design.md:86` §2.7.5!).

8. **2024–2026 Lookup & IVC Memory Papers:**
   - **LogUp** (Haböck, `https://eprint.iacr.org/2022/1530`) and **cq** (Eagen, Fiore, Gabizon, `https://eprint.iacr.org/2022/1763`): sublinear cached-quotient KZG lookups ($O(m \log m)$ prover independent of table size $N$), unusable directly inside `midnight-circuits-7.3.0` without changing `VerifierGadget` (`verifier/lookup.rs`).
   - **Nebula** (Arun & Setty, IEEE S&P 2026, `https://eprint.iacr.org/2024/1605`): commitment-carrying IVC with offline multiset memory over CycleFold/HyperNova — achieves Merkle-free IVC **only** via folding-native relaxed R1CS commitments, not fixed-VK Halo2 KZG `VerifierGadget`.
   - **Celer** (Qu et al., CRYPTO 2026, `https://eprint.iacr.org/2026/1453`): $O((m+n)\log n)$-free sublinear-in-queries lookup when $m \gg n$ via binary decomposition of query vectors.
   - **Ring Lookups & Batch RAM** (Bootle et al., Mar 2026, `https://eprint.iacr.org/2026/471`): amortizes parallel RAM accesses per step.

---

## 5. Question 5 (`Q5`): Concrete Recommendation to Prototype First in `crates/mori-chip`

### 5.1 One Design to Prototype First: Experiment `E9` — `DIPM` (`crates/mori-chip/src/page.rs` + Dense-Frontier `MEM` + `PROG_CACHE`)

Prototype **`DIPM`** directly on top of the E8 spike (`crates/mori-chip/src/{m3.rs,mem.rs,rfcore.rs,bus.rs}`) with **zero changes to `midnight-proofs`, `midnight-circuits`, `ivc.rs`, or `run_decider.rs`**:

1. **New module `crates/mori-chip/src/page.rs` (`PageCacheConfig`):**
   - **Read-only Memory Page Cache (`K_page_ro = 4` pages $\times$ `P = 8` cells = 32 cached external cells/leaf):** Each active page slot verifies one aligned 8-cell subtree (`16` record perms + `7` subtree compressions + Merkle spine walk to `pre.mem_root`, with optional LCA prefix sharing between sorted pages) and emits up to 8 provider tuples `(act_old * CHIP_MEM, act_old * (8 * page_idx + j), a_j, b_j, c_j)` on `cols.a[0..5]` with `q_prov = 1`. A gate enforces `8 * page_idx + j < pre.len` on every row with `act_old = 1` (enforcing D2 disjointness even inside a partially filled tail page!).
   - **Program / Type / Constant Page Cache (`K_prog = 2` pages $\times$ `8` rows = 16 cached `PROG` entries/leaf):** Authenticates 8-row `PROG` pages against `prog_root` (`design.md:84`) and emits `(act_p * CHIP_PROG, act_p * (8 * p_idx + j), word0, word1, const_or_ty_d)` on the **exact same 5-column bus** (`CHIP_PROG = 3`).
2. **Upgrade `SegMemConfig` & `WalkConfig` (`src/mem.rs`, `src/m3.rs`) from Aligned-Pad D3 to Dirty-Gated Dense-Index Frontier (`K_frontier = 2` pages):**
   - `pre.len` is any 64-bit integer (`0 <= pre.len < 2^64`), split into `p_f = pre.len >> 3` (`D = 61` bits, tested at both `D = 17` and `D = 61`) and `off = pre.len & 7` (`3` boolean bits).
   - **`post.len = len_end` (the exact running `len` of slot `S`, NOT `base + 8`!)** — zero leaf-close address padding!
   - **Dirty gate (`dirty = is_nonzero(len_end - pre.len)`):** When `dirty == 0`, constrain `post.mem_root == pre.mem_root` and skip frontier updates (zero-append leaves consume **0** addresses, `GP-001`). When `dirty == 1`, open `p_f` from `pre.mem_root` (verifying prefix `0..off` preserved and `off..8` empty `act_old == 0`), write active appended cells `pre.len .. len_end - 1` into `p_f` (and empty `p_f + 1` if `off + used > 8`), and walk `post.mem_root`.
   - Parameterize arena tag bits (`[arena_tag : 3 bits || dense_idx : 61 bits]`, with separate dense frontier `len_fields` for `DATA_FIELDS`) to test `SI-12` (`> 8`-field constructor across leaf boundary, `requirements.md:56`) directly!
3. **Core 2nd Request Row (`src/rfcore.rs:180,198`):**
   - Promote row `6` of `Arch::Narrow` from a replica of `(L0, L1)` to a second `q_req = 1` request pair `(L2, L3)`, giving **4 bus requests per step** (`PROG` fetch + `ALU` `LT` + `CELL` read/append + `vd`/second read) with **0 new lookup arguments**.
4. **Wire `D7` Cached `top` Frame Register (`d7-frames.md`) and `SVM-012` Cached `vd` Word (`c[2]` of `DATA` cells).**

---

### 5.2 Quantitative Confirmation Experiment & Pass/Fail Gates (`ESTIMATE`)

| Metric | E8 Baseline (Measured, `experiments-4.md:53-63`) | E9 `DIPM` Target (`ESTIMATE`) | Hard Gate |
|---|---|---|---|
| Leaf advice / equality / fixed columns | `12` / `13` / `20` | **`12` / `13` / `20..21`** (`<= +1` packed code col) | Node `<= 950,000` |
| Leaf `lookup` / `lookup_any` count | `4` (`2` bus, `1` `RANGE`, `1` `DEC`) | **`4` (unchanged!)** | `<= 4` (for `GP-4` `RunLift` `R`) |
| Leaf phases / rotations | `1` phase / `{-1, 0, +1}` | **`1` phase / `{-1, 0, +1}`** | Exact match (`VerifierGadget`) |
| Leaf rows (`S = 16`, `D = 17` / `D = 61` w/ LCA) | `6,511` (`k = 13`, no `PAGE`/`PROG`) | **`~16,500–24,000` (`k = 15` at `D = 17`) / `~26,000–42,000` (`k = 16` at `D = 61`)** | `k_leaf <= 16` at `S = 16` |
| `SHKIVC06` Node `cost_model` rows | `823,466` (`k = 20`) | **`823,466` to `829,928` (`k = 20`, `> 120,000` headroom)** | `<= 950,000` (`k = 20`) |
| `RunDeciderRelation` circuit / table rows | `929,166` / `1,032,467` (`k = 20`) | **`929,166` / `1,032,467` (`0` growth)** | `<= 1,000,000` / `<= 1,048,570` |

---

### 5.3 Quint Model (`models/sherlock/quint/chipLeafPaged.qnt`) & 12 Seeded Mutants

Extend `chipLeaf.qnt` / `chipLeafCached.qnt` to model a **3-leaf chain** over `DIPM` (`P = 4` cells/page in model scale, `S = 3`, `K_page_ro = 2`, `K_prog = 2`, `K_frontier = 2`, including a `Build` slice of `6 > P` fields crossing Leaf 1 -> Leaf 2 and a `0`-append Leaf 3), checked against a **partition-unaware dense reference machine** (`SVM-011`, `SRC-023`, `GP-001`):

1. `M1_tail_page_overlap`: Drop `addr < pre.len` check on `act_old` rows in `PAGE` -> caught by `invFunctional` & `invRefine` (reads unwritten slot inside partial frontier page).
2. `M2_partial_tail_clobbers_prefix`: Frontier write-back omits equality on slots `0 .. (pre.len mod P) - 1` -> caught by `invMem` & `invRefine` (corrupts earlier leaf's cells in shared page!).
3. `M3_dirty_gate_bypassed`: Leaf with `used > 0` sets `dirty = 0` and keeps `pre.mem_root` -> caught by `invRefine` & `invMem` (appends vanish at next leaf!).
4. `M4_zero_append_bumps_len`: Regresses `post.len` to D3 `base + P` on `used == 0` -> caught by **`invPartitionIndep`**, **`invZeroAllocNoAddr`** (`GP-001`), and **`invSliceContig`** (`SI-12`)!
5. `M5_page_unauthenticated`: `PAGE` slot skips `pre.mem_root` opening -> caught by `invBus` & `invRefine`.
6. `M6_prog_wrong_root`: `CHIP_PROG` page opens against `mem_root` instead of `prog_root` -> caught by `invBus` & `invRefine` (self-modifying code forge!).
7. `M7_dup_page_unconstrained`: Two `PAGE` slots claim same `page_idx` with omitted Merkle bind -> caught by `invFunctional`.
8. `M8_lca_prefix_splice`: Shared LCA spine omits bit-prefix equality between `p_0` and `p_1` -> caught by `invBus` & `invMem`.
9. `M9_vd_host_unconstrained`: `DATA` append accepts host `vd` without `H(tag, child_vd)` check (`SVM-012`) -> caught by `invOutcomeDigest`.
10. `M10_d7_top_omitted`: `pre_d`/`post_d` omits cached `top` register (`D7`) -> caught by `invFrames`.
11. `M11_unwritten_frontier_slot_active`: Frontier marks slot `>= len_end` with `act_new = 1` -> caught by `invRefine` (`root_all_rows` analogue).
12. `M12_slice_arena_interleaved`: `FRAME` push writes into `FIELDS` dense counter mid-`Build` -> caught by `invSliceContig` (`SI-12`).

---

### 5.4 `MockProver` Hostile-Witness Negative Suite (`crates/mori-chip/tests/e9_paged_cache.rs`)

All negative tests must construct raw witness mutations that reach `MockProver::verify() == Err(_)` at the constraint boundary (never synthesis panics, `SAS-003`):
1. `e9_vk_unit_rotations_and_single_phase`: Asserts all queries in `{-1, 0, +1}` and `cs.phases().count() == 1`.
2. `e9_partition_independence_3way`: Runs a 12-step allocating + `> 8`-field constructor trace across partitions `[12]`, `[6, 6]`, and `[1, 0_alloc_compute, 11]`, asserting **bit-identical** final `post_d`, `mem_root`, `len`, `len_fields`, and `outcome_d` (`SRC-023`, `GP-001`).
3. `neg_page_serves_addr_at_or_above_pre_len`: `PAGE` row tries to answer `addr == pre.len` -> fails `LT` / `D2` gate.
4. `neg_frontier_overwrites_existing_prefix_cell`: Leaf starts at `pre.len = 3` (`off = 3`), mutates slot `1` during frontier write-back -> fails old-page Merkle walk against `pre.mem_root`.
5. `neg_frontier_unwritten_tail_nonempty_in_old_walk`: Old walk of partial page `p_f` supplies non-zero record at slot `>= off` -> fails empty-tail constraint.
6. `neg_dirty_zero_when_used_nonzero`: Appends 1 cell (`len_end = pre.len + 1`) while keeping `post.mem_root = pre.mem_root` -> fails `dirty` `is_nonzero` / root mux gate.
7. `neg_zero_append_advances_len_or_mutates_root`: `used = 0` leaf tries to set `post.len = pre.len + 8` or alter `post.mem_root` -> fails `post.len == len_end` and `dirty == 0` mux.
8. `neg_page_forged_merkle_sibling`: Mutates 1 bit of a `PAGE` sibling -> fails `HASH` sponge copy to `pre.mem_root`.
9. `neg_prog_page_against_mem_root`: Swaps `prog_root` with `mem_root` on `CHIP_PROG` walk -> fails `prog_root` copy constraint.
10. `neg_padding_tuple_matched_by_active_request`: Active request sets `chip = 0` to match `(0,0,0,0,0)` padding -> fails core request chip-id gate (`D4`/`D8`).
11. `neg_spill_across_two_frontier_pages_drops_second_page`: `pre.len = 6`, `used = 5` (`len_end = 11`, crosses `7 -> 8`), omits `p_f + 1` update -> fails `mem chain` / `p_f + 1` walker gate.
12. `neg_lca_shared_spine_wrong_divergence_bit`: Forges divergence bit in shared upper 61-bit spine -> fails bit-reconstruction / order gate.
13. `neg_svm012_forged_cached_vd`: Supplies wrong `vd` on `Constructor` allocation -> fails `HASH` `vd` binding gate (`SVM-012`).
14. `neg_address_64bit_boundary`: Boundary witness check accepting `len = 2^64 - 2` (`+1` append -> `2^64 - 1`) and refusing wraparound `2^64 -> 0` (`GP-001`, `SRC-004`).

---

## 6. What Was Not Checked

1. **No builds, `cost_model` executions, `MockProver` runs, Quint runs, Lean builds, or real KZG proving were executed** during this session (`BRIEF.md:65` no-heavy-commands rule). Every row count, wall-clock duration, and memory figure beyond already-recorded repository measurements is calculated from measured linear fits and marked `ESTIMATE`.
2. **Exact E0(c) 108-rule micro-op width and max per-step bus requests:** Census E0 (`design.md:119`) covers `fact(3)`, `fold`, and `list123`, and `crates/mori-chip` currently wires 3 stand-in rules (`leaf.rs:17-19`). Whether the worst rule among all ~108 rules of `sherlock-helper-microstep/1` needs 4 or 6 bus requests per slot (2 vs 3 `q_req = 1` rows in `Arch::Narrow`) was not empirically enumerated across all 108 rules.
3. **Exact cause of the `+16,256` node-row step between `S = 1` (`768,872`) and `S = 16` (`785,128`) on the early E3 spike (`design.md:113,123`):** Flagged `[OPEN]` in `design.md:123` (likely selector/fixed column deduplication when `S = 1` makes two fixed columns identical, cf. `experiments-4.md:104-105`); not re-probed by running `cost_model`.
4. **Canonical normal form of `decl_schema_d` inside `ty_d` (`SRC-022`, `dissent.md:204-207`) and the input admission claim behind `val_d` at `h_pre = 0` (`SRC-021`, `requirements.md:46`):** Both remain open specification items upstream of `PROG_CACHE`; `DIPM` authenticates whatever canonical `ty_d` words are committed in `prog_root`.
5. **`midnight-proofs 0.8.2` vs `0.8.3` lockfile delta (`dissent.md:224`, `review.md:201`):** `Cargo.toml:42` pins `0.8.3`, whereas `midnight-ledger` tag `ledger-9.1.0.1`'s own `Cargo.lock` resolves `0.8.2`; byte-for-byte VK/transcript parity across that patch bump was not re-tested here (tracked by `tasks.md` 2.17).
6. **No other lane's `round1.md`, `LEAD-NOTE-*`, or `research/sherlock-final-design-20261111/` was listed or read** (strict Round 1 independence).
