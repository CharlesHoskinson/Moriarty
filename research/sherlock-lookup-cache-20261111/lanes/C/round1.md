# Sherlock Lookup & Cache Architecture — Round 1 Position Paper (Lane C)

**Study:** `sherlock-lookup-cache-20261111` — Round 1 (Independent Analysis)  
**Lane:** C — Decode, Program/Type Tables, `ALU`/`RANGE` Tables & Hash Memoization Lead  
**Repository Baseline:** Commit `9ffc2fe76f523453632cf3df03ca761095bc7c13` (`midnight-proofs 0.8.3`, `midnight-circuits 7.3.0`, `midnight-zk-stdlib 2.4.0`, ledger `9.1.0.1` tag `a2b30128`)

---

## Executive Summary

In STARK and multilinear sumcheck zkVMs (Jolt, SP1 Hypercube, OpenVM, Ziren, Stwo), lookup arguments over structured or small tables are cheap because the prover either avoids committing the table altogether (Lasso/Shout) or batches interactions via logarithmic derivatives (LogUp / LogUp-GKR) without paying per-lookup elliptic-curve multi-scalar multiplications inside a PLONK recursive verifier. **On Sherlock's pinned KZG/BLS12-381 recursive stack, that cost model is inverted:**

1. **Every `cs.lookup` or `cs.lookup_any` call is a 3-commitment Halo2 permuted-column argument (`A'`, `S'`, `Z_L`)** (`midnight-proofs-0.8.3/src/plonk/lookup.rs:53-67`, `midnight-circuits-7.3.0/src/verifier/lookup.rs:39-160`), adding **3 variable-base G1 points** and **5 scalar openings** to the leaf proof and **`~22,600` rows (`ESTIMATE`)** to the recursive `ChipNode` (`research/sherlock-processor-analogy-20261110/experiments-2.md:135-136`, `experiments-3.md:162-165`).
2. **Every `TableColumn` or distinct filled `Fixed` column adds `~6,004–6,462` Node rows** (`experiments-2.md:125`, `experiments-3.md:159, 174-175`), every advice column adds `~7,298` Node rows, and every equality-enabled column adds `~8,189` Node rows.
3. **Conversely, polynomial gate complexity (`deg <= 6`) is nearly free in the Node** (`+1,623` Node rows when core constraints are emitted 4x, `experiments-2.md:115, 120-121`), and **leaf row count below $2^{k_{\text{leaf}}}$ costs zero Node rows** as long as $k_{\text{leaf}} \le 16$ (`openspec/changes/sherlock-vm-consensus-20261107/design.md:117`).
4. **The pinned in-circuit verifier (`VerifierGadget`) enforces `cs.phases().count() == 1` (`midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:16, 310`) and rotations $\{-1, 0, +1\}$ (`:604-611`).** Any design requiring Halo2 `SecondPhase` user challenges panics at Node synthesis unless `midnight-circuits` is forked.

From these four facts and an exhaustive audit of `crates/mori-chip/src/` and all 108 Sherlock `/1` rules (`crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:363-398`), five conclusions follow:

- **Adopt intra-leaf `ROM` + `PAGE` read caching on the existing 2-lane `bus` (`lookup_any`) (`Q1(a)`, `Q1(c)`):** Authenticate each distinct touched 8-record `PROG`/`Const` or `MEM` cache line once per leaf via Merkle walk against `prog_root` or `pre_d.mem_root`, and serve arbitrary intra-leaf reads (plus causal same-leaf write-forwarding from `SegMemConfig`) over the existing `bus` for **0 new lookup arguments** and **0 new advice columns**. This saves `~33,000–43,000` leaf rows at $S = 16$ and `~150,000+` leaf rows at $S = 64$ (`ESTIMATE`) versus uncached per-read walks, keeping $S = 64$ inside $k_{\text{leaf}} = 16$.
- **Reject run-wide offline memory checking (`Q1(b)`):** Challenge-based grand-product/LogUp memory checking across leaves is blocked both by `verifier_gadget.rs:310` (`phases == 1`) and by causal single-pass IVC / DAG composition (`GP-2`, `GP-4`). Challenge-free Elliptic-Curve Multiset Hashing (ECMH, as in SP1 Turbo / Ziren) fails on write-once read multiplicities unless every read re-emits an incremented counter (`>= 140` Poseidon+ECC rows/read) and forces an $O(|\text{MEM}|)$ end-of-run retirement scan that violates `O(1)` Halt and `GP-1`.
- **Compress 108-rule `DEC` via gated syndrome normalization + a 54-bit packed control word, and multiplex `DEC` + `RANGE` into ONE 2-column tagged fixed table (`Q1(c)`, `Q1(d)`):** Raw `/1` dispatch has 36 split roots (`18` non-enum predicates including 2 arena-dereference predicates `equal_differ` and `validate_marker`, 5 `u128` cursor comparisons, and 11 wide zero tests); naive don't-care expansion explodes to $2^{18}\text{–}2^{22}$ rows. Normalizing those 18 predicates in degree-$\le 6$ gates (free in Node) collapses `DEC` to `<= 2,048` canonical rows with a 54-bit control word in one scalar. Because `CORE` (`q_dec`) and `ALU/RANGE` (`q_rng`) are **100% row-disjoint** (`crates/mori-chip/src/rfcore.rs:795-808`), merging `RANGE` (`4,096` rows) and `DEC` (`<= 2,048` rows) into a single 2-column domain-tagged table (`6,145` rows $< 8,192$, keeping $k = 13$) **eliminates 1 `lookup` argument and 1 `TableColumn`, saving `~29,062` Node rows (`ESTIMATE`)**.
- **Reject Jolt/Lasso-style decomposed subword `ALU` tables; keep gate `ALU` + 1-limb running-sum `RANGE` (`Q1(d)`):** Sherlock `/1` has **zero bitwise ops** and **zero `DIV`/`REM`** (`crates/mori-core/src/sherlock/helper_machine/shared.rs:7-22`). Currently `ALU + RANGE` is `192` rows (`2.9%` of `6,511`) at $S = 16$. Replacing gates with 8-bit binary `(op, x_b, y_b)` subtables forces $65,536\text{–}262,144$ table rows ($k_{\text{leaf}} = 17\text{–}18$, failing `k_leaf <= 16`), triples limb lookups per op, and adds `+67,800` to `+158,200` Node rows. Instead, `SUB`, zero-inverse `EQ`, and 64-bit-limb checked `MUL` (avoiding the $\mathbb{F}_q$ wraparound trap $(2^{128}-1)^2 > r \approx 2^{254.86}$) fit in `AluConfig` with **0 new columns and 0 new lookups**.
- **Adopt write-once allocation-time `vd` memoization + `(SHK_VAL, 1)` sponge-prefix precomputation + universal `VD_{NIL,FALSE,TRUE}` constants + domain-tagged 2-input `HASH` bus deduplication (`Q1(e)`):** By `SI-12` DAG order, `vd` is computed **only** in 2 of the 108 rules (`BHelperScalarExit`, `BHelperEqualExit`), never on reads or Halt. Precomputing `PoseidonPerm(0, SHK_VAL, 1)` saves **70 rows (1 full permutation) on every dynamic `vd`** without altering a single bit of `sherlock-value/1`, while constant `VD_{NIL,FALSE,TRUE}` eliminates **100%** of hashes on `BHelperEqualExit` and boolean `BHelperScalarExit`.

---

## 1. Baseline Audit: `crates/mori-chip/src/` and the 108 Sherlock `/1` Rules

### 1.1 Exact Key, Table, Bus, Selector, Lookup & Poseidon Inventory (`crates/mori-chip/src/`)

At commit `9ffc2fe76f523453632cf3df03ca761095bc7c13`, the E8 leaf circuit `RfLeaf` (`RfParams::m3_params()`, `crates/mori-chip/src/rfcore.rs:1621-1630`, measured in `research/sherlock-processor-analogy-20261110/experiments-4.md:52-111, 157-159`) operates over `F = midnight_curves::Fq` (BLS12-381 scalar field, $r \approx 2^{254.86}$, `crates/mori-chip/src/lib.rs:34`) with `NCOL = 12` advice columns (`:30`), `13` equality-enabled columns (`12` advice + `1` instance, `rfcore.rs:782-789`), max gate degree `6` (`vsel.rs:35`), `1` challenge phase, and rotations restricted to `{-1, 0, +1}` (`crates/mori-chip/tests/e3_spike.rs:615-650`).

| Chip / Module (`crates/mori-chip/src/...`) | Fixed / `TableColumn`s (20 total in `vk.cs()`) | Lookups (4 total) | Leaf Rows at $S=16$ (`6,511` circuit / `4,096` table) | Role & Constraints |
|---|---:|---:|---:|---|
| **`BusConfig`** (`bus.rs:28-92`) | **2** (`q_req`, `q_prov` raw `Column<Fixed>`, `:48-49`) | **2** `lookup_any` (lane 0 `a[0..5]`, lane 1 `a[5..10]`, `:50-61`) | Shared on `CORE`, `MEM`, `HASH` rows (`0` own rows) | 5-tuple `(chip, key, a, b, c)` dynamic set-inclusion bus; inactive rows map to `(0,0,0,0,0)` provided by disabled/pad rows (`mem.rs:193-199`). |
| **`RangeConfig`** (`range1`, `range.rs:30-140`) | **4** (`table: TableColumn` 4,096 rows `[0, 4096)`, `q_rng` complex, `q_first`, `q_step`, `:94-105`) | **1** `lookup` (`q_rng * a[11] in table`, `:98-101`) | Shared with `ALU`: `16 * 12 =` **192** (`2.9%`) | Decomposes one `u128` into $11 \times 12\text{b}$ limbs in `a[11]` + row-11 `pad = 16 * limb_10 < 4096` (enforcing `limb_10 < 256`); Horner sum in `a[10]`. |
| **`AluConfig`** (`alu.rs:23-125`) | **1** (`q_alu` at `center()` = row 6 of 12, `:89`) | `0` (uses `RangeConfig` + `bus`) | (Inside the **192** `RANGE` rows) | Implements `OP_ADD = 1` (`x + y = z + flag * 2^128`, `w = z`) and `OP_LT = 2` (`flag = 0`, `z` bool, `w = z*(y-x-1) + (1-z)*(x-y)`). Publishes `(CHIP_ALU=2, op+8*flag, x, y, z)`. |
| **`DecConfig`** (`dec.rs:18-124`, `rfcore.rs:884-888`) | **3** (`class: TableColumn`, `rule: TableColumn`, `q_dec` complex, `dec.rs:25-26`) | **1** `lookup` (`(q_dec*class, q_dec*rule)`, `:27-30`) | Shared on `CORE` slots (`0` own rows; 7 table rows in spike) | Maps `(class -> rule)` per slot; row `0` holds `(0, 0)` for disabled/non-decode rows (`dec.rs:60`). |
| **`CORE`** (`Arch::Narrow`, `pack_core`, `rfcore.rs:168-220, 897-945`) | **2** `VSel` code cols packing 7 selectors (`q_slot, q_link, q_init, q_last, q_step, q_cnt0, q_cnt`) | `0` (issues `bus` + `dec` queries) | `13 * (16 + 1) =` **221** (`3.4%`) | 32-word `Regs32` (`rf.rs:32-67`) folded across 4 rows $\times$ 8 cols + 9 routing/scratch rows = 13 rows/slot; 6 D7 rules (`rf.rs:101-118`). |
| **`SegMemConfig`** (`mem.rs:122-240`) | **1** `VSel` 3-code col (`act_row, step_row, last_row`, `:134, 157`) | `0` (publishes on `bus`) | `SEG =` **8** (`0.1%`) | Aligned 8-record write-once segment (`base = 8 * seg_idx`); monotone `act` prefix (`1..1, 0..0`), `addr = base + i`, `len_end = base + sum(act)`; publishes `(CHIP_MEM=1, act, addr, a, b)` or `(0,0,0,0,0)`. |
| **`PoseidonCfg`** (`one_abs`, `poseidon.rs:50-340`) | **6** (`rc[0..3]`, `q_full`, `q_part`, 1 `VSel` 3-code absorb col `(first, chain, tail)`, `:170-185`) | `0` (2-input hashes publish `(CHIP_HASH=3, 2, in0, in1, out)` on `bus`, `:268-286`) | `87 * 70 =` **6,090** (`93.5%`) | Width-3 Poseidon (`8` full + `60` partial = `68` rounds + `1` absorb + `1` output = `70` rows/perm, `:55-59`). *(Note: `hashchip.rs` is the retired E6 stdlib wrapper.)* |
| **`WalkConfig` / `M3`** (`m3.rs:50-385`) | **1** `VSel` 3-code col (`bit, step, swap`, `:105, 114`) | `0` (wired by equality copies to `PoseidonCfg`, `:321-383`) | (Drives all **87** `PoseidonCfg` perms) | **16** perms (`8 * 2`) record hashes `H(act,a,b,c)` (`1,120`r) + **7** seg-tree perms (`490`r) + **34** (`2 * 17`) old/new top-tree perms sharing `(sib, dir_bit, acc)` (`2,380`r) + **30** (`2 * 15`) 30-word `pre_d`/`post_d` perms (`2,100`r). |
| **Total E8 Leaf** | **20** (`cost_model` `fixed = 33` incl. 13 perm cols, `cost_model.rs:244`) | **4** | **6,511** circuit / **4,096** table (`k = 13`) | Node: **`823,466`** rows (`k = 20`); Decider: **`929,166`** circuit / **`1,032,467`** table rows (`k = 20`, **`16,109`** table-row margin). |

### 1.2 Complete Audit of the 108 Sherlock `/1` Rules (`helper_machine` & `mori-backend`)

`RuleIdC` defines 133 variants (`crates/mori-core/src/sherlock/helper_machine/shared.rs:400-534`), of which `supported_rules()` (`crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:363-398`; `testdata/dispatch/partition.json:560`) selects **exact 108 `/1` rules**:
- **3 terminal publication rules** (`:364-368`): `KAdminPublishValue` (`P1`, free), `KAdminPublishError` (`P1`, free), `KAdminPublishExhausted` (`P2`, free when `remaining_work == 0`).
- **50 active helper rules** across `HelperKind` `1..=9` (`:141-212, 369-375`; `BBookkeepingTaskRelease` shared between `Validate` and `Equal`): `Scalar` (5), `EnvGet` (4), `EnvPut` (2), `Find` (4), `Bind` (9), `Append` (5), `Validate` (7), `Equal` (6 slots / 5 unique), `Match` (9).
- **46 legacy + nominal ADT rules** across `27` `Site`s (`:109-138, 215-326, 376-382`; `24` read `result`, proven equivalent across Core/C6/Quint in `owner_runtime.rs:507-536` and `:297-326`).
- **9 helper entry rules** (`:384-396`): `BHelper{Scalar,EnvGet,EnvPut,Find,Bind,Append,Validate,Equal,Match}Entry`.
- *(Excluded 25 `RuleIdC` variants: 12 `XAdtHelper{Build,Plan}*` at `shared.rs:522-533` and `c6_dispatch_step.rs:30-32`, 7 disabled bookkeeping variants at `owner_runtime.rs:70-81`, 4 `/2` suspension/financial variants at `:82-90`, and 2 startup-only genesis rules at `shared.rs:508-509`.)*

Across all 108 `/1` rules, the complete resource-consumer audit establishes four tight structural bounds:
1. **Static `PROG`/`Const` tables (`crates/mori-core/src/sherlock/helper_machine/c6_phys_program.rs:419-838`):** Of 20 program tables, **only 11 are ever read at runtime in `/1`** (`expressions` [up to 2/step: `eval_row` + `origin_row`], `children` [1/step], `functions` [1/step], `parameters` [1/step], `arms` [1/step], `patterns` [1/step], `types` [1/step], `constructors` [1/step], `ctor_fields` [1/step], `constants` [up to 2/step], `constant_fields` [up to 2/step]), plus scalar bounds in `META`/`CTX`. Crucially, **`field_paths`, `plan_nodes`, and `plan_cases` have ZERO readers in the 108 `/1` rules** (they are read exclusively by the excluded `XAdtHelperPlan*` family, `owner_adt_runtime.rs:747-981`), and `occurrences`, `occurrence_paths`, `pattern_children`, `type_args`, `decl_*`, and `names` are admission/provenance-only (`shared.rs:1273-1274`).
2. **Write-once dynamic arenas (`Data`, `DataFields`, `Env`, `ValueSlots`):**
   - `Data`: allocated in **only 2 rules** (`BHelperScalarExit` at `fragments/scalar.proposed.rs:217-252` and `BHelperEqualExit` at `families/structural.rs:210-219`; `XAdtEvalConstructNullary` returns pre-interned `RefC::Const`, `owner_adt_runtime.rs:459-466`).
   - `DataFields`: allocated in **only 1 rule** (`BHelperScalarExit`: `+0` for `UInt`/`Bool`, `+1` for `Leaf`, `+2` contiguous for `Cons`/`Branch`, `scalar.proposed.rs:221-249`).
   - `Env`: allocated in **3 rules** (`BHelperEnvPutInsert`, `BHelperBindInsert`, `BHelperMatchBinding`, `+1` cell/step).
   - `ValueSlots`: allocated in **2 rules** (`BHelperAppendCopy`, `BHelperAppendTail`, `+1` slot/step, `families/environment.rs:722-774`).
   - Stacks: `Frames` pushed in **1 rule** (`KMachinePushAccept`, `legacy.proposed.rs:137-143`), popped in 9, top-rewritten in 6; `Tasks` pushed in 5 `Validate`/`Equal` rules, popped in 7, top-rewritten in 4 (`structural.rs:70-325`).
3. **Arithmetic & comparisons (`shared.rs:7-22`, `scalar.proposed.rs:142-185`):** Only checked `u128` `Add`, `Sub`, `Mul` (overflow/underflow stages `PrimError { code: 10 }` -> `Rejected(10)`), `u128` `Lt`, `bool` `Not`, structural `Eq` (`BHelperEqual*`), 5 `usize`/`u128` dispatch cursor `<`/`==` checks, and `+1`/`-1` counter/cursor updates. **Zero `DIV`, zero `REM`, zero bitwise `AND`/`OR`/`XOR`/shifts exist anywhere in `/1`.**

---

## 2. Question 1 — Architectural Evaluation of Lookup, Table & Cache Candidates

### Q1(a) Memory Read Cache (`PAGE` Bus Provider vs KZG-Committed Table vs Uncached Merkle Walks)

**Verdict: ADOPT an intra-leaf `PAGE` read-cache provider on the existing 2-lane `bus` (`lookup_any`), blocked in 8-record segments matching `SegMemConfig`, with causal same-leaf write-forwarding.**

1. **Why uncached per-read Merkle walks lose:** In `/1`, a single step can read up to 2 `expressions` rows + 2 `Const`/`Data` records (`equal_differ`, `BHelperScalarArithmetic`, `BHelperEqualField`) + 1 popped frame/task (`c6_dispatch_step.rs:629-689, 755-813`). At depth 20–32, 3 uncached walks per step cost `~60–96` Poseidon permutations (`4,200–6,720` rows/step `ESTIMATE`), or **`~67,000–107,000` leaf rows at $S = 16$** ($k_{\text{leaf}} = 17$, failing `k_leaf <= 16`).
2. **Why KZG-committed memory tables lose:** Fixed `TableColumn`s enter `vk_leaf` (`midnight-proofs-0.8.3/src/plonk/keygen.rs`), which is illegal for input or runtime memory under `GP-1`. Dynamic KZG committed instances (`assigned_committed_instances` in `verifier_gadget.rs:278`) cap total addressable rows at $2^{k_{\text{leaf}}} \le 65,536$ (violating `GP-1` unbounded memory), add variable-base G1 points to the Node's `collapse()` MSM, and still require a lookup argument (`+22,600` Node rows) to index at variable addresses.
3. **How the intra-leaf `PAGE` cache works at 0 new lookups and 0 new advice columns:**
   - **Cache-miss fill (`PAGE` region):** A leaf witnesses up to $P_{\text{rom}}$ static 8-record lines and $P_{\text{mem}}$ dynamic 8-record lines (or individual records). Each active line authenticates **once** via `WalkConfig` (`m3.rs:101-190`) against `prog_root` (`ROM`) or `pre_d.mem_root` (`MEM`) and emits its 8 records onto the **existing** `bus` (`q_prov = 1`, `bus.rs:88-92`).
   - **Cache hits:** Because `bus.rs:50-61` uses Halo2 `lookup_any` (set inclusion with arbitrary multiplicity, `plonk/lookup.rs:64-67`), **every subsequent read of any cached record in the same $S$-step leaf costs 0 Merkle walks, 0 Poseidon permutations, and 0 extra `CORE` rows** (it rides one of the 2 `bus` request lanes already present in the 13-row `Narrow` slot, `rfcore.rs:206-215`).
   - **Causal same-leaf write-forwarding without timestamps:** Records allocated during the current leaf are already published on `bus` by `SegMemConfig` (`mem.rs:193-199`). In general read/write RAM, serving reads from an unsorted intra-leaf write buffer without timestamps would allow a read at slot $t_1$ to read a value written later at slot $t_2 > t_1$. **In Sherlock's write-once dense arenas (`SI-12..SI-14`), a single range check `addr < slot_len[arena]` completely prevents time-travel reads:** at slot $t_1$, `slot_len[arena]` equals the number of records allocated strictly before $t_1$, and any record allocated at $t_2 \ge t_1$ receives `addr >= slot_len[arena](t_1)` (`owner_runtime.rs:3487-3600`). Thus same-leaf read-after-write hits cost **0 `PAGE` openings** and require **zero bus timestamp columns**.
   - **8-record cache-line spatial locality:** Because `children`, `parameters`, `ctor_fields`, `constant_fields`, `data_fields`, and `value_slots` are read contiguously as `start + k` (`families/environment.rs:454, 729`; `structural.rs:149, 306`), opening an aligned 8-record segment (`16` record perms + `7` internal perms + `17..32` walk perms = `40..55` perms for 8 records) amortizes to **`5–7` permutations (`350–490` rows `ESTIMATE`) per sequentially read record** — a **3.5x–4.5x reduction** over single-record openings.

---

### Q1(b) Run-Wide Offline Memory Checking (Challenge Fingerprints vs Elliptic-Curve Multiset Hashing)

**Verdict: REJECT run-wide offline memory checking across leaves (both Schwartz-Zippel/LogUp challenge accumulators and challenge-free Elliptic-Curve Multiset Hashing).**

We evaluate both cryptographic families of cross-shard offline memory checking (Blum et al. BEGKN92):

#### Family 1: Challenge-Based Grand-Product / LogUp Sums Across Leaves
In single-pass STARK continuations (e.g., SP1 v3, OpenVM volatile mode), shards emit memory access tuples into a read multiset $RS$ and write multiset $WS$ compressed with verifier challenges $(\theta, \gamma)$ into a running product $\prod (t_i + \gamma)$ or logarithmic-derivative sum $\sum m_i / (\gamma - t_i)$ (`https://eprint.iacr.org/2022/1530`). Even exploiting Sherlock's **write-once** property — which eliminates timestamps and reduces memory checking to set inclusion $RS \subseteq WS$, implementable via LogUp with prover-chosen per-write read multiplicities $m_w \in \mathbb{F}_q$ — this architecture fails on four independent grounds on Midnight's stack:
1. **Hard failure in `VerifierGadget` (`phases == 1`):** Computing $\prod (t_i + \gamma)$ or $\sum m_i / (\gamma - t_i)$ inside a leaf requires committing the memory trace and multiplicities $m_w$ in `FirstPhase`, squeezing $\gamma$, and assigning the running accumulator in `SecondPhase`. `midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:16, 309-310` explicitly executes `assert_eq!(cs.phases().count(), 1)` and panics on any multi-phase leaf circuit.
2. **Incompatibility with causal IVC and DAG PCD (`GP-2`, `GP-4`):** Soundness requires $\gamma$ to be unpredictable until **every** leaf in the run has committed its memory accesses and final multiplicities $m_w$:
   - *Write-once multiplicity causality:* When Leaf 1 allocates `Data(0)`, the prover **cannot know** how many times `Data(0)` will be read in Leaves $2 \dots N$ unless the entire $N$-leaf run is executed before Leaf 1 is proved (breaking streaming IVC and cross-run reuse `GP-2`).
   - *Fiat-Shamir binding:* Even with a two-pass host witness generator, Leaf 1's proof cannot bind $\gamma$ to Leaves $2 \dots N$ unless the Node carries Commitment-to-Transcript hashes of all leaves and a **second recursive pass** (or deferred randomness tree) checks them — doubling recursion complexity or breaking `GP-4` DAG composition (`research/sherlock-gp-architecture-20261110/architecture.md:274, 555`).

#### Family 2: Challenge-Free Elliptic-Curve Multiset Hashing (ECMH, as in SP1 V4 Turbo & Ziren)
To avoid cross-shard challenges, SP1 V4 Turbo and Ziren (`https://docs.zkm.io/design/memory-checking.html`) map each tuple $(a, v, c)$ deterministically via hash-to-curve $H_{\mathcal{G}}(a, v, c)$ into an embedded curve group (for Midnight, **Jubjub** over $\mathbb{F}_q$) and sum points homomorphically across shards ($M_{WS} - M_{RS}$), requiring **zero user challenge phases**. However, ECMH still loses decisively on Sherlock:
1. **Write-once multiplicity mismatch forces read-re-emission:** An abelian group sum checks **multiset equality** ($\text{count}_{WS}(x) = \text{count}_{RS}(x)$), NOT arbitrary-multiplicity set membership. Because a write-once record $(a, v)$ is allocated once but read $k \ge 0$ times, ECMH must attach an incrementing access counter $(a, v, c \to c+1)$ to every read — turning read-only immutable lookups back into read-modify-write state updates!
2. **Higher per-read leaf cost than `PAGE` + `bus`:** Each read consumes $(a, v, c_{\text{prev}})$ and emits $(a, v, c_{\text{now}})$, requiring **two** hash-to-Jubjub evaluations (`>= 140` Poseidon rows + curve-point validation + Edwards additions + `c_prev < c_now` range check `ESTIMATE`) **on every single read**, whereas `PAGE` + `bus` pays **0 Poseidon rows** on every intra-leaf cache hit.
3. **`O(|MEM|)` finalization scan breaks `O(1)` Halt and `GP-1`:** At run termination, `MemoryGlobalFinal` (`https://docs.zkm.io/design/memory-checking.html`) must hash and subtract the final tuple $(a, v, c_{\text{last}})$ of **every cell ever allocated during the run**. A long run allocating $10^7$ cells would require thousands of synthetic post-Halt retirement leaves, whereas `M3` Merkle roots finalize in **`O(1)` (`0` retirement leaves)** at Halt.
4. **`SRC-023` & Decider pressure:** Carrying Jubjub accumulator points across `IVC_STATE` (`crates/mori-recursion/src/ivc.rs:68`) widens `w_instance` in `RunDeciderRelation` (`crates/mori-recursion/src/run_decider.rs:91-97`), directly consuming the Decider's `16,109` table-row margin.

---

### Q1(c) [LEAD] Instruction & Decode Tables (`DEC`, `PROG`, `Const`, `TypeIdC`, `CtorIdC`, `fieldPaths`, `ty_d`)

#### 1. `DEC` Table Packing for All 108 Sherlock `/1` Rules

**Why naive flat `DEC` fails (`dissent.md` risk confirmed with exact numbers):**  
Inspection of `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:720-1255` and `c6_dispatch_partition.rs:77-147` (`testdata/dispatch/partition.json:2, 113-490`) shows that `/1` rule selection is **not** a function of compact register enums. Dispatch depends on **36 split roots**, including **18 non-enum predicates**:
- **2 dereferenced arena-value predicates** evaluated *before* dispatch completes:
  1. `equal_differ` (`c6_dispatch_step.rs:755-813, 1206-1216`): dereferences **both** `left` and `right` from `Const`/`Data` when `helper == Equal && status == Pending && task == EqualTask(Enter)` and compares `(kind_code, uint_128, bool_1, type_id, ctor_id)` to split `BHelperEqualDifferent` vs `BHelperEqualPair`.
  2. `validate_marker` (`:720-754, 1176-1186`): dereferences `r` from `Const`/`Data` when `helper == Validate && status == Pending && task == ValidateTask(Enter)` and tests `kind_code == 7` (`U128OutOfRange`) to split `BHelperValidateInvalid` vs `BHelperValidateOccurrence`.
- **5 relational `u128`/`usize` cursor comparisons (`order_atoms`, `c6_dispatch_partition.rs:116-147`):** `depth < stack_limit` (`PushAccept` vs `PushRefuse`), `Find.row_cursor < fn_len` (`FindRow` vs `FindEmpty`), `Append.src_cursor < src.len` (`AppendCell` vs `AppendEmpty`), and `(Bind.p_cursor < params.len, Bind.a_cursor < args.len)` (splitting `BindPair`, `BindEmpty`, `BindArity`).
- **11 wide-lane zero/presence tests:** `remaining_work == 0` (`P2`), `depth == 0` (`P1` `pub_val`), `task_live == 0`, `Cases.arms.len != 0`, `ArgsF.rest.len == 0`, `CtorFieldsF.rest.len == 0`, `eval_row` slice/option flags (`Call`, `Construct`, `Match`), and `origin_row` `Match.plan.is_some()` (`SI-20`, `:878, 1003, 1233`).

If all boolean/enum atoms are concatenated without mode muxing, the flat syndrome is **39–90 bits** ($2^{39}$ rows). Even splitting into `busy` (`helper_tag != 0`, 18 bits: `mode[1] || outcome_z[1] || fuel_z[1] || site_ok[1] || helper_tag[4] || res_none[1] || helper_mux[9]`) and `idle` (`helper_tag == 0`, 22 bits: `mode[1] || outcome_z[1] || fuel_z[1] || depth_z[1] || ctl_tag[4] || site_mux[7] || res_tag[5] || res_pay[2]`) and expanding don't-cares naively yields **$2^{18} + 2^{22} = 4,456,448$ table rows (`ESTIMATE`)** — forcing $k_{\text{leaf}} = 23$ and failing `k_leaf <= 16` by 7 orders of two!

**Optimal `DEC` Architecture — Gated Syndrome Normalization + 54-Bit Packed Microcode (`<= 2,048` rows):**  
`c6_dispatch_partition.rs:320-430` (`partition.json:2, 113`) already proves via decision-cube partitioning that the entire `/1` dispatch space decomposes into **3,153 disjoint cubes** (`1,920` rule-selecting + `1,233` refusal), and `1,300` of the `1,920` rule cubes are merely the `50` active helper rules crossed with `27` valid `Site`s (`c6_dispatch_step.rs:861-869` requires only `site_defined == 1` when `helper != None`). Furthermore, **21 of the 108 rules have internal data-dependent sub-branches** (10 `GuardId` rules in `shared.rs:347-358`, `owner_runtime.rs:412-499`, such as `EnvGetCellHit`, `FindRowHit`, `BindPairTypeOk`, `ValidateOccChildren`, `EqualPairCompound`, `MatchArmShape`, plus 11 deterministic payload muxes), yielding **`~142` unconditional micro-rules (`ESTIMATE`)**.

Because gate expressions in `CORE` cost **~0 Node rows** (`experiments-2.md:120-121`), we split decode cleanly between gates and a compact fixed table:
1. **In-slot gate normalization (0 lookups, 0 `TableColumn`s):**
   - Compute the 11 zero/non-zero booleans via standard field-inverse witnesses `z = 1 - x * inv` with `x * z = 0` (degree 2), `equal_differ` via sum-of-differing-lane inverses, and the cursor `<` bit via the slot's existing `ALU` `OP_LT` request (at most 1 active cursor comparison fires on any step — in `Bind`, `p_cursor == params.len` and `a_cursor == args.len` are zero-inverse tests on `len - cursor` since cursors satisfy `cursor <= len`!).
   - Compress `(ctl_tag, eval_tag, eval_flags, top_tag, top_flags, depth_z, depth_lt)` into `site_id` $\in \{0 \dots 27\}$ (`5 bits`, where `0 = invalid_site`) using a degree-$\le 5$ polynomial gate (or 32-row sub-entry in the shared fixed table).
   - Form a **11-bit canonical syndrome `syn_key` (`< 2,048` valid entries `ESTIMATE`)**:
     - When `helper_tag == 0` (`idle`): `(0, p0_p1_p2_code[2b], site_id[5b], res_class[3b])` = **11 bits (`<= 1,024` rows)** (since each `Site` only distinguishes `result == None` vs the 1–3 valid `HelperResultC` outcomes for that site, `c6_dispatch_step.rs:922-1069`!).
     - When `helper_tag != 0` (`busy`): `(1, p0_p2_siteok[1b], helper_tag_idx[4b], compact_helper_state[5b])` = **11 bits (`<= 1,024` rows)** (each individual helper `H_1..H_9` has only `2..9` rules and `<= 24` reachable `(phase, status, cmp, sub_branch)` states!).
2. **Single-Scalar 54-Bit Packed Microcode Output (`ctrl_word`):**  
   Instead of adding 10+ `TableColumn`s (`+64,620` Node rows!), `DEC` outputs **one 54-bit `F_q` scalar `ctrl_word`** encoding all deterministic micro-controls for that micro-rule (`ESTIMATE`):
   - `rule_id` (`7b`, `0..=108`, `0` = disabled slot per `models/sherlock/quint/leafSlots.qnt:45`),
   - `acct_mode` (`3b`: `Disabled, PubVal, PubErr, PubExh, LegacyComplete, HelperEntry, HelperInternal, HelperExit`, `owner_runtime.rs:2978-3005`),
   - `visits_inc` (`1b`) + `ref_cursor_inc` (`1b`) (`:1777-1792, 3448-3454`),
   - `ctl_mux` (`4b`) + `origin_mux` (`2b`) + `env_mux` (`2b`),
   - `helper_mode` (`4b`) + `result_mode` (`5b`),
   - `frame_op` (`2b`: `None, Push, Pop, RewriteTop`) + `task_op` (`3b`: `None, Push, Pop` + `rewrite_top`),
   - `arena_alloc_mode` (`4b`: `None, Data0, Data1, Data2, Env1, Slot1`),
   - `alu_sel` (`3b`) + `bus0_op` (`4b`) + `bus1_op` (`4b`) + `sub_guard_expect` (`5b`).
   In `CORE`'s 13-row `Narrow` slot, the prover witnesses the unpacked sub-codes into spare advice cells (`Arch::Narrow` has spare cells in rows 4–12 at **0 Node cost**, `experiments-2.md:143-144`), constrains each unpacked cell's small domain ($b(b-1)=0$ or `VSel` $L_c$ polynomial, degree $\le 5$), and checks the linear reconstruction $\sum 2^{\text{offset}_j} c_j = \text{ctrl\_word}$ (degree 1)! Because $54 < 254$, binary/bounded reconstruction in $\mathbb{F}_q$ is **bijective (zero modular wraparound)**.

#### 2. `PROG`, `Const`, `TypeIdC`, `CtorIdC`, `fieldPaths`, and `ty_d` Under `GP-1`

| Table / Datum | Fixed `TableColumn`? | KZG Committed Instance? | `prog_root` Merkle + `bus` `ROM` Cache? | Justification & Exact Handling |
|---|---|---|---|---|
| **Builtin `TypeIdC` `0..=3` & `CtorIdC` `0..=3`** (`UInt, Bool, List, Tree`; `Nil, Cons, Leaf, Branch`, `shared/adt.rs:6-40`) | **YES** (inline gates or 8 rows in shared table) | No | No (`0` bus/Merkle reads!) | Universal across **every** Sherlock `/1` program (`GP-1`-invariant). Builtin arities and field sorts (`Cons: [UInt, List]`, `Leaf: [UInt]`, `Branch: [Tree, Tree]`) resolve in `CORE` with 0 `ROM` traffic. |
| **`expressions`, `children`, `functions`, `parameters`, `arms`, `patterns`** (`c6_phys_program.rs:432-572`) | **FORBIDDEN** (violates `GP-1`, `chipLeaf.qnt:525`) | **REJECT** (caps AST size $\le 2^{k_{\text{leaf}}}$, adds Node MSM points) | **ADOPT** (8-row `ROM` cache lines under `prog_root`) | Program-dependent and unbounded in size under `GP-1` (`general-purpose.md:15`). In `fact(3)` and `list123`, 16 steps touch only **3–6** distinct `expressions` rows and 64 steps touch **5.75–11** (`experiments.md:208-210`) — **70%–90% cache hit rate (`ESTIMATE`)**. |
| **`constants` & `constant_fields`** (`:647-734`) | **FORBIDDEN** (`GP-1`) | **REJECT** | **ADOPT** (`ROM` under `prog_root`, **with precomputed `vd` column**) | Storing each constant's `sherlock-value/1` digest `vd` directly in its `constants` row (`design.md:193`) makes `Literal` evaluation and `XAdtEvalConstructNullary` **zero-hash** at runtime. |
| **User `types >= 4`, `constructors >= 4`, `ctor_fields`, and `ty_d(T)`** (`:575-644`, `design.md:188`) | **FORBIDDEN** (`GP-1`) | **REJECT** | **ADOPT** (`ROM` under `prog_root`, **with precomputed `ty_d` in `types`**) | Once Carbon's `decl_schema_d` normal form is fixed (`dissent.md` `D-2`), `ty_d(T)` is a static property of the program's type declarations; storing `ty_d` inside `types` under `prog_root` avoids any runtime schema hashing. |
| **`field_paths`, `plan_nodes`, `plan_cases`** (`:737-805`) | **N/A in `/1`** | **N/A in `/1`** | **0 reads in `/1`** (join `ROM` if `/2` enables `Plan*`) | Read **solely** by excluded `XAdtHelperPlan*` rules (`owner_adt_runtime.rs:747-981`). Zero circuit cost in `/1`. |

---

### Q1(d) [LEAD] `ALU` and `RANGE`: Jolt/Lasso-Style Subword Tables vs Current Gates (`alu.rs`, `range.rs`)

#### 1. Why Current Single-Witness `AluConfig` + Running-Sum `RangeConfig` Is Near-Optimal for `/1`
In `crates/mori-chip/src/alu.rs:23-125` and `range.rs:76-140`:
- Because $r \approx 2^{254.86} \gg 2^{129}$, integer addition $x + y = z + \text{flag} \cdot 2^{128}$ for $x, y, z \in [0, 2^{128})$, $\text{flag} \in \{0, 1\}$ **cannot wrap modulo $r$**. Since inputs $x, y$ come from authenticated registers/records already $< 2^{128}$, `AluConfig` needs to range-check **only ONE 128-bit witness $w$** (`12` limbs $\times$ `1` lookup column = `12` rows) per `ADD` or `LT`!
- Moving from `rc = 4` limb columns (`3` rows/op, `4` lookup arguments) to `range1` (`RangeConfig::configure_running`, `12` rows/op, `1` lookup argument) **saved `55,000` Node rows** (`891,770 -> 836,770` and `943,466 -> 888,466`, `experiments-3.md:144-147, 162-167`) at a leaf cost of `+144` rows (`48 -> 192` rows at $S = 16$).

#### 2. Completing All `/1` Operations (`SUB`, `EQ`, `NOT`, and 128-Bit Checked `MUL`) in Gates with Zero New Columns or Lookups
1. **`OP_SUB` (`u128::checked_sub`, `scalar.proposed.rs:163, 170`):** Underflow ($x < y$) must stage `PrimError(10)` (`flag = 1`, `z = 0`); success ($x \ge y$) produces $z = x - y \in [0, 2^{128})$ (`flag = 0`). One gate check `(1 - flag) * (x - y - z) + flag * z = 0` plus **1** 128-bit `RANGE` check on $w = \text{flag} \cdot (y - x - 1) + (1 - \text{flag}) \cdot z$ enforces $z = x - y < 2^{128}$ when `flag = 0` and $y - x - 1 \ge 0 \iff x < y$ when `flag = 1`. **Cost: 12 rows (reuses existing slot), 0 new columns, 0 new lookups.**
2. **`EQ` (`equal_differ`, `c6_dispatch_step.rs:785-812`) & `NOT` (`1 - b`):** Equality over $\mathbb{F}_q$ uses a zero-inverse witness `(x - y) * inv = neq`, `(x - y) * (1 - neq) = 0`, `neq * (1 - neq) = 0`. **Cost: 0 `RANGE` checks, 0 lookups.**
3. **`OP_MUL` (`u128::checked_mul`, `c6_scalar_arithmetic_step.rs:31-40, 643-664`) — Avoiding the Modulo-$r$ Soundness Trap:**
   - *Critical soundness trap:* Because $(2^{128} - 1)^2 \approx 2^{256} > 2.21 r$, checking $x \cdot y \equiv z \pmod r$ with $z < 2^{128}$ and `flag = 0` is **unsound**: any composite $x \cdot y = r + z$ with factors $x, y < 2^{128}$ and $z < 2^{128}$ would forge a non-overflowing product!
   - *Exact 64-bit schoolbook gate solution:* Following `c6_scalar_arithmetic_step.rs:31-40`, write $x = x_0 + 2^{64} x_1$, $y = y_0 + 2^{64} y_1$ ($x_i, y_i < 2^{64}$), witness $x_0 y_0 = c_0 2^{64} + r_0$ ($c_0, r_0 < 2^{64}$) and $x_0 y_1 + x_1 y_0 + c_0 = c_1 2^{64} + r_1$ ($r_1 < 2^{64}$, $c_1 < 2^{65}$). Every intermediate is $< 2^{130} \ll r$. Overflow holds iff $x_1 y_1 + c_1 \neq 0$ (checked via zero-inverse!).
   - *Synergy with `sherlock-value/1` (`design.md:182`):* `sherlock-value/1` already represents every `UInt(n)` as `(lo64(n), hi64(n))`! When `Const`/`Data` records store `(lo64, hi64)` (range-checked `< 2^{64}` once at allocation), $x_0, x_1, y_0, y_1$ are **already range-checked**, so `OP_MUL` needs to range-check only `(r0: 64b, c0: 64b)` and `(r1: 64b, c1: 65b)` — **two** 128-bit `RANGE` slots (`24` rows, or `4` slots = `48` rows if decomposing $x, y$ locally; note $64\text{b} = 5 \times 12\text{b} + 4\text{b}$ uses `pad = 256 * limb_5 < 4096` in `RangeConfig`!). Provisioning $2S = 32$ `RANGE` slots per leaf (`384` rows vs `192` at $S = 16$) handles `ADD`, `SUB`, `MUL`, and `LT` across all $S$ steps with **0 new columns and 0 new lookups**.

#### 3. Quantitative Comparison: Gate `ALU` vs Jolt/Lasso Subword Tables vs Multiplexed `DEC+RANGE` Table

| Architecture | Table Rows ($k_{\text{leaf}}$ Floor) | `ALU+RANGE` Leaf Rows ($S=16$) | Lookups (`ALU`+`RANGE`+`DEC`) | `TableColumn`s | Node Row Delta vs E8 (`ESTIMATE`) | Verdict |
|---|---:|---:|---:|---:|---:|---|
| **E8 Baseline** (`alu.rs` + `range1` + separate `dec.rs`) | `4,096` ($k \ge 13$) | `192` (`ADD`/`LT` only) | `2` (`1` `range` + `1` `dec`) | `3` (`1` + `2`) | `0` (`823,466` baseline) | Incomplete (`SUB`/`MUL`/108-`DEC` missing) |
| **Jolt/Lasso 8-bit Binary Subtables** (`(op, x_b, y_b -> out_b)`, 4 lookup lanes) | `65,536`–`262,144` (**$k = 17\text{–}18$, FAILS $k \le 16$!**) | `128` (`ADD`/`SUB`/`LT`) + `1,024` (`MUL` $16 \times 16$ bytes) | `5` (`4` subword + `1` `dec`) | `6`–`14` | **`+87,186` to `+138,882`** (`> 910k–962k`) | **REJECT:** Blows $k_{\text{leaf}} > 16$, 3x limb lookups, massive Node penalty, and `/1` has 0 bitwise ops! |
| **6-bit Binary Subtables Packed into `4,096`-Row Table** | `> 8,192` (`4,096` range + $4 \times 4,096$ ops $\implies k \ge 15$) | `352`–`704` (`22` 6b chunks $\times$ 2 inputs) | `2` | `4` | `+6,462` + $2\times$ slower base leaf | **REJECT:** Overflows `4,096` table and increases leaf rows (`22` chunks vs `11` limbs). |
| **Lane C Lead: Gate `ALU` (`ADD`/`SUB`/`MUL64`/`LT`/`EQ`) + Tagged 2-Col `DEC+RANGE` Table** | **`6,145`** (`4,096` `RANGE` + `<= 2,048` `DEC` + `1` zero row; **$k = 13$ holds!**) | **`288`–`384`** (`24`–`32` 12-row `RANGE` slots) | **`1`** (`-1` lookup!) | **`2`** (`-1` `TableColumn`!) | **`-29,062`** (`823,466 -> ~794,404`) | **ADOPT:** Supports full `/1` ISA, keeps $k=13$ table floor, **saves ~29,062 Node rows!** |

**How the 2-Column Tagged `DEC+RANGE` Table Works Soundly (`-29,062` Node rows `ESTIMATE`):**  
In `RfLeaf` (`rfcore.rs:795-808`), `CORE` slots (`q_dec = 1`) and `ALU/RANGE` slots (`q_rng = 1`) occupy **strictly disjoint row intervals** (`q_rng * q_dec == 0` identically). Define a single 2-column fixed table `(tbl_key, tbl_val)` of $\le 6,145$ rows:
- Row `0`: `(0, 0)` (matched by all inactive rows where `q_rng = 0` and `q_dec = 0`).
- Rows `1..=4096` (`RANGE`): `(2^64 + v, 1)` for $v \in \{0, \dots, 4095\}$.
- Rows `4097..=4096+D` (`DEC`, $D \le 2048$): `(2 * 2^64 + syn_i, 2^64 + ctrl_i)` with `ctrl_i < 2^54`.

Configure **one** `cs.lookup` with input expressions:
$$\text{in\_key} = q_{\text{rng}} \cdot (2^{64} + a_{11}) + q_{\text{dec}} \cdot (2 \cdot 2^{64} + \text{syn}), \qquad \text{in\_val} = q_{\text{rng}} \cdot 1 + q_{\text{dec}} \cdot (2^{64} + \text{ctrl})$$
- **Degree check:** `in_key` and `in_val` have degree `2`, `tbl_key` and `tbl_val` have degree `1`, so `required_degree = max(4, 2 + 2 + 1) = 5 <= 6` (`midnight-proofs-0.8.3/src/plonk/lookup.rs:81-87`).
- **Cross-domain soundness proof:** On a `RANGE` row (`q_rng = 1, q_dec = 0`), `in_val == 1`. The **only** rows in the table with `tbl_val == 1` are Rows `1..=4096`, forcing $2^{64} + a_{11} = 2^{64} + v \implies a_{11} = v \in [0, 4096)$. On a `DEC` row (`q_rng = 0, q_dec = 1`), `ctrl` is reconstructed from bounded sub-codes (`0 <= ctrl < 2^54`), so $2^{64} + \text{ctrl} \in [2^{64}, 2^{64} + 2^{54})$ in $\mathbb{F}_q$ cannot equal `0` or `1`; hence it can **only** match Rows `4097..=4096+D`, forcing $(\text{syn}, \text{ctrl}) = (\text{syn}_i, \text{ctrl}_i)$.

---

### Q1(e) [LEAD] Hash Memoization (`HASH` / `poseidon.rs`, `sherlock-value/1` `vd`, Domain & Arity Separation)

`PoseidonCfg` accounts for **93.5% (`6,090 / 6,511`)** of E8 leaf rows (`experiments-4.md:157`). Evaluating every Poseidon consumer yields four genuine wins and one clear trap to reject:

1. **Allocation-Time Write-Once `vd` Memoization (`SI-12` DAG Order — ADOPT):**  
   By `SI-12` (`owner_runtime.rs:4753-4760`), every child `RefC` of a newly allocated `Data` record already resides in `Const` or an earlier `Data(d < i)`. Storing `vd` as an authenticated lane of every `Const` and `Data` record means:
   - **`vd` is computed ONLY in 2 of the 108 `/1` rules:** `BHelperScalarExit` and `BHelperEqualExit` (`<= 0.22` allocations/step on census average, `design.md:206`).
   - **Every value read (`Scalar*`, `Validate*`, `Equal*`, `Match*`, `BindType`) and `Halt` (`KAdminPublishValue`) perform 0 `vd` tree-walk hashes** (only the single 2-perm `outcome_d` wrapper at terminal seal, `design.md:191, 204, 209`, avoiding `~1,400` rows per node of a Halt traversal).

2. **Three Universal Zero-Hash Constants (`VD_NIL`, `VD_FALSE`, `VD_TRUE`) + Static `Const.vd` / `Types.ty_d` (ADOPT):**  
   - `vd(Nil)`, `vd(Bool(0))`, and `vd(Bool(1))` (`design.md:183-184`) are **compile-time field constants** invariant across all programs (`GP-1`-safe).
   - `BHelperEqualExit` allocates `Data::Bool { b }` on **every** `==` operation (`structural.rs:212`), and `BHelperScalarExit` allocates `Data::Bool { b }` on **every** `<` and `!` operation (`scalar.proposed.rs:228`). Selecting `vd = b ? VD_TRUE : VD_FALSE` via a degree-2 mux (`0` Poseidon rows vs `210` rows!) eliminates **100% of `vd` hashes** on all boolean results!
   - All `Const` literals and nullary ADT constructors (`XAdtEvalConstructNullary`) read precomputed `vd` from `ROM` (`0` runtime hashes).

3. **Exact Sponge-Prefix Precomputation of `(SHK_VAL, 1)` (`-70` Rows per Dynamic `vd`, Bit-Exact with `sherlock-value/1` — ADOPT):**  
   In `design.md:178-187`, every `sherlock-value/1` hash begins with the identical 2-word header `(SHK_VAL, 1)` absorbed at rate 2 into initial state `(0, 0, 0)` (`poseidon.rs:146-156`). Therefore, the post-Block-0 sponge state
   $$(C_0, C_1, C_2) = \text{PoseidonPerm}(0,\, \text{SHK\_VAL},\, 1)$$
   is a **universal constant triple**! Initializing `PoseidonCfg`'s first absorb row for `vd` from `(C_0, C_1, C_2)` instead of `(0, 0, 0)` produces **bit-for-bit identical `sherlock-value/1` digests** while **eliminating 1 full 70-row permutation on every allocated `UInt`, `Leaf`, `Cons`, `Branch`, and `Adt` field step** (`210 -> 140` rows, a **33% reduction** `ESTIMATE`)!

4. **Domain-Tagged 2-Input `HASH` Deduplication on `bus` (`poseidon.rs:268-286` — ADOPT):**  
   Currently `poseidon.rs:200-202, 268-286` hardcodes initial capacity `s[0] == 0` and `bus` `key == 2`. Generalizing `q_first` so `s[0] == domain_tag` and publishing `(CHIP_HASH = 3, domain_tag, in0, in1, out)` on the 5-lane `bus`:
   - Enforces **cryptographic sponge-capacity domain & arity separation** across Merkle internal nodes (`DOM_MERKLE = 2`), `Leaf(x)` (`DOM_VAL_LEAF`, 1-perm from `(C_0, C_1, C_2)`!), and any other 2-word hash;
   - Automatically **deduplicates identical intra-leaf hashes** via `bus`'s `lookup_any`: e.g., if a loop in an $S=64$ leaf repeatedly allocates `UInt(0)` or `UInt(1)` (or sparse `M3` walks repeatedly hash identical empty-subtree pairs), all duplicates share **one** 70-row provider slot!
   - **Soundness rule:** Disabled/padding `PoseidonCfg` permutations must either suppress `q_prov` via an active bit (`q_first * act_hash` in `bus` provider tuple, degree $1 + 1 = 2$) or compute a real permutation on `(DOM_PAD, 0, 0)` — never publishing unconstrained `out` (`H8` `disabled_serves_bus`).

5. **Cross-Leaf Hash Memoization Table (REJECT):**  
   A dedicated cross-leaf `(inputs -> digest)` Merkle cache costs `17–32` permutations (`1,190–2,240` rows) per lookup just to save `1–2` permutations (`70–140` rows) — a **10x–30x net loss**. Cross-leaf hash reuse is profitable **only** by storing `vd` inside the already-authenticated write-once `Const`/`Data` records (item 1).

---

### Q1(f) Synthesis: Recommended Hybrid Architecture (`Sherlock-L1-Tagged`)

Combining strictly the net-positive components yields a unified leaf architecture that **reduces** Node rows below the E8 baseline while supporting all 108 `/1` rules:
1. **Shared 2-Column Tagged `DEC+RANGE` Table (`6,145` rows, `1` `lookup`):** Replaces separate `dec` and `range` tables (`-1` lookup, `-1` `TableColumn` = **`-29,062` Node rows `ESTIMATE`**), resolving `[0, 4096)` limb checks and 54-bit microcode words for all `<= 2,048` normalized `/1` dispatch syndromes.
2. **Gate-Normalized Dispatch + Full `/1` Gate `ALU` (`ADD`, `SUB`, `MUL64`, `LT`, `EQ`, `NOT`):** Zero subword `ALU` tables; `(lo64, hi64)` `UInt` storage shared between `OP_MUL` and `sherlock-value/1` `vd`.
3. **Intra-Leaf `ROM` + `PAGE` Read Cache on the Existing 2-Lane `bus` (`lookup_any`):** 8-record cache-line fills from `prog_root` (`ROM`) and `pre_d.mem_root` (`MEM`), plus zero-walk same-leaf write-forwarding (`addr < slot_len[arena]`) and D7 register-cached top frame/task (`fp/top`, `tp/task`).
4. **Quadrant-Partitioned 4-Arena Merkle Memory (`Data`, `DataFields`, `Env`, `ValueSlots`) + Conditional Empty-Segment Walk Skip (`used > 0`) + Widened `TOP = 32`:** Preserves dense per-arena `start + k` indexing (`SI-12..SI-14`), decouples run length (`2^{64}` steps) from memory segments, and scales memory to $2^{35}$ cells under one frozen key.
5. **Write-Once `vd` Storage + `(SHK_VAL, 1)` Prefix State + Constant `VD_{NIL,FALSE,TRUE}` + Capacity-Tagged `HASH` Bus.**

---

## 3. Question 2 — Quantitative Cost, Soundness, Challenge & Boundary Analysis

### 3.1 Calibrated Cost Model Formulas (`experiments-2.md`, `experiments-3.md`, `experiments-4.md`)

- **Measured E8 Baseline (`experiments-4.md:80, 157-159`):**
  - **Leaf ($S = 16$):** `6,511` circuit rows, `4,096` table rows (`k_leaf = 13`), `12` advice, `13` equality (`12` advice + `1` instance), `20` fixed (`vk.cs().num_fixed_columns()`; `cost_model` reports `33` including `13` permutation commitments per `midnight-proofs-0.8.3/src/dev/cost_model.rs:244`), `4` lookups, max degree `6`.
  - **Node (`ChipNode` / `SHKIVC06`, `nr_cols = 4`, `k = 20`):** `823,466` circuit rows (`126,534` rows below the `950,000` gate; `225,110` below $2^{20} = 1,048,576$).
  - **Decider (`RunDeciderRelation`, `nr_cols = 4`, `k = 20`):** `929,166` circuit rows, **`1,032,467` table rows** (**`16,109` fixed-region margin** below $2^{20}$, `~16,103` usable after `cs.blinding_factors() + 1`, `cost_model.rs:343-349, 370`).
- **Node Row Fit Calibration Note:** In `experiments-2.md:125`, the least-squares fit
  $$\text{node\_rows}_{n_{\text{lookup}}=7} \approx 570{,}957 + 7{,}298\,n_{\text{adv}} + 8{,}189\,n_{\text{eq}} + 6{,}004\,n_{\text{fix}}$$
  was fit across 7 leaf keys that **all had $n_{\text{lookup}} = 7$** (`experiments-3.md:142-145`). Separating the `22,600` rows/lookup slope (`experiments-2.md:136`, `experiments-3.md:163-165`) gives the explicit 4-parameter form (`ESTIMATE`):
  $$\text{node\_rows} \approx 412{,}757 + 7{,}298\,n_{\text{adv}} + 8{,}189\,n_{\text{eq}} + 6{,}004\,n_{\text{fix}} + 22{,}600\,n_{\text{lookup}}$$
  (and marginal deltas from E8 `823,466` follow $\Delta\text{node} \approx +7{,}298\,\Delta n_{\text{adv}} + 8{,}189\,\Delta n_{\text{eq}} + 6{,}462\,\Delta n_{\text{fix,distinct}} + 22{,}600\,\Delta n_{\text{lookup}}$, reproducing E8 `823,466` within `< 0.05%`).
- **Why Decider `table_rows = 1,032,467` Is Leaf-Invariant Unless `node_cs` or `IVC_STATE` Grows:** In `crates/mori-recursion/src/ivc.rs:149-153, 185-196` and `crates/mori-chip/tests/e3_spike.rs:223-236`, the Leaf's fixed bases are resolved **inside the Node** (`leaf_acc.resolve_fixed_bases`), while the Node's own `ConstraintSystem` (`node_cs(4, 20)`) depends only on `(nr_cols = 4, node_k = 20)`. Consequently, **any Leaf change that keeps `node_k == 20` and `IVC_STATE` at constant scalar width leaves Decider `table_rows` at exact `1,032,467` (`+0` table rows)** (`experiments-3.md:154-155`, `experiments-4.md:159`). By contrast, pushing unresolved bases or curve points into `IVC_STATE` (`run_decider.rs:91-97, 218-220`) directly consumes the `16,109` margin.

### 3.2 Master Quantitative Comparison Table (All Projections Marked `ESTIMATE`)

*Reference baseline for full `/1` leaf rows:* An **Uncached Full `/1` Leaf** executing all 108 rules without `ROM`/`PAGE` bus caching or `vd` memoization requires per-step Merkle walks for `expressions` (`~1.5` walks/step) + `Const`/`Data`/`Env`/`Slots` (`~1.5` walks/step) = `~3` depth-20 walks/step (`~60` perms = `4,200` rows/step) + `M3` append + boundary digests $\approx$ **`56,000` leaf rows (`k = 16`) at $S = 16$** and **`215,000` leaf rows (`k = 18`, FAILS `k <= 16`) at $S = 64$** (`ESTIMATE`).

| Candidate Architecture | Leaf Rows $S = 16$ (`k`) (`ESTIMATE`) | Leaf Rows $S = 64$ (`k`) (`ESTIMATE`) | Leaf Rows Saved vs Uncached `/1` ($S=16$ / $S=64$) (`ESTIMATE`) | Leaf Key `(adv/eq/fix/lkp)` | Node Rows (`Δ` vs `823,466` E8) (`ESTIMATE`) | Decider Circuit / Table Rows (`16,109` margin) (`ESTIMATE`) | Verdict |
|---|---:|---:|---:|---:|---:|---:|---|
| **0. E8 Measured Spike** (`rfcore.rs` D7 + `M3` `TOP=17`, no `ROM`/`PAGE`) | `6,511` (`k=13`, measured) | `~12,860` (`k=14`) | *(Spike baseline, no `ROM`/`PAGE`)* | `12 / 13 / 20 / 4` | `823,466` (`0`, measured) | `929,166` / `1,032,467` (`+0`, measured) | Baseline |
| **1. Uncached Full `/1` Leaf** (per-read Merkle walks, unmemoized `vd`) | `~56,000` (`k=16`) | `~215,000` (**`k=18` FAIL**) | `0` / `0` | `12 / 13 / 22 / 4` | `~836,390` (`+12,924`) | `~930,000` / `1,032,467` (`+0`) | **FAILS $S=64$** |
| **Q1(a)+(c) Intra-Leaf `ROM` + `PAGE` Bus Cache** (`4+4` 8-rec lines at $S=16$; `8+8` at $S=64$) | `~19,800` (`k=15`) | `~44,500` (`k=16`) | **`-36,200` / `-170,500`** | `12 / 13 / 21 / 4` | `~829,928` (`+6,462` for 1 `VSel` col) | `~930,000` / `1,032,467` (`+0`) | **ADOPT** |
| **Q1(b1) Offline Memory (Challenge Grand-Sum / LogUp)** | *(Blocked: `phases=2`)* | *(Blocked: `phases=2`)* | *(Panics at `verifier_gadget.rs:310`)* | `14 / 15 / 21 / 4` (`phases=2`) | **PANIC** (if forked: `~875,000`, `+51,534`) | `~935,000` / `~1,036,500` (`-4,000` margin) | **REJECT** |
| **Q1(b2) Offline Memory (ECMH Jubjub Multiset Hash)** | `~28,500` (`k=15`) + $O(|\text{MEM}|)$ retirement leaves | `~98,000` (**`k=17` FAIL**) + retirement | `-27,500` / `-117,000` (before retirement scan!) | `14 / 15 / 24 / 4` | `~880,256` (`+56,790` + Jubjub PI lanes) | `~941,000` / `~1,041,000` (**erodes margin to `~7.5k`!**) | **REJECT** |
| **Q1(c-naive) Flat Unnormalized 10-Col `DEC` Table** | `6,511` circuit / **`4,456,448` table (`k=23` FAIL)** | `~44,500` circuit / **`4,456,448` table (`k=23` FAIL)** | `0` (table blows $k$ to `23`!) | `12 / 13 / 28 / 4` | `~875,162` (`+51,696` for `+8` `TableColumn`s) | `~930,000` / `1,032,467` (`+0`) | **REJECT** |
| **Q1(c+d Lead) Normalized `DEC-2048` + 54b Ctrl Merged into 2-Col Tagged `DEC+RANGE`** | `+0` circuit / `6,145` table (`k=13` table floor) | `+0` circuit / `6,145` table (`k=13` table floor) | Preserves $k=13$ table floor | `12 / 13 / 19 / 3` | **`~794,404` (`-29,062`: `-1` lkp, `-1` fix!)** | `~930,000` / `1,032,467` (`+0`) | **ADOPT** |
| **Q1(d-Lasso) 8-bit Binary `(op, x_b, y_b)` Subword `ALU` Tables** (4 lookup lanes) | `+950` circuit / **`262,144` table (`k=18` FAIL)** | `+3,800` circuit / **`262,144` table (`k=18` FAIL)** | **Negative** (`-950` / `-3,800` worse!) | `12 / 13 / 26 / 7` | **`~930,038` (`+106,572`: `+3` lkp, `+6` fix)** | `~930,000` / `1,032,467` (`+0`) | **REJECT** |
| **Q1(d Lead) Gate `ALU` (`ADD`/`SUB`/`MUL64`/`LT`/`EQ`) + `range1`** | `+96..192` rows (`288..384` total `ALU` rows) | `+384..768` rows (`1,152..1,536` total) | Avoids subtable blow-up | `12 / 13 / 20 / 4` (`19/3` w/ tagged table) | **`+0`** (`-29,062` w/ tagged table) | `~930,000` / `1,032,467` (`+0`) | **ADOPT** |
| **Q1(e Lead) `SI-12` Stored `vd` + `(SHK_VAL,1)` Prefix + `VD_{NIL,BOOL}` + Tagged `HASH` Bus** | **`-750` to `-1,680`** rows on alloc-heavy leaf | **`-3,000` to `-6,720`** rows on alloc-heavy leaf | **`-750..1,680` / `-3,000..6,720`** (+ `-1,400/node` at Halt) | *(No new cols/lookups)* | **`+0`** | `~930,000` / `1,032,467` (`+0`) | **ADOPT** |
| **Q1(f) Combined Hybrid (`Sherlock-L1-Tagged`, `TOP=32`, 4-Quadrant `M3` + `ROM`/`PAGE`)** | **`~21,900` (`k=15`)** | **`~51,200` (`k=16`)** | **`-34,100` (`61%`) / `-163,800` (`76%`)** vs Uncached `/1` | **`12 / 13 / 21 / 3`** | **`~807,328` (`-16,138` BELOW E8 baseline!)** | **`~930,000` / `1,032,467` (`16,109` margin intact!)** | **RECOMMENDED** |

### 3.3 Soundness Obligations, Randomness/Challenges, and Cross-Leaf/Node Boundaries

1. **Soundness Arguments Required for `Sherlock-L1-Tagged`:**
   - **Tagged Fixed Table Separation:** `q_rng * q_dec == 0` identically by disjoint row layout; `tbl_val` partitions Row `0` (`0`), `RANGE` rows (`1`), and `DEC` rows (`2^64 + ctrl_i` with `0 <= ctrl_i < 2^54`). Unpacked control cells $(c_0, \dots, c_m)$ are range/boolean-constrained in `CORE` gates before linear combination $\text{ctrl} = \sum 2^{\text{off}_j} c_j < 2^{54}$.
   - **64-Bit-Limb `MUL` Non-Wraparound:** $x = x_0 + 2^{64} x_1$, $y = y_0 + 2^{64} y_1$ with $x_i, y_i, r_0, c_0, r_1 \in [0, 2^{64})$ and $c_1 \in [0, 2^{65})$ ensure $x_0 y_0 < 2^{128}$ and $x_0 y_1 + x_1 y_0 + c_0 < 3 \cdot 2^{128} < r$, so field equality implies exact integer equality over $\mathbb{Z}$.
   - **`ROM`/`PAGE` Cache Authenticity & Causal Read Ordering:** Every `PAGE` provider row on `bus` with `q_prov = 1` either (i) belongs to an active `WalkConfig` segment whose root copy-equals `prog_root` (`CHIP_ROM`) or `pre_d.mem_root` (`CHIP_MEM_OLD`), or (ii) belongs to an active `SegMemConfig` write row (`act = 1`, `CHIP_MEM_NEW`), or (iii) emits the neutral `(0,0,0,0,0)` tuple when `act = 0`. Every `CORE` arena read at address `addr` asserts `addr < slot_len[arena]`, preventing reads of unallocated padding or later slots in the same leaf.
   - **`HASH` Capacity Domain Separation:** `PoseidonCfg` `q_first` constrains `s[0] == domain_tag` and binds `key = domain_tag` on `bus` (`domain_tag != 0`), preventing cross-domain collisions between Merkle compressions, record hashes, state digests, and `sherlock-value/1` `vd` hashes.

2. **Randomness & Challenge Budget (`verifier_gadget.rs`):**
   - Requires **zero user challenge phases** (`cs.phases().count() == 1`, satisfying `midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:310`).
   - Uses only Halo2's built-in PLONK challenges ($\theta, \beta, \gamma, y, x$) across **3 lookup arguments** (`2` `bus` `lookup_any` + `1` shared `DEC+RANGE` `lookup`), **1** `trash_challenge` (unused), and Poseidon Fiat-Shamir inside `CircuitTranscript<PoseidonState<F>>` (`crates/mori-recursion/src/ivc.rs:322`).

3. **Cross-Leaf and Node Boundary Behavior:**
   - All lookup tables (`DEC+RANGE`) and dynamic buses (`bus` for `ROM`, `PAGE`, `SegMem`, `ALU`, `HASH`) close **100% inside each single leaf**.
   - Across leaves, **only** the canonical state digest `pre_d` / `post_d` (`Poseidon` over architectural registers, `prog_root`, `mem_root`, `len_{data,fields,env,slots}`, `fp`, `tp`, `deps_d`) and step heights `(h_pre, steps)` cross into `ChipNode` (`ivc.rs:263-273`). Neither `ChipNode` nor `RunDeciderRelation` knows how many cache hits occurred inside a leaf.

---

## 4. Question 3 — Interaction with Consensus, Ceilings, `SI-12..SI-14` & `GP-1..GP-4`

### 4.1 C6 Outcome-Only Publication & `SRC-023` Partition Independence
Under C6 (`research/sherlock-c6-outcome-ledger9-review-20261110/review.md:182`; `openspec/changes/sherlock-vm-consensus-20261107/design.md:72-97, 275`), the decided claim `[prog_d, input_d, outcome_d, n_steps]` omits `rules_d` and requires **partition independence (`SRC-023`)**: chunking the same run at $S = 1$, $S = 16$, $S = 64$, or closing a leaf early when its `PAGE` or `SegMem` budget fills ($1 \le \text{steps} < S$) must produce the **exact same public claim**.
- **Intra-leaf caches (`ROM`, `PAGE`, `HASH` bus) are strictly partition-independent:** Because cache contents live solely in leaf-local advice columns and never enter `pre_d`/`post_d`, `post_d` after semantic step $h$ is a **canonical function of the reference machine state at step $h$**, regardless of where leaf boundaries fell in steps $0 \dots h$.
- **Why cross-leaf LRU/victim caches in `pre_d`/`post_d` should NOT be used:** While D7's register-resident `top` frame and `task` (`rf.rs:32-67`) preserve canonical state digests because `top` is uniquely determined by the semantic frame stack (`0` when `depth == 0`, `frames.last()` when `depth > 0`), carrying history-dependent cache-eviction state inside `pre_d`/`post_d` would make `post_d(h)` depend on leaf-chunking history — breaking witness-stream restartability and preventing two DAG proofs (`GP-4`) from joining on a shared semantic state. Keeping `ROM`/`PAGE` intra-leaf preserves **both** `SRC-023` claim invariance and state-digest canonicity.

### 4.2 Run-Length & Memory Ceiling (`dissent.md` `D3` vs `GP-1` $2^{64}$)
`dissent.md` (`D3`) identified that E8's `m3.rs` caps runs at $2^{17} = 131,072$ leaves ($2^{21}$ steps at $S = 16$) and $8 \times 2^{17} = 2^{20}$ cells, far short of `GP-1`'s $2^{64}$ step ceiling (`general-purpose.md:15`). Two surgical changes in `m3.rs` remove both bottlenecks **without adding a single Node row**:
1. **Decouple run length from memory segments (`used > 0` conditional segment advance):** In `m3.rs` today, every leaf consumes one top-tree segment index even when `used == 0` (`0` allocations). Instead, carry `seg_cursor` in state, advance `seg_cursor' = seg_cursor + (used > 0 ? 1 : 0)`, and select `post_root = (used > 0) ? walked_new_root : pre_root`. A pure-compute or register-cached loop now runs up to **`H_BITS = 64` ($2^{64} - 1$ steps, `ivc.rs:24, 79`)** consuming **zero** memory segments!
2. **Widen `WalkConfig` depth `TOP` (`m3.rs:54`):** Each additional tree level adds `2` Poseidon permutations (`140` leaf rows) and **`0` Node rows** (`WalkConfig` reuses the same 1 `VSel` column). Raising `TOP` from `17` to `32` (`+2,100` leaf rows) raises arena capacity from $2^{20}$ (`1M` cells) to **$2^{35}$ (`34.3 billion` cells)**; raising `TOP` to `61` (`+6,160` leaf rows, still `k <= 16`!) addresses **$2^{64}$ cells** under one frozen key.

### 4.3 `SI-12..SI-14` Contiguous Field Slices (`dissent.md` `D-1`)
Our 108-rule audit confirmed that `BHelperAppendCopy/Tail` (`families/environment.rs:703-775`), `BHelperBindPair` (`:454-455`), `BHelperValidateField` (`structural.rs:149`), `BHelperEqualField` (`:306-318`), `BHelperMatchOccurrence` (`:505`), and `BHelperMatchField` (`:609`) all compute slice addresses arithmetically as **`start + k`** and enforce **`fresh_id == active_count[arena]`** (`owner_runtime.rs:2405-2478, 2970-2977, 3487-3600`).
- Interleaving `Data`, `DataFields`, `Env`, and `ValueSlots` into a single shared counter breaks `start + k` unless either (a) C6 slices are rewritten into linked lists (altering rule step counts and breaking 1-to-1 C6 step correspondence) or (b) a logical-to-physical translation tree is queried on every read (doubling Merkle walks).
- **Cleanest resolution — Quadrant-Partitioned 4-Arena Merkle Tree:** Address the unified `MEM` tree by `(arena_id: 2 bits || dense_idx: 32 bits)`:
  - Quadrant `00`: `Data[0 .. len_data)`
  - Quadrant `01`: `DataFields[0 .. len_fields)` (`fields.start + k` maps **directly** to `(01 || fields.start + k)`!)
  - Quadrant `10`: `Env[0 .. len_env)`
  - Quadrant `11`: `ValueSlots[0 .. len_slots)` (`src.start + k` maps **directly** to `(11 || src.start + k)`!)
  Because `M3` (`m3.rs:195-280`) already proves an **old-segment $\to$ new-segment update** along a shared Merkle path, a leaf that appends to partially filled 8-cell tail segments simply updates the active tail segment(s) from their `pre_d.mem_root` prefix to their extended `post_d.mem_root` prefix ($W = 2$ segment writers handle any leaf touching up to 2 arenas, early-closing the leaf if a 3rd arena allocates). `SI-12..SI-14` hold **verbatim** with zero linked-list rewrites and zero translation maps.

### 4.4 Compliance with `GP-1` through `GP-4` (`general-purpose.md`)
- **`GP-1` (Universal VM):** `vk_leaf`, `vk_node`, and `vk_decider` contain **zero** program tables, input sizes, step bounds, or arena capacities (`prog_root` and `input_d` enter solely as statement digests; `DEC+RANGE` encodes only the universal `/1` ISA).
- **`GP-2` (Cross-Run Composition):** Leaf-local `ROM`/`PAGE` caches do not leak across run boundaries; `deps_d` rides cleanly in the leaf statement (`architecture.md:656`).
- **`GP-3` (Foreign Proofs via Registry):** Preserves `~142,000` Node row headroom below `950,000`, preserves **100%** of the `16,109` Decider table-row margin, and requires zero custom Halo2 phases or rotations outside `VerifierGadget`'s standard PLONK profile.
- **`GP-4` (Any Tree/DAG Shape):** Zero cross-leaf random challenges and zero post-run `MemoryGlobalFinal` retirement shards; every leaf is proved independently in arbitrary parallel tree or diamond DAG topologies.

---

## 5. Question 4 — Production zkVM Prior Art & 2025–2026 Literature

| System / Paper (Verified URL) | Memory, Lookup & Precompile Architecture | Why It Wins in Its Native Stack — and What Transfers (or Fails) on Sherlock's KZG/PLONK IVC Stack |
|---|---|---|
| **Jolt** (`https://eprint.iacr.org/2023/1217`), **Lasso** (`https://eprint.iacr.org/2023/1216`), **Twist & Shout** (`https://eprint.iacr.org/2025/105`, rev. Oct 2026) | Decomposes 64-bit RISC-V ops into structured $2^{64}\text{–}2^{128}$ virtual tables proved via multilinear sumcheck + **Shout** (read-only one-hot addressing, $\ge 8\times$ fewer nonzero commitments than Lasso at $d=2$); RAM/registers via **Twist** (one-hot increments, no grand products). | **Does NOT transfer for `ALU`:** Shout/Lasso rely on structured virtual tables with **zero table commitments** and sumcheck verification. In Halo2/KZG (`verifier_gadget.rs`), fixed tables are committed in full ($2^{16+}$ rows blows $k_{\text{leaf}} > 16$) and each lookup adds 3 G1 points (`+22,600` Node rows). |
| **SP1 / SP1 Hypercube** (`https://docs.succinct.xyz/docs/sp1/introduction`, `https://docs.succinct.xyz/docs/sp1/hypercube/logupgkr`, LogUp-GKR `https://eprint.iacr.org/2023/1284`) | Modular CPU + precompile chips connected via virtual multilinear interaction buses verified by **LogUp-GKR** ($\sum m(\vec{x})/(\rho - t(\vec{x})) = \sum 1/(\rho - a(\vec{x}))$ via un-committed GKR layer reductions); cross-shard RAM via **ECMH** (`MemoryLocal`/`MemoryGlobal`). | **Bus pattern transfers; ECMH does not:** Routing Poseidon and `ALU` over a shared 2-lane `bus` (`bus.rs`) mirrors SP1's precompile bus, but SP1's cross-shard ECMH requires read re-emission + $O(|\text{RAM}|)$ `MemoryGlobalFinal` shards that violate `O(1)` Halt on write-once memory. |
| **RISC Zero** (`https://dev.risczero.com/terminology`, `https://dev.risczero.com/api/zkvm/optimization`) | Splits sessions via **Continuations** into bounded segments; **1 kB Merkle-authenticated pages** are paged-in on first touch against `ImageID`/state root, accessed in 1 cycle locally during the segment, and dirty pages are paged-out at segment end. | **Direct architectural match (`ADOPT`):** Sherlock's intra-leaf `ROM`/`PAGE` cache + `M3` segment root update is the exact KZG/PLONK analogue of RISC Zero's Merkle paging — and **simpler** because `PROG` and `MEM` are write-once (zero dirty-page write-backs of old pages!). |
| **OpenVM** (`https://docs.openvm.dev/specs/architecture/memory/`, `https://docs.openvm.dev/specs/architecture/continuations/`) | Intra-segment BEGKN92 offline memory checking (`prev_timestamp < timestamp`, bounded $< 2^{29}$) on `MEMORY_BUS`; across multi-segment continuations, **`PersistentBoundaryChip`** authenticates touched initial/final chunks (`expand_direction` $\in \{+1, -1, 0\}$) against a **Merkle root** carried in public values. | **Direct validation of hybrid boundary design:** OpenVM proves that combining **fast intra-segment bus lookups** with **inter-segment Merkle boundary commitments** is the right split for tree-aggregated continuations (`GP-4`). |
| **Ziren (ZKM)** (`https://github.com/ProjectZKM/Ziren`, `https://docs.zkm.io/design/memory-checking.html`) | MIPS32r2 over KoalaBear ($p = 2^{31} - 2^{24} + 1$) + Plonky3/WHIR; intra-shard LogUp-GKR `Memory` bus (`MemoryLocal` + `MemoryBump` for 36 registers, 16b+10b clock diff) + cross-shard septic-extension ECMH (`MemoryGlobalInit`/`Final`). | **Register lesson transfers:** Ziren's `MemoryBump` / local register handling confirms hot architectural state (`Regs32`, D7 `top` frame & `task`) belongs in direct slot registers/local state rather than Merkle trees. |
| **Stwo / Cairo** (`https://github.com/starkware-libs/stwo-cairo`, Cairo AIR `https://eprint.iacr.org/2021/1063`, Circle STARKs `https://eprint.iacr.org/2024/278`, LogUp `https://eprint.iacr.org/2022/1530`) | **Write-once continuous relocated memory** across `prog`, frame (`fp`), allocation (`ap`), and builtin segments, verified via LogUp over Mersenne-31 Circle STARKs. | **Direct semantic match (`ADOPT`):** Cairo demonstrates the huge simplification of **write-once contiguous-segment memory**: zero timestamps, immutable `(addr, val)` bindings, and dense `base + offset` slice addressing (`SI-12..SI-14`). |
| **Valida (Lita)** (`https://github.com/valida-xyz/valida`) & **Nexus zkVM** (`https://github.com/nexus-xyz/nexus-zkvm`, `https://github.com/nexus-xyz/nexus-zkvm/tree/main/specification`) | Valida: register-less stack/RAM ISA on Plonky3 with multiplexed LogUp buses (`deg <= 3`). Nexus 3.0: modular RISC-V zkVM on Stwo Circle STARKs with localized component traces and LogUp memory buses. | **Multiplexing lesson transfers:** Multiplexing heterogeneous chip interactions onto a minimal number of lookup arguments (`2` `bus` lanes + `1` shared `DEC+RANGE` lookup = `3` total) avoids commitment bloat. |
| **2025–2026 Lookup & RAM Papers:** **LogUp\*** (`https://eprint.iacr.org/2025/946`) & **Ring Lookups / Batch RAM** (`https://eprint.iacr.org/2026/471`) | **LogUp\*** (Soukhanov 2025): indexed small-table LogUp committing **zero** extra arrays of the indexing-array length (eliminating commitment scaling with trace length). **Bootle et al. 2026**: ring Plookup/LogUp & soundness pitfalls in batch RAM updates. | Explains **why** small-table indexed lookups are nearly free in modern multilinear/STARK provers (LogUp\*, Shout) yet cost `3` full-domain KZG commitments (`+22,600` Node rows) per table in Halo2 (`midnight-proofs 0.8.3`). |

---

## 6. Question 5 — Concrete First Prototype Recommendation for `crates/mori-chip`

### 6.1 What to Prototype First (`P1`: `TaggedTableConfig` + `PageConfig` + Full `/1` `AluConfig` + Tagged `HASH`)

The highest-leverage, lowest-risk first prototype in `crates/mori-chip` retires **all three** unmeasured leaf/node risks (`PAGE` cache, 108-rule `DEC` Node cost, and 128-bit `MUL`/domain-tagged `HASH`) in one vertical slice before touching `mori-recursion`:

1. **`crates/mori-chip/src/table.rs` (`TaggedTableConfig` replacing separate `dec.rs` + `range.rs` lookups):**
   - Implement the 2-column tagged fixed table `(tbl_key, tbl_val)` of `6,145` rows (`Row 0 = (0,0)`, `Rows 1..=4096 = (2^64 + v, 1)`, `Rows 4097..=6144 = (2*2^64 + syn_i, 2^64 + ctrl_i)` with 54-bit packed `ctrl_i`), using **1 `cs.lookup`** across disjoint `q_rng` and `q_dec` selectors.
2. **`crates/mori-chip/src/alu.rs` (Full `/1` Arithmetic):**
   - Add `OP_SUB = 3` (1 `RANGE` slot) and `OP_MUL = 4` (64-bit schoolbook $x_0, x_1, y_0, y_1 \to r_0, c_0, r_1, c_1$ using 2–4 `RANGE` slots, checking `x1*y1 + c1 != 0` via zero-inverse).
3. **`crates/mori-chip/src/poseidon.rs` (Capacity-Tagged `HASH` + `(SHK_VAL, 1)` Prefix Constant):**
   - Generalize `q_first` so `s[0] == domain_tag` (bound to `bus` `key = domain_tag`), gate `q_prov` by an active bit so padding slots emit `(0,0,0,0,0)`, and add a `q_val_prefix` code initializing `(C_0, C_1, C_2) = PoseidonPerm(0, SHK_VAL, 1)` for 1–2 perm `sherlock-value/1` `vd` hashing.
4. **`crates/mori-chip/src/page.rs` (`PageConfig` Intra-Leaf `ROM` + `MEM` Read Cache):**
   - Wire `P_rom = 2` 8-record `ROM` cache lines (authenticated against `prog_root`) and `P_mem = 2` 8-record `MEM` cache lines (authenticated against `pre_d.mem_root`) onto the existing `bus` (`q_prov = 1`), plus causal `addr < slot_len` checks on `CORE` reads so same-leaf `SegMemConfig` writes forward at 0 walks.

### 6.2 Exact `cost_model` Experiment (`crates/mori-chip/tests/e3_spike.rs`)

Add an ignored probe `chip_e9_tagged_page_node_decider` mirroring `chip_e5_node` (`e3_spike.rs:802-852`) that runs `circuit_model::<F, 48, 32>` on the leaf at $S \in \{16, 64\}$, `cost_model(&ChipNode, Some(20))`, and `cost_model(&RunDeciderRelation, None)`:
- **Hard Pass Criteria:**
  1. `vk.cs().lookups().len() == 3` (down from `4`), `advice == 12`, `permutation == 13`, `fixed <= 21`, `degree <= 6`, `phases == 1`, all rotations $\in \{-1, 0, +1\}$.
  2. `leaf_k <= 15` at $S = 16$ and `leaf_k <= 16` at $S = 64$.
  3. **`node.rows <= 815,000`** (**lower** than the `823,466` E8 baseline despite adding `PAGE`, `DEC-2048`, `SUB`, and `MUL`!).
  4. **`decider.table_rows == 1,032,467`** (`0` rows consumed from the `16,109` margin).

### 6.3 Exact Quint Model & Seeded Mutants (`models/sherlock/quint/chipLeafTaggedPage.qnt`)

Extend `models/sherlock/quint/chipLeaf.qnt` (`:315-355, 702-722`) with `TaggedTable`, 64-bit `Mul`, domain-tagged `HASH`, and causal `PAGE` + write-forwarding, requiring `quint test` to catch **all 7 seeded mutants**:
1. `mutant_tag_collide`: drops `2^64` domain offset on `tbl_key`/`tbl_val` so a `RANGE` cell matches a `DEC` row (`invRangeSound` falsified).
2. `mutant_dec_unpack_unbounded`: omits boolean/domain checks on unpacked `ctrl_word` fields while keeping $\sum 2^{\text{off}_j} c_j = \text{ctrl}$ (`invRules` falsified via compensating $+2 / -1$ bit forgery).
3. `mutant_mul_mod_wrap`: checks `x * y == z (mod p)` with `z < 2^W` instead of half-width limb schoolbook (`invAluMul` falsified on composite $x \cdot y = p + z$).
4. `mutant_page_future_read`: omits `addr < slot_len[arena]` on `CORE` bus reads (`invRefine` falsified when slot $t_1$ reads a record allocated later at slot $t_2 > t_1$).
5. `mutant_page_root_swap`: authenticates a `CHIP_MEM` `PAGE` line against `prog_root` instead of `pre_d.mem_root` (`invMem` falsified).
6. `mutant_hash_domain_bleed`: sets `s[0] = 0` for both `DOM_MERKLE` and `DOM_VAL_LEAF` (`invHashSep` falsified).
7. `mutant_empty_seg_drift`: skips `M3` walk when `used == 0` without enforcing `post_root == pre_root && post_seg_cursor == pre_seg_cursor` (`invMem` falsified).

### 6.4 Exact `MockProver` Negative Tests (`crates/mori-chip/tests/`)

1. **`neg_tagged_range_4096_and_cross_tag`:** Set `a[11] = 4096` (and in a second case `a[11] = 2^64 + syn_0`) on an active `q_rng = 1` row $\implies$ `MockProver::verify()` fails on the `tagged_dec_range` lookup.
2. **`neg_dec_forged_microcode_decomposition`:** Keep `ctrl_word` equal to a valid `DEC` entry while setting `c_0 = 2, c_1 = 0` instead of `c_0 = 0, c_1 = 1` ($\Delta = 0$ in `2^0 c_0 + 2^1 c_1`) $\implies$ fails `CORE` boolean/VSel gate.
3. **`neg_alu_mul_overflow_field_wrap`:** Witness $x = 2^{127} + 3, y = 2^{127} + 5$ ($x \cdot y \ge 2^{128}$) with forged `flag = 0` (`PrimError(10)` suppressed) $\implies$ fails `q_alu` `x1*y1 + c1 == 0` zero-inverse gate.
4. **`neg_alu_sub_underflow_masked`:** Witness $x = 5, y = 9$, `flag = 0`, $z = 5 - 9 \pmod r$ $\implies$ fails `tagged_dec_range` lookup on $w$.
5. **`neg_page_tampered_rom_constant_vd`:** Corrupt one `vd` field in a cached `ROM` `constants` row while keeping `prog_root` fixed $\implies$ fails `WalkConfig` root equality against `prog_root`.
6. **`neg_page_time_travel_same_leaf_write`:** At slot 0 (`len_data == 0`), request `(CHIP_MEM, DATA, addr = 0, ...)` matching a `Data(0)` record appended later at slot 3 of the same leaf $\implies$ fails `addr < slot_len` `ALU`/`RANGE` check at slot 0.
7. **`neg_hash_disabled_slot_forges_bus_tuple`:** Leave `q_prov = 1` with arbitrary `out` on an unused `PoseidonCfg` slot (`act_hash = 0`) $\implies$ fails `act_hash` zeroing gate on `bus` provider tuple.
8. **`neg_m3_zero_used_mutates_root`:** Run a 16-step register-only leaf (`used = 0`) and set `post_d.mem_root != pre_d.mem_root` $\implies$ fails conditional root-carry equality gate.

---

## 7. What Was Not Checked

In accordance with Round 1 workspace and execution rules, the following items were **not** executed or empirically measured in this pass:
1. **No live `cargo test`, `cost_model`, `MockProver`, `quint`, `lake build`, or keygen/proving runs were executed.** All projected row counts, `k` thresholds, and Node/Decider deltas beyond previously committed measurements (`experiments.md` through `experiments-4.md`) are analytical projections marked `ESTIMATE`.
2. **Exact BDD/cube minimization count of the normalized 108-rule + 21-sub-branch `DEC` table:** While `c6_dispatch_partition.rs` (`testdata/dispatch/partition.json:2, 113`) proves the raw cube bound (`3,153` cubes; `~620` active after collapsing `site` on `helper != None`), the exact row count (`ESTIMATE` `<= 2,048`) of the 2-mode (`idle`/`busy`) `syn_key` encoding has not yet been generated by a Rust table-builder test against `partition.json`.
3. **Carbon `decl_schema_d` normal form (`dissent.md` `D-2`) and `ENTRY`/`ARG` input-loading prefix (`design.md:194, 216`):** While our `ROM` (`Types.ty_d`) and `Data`/`Fields` quadrant layout support any canonical 1-scalar `ty_d(T)` and initial prefix `(n_d, n_f)`, the upstream language-level normal form of `decl_schema_d` and `VAL_NONE` remains an open specification item outside `mori-chip`.
4. **Wall-clock proving time and RSS under mixed CPU/GPU worker contention** at $k_{\text{leaf}} = 15$ ($S = 16$) vs $k_{\text{leaf}} = 16$ ($S = 64$) (`design.md:291-308`).
