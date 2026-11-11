# Modular Sherlock Architecture — Area 1: The Processor (`BUS`, `CORE`, `DEC`, `ALU + RANGE`, `HASH`)

Date: 2026-11-11
Scope: Area 1 (`research/sherlock-final-design-20261111/ARCH-BRIEF.md:19, 23-38, 48-68`; `research/sherlock-final-design-20261111/MODULARITY.md:1-12`).

---

## 0. Conventions, Verbatim `GP-1`–`GP-4` Target, and Area 1 Proof-System Envelope

### 0.1 Citation and Estimate Conventions

- Repository paths are relative to `/home/hoskinson/Projects/Moriarty/`.
- `MP:<path>:<lines>` denotes pinned `midnight-proofs-0.8.3/src/<path>:<lines>`.
- `MC:<path>:<lines>` denotes pinned `midnight-circuits-7.3.0/src/<path>:<lines>`.
- Scalar field `F` is BLS12-381 `Fq` (`crates/mori-chip/src/lib.rs:41`), prime modulus `r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001` (`254.86` bits; `2^254 < r < 2^255`).
- Every unmeasured row count, column-fit delta, or proving-time figure is tagged `ESTIMATE`. Measured figures cite their exact source and line.

### 0.2 Verbatim Preservation of `GP-1` through `GP-4`

Area 1 preserves `GP-1` through `GP-4` (`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:7-53`; `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56-59`) verbatim without narrowing:

- **`GP-1` Universal VM (`general-purpose.md:7-17`, `requirements.md:56` `GP-001`):**
  > "One fixed key set proves the execution of every program of the Sherlock language, on every input, for every run length and every outcome. A new program needs no new key, no new contract and no new ceremony."
  - Programs and inputs are data (`prog_d`, `input_d` in public statement, never circuit constants; `general-purpose.md:11`).
  - Every outcome is provable: `Halted(v)` for every value type (as a `sherlock-value/1` canonical value digest), `Rejected(code)`, and `Exhausted(kind)` (`general-purpose.md:12`).
  - Capacity does not depend on the key: state exceeding in-leaf carriers runs against committed memory (`general-purpose.md:13`).
  - Run length is bounded only by the 64-bit step counter `h < 2^64` (`SRC-004`, `general-purpose.md:14`, `requirements.md:56`): no circuit domain, key, final proof size, leaf count, memory address counter, or segment index sets a lower limit, and a step that allocates no memory consumes no memory address. Proving may stop and resume under the same key set with identical `(prog_d, input_d, outcome_d, n_steps)`, and execution is partition-independent (`SRC-023`, `requirements.md:27`).
- **`GP-2` Cross-run composition (`general-purpose.md:20-28`, `requirements.md:57` `GP-002`):**
  > "A proven run can be consumed by another proof, and many runs can be combined into one proof."
  - Consumed claims `(prog_d', input_d', outcome_d')` are sealed terminal claims checked in-circuit; when the consumer inspects the value behind `outcome_d'`, it verifies the `sherlock-value/1` opening (`requirements.md:57`).
- **`GP-3` Foreign proofs (`general-purpose.md:31-39`, `requirements.md:58` `GP-003`):**
  > "The recursion verifies and aggregates proofs of circuits other than the Sherlock leaf. These include Compact contract circuits and other PLONK/halo2 circuits on the same curve and commitment scheme."
  - Foreign verifying keys are authenticated against a registry commitment in-circuit and mapped via an authenticated statement adapter (`requirements.md:58`).
- **`GP-4` Any shape (proof-carrying data) (`general-purpose.md:42-51`, `requirements.md:59` `GP-004`):**
  > "Aggregation is not limited to a linear chain: it covers chains, trees and DAGs of any depth, with a constant-size final proof."
  - Node arity `>= 2` plus new work under one key set; shape commitment binds address-free descriptors only (`requirements.md:59`).

**Area 1 Non-Narrowing Consequences:** No Area 1 module bakes program AST constants, `Bounds`, rule subsets, leaf indices, or step-indexed memory offsets into verifying keys (`GP-001`). `CORE` tracks `h < 2^64` (`SRC-004`), advances allocation counters strictly by real allocations (`GP-001`), preserves `SRC-023` partition independence across arbitrary leaf cuts (`AUDIT.md:67-69`), and exposes `CHIP_HASH = 6` (`DOM_CLAIM`, `SHK_VAL`) so `GP-2` claim openings execute as ordinary data-driven steps.

### 0.3 Area 1 Recursive Verifier Envelope (`VerifierGadget` Hard Constraints)

Every leaf circuit proved under Area 1 is verified inside the recursive node by `midnight-circuits-7.3.0` `VerifierGadget` (`MC:verifier/verifier_gadget.rs`). Three hard constraints govern every Area 1 gate, lookup, and table (`research/sherlock-lookup-cache-20261111/CONSENSUS.md:69-83`, `research/sherlock-runlength-consensus-20261111/AUDIT.md:89`):

1. **Strictly Single-Phase Advice (`Phase(0)` Only):** `VerifierGadget::verify` asserts `assert_eq!(cs.phases().count(), 1)` (`MC:verifier/verifier_gadget.rs:310`) and absorbs all `0..cs.num_advice_columns()` commitments in one transcript pass before squeezing internal PLONK `theta`, `beta`, `gamma`, and `trash_challenge` (`MC:verifier/verifier_gadget.rs:314-341`).
2. **Zero Verifier Challenges Inside Gates:** `MC:verifier/expressions/mod.rs:49` panics on `Expression::Challenge(_)` (`panic!("We do not suport multi-phase yet")`). No Area 1 module may allocate `Phase(1)` columns or reference `Challenge` in `create_gate` or `lookup_any`.
3. **Unit Advice/Fixed Rotations Only (`{-1, 0, +1}`):** `MC:verifier/verifier_gadget.rs:604-610` (`get_point`) maps `rotation.0` exclusively over `-1 => &x_prev`, `0 => &x`, `1 => &x_next`, panicking on `|rotation.0| > 1`. Multi-row connections (`CORE` step boundaries, `PoseidonCfg` block chaining) MUST use adjacent rows or copy constraints (`constrain_equal`), never `Rotation(+-2)` or wider.

### 0.4 Leaf Shape and Recursive Node Cost Model Baseline

- **Measured E8 Leaf Baseline (`openspec/changes/sherlock-vm-consensus-20261107/design.md:125, 130-131`; `crates/mori-chip/src/rfcore.rs:833-843`; `research/sherlock-lookup-cache-20261111/CONSENSUS.md:129-136`):**
  - E8 1-leaf recursive node (`k = 20`, gate `<= 950,000` rows, `design.md:105`): `823,466` circuit rows (`design.md:130`).
  - E8 decider (`k = 20`, gate `<= 1,000,000` circuit rows, usable `2^20 - 6 = 1,048,570` rows, `design.md:105`): `929,166` circuit rows and `1,032,467` `Pow2Range` table rows (`16,103` `Pow2Range` rows of headroom below `1,048,570`, `design.md:131`, `AUDIT.md:76`).
- **Linear Recursive Node Sensitivity (`design.md:134`, `crates/mori-chip/src/vsel.rs:4`):**
  - 1-leaf `ChipNode` row model (`ESTIMATE` for deltas off E8):
    `node_rows ≈ 570,957 + 7,298 * N_adv + 8,189 * N_eq + 6,004 * N_fix + 22,600 * N_lookup` (`~6,460` Node rows per filled fixed/table column in E6/E8 marginal measurements, `vsel.rs:4`, `CONSENSUS.md:145`).
  - 2-leaf `RunLift` node `R` (`dissent.md:88`, `CONSENSUS.md:134-136`): verifies 2 leaves (`+1` leaf verifier pass), scaling per-leaf column/lookup marginal costs by `ESTIMATE ~1.76x..1.78x` (`~40,000` `R` rows per PLONK lookup; `~11,200..11,660` `R` rows per fixed/table column).
  - Leaf row count `2^{k_leaf}` enters `ChipNode` only logarithmically (`+4` Node rows per `+1` increase in `k_leaf`, `+0` Decider rows; `design.md:135`).

---

## 1. Module `BUS` — Inter-Module Dynamic Lookup Bus, 2-Column `TaggedTableConfig`, and Copy-Cell Protocol

### 1.1 Purpose

`BUS` defines the sole inter-module communication substrate of the Sherlock leaf circuit, Partitioned into two dynamic 5-tuple `lookup_any` channels (`Bus`, `crates/mori-chip/src/bus.rs:33-62`), one merged 2-column domain-tagged static `lookup` table (`TaggedTableConfig`), and explicit boundary cell-copy ports (`constrain_equal`). It enforces module isolation (`MODULARITY.md:7-10`) so every processor and memory module is implemented, tested, and formally verified solely against its tuple contract without reading any sibling module's columns or polynomial gates.

### 1.2 Frozen Interface

#### A. Dynamic Lookup Bus Schema (`crates/mori-chip/src/bus.rs:1-62`, Extended)

Every dynamic bus tuple is a 5-element `Fq` vector `(chip, key, a, b, c)` carried over shared advice columns `a[0..5]` (channel `bus0`) and `a[5..10]` (channel `bus1`). Both channels look up against the single provider column slice `a[0..5]` (`bus.rs:50-61`):
- Request side (`q_req` selector): `(q_req * chip, q_req * key, q_req * a, q_req * b, q_req * c)`.
- Provider side (`q_prov` selector): `(q_prov * chip, q_prov * key, q_prov * a, q_prov * b, q_prov * c)`.
- Disjointness gate (`bus.rs:64`): `Expression::Constant(0)` structurally enforced by region layouter disjointness and explicit invariant `q_req(row) * q_prov(row) == 0`.

**Why `DEC` and `RANGE` Moved Off the Dynamic Bus onto 2-Column `TaggedTableConfig`:**
1. *Static vs witness-dependent tables:* `DEC` (`<= 2,048` rows) and `RANGE` (`4,096` or `16,384` rows) depend only on the Sherlock `/1` language specification, never on program or run witness (`GP-001`). Serving them over the dynamic witness bus (`a[0..5]` with `q_prov = 1`) would require materializing `6,144..18,432` advice rows plus multiplicity or set-inclusion witness assignments per leaf, wasting advice capacity under `k_leaf = 16`.
2. *Eliminating 1 PLONK lookup and 1 fixed `TableColumn`:* In the E8 spike (`rfcore.rs:839-843, 941`, `dec.rs:25-30`, `range.rs:94-102`), `RANGE` used 1 `TableColumn` + 1 PLONK `lookup` and `DEC` used 2 `TableColumn`s + 1 PLONK `lookup` (`3` `TableColumn`s, `2` static lookups, `4` leaf lookups total). Packing a 1-bit domain tag into bit `16` of the key column merges both static tables into **2 `TableColumn`s (`t_dom_key`, `t_val`) and 1 PLONK `lookup`**, cutting total leaf PLONK lookups `4 -> 3` and fixed `TableColumn`s `3 -> 2` (`ESTIMATE -29,062` 1-leaf `ChipNode` rows and `ESTIMATE -51,204..-51,662` 2-leaf `RunLift` `R` rows; `CONSENSUS.md:145, 208`).
3. *Retiring `chip = 1`:* Production reserves `chip = 1` unused (`UNASSIGNED`) so legacy spike tuples (`CHIP_ALU = 1` in `bus.rs:30`) fail fast rather than aliasing production `CHIP_ALU = 2`.

#### B. Frozen Dynamic `chip` ID Registry (`0, 2, 3, 4, 5, 6`)

| `chip` | Constant | Requester(s) | Provider(s) | `key` Layout & Bit-Width | `a` (`col 2`) | `b` (`col 3`) | `c` (`col 4`) | Range-Check Owner |
|---|---|---|---|---|---|---|---|---|
| `0` | `CHIP_NOP` | `CORE` (unused lanes, `is_pad = 1` rows) | Any row with `q_prov = 0` (evaluates to `(0,0,0,0,0)`) | `0` (`0` bits) | `0` | `0` | `0` | `CORE`: constrains `(key, a, b, c) == (0,0,0,0)` whenever `chip == 0` (`D8` + requester zero-gate). |
| `1` | *(reserved)* | None | None (`0` provider rows) | Unsatisfiable if `q_req = 1` | — | — | — | Structural absence on `q_prov = 1` rows. |
| `2` | `CHIP_ALU` | `CORE`, `MEM` (bounds, if not inline) | `ALU` (`alu.rs`) | `op_key = op \| (ovf << 4)` (`5` bits: `op in 1..=9`, `ovf in {0,1}`) | `x` (`u128`, `< 2^128`) | `y` (`u128`, `< 2^128`) | `w` (`u128`, `< 2^128`) | **`ALU` unconditionally** range-checks `x, y, w < 2^128` via `RANGE` (`tag = 0`), plus boolean `ovf` and opcode selector `op in 1..=9` (`Exploit D1` fix). |
| `3` | `CHIP_MEM_ALLOC` | `CORE` (heap/env/list/ctor cell append) | `MEM` (`WindowCfg`, `membackend.rs`) | `mem_key = (arena << 67) \| (addr << 3) \| kind` (`70` bits: `arena in 0..=3`, `addr < 2^64`, `kind in 1..=7`) | `f0` (`< r`; `< 2^128` when scalar) | `f1` (`< r`; `< 2^128` when scalar/ptr) | `f2` (`< r`; `< 2^128` when scalar/len) | `MEM` checks `addr == len_curr` (sequential append, `GP-001`), `addr < 2^64` (`RANGE`), `arena`, `kind`; `CORE` + `ALU` range-check scalar payloads `< 2^128` on scalar write rules. |
| `4` | `CHIP_MEM_READ` | `CORE`, `VALUE DIGEST` | Disjoint split (`D2`): `MEM` (`base <= addr < len_post`) $\sqcup$ `MEMORY BACKEND` (`addr < base`) | `mem_key = (arena << 67) \| (addr << 3) \| kind` (`70` bits: `arena in 0..=3`, `addr < 2^64`, `kind in 1..=7`) | `f0` (`< r`) | `f1` (`< r`) | `f2` (`< r`) | `MEM` / `MEMORY BACKEND` owns `addr` range (`base <= addr < len_post` vs `addr < base`, `D2`, `membackend.rs:29, 258-278`); `CORE` (via `ALU` `OP_LT_U`) enforces `addr < len_curr` before read (`AUDIT.md:50`). |
| `5` | `CHIP_ROM` | `CORE` (AST, const, decl, dispatch plan rows) | `ROM` (`PAGE` static cache under `prog_d`) | `rom_key = (tbl_id << 32) \| row_idx` (`36` bits: `tbl_id in 1..=14`, `row_idx < 2^32`) | `col0` (`< 2^128` or `< r` digest limb) | `col1` (`< 2^128` or `< r`) | `col2` (`< 2^128` or `< r`) | `ROM` enforces `tbl_id in 1..=14`, `row_idx = 8 * blk_idx + off` (`blk_idx < 2^29`, `off in 0..=7`), and authenticates `(col0, col1, col2)` to `prog_d` (`DOM_LEAF8` / `DOM_PROG`); compiler admission guarantees scalar column ranges. |
| `6` | `CHIP_HASH` | `CORE` (`FRAMES`/`TASKS` push/pop, `DOM_CLAIM`), `VALUE DIGEST` (`SHK_VAL`) | `HASH` (`PoseidonCfg`, `poseidon.rs`) | `hash_key = (dom << 32) \| (sub << 16) \| absorb_len` (`40` bits: `dom in {9,10,11,12}`, `sub < 2^16`, `absorb_len in 1..=16`) | `in0` (`< r`) | `in1` (`< r`, or chain `prev_d`) | `out_d` (`< r` Poseidon rate-0 squeeze) | `HASH` enforces fixed/checked `(dom, sub, absorb_len)` mapped into `st[2] = absorb_len + (((dom << 16) \| sub) << 32)` (`dense.rs:60-62`, `poseidon.rs:101-110`) and executes all 68 Poseidon rounds unconditionally (`Exploit E3` fix). |

#### C. 4 Dynamic Bus Requests per Step at `+0` Node Rows (`Arch::Narrow`)

`Bus` exposes two `lookup_any` arguments (`bus0` on `a[0..5]`, `bus1` on `a[5..10]`, `bus.rs:50-61`). In the 7-row `Arch::Narrow` step layout (Section 2.2; compacted from the 13-row spike `rfcore.rs:169-221` where rows `6, 10` were passive copy-replicas of row `2`, `rfcore.rs:198-200`), `CORE` asserts `q_req = 1` on **two adjacent rows per step**:
- Step row `+5`: `bus0 = L0` (`a[0..5]`), `bus1 = L1` (`a[5..10]`).
- Step row `+6`: `bus0 = L2` (`a[0..5]`), `bus1 = L3` (`a[5..10]`).

Because Halo2 `lookup_any` evaluates one vector polynomial per registered lookup argument across all rows where `q_req = 1` (`MP:plonk/lookup.rs`), enabling `q_req = 1` on two rows per step reuses the exact same 2 `lookup_any` arguments, the same 10 advice columns `a[0..10]`, and the same `q_req` fixed column: **`4` dynamic bus requests per VM step at `+0` advice columns, `+0` fixed columns, `+0` PLONK lookups, and `+0` recursive `ChipNode` rows** (`CONSENSUS.md:26, 220`).

#### D. 2-Column Domain-Tagged Fixed Lookup (`TaggedTableConfig`)

`TaggedTableConfig` registers **one** Halo2 `cs.lookup` over **two** `TableColumn`s `(t_dom_key, t_val)`:
- **Domain encoding (`t_dom_key = (tag << 16) + key`):**
  - Row `0`: `(0, 0)` (`tag = 0, key = 0, val = 0`), absorbing every row where `q_rng == 0 && q_dec == 0`.
  - `tag = 0` (`RANGE`, rows `0..2^B`): `t_dom_key = limb` (`0 <= limb < 2^B`, `B = 12` default `4,096` rows or `B = 14` `16,384` rows; since `B <= 14 < 16`, `t_dom_key < 2^16` never collides with `tag = 1`), `t_val = 0`.
  - `tag = 1` (`DEC`, `N_dec <= 2,048` rows at indices `2^B .. 2^B + N_dec`): `t_dom_key = 65536 + packed_key` (`0 <= packed_key < 2^16`, so `65536 <= t_dom_key < 131072`), `t_val = ctrl_word` (`54`-bit packed microcode word, `< 2^54 < r`).
- **Mutual-exclusion & query expression (`q_rng * q_dec == 0`):**
  - Query on shared lookup cell `(x_lookup, v_lookup)` at `a[10], a[11]` (or per-region designated columns):
    - `expr_dom_key = q_rng * x_rng + q_dec * (Expression::Constant(F::from(65536)) + key_dec)`
    - `expr_val = q_dec * ctrl_dec` (when `q_rng == 1`, `q_dec == 0` so `expr_val == 0`, matching `t_val == 0` on all `tag = 0` rows).
  - Structural disjointness gate: `q_rng(row) * q_dec(row) == 0` enforced by region non-overlap (`RANGE` rows vs `CORE` decode row `+0`) and verified by a degree-2 fixed-column audit check at keygen.

#### E. Copy-Constrained Boundary Port Cells (`constrain_equal`)

Cross-module state handoffs outside per-step `Bus` queries use `Region::constrain_equal` (`MP:circuit/layouter.rs`) on `AssignedCell<F, F>` handles:
1. `ArchStatePort` (`CORE` slot `0` pre-state and slot `S - 1` post-state $\leftrightarrow$ `BOUNDARY COMMITMENT`): exact `NDIG` canonical state lanes (`prog_d`, `input_d`, `h`, `ctl`, `outcome`, `fuel_lo`, `fuel_hi`, `consumed_work`, `helper_work`, `helper_visits`, `reference_cursor`, `stack_limit`, `fcount`, `tcount`, `fp_hash`, `tp_hash`, `top_frame[0..4]`, `top_task[0..4]`, `env_ptr`, `alloc_len[0..4]`, `helper_packed[0..6]`, `result[0..3]`, `mem_root_or_acc`, `tail_cells[0..24]`). `CORE` owns internal step transitions; `BOUNDARY COMMITMENT` binds `pre_d` / `post_d` to public instance inputs.
2. `WindowBoundaryPort` (`CORE` $\leftrightarrow$ `MEM`): `len_pre`, `len_post`, and 8-record tail register array (`tail_pre[0..8]`, `tail_post[0..8]`).
3. `HashFixedPort` (`BOUNDARY COMMITMENT`, `MEM` flush, `ROM` Merkle $\leftrightarrow$ `HASH`): direct `AssignedCell<F, F>` chaining into dedicated `PoseidonCfg` 70-row blocks (`q_prov = 0` on bus, zero dynamic bus overhead; `poseidon.rs:433-460`).

### 1.3 Invariants

**Guaranteed by `BUS`:**
1. **Lookup Soundness (`LookupSound`, `bus.rs:50-61`):** Every row with `q_req == 1` has each of its two 5-tuples `(chip, key, a, b, c)` equal to `(0, 0, 0, 0, 0)` (matched by any `q_prov == 0` row) or equal to `(chip, key, a, b, c)` on at least one row with `q_prov == 1`.
2. **Static Domain Isolation (`TaggedTableConfig`):** Because `0 <= limb < 2^14 < 65536` for `tag = 0` and `65536 <= 65536 + packed_key < 131072` for `tag = 1`, no `q_rng = 1` query can ever match a `DEC` row and no `q_dec = 1` query (with `0 <= packed_key < 65536`) can ever match a `RANGE` row, including `(0, 0)`.
3. **Single-Phase Unit-Rotation Compliance:** `Bus` and `TaggedTableConfig` register zero `Phase(1)` columns, zero `Expression::Challenge` nodes, and exclusively `Rotation::cur()` (`0`) queries (`MC:verifier/verifier_gadget.rs:310, 604-610`).

**Mandatory Universal Provider Rules (Assumed from Every Provider Module & Enforced by Per-Module Tests, `design.md:94-102` `D1, D4, D8`, `bus.rs:12-20`):**
1. **Non-Zero Fixed Chip Constant (`D4`, `D8`):** Every active provider row (`q_prov == 1` and slot active `act == 1`) constrains `chip == CHIP_ID` for a compile-time constant `CHIP_ID in {2, 3, 4, 5, 6}` (`CHIP_ID != 0`). No active provider row may emit a witness-chosen `chip` or `chip == 0`.
2. **Inactive Provider Zero-Gating (`membackend.rs:258-264`):** Any provider region that sets fixed `q_prov = 1` across `N` potential slots with a boolean activity witness `act in {0, 1}` MUST gate all five columns to zero when `act == 0`: `chip = act * CHIP_ID`, `(1 - act) * key == 0`, `(1 - act) * a == 0`, `(1 - act) * b == 0`, `(1 - act) * c == 0`, collapsing unused slots to the canonical idle tuple `(0, 0, 0, 0, 0)`.
3. **Requester Unused-Lane Zero-Gating (`D8`):** Whenever a requester lane sets `chip == 0`, the requester MUST constrain `key == 0`, `a == 0`, `b == 0`, `c == 0` (otherwise `(0, k, a, b, c)` with `(k,a,b,c) != 0` fails lookup, or if any provider leaked `chip = 0`, would forge responses).
4. **Functional Determinism & Partition Disjointness (`D2`, `bus.rs:18-20`):** Within each `(chip, key)` (for `CHIP_MEM_ALLOC`, `CHIP_MEM_READ`, `CHIP_ROM`) or `(chip, key, a, b)` (for `CHIP_ALU`, `CHIP_HASH`), the remaining tuple fields are a deterministic partial function of the key/inputs. When two modules share one `chip` (`CHIP_MEM_READ = 4` shared by `MEM` and `MEMORY BACKEND`), their key predicates MUST be provably disjoint (`addr >= base` vs `addr < base`, `D2`).
5. **Single Declared Range-Check Owner:** Every tuple field has exactly one owner listed in Section 1.2.B; no provider may assume requester-side range bounds on inputs unless enforced by a strictly lower-level lookup (`ModularComposition.lean:63-66`).
6. **DAG Level Stratification (`ModularComposition.lean:17-19, 64`):** Modules obey strict tuple dependency level `level(TaggedTable[RANGE=0, DEC=1]) = 0 < level(CHIP_ALU=2, CHIP_MEM_ALLOC=3, CHIP_MEM_READ=4, CHIP_ROM=5, CHIP_HASH=6) = 1 < level(CORE_Goal) = 2` (with `VALUE DIGEST` at level `2` above `CHIP_MEM_READ`/`CHIP_HASH` if invoked by `CORE` at level `3`, or `CORE` invoking `CHIP_HASH` directly). No cycle exists on the bus.

### 1.4 Row Budget (`ESTIMATE`)

- **Leaf Circuit Rows (`S = 16` and `S = 64`):**
  - `Bus` itself consumes **`0` dedicated advice rows** (`q_req` and `q_prov` overlay `CORE` and provider regions).
  - `TaggedTableConfig` fixed table occupies `max(leaf_advice_rows, 2^B + N_dec)` rows:
    - With `B = 12` (`4,096` `RANGE` rows) and `N_dec <= 2,048` (`DEC` rows): `<= 6,144` table rows (`9.4%` of `k_leaf = 16` `65,530` usable rows; `4.7%` of `k_leaf = 17` `131,066` usable rows), adding **`0` extra `k_leaf` increments** at both `S = 16` and `S = 64` (`ESTIMATE`).
    - With `B = 14` (`16,384` `RANGE` rows) and `N_dec <= 2,048`: `<= 18,432` table rows (`28.1%` of `k_leaf = 16`), still **`0` extra `k_leaf` increments** (`ESTIMATE`).
- **Column & Lookup Footprint (`BUS` + `TaggedTableConfig` vs E8 Spike):**
  - Advice columns: `0` dedicated (uses shared `a[0..12]`, `lib.rs:44`).
  - Fixed selector columns: `2` (`q_req`, `q_prov` in `Bus`; `q_rng` and `q_dec` pack into `vsel` or `1` shared 3-code selector `0 = idle, 1 = rng, 2 = dec` of degree 2 -> `-1` fixed selector column vs separate `q_rng, q_dec`, `ESTIMATE`).
  - Fixed `TableColumn`s: `2` (`t_dom_key`, `t_val`), **`-1` fixed `TableColumn`** relative to E8 (`1` `RANGE` + `2` `DEC` = `3` `TableColumn`s, `rfcore.rs:839-843`).
  - PLONK lookups: `3` total (`bus0`, `bus1` via `lookup_any`, `tagged_table` via `lookup`), **`-1` PLONK lookup** relative to E8 (`4` lookups).
- **1-Leaf `ChipNode` and 2-Leaf `RunLift` `R` Impact (`ESTIMATE`):**
  - 1-leaf `ChipNode`: `-1` PLONK lookup (`-22,600` rows) + `-1` fixed `TableColumn` (`-6,462` rows) = **`ESTIMATE -29,062` Node rows** (and up to `ESTIMATE -35,066` Node rows if `q_rng`/`q_dec` share one 2-code `vsel` column).
  - 2-leaf `RunLift` `R`: **`ESTIMATE -51,204 .. -51,662` `R` rows** (`CONSENSUS.md:145`).

### 1.5 Verification

- **Quint Model (`specs/quint/sherlock_bus.qnt`, new ~180 LOC):**
  - Models `Tup = { chip: int, key: int, a: int, b: int, c: int }`, `TaggedEntry = { dom_key: int, val: int }`, provider sets per module (`CHIP_ALU`, `CHIP_MEM_ALLOC`, `CHIP_MEM_READ` split at `base`, `CHIP_ROM`, `CHIP_HASH`), and non-deterministic malicious witness injection on `act == 0` and `q_prov == 0` rows.
  - Checks invariants `InvNoZeroChipProvider`, `InvDisjointMemReadProviders`, `InvTaggedDomainNonAlias` (`limb < 65536` vs `65536 + packed_key`), and `InvDAGLevelOrdering`.
- **`MockProver` Positive Tests (`crates/mori-chip/src/bus.rs` & `tagged_table.rs`):**
  - `bus_four_requests_per_step_ok`: 2-row requester (`L0..L3`) mixing `CHIP_NOP`, `CHIP_ALU`, `CHIP_MEM_READ`, and `CHIP_HASH` passes `MockProver::run(13, ...)`.
  - `tagged_table_range_and_dec_coexist_ok`: simultaneous `q_rng = 1` (`limb = 4095`) and `q_dec = 1` (`packed_key = 0` and `packed_key = 2047`) rows pass on one 2-column table.
- **Named Negative Mutants (`SAS-003`, `SAS-004`):**
  1. `mut_bus_active_provider_zero_chip`: active provider sets `chip = 0` with non-zero payload `(0, 7, 99, 0, 0)` while requester queries `(0, 7, 99, 0, 0)` -> MUST fail `chip == CHIP_ID` provider gate (`D8`).
  2. `mut_bus_inactive_slot_leaks_payload`: provider row with `q_prov = 1, act = 0` leaves `(0, 0, 42, 0, 0)` unzeroed -> MUST fail `(1 - act) * a == 0` gate AND fail requester `(0,0,0,0,0)` lookup.
  3. `mut_bus_unassigned_chip_1`: requester emits `(1, 0, 0, 0, 0)` with `q_req = 1` -> MUST fail `lookup_any` (`Lookup` error at verifier stage, zero panics).
  4. `mut_tagged_dec_overflows_into_range`: malicious `CORE` witness sets `packed_key = p - 65536 + 17` (`expr_dom_key == 17` mod `r`, matching `RANGE` row `17` with `t_val = 0`) -> MUST fail because `packed_key` is reconstructed purely from boolean/range-bounded `CORE` pre-lookup fields (`0 <= packed_key < 65536`).
  5. `mut_tagged_simultaneous_q_rng_q_dec`: test harness deliberately overlaps `q_rng = 1` and `q_dec = 1` on one row -> MUST fail keygen selector-disjointness check (`q_rng * q_dec == 0`).

### 1.6 Lean Obligations

- **Theorem (`sherlock_leaf_modular_refinement`):** Instantiate `Clean.Sherlock.Modular.leaf_refines_step` (`clean/fork/Clean/Sherlock/ModularComposition.lean:131-137`) with concrete Sherlock tuple sum type `SherlockTup` (covering dynamic `Bus` `chip in {0, 2, 3, 4, 5, 6}` plus static `TaggedTable` `tag in {0, 1}`), concrete DAG level function `sherlockLevel : SherlockTup -> Nat` (`0` for `TaggedTable`, `1` for `ALU`/`MEM`/`MEMORY_BACKEND`/`ROM`/`HASH`, `2` for `VALUE_DIGEST`), and module list `[mTagged, mAlu, mMemWin, mMemBack, mRom, mHash, mVal, mCore]`. Prove that if every module satisfies `Contract SherlockValid sherlockLevel m` (`ModularComposition.lean:63-66`) and `LookupSound` holds on the leaf witness, then any accepted leaf witness establishes `Aggregation.Run (fun x y => y = sherlockStep1 x) (rows w).length (pre w) ((rows w).getLastD (pre w))`, discharging `Premises.leaf_sound` and `Premises.det` (`fun_det`, `ModularComposition.lean:123-125`; `clean/fork/Clean/Sherlock/Aggregation.lean:81-96`).
- **Lean File Extended / Created:** Creates `clean/fork/Clean/Sherlock/SherlockBusContract.lean` (plus `SherlockBusContractAxioms.lean` with `#print axioms` checks matching `ModularCompositionAxioms.lean:9-19`), importing `Clean.Sherlock.ModularComposition`.
- **Proof-System Premises / Axioms Relied On:** Zero custom Lean `axiom` declarations (`ModularCompositionAxioms.lean:3-7`). PLONK `lookup_any` / `lookup` soundness (establishing `LookupSound` up to Schwartz-Zippel soundness error `<= deg * 2^{k_leaf} / r < 2^{-235}`) is carried as the explicit theorem hypothesis `hl : LeafRel sherlockMods w`, matching `AggregationAxioms.lean` discipline.
- **Module Contract Discharged:** Discharges **Lean Obligation 3 (Modular Composition)** (`ARCH-BRIEF.md:57`), proving that each module kit can verify its `Contract` in complete isolation.

### 1.7 Code Skeleton

Owned files: `crates/mori-chip/src/bus.rs` (updated registry) and `crates/mori-chip/src/tagged_table.rs` (new).

```rust
// crates/mori-chip/src/bus.rs
use midnight_proofs::{
    circuit::{AssignedCell, Region},
    plonk::{Advice, Column, ConstraintSystem, Error, Selector},
};
use crate::F;

pub const BUS_WIDTH: usize = 5;
pub const CHIP_NOP: u64 = 0;
// ID 1 is intentionally unassigned (retired legacy spike ID).
pub const CHIP_ALU: u64 = 2;
pub const CHIP_MEM_ALLOC: u64 = 3;
pub const CHIP_MEM_READ: u64 = 4;
pub const CHIP_ROM: u64 = 5;
pub const CHIP_HASH: u64 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BusTuple {
    pub chip: F,
    pub key: F,
    pub a: F,
    pub b: F,
    pub c: F,
}

#[derive(Clone, Debug)]
pub struct Bus {
    pub q_req: Selector,
    pub q_prov: Selector,
    pub cols: [Column<Advice>; 10],
}

impl Bus {
    /// Requires: `cs.phases().count() == 1`; `cols` are Phase(0) advice columns.
    /// Ensures: Registers `bus0` (`cols[0..5]`) and `bus1` (`cols[5..10]`) `lookup_any` against `cols[0..5]` with `Rotation::cur()` only.
    pub fn configure(cs: &mut ConstraintSystem<F>, cols: [Column<Advice>; 10]) -> Self;

    /// Requires: `row` is not enabled for `q_prov`; if `lanes[i].chip == 0`, all fields of `lanes[i]` are `0`.
    /// Ensures: Enables `q_req` at `row` and assigns `lanes[0]` to `cols[0..5]` and `lanes[1]` to `cols[5..10]`.
    pub fn assign_req_pair(
        &self,
        region: &mut Region<'_, F>,
        row: usize,
        lanes: [BusTuple; 2],
    ) -> Result<[[AssignedCell<F, F>; BUS_WIDTH]; 2], Error>;

    /// Requires: `row` is not enabled for `q_req`; if active, `tup.chip` equals a non-zero registered `CHIP_*` constant; if inactive, `tup == (0,0,0,0,0)`.
    /// Ensures: Enables `q_prov` at `row` and assigns `tup` to `cols[0..5]`.
    pub fn assign_prov(
        &self,
        region: &mut Region<'_, F>,
        row: usize,
        tup: BusTuple,
    ) -> Result<[AssignedCell<F, F>; BUS_WIDTH], Error>;
}

// crates/mori-chip/src/tagged_table.rs
use midnight_proofs::{
    circuit::Layouter,
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed, TableColumn},
};
use crate::F;

pub const TAG_RANGE: u64 = 0;
pub const TAG_DEC: u64 = 1;
pub const TAG_DEC_OFFSET: u64 = 1 << 16; // 65,536

#[derive(Clone, Debug)]
pub struct TaggedTableConfig {
    /// Code column: 0 = idle, 1 = RANGE (`tag = 0`), 2 = DEC (`tag = 1`).
    pub q_code: Column<Fixed>,
    pub key_col: Column<Advice>,
    pub val_col: Column<Advice>,
    pub t_dom_key: TableColumn,
    pub t_val: TableColumn,
    pub range_bits: usize,
}

impl TaggedTableConfig {
    /// Requires: `range_bits` in `{12, 14}`; `key_col` and `val_col` are Phase(0) advice columns.
    /// Ensures: Registers a single degree-3 PLONK `lookup` over `(t_dom_key, t_val)` using Lagrange codes `{0, 1, 2}` on `q_code` and `Rotation::cur()` only.
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        q_code: Column<Fixed>,
        key_col: Column<Advice>,
        val_col: Column<Advice>,
        range_bits: usize,
    ) -> Self;

    /// Requires: `dec_rows.len() <= 2048`; every `(packed_key, ctrl_word)` has `packed_key < 65536` and unique `packed_key`.
    /// Ensures: Loads `[0..2^range_bits)` with `(limb, 0)` and `dec_rows` with `(65536 + packed_key, ctrl_word)`.
    pub fn load(
        &self,
        layouter: &mut impl Layouter<F>,
        dec_rows: &[(u16, u64)],
    ) -> Result<(), Error>;
}
```

### 1.8 Agent Context Boundary

- **MUST Read (`<= 4` files):**
  1. `crates/mori-chip/src/lib.rs:1-68` (`F`, `NCOL`, shared constants).
  2. `crates/mori-chip/src/bus.rs:1-254` (existing `Bus` spike to update).
  3. `crates/mori-chip/src/vsel.rs:1-120` (Lagrange code degree helper for `q_code`).
  4. `MC:verifier/verifier_gadget.rs:308-345, 604-610` (single-phase and unit-rotation verifier rules).
- **MUST NOT Need to Read:**
  - `crates/mori-core/src/sherlock/...` (`owner_runtime.rs`, AST/rule semantics).
  - `crates/mori-chip/src/{rfcore.rs, alu.rs, poseidon.rs, membackend.rs}` (provider internal gate equations).
  - `crates/mori-recursion/` or `crates/mori-backend/` (node/decider orchestration).

### 1.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-bus/spec.md`)

- **`BUS-001` (Single-Phase Dynamic Bus):** The `BUS` module SHALL register two 5-column `lookup_any` channels over Phase(0) advice columns using `Rotation::cur()` only and zero verifier challenges, serving frozen chip IDs `{0, 2, 3, 4, 5, 6}` with ID `1` unassigned. *(246 chars)*
  - **Scenario `S-BUS-001a` (`MockProver` + `mut_bus_unassigned_chip_1`):** **WHEN** a step emits four valid requests (`L0..L3`) on two `q_req = 1` rows **THEN** `MockProver` accepts; **WHEN** any request sets `chip = 1` **THEN** `MockProver` rejects at the lookup check with zero panics.
- **`BUS-002` (Provider Zero-Gate and Non-Zero Chip Law):** Every provider region SHALL constrain active rows to its fixed non-zero `CHIP_ID` and SHALL gate all five tuple columns of inactive `q_prov = 1` slots to `(0, 0, 0, 0, 0)`; every requester SHALL gate `(key, a, b, c)` to `0` when `chip == 0`. *(246 chars)*
  - **Scenario `S-BUS-002a` (`mut_bus_active_provider_zero_chip`, `mut_bus_inactive_slot_leaks_payload`, Quint `InvNoZeroChipProvider`):** **WHEN** a malicious witness assigns `chip = 0` on an active provider row or non-zero payload on an inactive `q_prov = 1` row **THEN** the local provider gate rejects the witness.
- **`BUS-003` (Merged 2-Column Tagged Static Lookup):** `TaggedTableConfig` SHALL merge `RANGE` (`tag = 0`, `t_dom_key = limb < 2^14`, `t_val = 0`) and `DEC` (`tag = 1`, `t_dom_key = 65536 + packed_key`, `t_val = ctrl_word`) into one 2-column PLONK lookup with disjoint domain keys. *(233 chars)*
  - **Scenario `S-BUS-003a` (`mut_tagged_dec_overflows_into_range`, Lean `sherlock_leaf_modular_refinement`):** **WHEN** `CORE` queries `DEC` with bounded `packed_key < 65536` **THEN** `expr_dom_key >= 65536` cannot match any `RANGE` row, and `Clean/Sherlock/SherlockBusContract.lean` discharges `LeafRel -> Run`.

---

## 2. Module `CORE` — Register File, 7-Row `Arch::Narrow` Step Slots, `vsel` Packing, and Top-of-Stack Chains

### 2.1 Purpose

`CORE` owns the canonical architectural state of the Sherlock `/1` virtual machine across `S` consecutive step slots per leaf, executing decoded micro-operations over registers, top-of-stack `Frame` and `Task` caches, and four dynamic bus request lanes per step. It enforces monotone halt padding (`is_pad`), 64-bit step height `h < 2^64` (`SRC-004`), strict write-before-read zeroing on intra-step scratch lanes `v0..v2` (`SRC-023`, `AUDIT.md:67-69`), and copy-constrained boundary handoff.

### 2.2 Frozen Interface

#### A. Canonical Architectural State (`ArchState`, Copy-Constrained Port)

`CORE` exposes `pre_state: ArchStateCells` at slot `0` and `post_state: ArchStateCells` at slot `S - 1` via `constrain_equal` to `BOUNDARY COMMITMENT` and `MEM`. Every lane in `ArchState` is hashed into `pre_d` / `post_d` (`DOM_STATE = 8`), leaving **zero unhashed inter-step witness lanes**:

| Register Group | Lanes | Bit-Width / Domain | Range-Check & Representation Owner |
|---|---|---|---|
| Run Identity | `prog_d`, `input_d` (`2` field cells) | `< r` (digest) | Preserved `post == pre` across every slot in `CORE` (`GP-001`, `SRC-001`). |
| Step Height | `h` (`1` cell) | `0 <= h < 2^64` | `CORE` increments `h' = h + enable` (`enable in {0,1}`); `NODE` / `CORE` boundary checks `h_post < 2^64` via `RANGE` (`5` x `12`-bit + `4`-bit limb or `ALU` `< 2^64`, `SRC-004`). |
| Control & Outcome | `ctl_tag`, `ctl_aux`, `outcome_tag`, `outcome_code` (`4` cells) | `ctl_tag in 0..=7`, `outcome_tag in 0..=3`, `aux/code < 2^32` | `CORE` + `DEC` transition gates; `outcome_tag`: `0 = Running`, `1 = Halted`, `2 = Rejected`, `3 = Exhausted` (`GP-001`). |
| Fuel & Accounting | `fuel_lo`, `fuel_hi`, `consumed_work`, `helper_work`, `helper_visits`, `ref_cursor`, `stack_limit` (`7` cells) | `u64` limbs (`< 2^64`) | 128-bit `remaining_fuel = fuel_lo + 2^64 * fuel_hi` decremented via `ALU` (`OP_SUB`) on fuel-consuming steps; counters checked `< 2^64`. |
| LIFO Stack Anchors | `fcount`, `tcount`, `fp_hash`, `tp_hash` (`4` cells) | `fcount, tcount < 2^64`; `fp_hash, tp_hash < r` | `CORE` enforces `fcount == 0 => fp_hash == EMPTY_STACK_FRAME != 0` and `tcount == 0 => tp_hash == EMPTY_STACK_TASK != 0` (`AUDIT.md:63`); `fcount <= stack_limit`. |
| Top-of-Stack Cache | `top_frame[0..4]`, `top_task[0..4]` (`8` cells) | `kind/pc < 2^32`, payload `< 2^128` or `< r` | Held in registers (`owner_runtime.rs:3566-3591`): `RewriteTop` and top reads cost **`0` bus tuples and `0` hashes**; only deep push/pop (`fcount >= 1` push or `fcount >= 2` pop) invokes `CHIP_HASH = 6` (`DOM_FRAME = 9`, `DOM_TASK = 10`). |
| Heap & Env Cursors | `env_ptr`, `len_heap`, `len_env`, `len_list`, `len_ctor` (`5` cells; or unified `len_mem` + typed counters) | `< 2^64` | Advance **strictly by `k_alloc in 0..=4`** on allocating rules (`0` advance on non-allocating steps, `GP-001`, `AUDIT.md:45`). |
| Packed Helper & Result | `helper_packed[0..6]`, `result[0..3]` (`10` cells) | Packed tags/cursors `< 2^128`; `result = (v_tag, v_lo, v_hi_or_ptr)` | `23` logical helper scalars (`rf.rs:31-35`) packed into `6` canonical `u128` register cells via non-overlapping 32/64-bit sub-lanes; unpacked only by active helper rules and range-checked via `RANGE`/`ALU`. |

*Scratch Lanes `v0, v1, v2` (`rf.rs:34, 65-67`):* Explicitly **excluded** from `ArchState` and `pre_d`/`post_d`. Never copied across step boundaries (see Invariant 4 below).

#### B. Compacted 7-Row `Arch::Narrow` Step Slot Layout (`NCOL = 12` Shared Advice Columns)

Compacting the 13-row E8 spike (`rfcore.rs:169-221`, which interleaved four `F` rows and three duplicate `L` rows) into a 7-row `Arch::Narrow` slot over `a[0..12]` preserves `Rotation(-1, 0, +1)` adjacency while cutting `CORE` step rows by `46%`:

| Row Offset | Role | `a[0..10]` Assignment | `a[10..12]` Assignment | Active Lookups / Gates |
|---|---|---|---|---|
| `+0` | `PreState A` + `DEC` Query | `pre` architectural bank A (`10` cells; `constrain_equal` to previous slot row `+3` / boundary) | `a[10] = packed_key`, `a[11] = ctrl_word` | `TaggedTableConfig` (`q_code = 2`, `tag = 1` `DEC`) |
| `+1` | `PreState B` + Unpacked Ctrl A | `pre` architectural bank B (`10` cells) | `IsZero` inverses (`fuel_zero_inv`, `fc_zero_inv`) | Adjacency gates (`Rotation(-1, 0, +1)` against `+0` and `+2`) |
| `+2` | `PreState C` + Unpacked Ctrl B | `pre` architectural bank C (`10` cells, incl. `top_frame`, `top_task`) | Unpacked boolean control sub-fields + `v0` | `vsel` micro-op group A + boolean/domain checks on `ctrl_word` |
| `+3` | `PostState A` | `post` architectural bank A (`10` cells) | `v1, v2` intra-step scratch cells | `vsel` transition gates (`Rotation(-1)`/+`0`/ `+1`) |
| `+4` | `PostState B & C` Delta | `post` architectural bank B/C changed lanes (`constrain_equal` to next slot `+1/+2` or delta mux) | `IsZero` inverses (`tc_zero_inv`, `eq_diff_inv`) | Monotone `is_pad` & `EMPTY_STACK` gates |
| `+5` | `Bus Req Pair 0` (`L0, L1`) | `L0 = (chip0, key0, a0, b0, c0)` in `a[0..5]`; `L1 = (chip1, key1, a1, b1, c1)` in `a[5..10]` | `enable`, `is_pad` (`1 - enable`) | `Bus::q_req = 1` (`bus0`, `bus1`) |
| `+6` | `Bus Req Pair 1` (`L2, L3`) | `L2 = (chip2, key2, a2, b2, c2)` in `a[0..5]`; `L3 = (chip3, key3, a3, b3, c3)` in `a[5..10]` | Step-end link / `h_next` | `Bus::q_req = 1` (`bus0`, `bus1`) |

*(Note: Because `ArchState` has `41` cells and `3` pre-rows hold `30` cells, the remaining `11` cold/preserved cells or `PostState C` either occupy a tiny 8th row `h_slot = 8` — still `512` rows at `S = 64` — or pack bounded `< 2^32` control/counter scalars `(ctl_tag, ctl_aux, outcome_tag, outcome_code)` and `(fcount, tcount)` into single `< 2^128` carrier words decomposed on active transitions, fitting `30` carrier cells cleanly in `7` rows. Both `7`-row and `8`-row slots satisfy `k_leaf = 16` with `> 60,000` rows to spare; Section 2.4 budgets both.)*

#### C. `vsel` Lagrange Selector Packing (`crates/mori-chip/src/vsel.rs:1-173`)

Virtual selectors `Vsel` (`vsel.rs:36-41`) pack mutually exclusive rule micro-operations onto shared fixed code columns `Column<Fixed>` using univariate Lagrange basis polynomials `L_c(q) = prod_{m != c} (q - m) / (c - m)` (`vsel.rs:29-34`):
- A fixed column with `n` active codes (`1..=n`, `0` = off) contributes polynomial degree `n` (`vsel.rs:97`), keeping total gate degree `n + d_body <= MAX_DEG = 6` (`vsel.rs:25, 97`; e.g., degree-2 body -> up to `n = 4` mutually exclusive micro-ops per fixed column; degree-3 body -> `n = 3`).
- Greedy largest-degree-first bin packing (`VselPacker::pack`, `vsel.rs:90-119`) eliminates ~14 sparse fixed selector columns (`ESTIMATE -84,000 .. -90,440` Node rows vs 1-hot fixed selectors, `vsel.rs:4`).

#### D. Bus Tuples Requested and Provided by `CORE`

- **Provided on `Bus`:** None (`CORE` never sets `q_prov = 1`).
- **Requested on `Bus` (up to `4` tuples `L0..L3` per step on rows `+5, +6`):**
  - `CHIP_NOP = 0`: unused lanes and all 4 lanes when `is_pad == 1`.
  - `CHIP_ALU = 2`: fuel decrement (`OP_SUB`), scalar arithmetic (`BHelperScalarArithmetic`), unsigned/signed comparisons, pointer bounds (`addr < len_curr`), stack limit (`fcount < stack_limit`).
  - `CHIP_MEM_ALLOC = 3`: up to 2–3 cell appends per step (`EnvPutInsert`, `AppendCell`, `XAdtHelperBuildField`, `ScalarConstruct`).
  - `CHIP_MEM_READ = 4`: environment lookup (`EnvGetCell`), list/constructor field traversal (`BindPair`, `EqualField`, `MatchField`, `AppendCopy`).
  - `CHIP_ROM = 5`: AST node fetch, literal/const fetch, function table lookup (`FindRow`), ADT plan walk (`XAdtHelperPlanWalk`).
  - `CHIP_HASH = 6`: deep `Frame` push/pop (`DOM_FRAME = 9`) and deep `Task` push/pop (`DOM_TASK = 10`) when `fcount >= 1` (push) / `fcount >= 2` (pop), plus `GP-2` claim digest check (`DOM_CLAIM = 12`).
- **Requested on `TaggedTableConfig`:** `1` `DEC` lookup (`tag = 1`, row `+0`) per step when `enable == 1` (`q_code = 2 * enable`, so `is_pad == 1` sets `q_code = 0`, querying `(0, 0)`).

### 2.3 Invariants

**Guaranteed by `CORE`:**
1. **Non-Empty Leaf & Monotone Halt Padding (`SRC-001`, `SRC-014`, `design.md:44`):**
   - Slot `0` constrains `is_pad_0 == 0` (`enable_0 == 1`, `steps = sum_{j=0}^{S-1} enable_j >= 1`).
   - Every slot `j` constrains `is_pad_j * (1 - is_pad_j) == 0`; adjacent slots `j, j+1` constrain `(1 - is_pad_{j+1}) * is_pad_j == 0` (`is_pad` is 0-then-1 monotone).
   - Whenever slot `j` has `outcome_tag_post != 0` (`Halted`, `Rejected`, or `Exhausted`), if `j + 1 < S` then `is_pad_{j+1} == 1`; if `enable_j == 1`, `outcome_tag_pre == 0` (no active step executes from an already-terminal state).
   - When `is_pad_j == 1`, `post_state == pre_state` across every lane of `ArchState`, `h_post == h_pre`, `L0 == L1 == L2 == L3 == (0, 0, 0, 0, 0)`, and `q_code_dec == 0`.
2. **64-Bit Step Height Progression (`SRC-004`):** `h_{j+1} = h_j + (1 - is_pad_j)` and `h_S < 2^64` (checked once per leaf on `h_S` and `h_0` via `ALU`/`RANGE`, since `0 <= S < 2^64` prevents intra-leaf wrap over `Fq`).
3. **Top-of-Stack Cache & Non-Zero Empty Stack Sentinels (`AUDIT.md:63`, `owner_runtime.rs:3566-3591`):**
   - `fcount == 0 => fp_hash == EMPTY_STACK_FRAME` (`EMPTY_STACK_FRAME != 0`) and `top_frame == (0, 0, 0, 0)` (enforced via `fc_is_zero * (fp_hash - EMPTY_STACK_FRAME) == 0`).
   - `tcount == 0 => tp_hash == EMPTY_STACK_TASK` (`EMPTY_STACK_TASK != 0`) and `top_task == (0, 0, 0, 0)` (enforced via `tc_is_zero * (tp_hash - EMPTY_STACK_TASK) == 0`).
   - Push when `fcount == 0` writes `top_frame'` directly (`fcount' = 1`, `fp_hash' = EMPTY_STACK_FRAME`, `0` hashes); push when `fcount >= 1` spills `top_frame` via `CHIP_HASH = 6` (`fp_hash' = H(DOM_FRAME, fp_hash, pack(top_frame))`) and loads `top_frame'`; pop when `fcount >= 2` verifies `fp_hash == H(DOM_FRAME, fp_hash', pack(restored_frame))` via `CHIP_HASH = 6`; pop when `fcount == 1` sets `fcount' = 0, fp_hash' = EMPTY_STACK_FRAME, top_frame' = 0` (`0` hashes). Symmetric rules govern `TASKS`.
4. **Per-Slot Scratch Lane Isolation (`v0..v2` Write-Before-Read / Zeroing, `AUDIT.md:67-69` Finding 9, `SRC-023`):**
   - Scratch cells `v0, v1, v2` exist solely at row offsets `+2, +3` inside slot `j` with **zero equality constraints and zero rotation gates linking `v0..v2` of slot `j` to slot `j - 1` or `j + 1`**.
   - Every rule micro-op reading `v_k` MUST write `v_k` within the same slot `j` from `pre_state` or bus outputs `L0..L3`; when `v_k` is unused by the active rule (or `is_pad_j == 1`), `CORE` constrains `v_k == 0`. Thus `post_state` after `N` steps is a pure deterministic function of `pre_state_0`, preserving `SRC-023` across every leaf partition (`S = 16`, `S = 64`, or mixed).
5. **Parameterized `LeafShape` (`GP-001`):** `CoreConfig` takes `LeafShape { s_max: usize, .. }` (`s_max = 16` for M4 `SRC-014`; `s_max = 64` post-M4) with identical gates, columns, and lookup structure; changing `s_max` changes only region loop count and `k_leaf` (`16 -> 17`).

**Assumed from Other Modules:**
- `BUS` + `TaggedTableConfig`: lookup soundness (`LookupSound`).
- `DEC`: `ctrl_word` returned for `(tag = 1, packed_key)` encodes the exact `/1` rule transition for `packed_key`.
- `ALU`, `MEM` / `MEMORY BACKEND`, `ROM`, `HASH`: valid responses on `chip in {2, 3, 4, 5, 6}` per Section 1.2.B.
- `BOUNDARY COMMITMENT`: binds `slot[0].pre_state` and `slot[S-1].post_state` to public leaf statement digests `(pre_d, post_d)`.

### 2.4 Row Budget (`ESTIMATE`)

- **Leaf Circuit Rows:**
  - At `7` rows/slot (or `8` rows/slot if unpacked cold carriers use a 4th bank row):
    - `S = 16` (M4 default): `16 * 7 = 112` rows (`128` rows at `8` rows/slot) + `2` boundary staging rows = **`ESTIMATE 114 .. 130` advice rows** (`0.2%` of `k_leaf = 16` `65,530` usable rows).
    - `S = 64` (post-M4 amortized leaf): `64 * 7 = 448` rows (`512` rows at `8` rows/slot) + `2` boundary staging rows = **`ESTIMATE 450 .. 514` advice rows** (`0.4%` of `k_leaf = 17` `131,066` usable rows).
- **Column & Lookup Delta:**
  - Advice columns: `12` shared advice columns (`a[0..12]`, `lib.rs:44`) — `+0` advice columns over baseline.
  - Fixed columns: `ESTIMATE 4 .. 5` packed `vsel` code columns (`vsel.rs:90-119`, degree `<= 6`), `-3 .. -4` fixed columns relative to unpacked spike (`rfcore.rs`).
  - Equality-enabled columns: `12` advice + `2` boundary/instance = `14` (`+0` delta vs `CONSENSUS.md:129`).
  - Lookups: `0` new (`uses Bus + TaggedTableConfig`).
- **1-Leaf `ChipNode` and 2-Leaf `RunLift` `R` Impact (`ESTIMATE`):**
  - Moving `S = 16 -> S = 64` (`k_leaf = 16 -> 17`) adds **`ESTIMATE +4` rows** to 1-leaf `ChipNode` (`design.md:135`), **`ESTIMATE +8` rows** to 2-leaf `RunLift` `R`, and **`+0` rows** to the Decider (`design.md:135`), while cutting leaf count and recursion folds by **`4x`** on long runs.

### 2.5 Verification

- **Quint Model (`specs/quint/sherlock_core_slots.qnt`, new ~260 LOC):**
  - Models `S`-slot execution with top-of-stack `Frame`/`Task` caches, `fp_hash`/`tp_hash` chains with `EMPTY_STACK != 0`, `is_pad` monotonicity, `h < 2^64`, and non-deterministic scratch initialization if `v0..v2` were persistent vs per-slot fresh.
  - Proves `InvPartitionIndependence` (`run(steps, S=3) == run(steps, S=5)` for arbitrary multi-leaf runs, discharging finite-model check of `SRC-023`), `InvMonotonePad`, `InvEmptyStackSentinel`, and `InvZeroAllocNoAddrAdvance` (`GP-001`).
- **`MockProver` Positive Tests (`crates/mori-chip/src/rfcore.rs`):**
  - `core_leaf_all_rule_classes_s16_ok`: executes 16-step trace covering `Eval*`, `Return*`, `BHelper*`, `XAdt*`, deep `Frame`/`Task` spill + restore, and terminal `Halted` + 4 `is_pad = 1` rows.
  - `core_partition_s16_vs_s4_equal_post_state`: runs a 20-step allocating + constructor-building trace across `2 x (S=16)` vs `5 x (S=4)` leaves and asserts identical `post_state` and `n_steps = 20` (`GP-001`, `SRC-023`).
  - `core_height_max_u64_boundary_ok`: accepts `h_pre = 2^64 - 2, steps = 1 -> h_post = 2^64 - 1` (`GP-001`, `SRC-004`).
- **Named Negative Mutants (`SAS-003`, `SAS-004`):**
  1. `mut_core_pad_revival`: sets `is_pad = [0, 1, 0, 1, ...]` -> MUST fail `(1 - is_pad_{j+1}) * is_pad_j == 0`.
  2. `mut_core_all_padded_zero_step_leaf`: sets `is_pad_0 = 1` (`steps = 0`) -> MUST fail `is_pad_0 == 0` (`SRC-001`, `SRC-014`).
  3. `mut_core_step_after_terminal`: sets `outcome_tag_0 = 1` (`Halted`) and `is_pad_1 = 0` -> MUST fail post-terminal pad gate.
  4. `mut_core_height_wrap_2pow64`: sets `h_pre = 2^64 - 1, enable_0 = 1 -> h_post = 2^64` -> MUST fail `h_post < 2^64` range check (`SRC-004`, `GP-001`).
  5. `mut_core_scratch_cross_step_leak`: weakened control mutant passes `v0` across slot boundary vs production gate -> production rejects forged `v0` when unread/unwritten, confirming `AUDIT.md:67-69` closure.
  6. `mut_core_empty_stack_zero_digest`: sets `fcount = 0, fp_hash = 0` -> MUST fail `fc_is_zero * (fp_hash - EMPTY_STACK_FRAME) == 0` (`AUDIT.md:63`).
  7. `mut_core_pop_forged_restored_frame`: pops at `fcount = 2` with altered `restored_frame` -> MUST fail `CHIP_HASH = 6` lookup against `fp_hash`.

### 2.6 Lean Obligations

1. **Step Refinement (`core_spec_follows_step1`):** Prove the `hcoreSpec` hypothesis of `ModularComposition.leaf_refines_step` (`clean/fork/Clean/Sherlock/ModularComposition.lean:134`): whenever `mCore.rel w` holds and every tuple in `mCore.requests w` satisfies `SherlockValid`, the sequence of non-padded slot states `rows w` satisfies `Follows sherlockStep1 (pre w) (rows w)`, `rows.length >= 1` (`is_pad_0 == 0`), and padded rows (`is_pad == 1`) satisfy `post == pre` without advancing `h` or `rows.length`.
   - **Lean File:** Creates `clean/fork/Clean/Sherlock/CoreRefinement.lean` + `CoreRefinementAxioms.lean`.
   - **Premises / Axioms:** Collision resistance of Poseidon (`DOM_FRAME = 9`, `DOM_TASK = 10`) is stated as an explicit structure hypothesis `HashInjectiveOnReachableStacks` (matching `Premises` style in `Aggregation.lean:64-105`, zero Lean `axiom` keywords) so LIFO hash-chain pop uniquely recovers the pushed `Frame`/`Task` and `fp_hash == EMPTY_STACK_FRAME` holds only on the empty stack (`AUDIT.md:63`).
2. **Partition Independence (`core_partition_independent`, `SRC-023`, Lean Obligation 2 in `ARCH-BRIEF.md:56`):** Extend `clean/fork/Clean/Sherlock/PartitionIndependence.lean` (`PartitionIndependenceAxioms.lean`) to include top-of-stack `Frame`/`Task` caches and per-slot fresh `v0..v2` scratch variables, proving that for any two leaf partitions `(S_1, k_1)` and `(S_2, k_2)` of an `N`-step run (`sum steps_i = N`), `Aggregation.Run` determinism (`Modular.fun_det`, `ModularComposition.lean:123-125`; `Premises.det`, `Aggregation.lean:81`) yields identical terminal `ArchState`, `outcome_d`, and `n_steps`.

### 2.7 Code Skeleton

Owned files: `crates/mori-chip/src/rfcore.rs`, `crates/mori-chip/src/rf.rs`, `crates/mori-chip/src/vsel.rs`.

```rust
// crates/mori-chip/src/rfcore.rs
use midnight_proofs::{
    circuit::{AssignedCell, Layouter},
    plonk::{Advice, Column, ConstraintSystem, Error},
};
use crate::{bus::Bus, tagged_table::TaggedTableConfig, vsel::VselPacker, F};

pub const ARCH_STATE_CELLS: usize = 41;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeafShape {
    pub s_max: usize,       // 16 (M4) or 64 (post-M4)
    pub w_flush: usize,     // 8-cell flush blocks per leaf
    pub k_page: usize,      // Memory backend query budget
    pub k_rom: usize,       // Static ROM query budget
}

#[derive(Clone, Debug)]
pub struct ArchStateCells {
    pub cells: [AssignedCell<F, F>; ARCH_STATE_CELLS],
}

#[derive(Clone, Debug)]
pub struct LeafBoundary {
    pub pre: ArchStateCells,
    pub post: ArchStateCells,
    pub steps_taken: AssignedCell<F, F>,
}

#[derive(Clone, Debug)]
pub struct CoreConfig {
    pub shape: LeafShape,
    pub a: [Column<Advice>; 12],
    pub bus: Bus,
    pub tagged: TaggedTableConfig,
    pub vsel: VselPacker,
}

impl CoreConfig {
    /// Requires: `cs.phases().count() == 1`; `shape.s_max >= 1`; `bus` and `tagged` share `a[0..12]`.
    /// Ensures: Configures 7-row (or 8-row) `Arch::Narrow` step gates with degree <= 6, rotations in `{-1, 0, +1}`, monotone `is_pad`, `EMPTY_STACK != 0`, and scratch `v0..v2` isolation.
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        shape: LeafShape,
        a: [Column<Advice>; 12],
        bus: Bus,
        tagged: TaggedTableConfig,
    ) -> Self;

    /// Requires: `steps` contains `1..=shape.s_max` valid `/1` step witnesses; `steps[0]` is non-terminal on entry.
    /// Ensures: Assigns `shape.s_max` slots (padding remaining slots with `is_pad = 1`), emits 4 bus requests/slot and 1 `DEC` lookup/slot, checks `h_post < 2^64`, and returns boundary cells for `constrain_equal`.
    pub fn assign_leaf(
        &self,
        layouter: &mut impl Layouter<F>,
        steps: &[StepWitness],
    ) -> Result<LeafBoundary, Error>;
}
```

### 2.8 Agent Context Boundary

- **MUST Read (`<= 6` files):**
  1. `crates/mori-chip/src/bus.rs` & `crates/mori-chip/src/tagged_table.rs` (frozen tuple and `DEC` query interfaces).
  2. `crates/mori-chip/src/dec.rs` (unpacked `CtrlSignals` layout).
  3. `crates/mori-chip/src/rf.rs:1-302` & `crates/mori-chip/src/vsel.rs:1-173` (register indices and Lagrange packer).
  4. `crates/mori-chip/src/rfcore.rs:1-450` (existing `Arch::Narrow` slot prototype).
  5. `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:65-110, 3550-3600` (`/1` enabled rules and stack/helper transition semantics).
- **MUST NOT Need to Read:**
  - `crates/mori-chip/src/{alu.rs, range.rs, poseidon.rs, hashchip.rs, membackend.rs}` (internal gates of provider modules).
  - `crates/mori-recursion/` or Compactverifier contracts.

### 2.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-core/spec.md`)

- **`CORE-001` (Monotone Halt Padding and 64-Bit Height):** `CORE` SHALL execute `1..=S` active `/1` steps per leaf (`is_pad_0 == 0`), enforce monotone `is_pad_{j+1} >= is_pad_j` and `is_pad == 1` after any terminal outcome, freeze all `ArchState` cells on padded slots, and constrain `h_post = h_pre + steps < 2^64`. *(258 chars)*
  - **Scenario `S-CORE-001a` (`mut_core_pad_revival`, `mut_core_all_padded_zero_step_leaf`, `mut_core_height_wrap_2pow64`):** **WHEN** `is_pad_0 = 1`, `is_pad` flips `1 -> 0`, or `h_post = 2^64` **THEN** `MockProver` rejects the witness; **WHEN** `h_post = 2^64 - 1` **THEN** `MockProver` accepts (`GP-001`, `SRC-004`).
- **`CORE-002` (Scratch Isolation and Partition Independence):** `CORE` SHALL exclude intra-step scratch cells `v0..v2` from cross-slot persistence and require per-slot write-before-read or zeroing so multi-leaf `ArchState` after `N` steps is invariant under leaf partitioning (`SRC-023`). *(225 chars)*
  - **Scenario `S-CORE-002a` (`core_partition_s16_vs_s4_equal_post_state`, Quint `InvPartitionIndependence`, Lean `core_partition_independent`):** **WHEN** an allocating multi-leaf run is proved at `S = 16` and `S = 4` **THEN** both runs yield identical terminal `ArchState`, `outcome_d`, and `n_steps`.
- **`CORE-003` (Top-of-Stack Register Cache and Non-Zero Empty Sentinels):** `CORE` SHALL hold top `Frame` and `Task` records in registers (`0` hashes on `RewriteTop` and `1 <-> 0` transitions), spill/restore deeper entries via `CHIP_HASH = 6`, and enforce `fcount == 0 => fp_hash == EMPTY_STACK_FRAME != 0` (and symmetric `TASKS`). *(259 chars)*
  - **Scenario `S-CORE-003a` (`mut_core_empty_stack_zero_digest`, `mut_core_pop_forged_restored_frame`):** **WHEN** `fcount = 0` with `fp_hash = 0` or a deep pop alters a restored frame field **THEN** `MockProver` rejects the witness.

---

## 3. Module `DEC` — Gate-Normalized `/1` Dispatch Table and Bijective Control Unpacking

### 3.1 Purpose

`DEC` maps a normalized pre-step architectural classifier (`packed_key`) to the unique Sherlock `/1` rule identifier and 54-bit packed microcode word (`ctrl_word`) via the `tag = 1` partition of `TaggedTableConfig` (`<= 2,048` static rows). It covers the entire enabled `/1` rule inventory of `owner_runtime.rs` (`GP-001`), rejects disabled and `/2` extension pre-states as unsatisfiable lookups (`SVM-004`, `SVM-009`), and enforces boolean/domain constraints on every unpacked sub-field of `ctrl_word` to prevent modular wrap-around decomposition forgery (`Exploit C1`).

### 3.2 Frozen Interface

#### A. Complete `/1` Rule Inventory Accounting (`shared.rs:400-534`, `owner_runtime.rs:70-108, 1346`)

`RuleIdC` (`crates/mori-core/src/sherlock/helper_machine/shared.rs:400-534`) declares **134** variants across five families (`37` `KMachine*`, `66` `BHelper*`, `4` `BBookkeeping*`, `5` `KAdmin*`, `22` `XAdt*`):
- **Excluded in `/1` (`11` variants, `0` rows in `DEC`):**
  - `7` disabled variants (`owner_runtime.rs:70-80` `is_disabled`): `BHelperEnvPutCell`, `BHelperEnvPutEmpty`, `BHelperEnvPutFailure`, `BHelperAppendFailure`, `BHelperEqualFailure`, `BHelperMatchEmpty`, `BBookkeepingTaskEnqueue`.
  - `4` `/2` extension variants (`owner_runtime.rs:82-90` `is_extension`): `BHelperValidateDeclaration`, `BHelperMatchDiscard`, `BBookkeepingResumeStore`, `BBookkeepingResumeRestore`.
- **Enabled in `/1` (`123` variants total — often cited in study shorthand as `"120 + 2"`, see Section 8 Conflict C-4):**
  - **`2` genesis / startup rules (`owner_runtime.rs:1346` `EventKind::Startup`):** `KAdminStartupValid`, `KAdminStartupInvalid` (`h == 0` admission step binding `prog_d` + `input_d` -> initial `ArchState`).
  - **`121` active `/1` step rules:** `37` `KMachine*` + `58` enabled `BHelper*` (`9` `*Entry` + `49` helper body/exit/failure rules across kinds `1..=9`) + `1` `BBookkeepingTaskRelease` + `3` `KAdminPublish*` (`PublishValue`, `PublishError`, `PublishExhausted`) + `22` `XAdt*` (`9` `XAdt{Eval,Return,Materialize,Cases}*` + `4` `XAdtHelperBuild*` + `9` `XAdtHelperPlan*` — closing the 13-rule `XAdtHelper*` gap in `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:363-409`).

#### B. Pre-Lookup Guard Normalization (`packed_key`, `15` Bits `< 32,768`)

Unbounded field comparisons cannot enter a finite static table directly. `CORE` row `+0..+1` normalizes unbounded witnesses using degree-2 `IsZero` gadgets (`inv`, `z = 1 - x * inv`, `x * z == 0`, hence `z in {0, 1}`) and bounded carrier tags before assembling `packed_key`:

| Bits | Width | Sub-Field | Meaning & Normalization Owner |
|---|---|---|---|
| `0..=2` | `3` bits (`0..=7`) | `ctl_mode` | `0 = Startup (h==0)`, `1 = Eval`, `2 = Return`, `3 = Helper`, `4 = Cases/Materialize`, `5 = AdminPublish`; range-checked `< 8` in `CORE`. |
| `3..=7` | `5` bits (`0..=31`) | `site_or_kind` | AST/frame site (`0..=26`, `c6_dispatch_step.rs:263-294`) when in `Eval`/`Return`/`Cases`, or `helper_kind` (`1..=11`, including `10 = AdtBuild`, `11 = AdtPlan`) when in `Helper`; range-checked `< 32` in `CORE`. |
| `8..=11` | `4` bits (`0..=15`) | `sub_disc` | Bounded sub-state discriminator (AST node kind, value tag `Unit/Bool/Int/Nat/String/List/Ctor`, or helper cursor state); range-checked `< 16` in `CORE`. |
| `12` | `1` bit | `fuel_is_zero` | `IsZero(fuel_lo + 2^64 * fuel_hi)` (`0` when `remaining_fuel > 0`, `1` when `remaining_fuel == 0`; `Exhausted(Fuel)` dispatch). |
| `13` | `1` bit | `count_guard` | Context-multiplexed `IsZero` boolean: `fcount == stack_limit` on `Push`, `list/field_rem == 0` on `Helper` loops, `args_rem == 0` on `ReturnArguments`. |
| `14` | `1` bit | `cond_guard` | Context-multiplexed boolean outcome: `result_bool`, `alu_ovf_or_eq`, `var_found`, or `ctor_tag_match` (`b * (1 - b) == 0` in `CORE`). |

- **Key Bound & Table Density:** Every sub-field of `packed_key` is constrained in `CORE` to its exact bit-width, so `0 <= packed_key < 2^15 = 32,768 < 65,536`. Only reachable valid `/1` combinations are populated into `TaggedTableConfig` (`tag = 1`, `t_dom_key = 65,536 + packed_key`), yielding **`ESTIMATE 1,180 .. 1,850` populated rows (`<= 2,048` cap)**. Any unreachable, disabled (`is_disabled`), or `/2` extension (`is_extension`, `Waiting`) pre-state produces a `packed_key` absent from `t_dom_key`, causing `MockProver` and PLONK lookup verification to reject immediately (`SVM-004`, `SVM-009`).

#### C. 54-Bit Packed `ctrl_word` Schema and Bijective Unpacking (`Exploit C1` Closure)

`TaggedTableConfig` returns `ctrl_word = sum_i f_i * 2^{off_i}` (`< 2^54 < r`). `DEC` unpacks `ctrl_word` into witness cells `f_i` on `CORE` row `+2` and enforces **mandatory domain constraints on every single `f_i`**:

| Bits | Width | Unpacked Field | Meaning | Mandatory Unpacking Constraint (`Exploit C1` Defense) |
|---|---|---|---|---|
| `0..=7` | `8` bits | `rule_id` | Canonical `RuleIdC` index (`0..=133`, `0` on pad) | Decomposed via `RANGE` (`rule_id * 16 < 4096` -> `rule_id < 256`). |
| `8..=11` | `4` bits | `vsel_grp_a` | `Vsel` code for register-bank A transition (`0..=15`) | 4 boolean bits `b_k * (1 - b_k) == 0` (or `256 * vsel_grp_a < 4096`). |
| `12..=15` | `4` bits | `vsel_grp_b` | `Vsel` code for register-bank B/helper transition (`0..=15`) | 4 boolean bits `b_k * (1 - b_k) == 0`. |
| `16..=19` | `4` bits | `vsel_grp_c` | `Vsel` code for result/outcome transition (`0..=15`) | 4 boolean bits `b_k * (1 - b_k) == 0`. |
| `20..=31` | `4 x 3` bits | `chip0..chip3` | Target `Bus` chip IDs (`0, 2, 3, 4, 5, 6`) for `L0..L3` | Each `chip_j` decomposed into 3 boolean bits (`0..=7`). |
| `32..=35` | `4` bits | `alu_op` | `ALU` opcode (`0` or `1..=9`) | 4 boolean bits (`0..=15`). |
| `36` | `1` bit | `alu_expect_ovf` | Expected `ovf` bit (`0` value, `1` primitive error) | `b * (1 - b) == 0`. |
| `37..=39` | `3` bits | `stack_op` | `0=Nop, 1=RewriteFrame, 2=PushFrame, 3=PopFrame, 4=PushTask, 5=PopTask, 6=ReleaseTask` | 3 boolean bits (`0..=7`). |
| `40..=42` | `3` bits | `alloc_cnt` | Heap cells appended this step (`0..=4`) | 3 boolean bits (`0..=7`). |
| `43` | `1` bit | `consumes_fuel` | `1` if rule decrements 128-bit `remaining_fuel` | `b * (1 - b) == 0`. |
| `44..=46` | `3` bits | `next_ctl_mode` | Next slot `ctl_mode` (`0..=7`) | 3 boolean bits (`0..=7`). |
| `47..=48` | `2` bits | `next_outcome` | `0=Running, 1=Halted, 2=Rejected, 3=Exhausted` | 2 boolean bits (`0..=3`). |
| `49..=53` | `5` bits | `micro_flags` | `reads_rom`, `reads_mem`, `checks_eq`, `resets_helper`, `is_startup` | 5 boolean bits `b_k * (1 - b_k) == 0`. |

**Why Unconditional Unpacking Bounds Close `Exploit C1` (`CVE-2026-62333` Class):** Without per-field bounds, a linear reconstruction gate `sum_i f_i * 2^{off_i} == ctrl_word (mod r)` admits `f_0 = ctrl_word + 2 * k (mod r)` and `f_1 = r - k` for arbitrary `k`, forging arbitrary `chip0..3`, `next_outcome`, or `alloc_cnt` while matching a legitimate `ctrl_word` from `TaggedTableConfig`. With boolean constraints on bits `8..=53` (`46` bits, degree-2 each) and one `RANGE` check `16 * rule_id < 4096` (`rule_id < 256`), every sub-field satisfies `0 <= f_i < 2^{w_i}`, the integer sum satisfies `0 <= sum_i f_i * 2^{off_i} < 2^{54} < r`, modular wrap-around is impossible, and bit-unpacking is a **bijection** onto `{0, 1}^{54}` (using 46 boolean cells Across unused `CORE`/helper witness slots or `5` shifted `12`-bit `RANGE` checks: `5` `RANGE` lookups check eleven 4-bit/5-bit fields at `1/3` the advice cells; see Section 3.4).

### 3.3 Invariants

**Guaranteed by `DEC`:**
1. **Deterministic Function on `packed_key`:** `dec_rows` contains no duplicate `packed_key` entries (`t_dom_key = 65,536 + packed_key` is strictly injective).
2. **Exact `/1` Rule Coverage (`GP-001`, `SVM-004`, `SVM-009`):** Every enabled `/1` transition (`121` step rules + `2` startup rules in `owner_runtime.rs:70-108, 1346`) has its exact guard expansion present; all `7` disabled and `4` `/2` extension rules have **zero** rows in `dec_rows`.
3. **Bijective `ctrl_word` Unpacking (`Exploit C1`):** For any `ctrl_word < 2^54`, the unpacked control tuple `(rule_id, vsel_grp_a..c, chip0..3, alu_op, alu_expect_ovf, stack_op, alloc_cnt, consumes_fuel, next_ctl_mode, next_outcome, micro_flags)` is uniquely determined in `Fq`.

**Assumed from Other Modules:**
- `CORE`: constrains every input sub-field of `packed_key` to its declared bit-width (`ctl_mode < 8`, `site_or_kind < 32`, `sub_disc < 16`, `fuel_is_zero, count_guard, cond_guard in {0, 1}`) and correctly computes `IsZero` flags from `ArchState`.
- `BUS` / `TaggedTableConfig`: `LookupSound` for `q_code == 2` (`tag = 1`) and `RANGE` (`tag = 0`) soundness for sub-field range checks.

### 3.4 Row Budget (`ESTIMATE`)

- **Static Table Rows (`TaggedTableConfig` `tag = 1`):** `ESTIMATE 1,180 .. 1,850` rows (`<= 2,048` hard ceiling), stacked above `RANGE` (`4,096` + `2,048` = `6,144` total static table rows `< 65,530` at `k_leaf = 16`).
- **Per-Step Advice Rows (`S = 16` and `S = 64`):**
  - `DEC` query sits inline on `CORE` row `+0` (`+0` extra `CORE` rows).
  - Bijective `ctrl_word` unpacking via `5` `12`-bit `RANGE` limb lookups (`5 x 11..12` bits covering `54` bits + `1` bit `alu_expect_ovf` / `consumes_fuel` boolean gates): `1..2` `RANGE` rows/step (`ESTIMATE 16..32` advice rows at `S = 16`; `ESTIMATE 64..128` advice rows at `S = 64`).
- **Column & Lookup Delta:** `+0` advice columns, `+0` fixed columns, `+0` lookups beyond `TaggedTableConfig` (already accounted in Section 1.4: **`ESTIMATE -29,062` 1-leaf `ChipNode` rows** and **`ESTIMATE -51,204..-51,662` 2-leaf `RunLift` `R` rows** vs E8 separate `DEC` + `RANGE`).

### 3.5 Verification

- **Quint Model (`specs/quint/sherlock_dec_bijection.qnt`, new ~190 LOC):**
  - Exhaustively enumerates all `134` `RuleIdC` variants against `is_disabled` and `is_extension`, checking partition completeness (`121 + 2 + 7 + 4 == 134`), disjointness of `packed_key` across all enabled rules, and injectivity of `pack_ctrl_word` / `unpack_ctrl_word`.
- **`MockProver` Positive Tests (`crates/mori-chip/src/dec.rs`):**
  - `dec_all_123_enabled_rules_roundtrip_ok`: queries one canonical `packed_key` for each of the `121` `/1` step rules and `2` startup rules, asserting exact `CtrlSignals` recovery.
- **Named Negative Mutants (`SAS-003`, `SAS-004`):**
  1. `mut_dec_cve_62333_wrap_forgery` (`Exploit C1`): keeps `ctrl_word` unchanged from valid `TaggedTableConfig` lookup, sets `next_outcome = 1` (`Halted`) on a running `EvalLet` step, and compensates in `rule_id` (`rule_id' = rule_id - 2^47 mod r`) -> MUST fail `rule_id` / sub-field range/boolean constraint (with a weakened control test removing the sub-field range check to prove the exploit otherwise succeeds, per `SAS-004`).
  2. `mut_dec_disabled_rule_query`: constructs `packed_key` corresponding to `BBookkeepingTaskEnqueue` (`is_disabled`) with `enable = 1` -> MUST fail `TaggedTableConfig` lookup (`SVM-004`).
  3. `mut_dec_extension_rule_query`: constructs `packed_key` for `BBookkeepingResumeStore` (`is_extension`) with `enable = 1` -> MUST fail `TaggedTableConfig` lookup (`SVM-009`).
  4. `mut_dec_duplicate_key_panic_at_load`: passes duplicate `packed_key` to `DecTable::build` -> MUST fail deterministic keygen invariant check before proving.

### 3.6 Lean Obligations

- **Theorem (`dec_unpack_bijective_and_sound`):** Prove in Lean 4 that (1) over `Fin r` (`r > 2^54`), the linear packing map `packCtrl : CtrlSignals -> Fin r` restricted to domain-bounded `CtrlSignals` (`0 <= f_i < 2^{w_i}`) is injective, so `packCtrl s1 = packCtrl s2 -> s1 = s2` (formally closing `Exploit C1`); and (2) `mDec` (`tag = 1` slice of `mTagged` plus `DEC` unpacking gates at `level = 0..1`) satisfies `Modular.Contract SherlockValid sherlockLevel mDec` (`clean/fork/Clean/Sherlock/ModularComposition.lean:63-66`), guaranteeing that every decoded `CtrlSignals` matches `sherlockDispatch1(packed_key)` and excludes all `11` disabled/extension `RuleIdC` variants.
- **Lean File Extended / Created:** Creates `clean/fork/Clean/Sherlock/DecDispatch.lean` + `DecDispatchAxioms.lean` (extending `Clean.Sherlock.DecodedCounterStages` / `ModularComposition`).
- **Premises / Axioms Relied On:** Zero custom `axiom` declarations (`LookupSound` for `tag = 0` `RANGE` and `tag = 1` `DEC` passed as `LeafRel` hypothesis).
- **Module Contract Discharged:** Discharges `Contract` for `DEC` in `sherlock_leaf_modular_refinement`.

### 3.7 Code Skeleton

Owned file: `crates/mori-chip/src/dec.rs` (replacing the 16-row stub `dec.rs:1-125`).

```rust
// crates/mori-chip/src/dec.rs
use midnight_proofs::{
    circuit::{AssignedCell, Region},
    plonk::{ConstraintSystem, Error},
};
use crate::{range::RangeCfg, tagged_table::TaggedTableConfig, F};

pub const MAX_DEC_ROWS: usize = 2048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecKey {
    pub ctl_mode: u8,       // 3 bits
    pub site_or_kind: u8,   // 5 bits
    pub sub_disc: u8,       // 4 bits
    pub fuel_is_zero: bool, // 1 bit
    pub count_guard: bool,  // 1 bit
    pub cond_guard: bool,   // 1 bit
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CtrlSignals {
    pub rule_id: u8,
    pub vsel_grp: [u8; 3],
    pub chips: [u8; 4],
    pub alu_op: u8,
    pub alu_expect_ovf: bool,
    pub stack_op: u8,
    pub alloc_cnt: u8,
    pub consumes_fuel: bool,
    pub next_ctl_mode: u8,
    pub next_outcome: u8,
    pub micro_flags: u8,
}

#[derive(Clone, Debug)]
pub struct DecUnpackCfg {
    pub tagged: TaggedTableConfig,
}

impl DecKey {
    /// Requires: all fields are within their declared bit-widths.
    /// Ensures: Returns unique `u16 < 32768` bit-packing of `self`.
    pub const fn pack(self) -> u16;
}

impl CtrlSignals {
    /// Requires: all fields are within their declared bit-widths.
    /// Ensures: Returns canonical 54-bit `u64 < (1 << 54)` encoding of `self`.
    pub const fn pack(self) -> u64;
}

/// Requires: None (derived purely from Sherlock `/1` specification).
/// Ensures: Returns deterministic `(packed_key, ctrl_word)` table (`<= 2048` rows) covering all 121 `/1` step rules + 2 startup rules and excluding all 7 disabled + 4 extension rules.
pub fn build_sherlock_v1_dec_table() -> Vec<(u16, u64)>;

impl DecUnpackCfg {
    /// Requires: `cs.phases().count() == 1`.
    /// Ensures: Registers linear `ctrl_word` reconstruction and boolean/range sub-field gates (`Exploit C1` defense) with degree <= 6 and rotations in `{-1, 0, +1}`.
    pub fn configure(cs: &mut ConstraintSystem<F>, tagged: TaggedTableConfig) -> Self;

    /// Requires: `packed_key_cell` is constrained to `< 32768` by caller (`CORE`).
    /// Ensures: Queries `(tag = 1, packed_key)` when `enable == 1` (or `(0, 0)` when `enable == 0`) and returns range/boolean-verified unpacked `CtrlSignals` cells.
    pub fn assign_and_unpack(
        &self,
        region: &mut Region<'_, F>,
        row: usize,
        enable: bool,
        packed_key_cell: &AssignedCell<F, F>,
        signals: &CtrlSignals,
        range: &RangeCfg,
    ) -> Result<AssignedCtrlCells, Error>;
}
```

### 3.8 Agent Context Boundary

- **MUST Read (`<= 5` files):**
  1. `crates/mori-chip/src/dec.rs:1-125` & `crates/mori-chip/src/tagged_table.rs`.
  2. `crates/mori-chip/src/range.rs:1-120` (limb range-check helper for unpacking).
  3. `crates/mori-core/src/sherlock/helper_machine/shared.rs:400-534` (`RuleIdC` enum).
  4. `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:70-108, 1335-1460` (`is_disabled`, `is_extension`, rule classification).
  5. `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:260-409` (dispatch sites + noting the 13 `XAdtHelper*` rules to include).
- **MUST NOT Need to Read:**
  - `crates/mori-chip/src/{poseidon.rs, membackend.rs, alu.rs}`.
  - `crates/mori-recursion/`.

### 3.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-dec/spec.md`)

- **`DEC-001` (Complete `/1` Dispatch Coverage and `/2` Refusal):** `DEC` SHALL populate `TaggedTableConfig` (`tag = 1`, `<= 2,048` rows) with every reachable guard key of all `121` active `/1` step rules and `2` startup rules (`GP-001`), and SHALL omit all `7` disabled and `4` `/2` extension rules (`SVM-004`, `SVM-009`). *(256 chars)*
  - **Scenario `S-DEC-001a` (`dec_all_123_enabled_rules_roundtrip_ok`, `mut_dec_disabled_rule_query`, `mut_dec_extension_rule_query`):** **WHEN** any enabled `/1` rule guard is queried **THEN** `MockProver` accepts; **WHEN** a disabled or `/2` extension pre-state is queried with `enable = 1` **THEN** `MockProver` rejects at the lookup stage.
- **`DEC-002` (Bijective `ctrl_word` Unpacking):** `DEC` SHALL enforce boolean or `RANGE` bounds on every unpacked sub-field of the 54-bit `ctrl_word` so linear reconstruction modulo `r` is strictly bijective and immune to modular wrap-around forgery (`Exploit C1`). *(218 chars)*
  - **Scenario `S-DEC-002a` (`mut_dec_cve_62333_wrap_forgery`, Lean `dec_unpack_bijective_and_sound`):** **WHEN** a malicious witness perturbs `next_outcome` or `chips` and compensates in another sub-field modulo `r` **THEN** `MockProver` rejects, and `DecDispatch.lean` proves unpacking injectivity over `Fin r`.

---

## 4. Module `ALU + RANGE` — Self-Ranged 128-Bit Unsigned/Signed Arithmetic and `RANGE` Limb Provider

### 4.1 Purpose

`ALU + RANGE` provides self-contained 128-bit unsigned (`Nat`) and two's-complement signed (`Int`) scalar arithmetic, comparisons, Euclidean division/remainder, and bitwise operations on `CHIP_ALU = 2`, backed by limb decomposition in `RANGE` (`tag = 0` of `TaggedTableConfig`). Unlike the E8 spike (`crates/mori-chip/src/alu.rs:8-9, 111-125`), `ALU` unconditionally range-checks both inputs and output `x, y, w < 2^128` on every active row (`Exploit D1` fix), computes overflow-exact 64-bit schoolbook multiplication with a boolean carry split `c1_bit`, and extracts explicit MSB sign bits for `Int` (`Exploit D2` fix).

### 4.2 Frozen Interface

#### A. `CHIP_ALU = 2` Bus Tuple Contract (`(2, op_key, x, y, w)`)

- **Tuple Fields:** `chip = 2`, `op_key = op | (ovf << 4)` (`op in 1..=9`, `ovf in {0, 1}`), `x` (`col 2`), `y` (`col 3`), `w` (`col 4`).
- **Range-Check Ownership:** **`ALU` unconditionally enforces `0 <= x < 2^128`, `0 <= y < 2^128`, `0 <= w < 2^128`, `ovf in {0, 1}`, and `op in 1..=9` on every active provider row** (`q_prov == 1 && act == 1`). No caller is ever trusted to pre-range `x` or `y`.

| `op` | Mnemonic | `ovf == 0` Semantics (`x, y, w < 2^128`) | `ovf == 1` Semantics (Checked Error / Overflow Path) | Auxiliary Range Checks Owned by `ALU` |
|---|---|---|---|---|
| `1` | `OP_ADD` | `x + y == w` (`< 2^128`) | `x + y == w + 2^128` (`x + y >= 2^128`, wraps to `w < 2^128`) | None beyond `x, y, w < 2^128` (`x + y < 2^129 << r`). |
| `2` | `OP_SUB` | `x == w + y` (`x >= y`) | `x + 2^128 == w + y` (`x < y`, underflow) | For signed `Int` add/sub overflow (`BHelperScalarArithmetic`), MSB sign bits `s_x, s_y, s_w` extracted; signed overflow iff `(s_x == s_y) && (s_w != s_x)` (`ADD`) or `(s_x != s_y) && (s_w != s_x)` (`SUB`). |
| `3` | `OP_MUL` | `x * y == w < 2^128` (`H == 0`, see §4.2.B) | `x * y >= 2^128` (`H != 0`, `w = (x * y) mod 2^128`) | 64-bit schoolbook limbs `p0_lo, c0, p1_lo, c1_lo < 2^64`, boolean `c1_bit in {0, 1}` (§4.2.B). |
| `4` | `OP_DIV_MOD_U` | `y != 0`, `x == w * y + rem` (`w = floor(x/y)`, `0 <= rem < y`) *(pair row or mode bit selects `rem`)* | `y == 0` (`w == 0`, primitive div-by-zero error) | `rem < 2^128`, `diff = y - 1 - rem < 2^128` (`rem < y`), and `w * y` no-overflow check via 64-bit schoolbook (`w * y <= x < 2^128`). |
| `5` | `OP_DIV_MOD_S` | `y != 0`, `!(x == 2^127 && y == 2^128 - 1)`, signed truncating quotient/remainder via `\|x\| = q_u * \|y\| + r_u` (`0 <= r_u < \|y\|`) | `y == 0` or `INT128_MIN / -1` (`x == 2^127 && y == 2^128 - 1`, `w == 0`) | `s_x, s_y in {0, 1}`, `\|x\|, \|y\|, q_u, r_u < 2^128`, `\|y\| - 1 - r_u < 2^128`. |
| `6` | `OP_LT_U` | `ovf == 0`; `w in {0, 1}`; if `w == 1` then `y - 1 - x = diff < 2^128`; if `w == 0` then `x - y = diff < 2^128` | Disallowed (`ovf == 0` constrained) | `diff < 2^128`, `w * (1 - w) == 0`. |
| `7` | `OP_LT_S` | `ovf == 0`; `w in {0, 1}`; if `s_x != s_y` then `w == s_x`; if `s_x == s_y` then same `diff < 2^128` as `OP_LT_U` (`Exploit D2` fix) | Disallowed (`ovf == 0` constrained) | MSB sign extraction `s_x, s_y in {0, 1}` (§4.2.C), `diff < 2^128`. |
| `8` | `OP_EQ` | `ovf == 0`; `w = IsZero(x - y) in {0, 1}` | Disallowed (`ovf == 0` constrained) | `(x - y) * w == 0`, `1 - w == (x - y) * inv`. |
| `9` | `OP_BITOPS` | Sub-modes (`ovf`/sub-key `0..=3`): `NOT_BOOL` (`x in {0,1} -> 1 - x`), `BIT_NOT_128` (`2^128 - 1 - x`), `BIT_AND_128` / `BIT_XOR_128` (4-bit nibble decomposition via `RANGE` `256 * nib < 4096`) | Disallowed | `x, y, w < 2^128` (plus nibble sum consistency on bitwise binary ops). |

#### B. Overflow-Exact 64-Bit Schoolbook `OP_MUL` with Boolean `c1_bit`

Direct `x * y` over `u128` reaches `(2^128 - 1)^2 ≈ 2^256 > r ≈ 2^254.86`, wrapping modulo `r`! `ALU` splits `x = x_lo + 2^64 * x_hi` and `y = y_lo + 2^64 * y_hi` (`x_lo, x_hi, y_lo, y_hi < 2^64` obtained directly from splitting the middle limb of the 128-bit `RANGE` decomposition of `x` and `y`):
1. Low product: `x_lo * y_lo == p0_lo + 2^64 * c0` with `p0_lo, c0 < 2^64` (`0 <= p0_lo + 2^64 * c0 < 2^128 < r`, zero wrap).
2. Cross-term + low carry: `m = x_lo * y_hi + x_hi * y_lo + c0`. Over `Z`, `0 <= m <= 2*(2^64 - 1)^2 + (2^64 - 1) = 2^129 - 3*2^64 + 1 < 2^129 << r` (zero wrap in `Fq`).
3. **Boolean carry split (`c1_bit`, closing 65-bit `c1` overflow):** Because `m / 2^64` can reach `2^65 - 3` (exceeding 64 bits), `ALU` decomposes:
   `m == p1_lo + 2^64 * c1_lo + 2^128 * c1_bit`
   with `p1_lo < 2^64`, `c1_lo < 2^64`, and boolean `c1_bit * (1 - c1_bit) == 0`. Max RHS is `(2^64 - 1) + 2^64*(2^64 - 1) + 2^128 = 2^129 - 1 < r`, uniquely representing every possible `m` without modular wrap.
4. Low 128-bit output: `w == p0_lo + 2^64 * p1_lo` (`< 2^128`).
5. **Exact Overflow Flag (`ovf in {0, 1}`):** Define `H = x_hi * y_hi + c1_lo + c1_bit`. Because `x_hi * y_hi >= 0`, `c1_lo >= 0`, `c1_bit >= 0`, and `0 <= H <= (2^64 - 1)^2 + (2^64 - 1) + 1 = 2^128 - 2^64 + 1 < r`, **`H == 0 (mod r)` holds if and only if `x_hi * y_hi == 0 ∧ c1_lo == 0 ∧ c1_bit == 0` in `Z`, which holds if and only if `x * y < 2^128`**. An `IsZero` gate (`ovf = 1 - (1 - H * H_inv)`, `H * (1 - ovf) == 0`) binds `ovf` exactly.

#### C. Two's-Complement Sign Extraction (`Exploit D2` Closure)

Under `B = 12` (`range.rs:30-35`), `x = sum_{i=0}^{9} l_i * 2^{12i} + l_{10} * 2^{120}` with `l_0..l_9 < 4096` and `l_{10} < 256` (`16 * l_{10} < 4096`). To extract bit `127` (`s_x in {0, 1}`):
- Witness `s_x` (`s_x * (1 - s_x) == 0`) and `lo7 = l_{10} - 128 * s_x`; range-check `32 * lo7 < 4096` (`0 <= lo7 < 128`) in place of `16 * l_{10} < 4096` (**`+0` extra `RANGE` cells**!).
- Under `B = 14` (`9 * 14 = 126` bits + 2-bit `l_9`), `l_9 = b_{126} + 2 * s_x` with two boolean gates `b_{126} in {0,1}`, `s_x in {0,1}` (**`-1` `RANGE` cell**!).

#### D. `RANGE` Limb Sizing (`12`-Bit vs `14`-Bit) and Rejection of Jolt/Lasso 2D Subword Tables

- **`RANGE` `12`-Bit (`4,096` rows, `range.rs:30-35`) vs `14`-Bit (`16,384` rows):**
  - `B = 12`: `4,096` static rows (`11` limbs/u128, or `12` cells with `64`-bit split limb `l_5 = lo4 + 16 * hi8` for `OP_MUL`); fits `k = 13` unit `MockProver` tests (`8,192` rows -> sub-second iteration) and `k_leaf = 16`.
  - `B = 14`: `16,384` static rows (`9` `14`-bit limbs + `2` boolean bits = `9` `RANGE` lookups/u128, `-18%` `RANGE` advice rows); fits `k_leaf = 16` (`65,530` rows) at **`+0` fixed columns and `+0` Node rows**, while raising unit-test minimum `k` from `13` to `15`. `RangeCfg` is parameterized by `B in {12, 14}` (Recommendation in Section 7.1: `B = 12` default for M4 fast `k = 13` unit tests; flip `B = 14` constant if `S = 64` leaf advice row utilization exceeds `75%`).
- **Why Jolt / Lasso 2D Subword Lookup Tables Are Rejected:**
  1. *Univariate Halo2/KZG table blowup:* A 16-bit 2D table `(x_16, y_16, op) -> z` has `>= 2^32` rows (impossible at `k_leaf = 16..17`); even an 8-bit 2D table `(x_8, y_8)` requires `65,536` rows PER opcode, exceeding `k_leaf = 16` usable rows (`65,530`) for a single opcode and blowing `k_leaf >= 19..20` across multiple bitwise/comparison opcodes.
  2. *`VerifierGadget` blocks Shout/Lasso multilinear/LogUp challenge tables:* Dynamic subword decomposition with `Phase(1)` random-linear-combination challenges violates `MC:verifier/verifier_gadget.rs:310` (`cs.phases().count() == 1`).
  3. *Bus request & Node lookup explosion:* Decomposing 128-bit `ADD/MUL/DIV/LT` into sixteen 8-bit 2D lookups costs `16..48` lookups per `ALU` op (vs `0` dynamic bus requests and `3..5` compact `RANGE` rows in schoolbook `ALU`) or `+2..+4` dedicated PLONK lookup arguments (`ESTIMATE +45,200 .. +90,400` 1-leaf `ChipNode` rows), while still requiring explicit carry/sign glue gates (`Exploit D2`).

### 4.3 Invariants

**Guaranteed by `ALU + RANGE`:**
1. **Unconditional Input/Output Boundedness (`Exploit D1` Closed):** Every active `CHIP_ALU = 2` tuple `(2, op_key, x, y, w)` satisfies `0 <= x, y, w < 2^128` in `Z`, `op in 1..=9`, and `ovf in {0, 1}`.
2. **Exact Arithmetic & Overflow Fidelity:** Every `op in 1..=9` matches exact integer / two's-complement 128-bit semantics of `sherlock-k-a/1` (`owner_runtime.rs`), with zero `mod r` wrap-around on `ADD`, `SUB`, `MUL` (`c1_bit` + `H` `IsZero`), `DIV_MOD_U`, `DIV_MOD_S` (`INT128_MIN / -1` and `/ 0` caught), `LT_U`, `LT_S`, `EQ`, and `BITOPS`.
3. **Inactive Zero-Gate:** Unused `ALU` provider slots (`act == 0`) constrain `(chip, key, x, y, w) == (0, 0, 0, 0, 0)`.

**Assumed from Other Modules:**
- `BUS` + `TaggedTableConfig`: `LookupSound` for `tag = 0` (`RANGE` column contains `{0, ..., 2^B - 1}` only).

### 4.4 Row Budget (`ESTIMATE`)

- **Per-Operation Cost (`ALU` + `RANGE` Combined):**
  - `ALU` block: `4` rows per allocated `ALU` slot (row `0`: bus provider `(2, op_key, x, y, w)` + opcode code selector; rows `1..3`: `x, y, w` 64-bit half-limbs, `OP_MUL` `p0_lo, c0, p1_lo, c1_lo, c1_bit, H_inv`, `DIV_MOD` `q, rem, diff`).
  - `RANGE` (`B = 12`, `4` limb lookups packed per row across `a[0..8]` or `1` running-sum column; using `3` rows per `u128` across 4 `q_code = 1` lookup sub-rows or sequential 1-col rows): `3` `u128` words (`x, y, w`) + `2` auxiliary words (`MUL`/`DIV_MOD` worst case) = `ESTIMATE 36 .. 60` single-column `RANGE` rows per active `ALU` slot (`30 .. 45` rows at `B = 14`).
- **Leaf Circuit Rows (`S = 16` and `S = 64`):**
  - Provisioning `A = S` `ALU` slots (`1` `ALU` op/step peak; most steps use `1` fuel `OP_SUB`, arithmetic steps use `2` slots amortized across non-ALU steps):
    - `S = 16` (`A = 24` `ALU` slots): `24 * 4 = 96` `ALU` rows + `ESTIMATE ~1,152` `RANGE` advice rows + `4,096` static table rows = **`ESTIMATE ~1,248` advice rows** (`1.9%` of `k_leaf = 16`).
    - `S = 64` (`A = 96` `ALU` slots): `96 * 4 = 384` `ALU` rows + `ESTIMATE ~4,608` `RANGE` advice rows = **`ESTIMATE ~4,992` advice rows** (`3.8%` of `k_leaf = 17`, or `7.6%` of `k_leaf = 16`).
- **Column & Lookup Delta:** `+0` advice columns (shares `a[0..12]`), `1` `vsel`-packed fixed column for `ALU` opcodes (`+0` net over E8 `alu.rs:34-43`, actually `-2` fixed selectors after `vsel` packing `q_add, q_lt, q_eq, ...`, `ESTIMATE -12,008` 1-leaf `ChipNode` rows and `ESTIMATE -21,600` `RunLift` `R` rows), `+0` lookups (uses `Bus` + `TaggedTableConfig`).

### 4.5 Verification

- **Quint Model (`specs/quint/sherlock_alu128.qnt`, new ~240 LOC):**
  - Models field `mod P` arithmetic (with scaled prime `P` and symbolic `2^254 < r < 2^255` bounds) for 64-bit schoolbook `OP_MUL`, `c1_bit`, `H`, signed `LT_S`, and `DIV_MOD_S`, testing all boundary corner cases `{0, 1, 2^64 - 1, 2^64, 2^127 - 1, 2^127, 2^128 - 1, r - 1}`.
- **`MockProver` Positive Tests (`crates/mori-chip/src/{alu.rs, range.rs}`):**
  - `alu_all_opcodes_boundary_vectors_ok`: tests `0`, `1`, `u64::MAX`, `i128::MAX` (`2^127 - 1`), `i128::MIN` (`2^127`), `-1` (`2^128 - 1`), `u128::MAX`, checked `ADD`/`MUL` overflow (`ovf = 1`), `DIV_MOD_U`/`S` by `0` (`ovf = 1`), and `INT128_MIN / -1` (`ovf = 1`).
- **Named Negative Mutants (`SAS-003`, `SAS-004`):**
  1. `mut_alu_unranged_input_wrap` (`Exploit D1`): feeds `x = r - 5`, `y = 10`, `w = 5` on `OP_ADD` (`x + y == 5 mod r`) -> MUST fail unconditional `x < 2^128` `RANGE` check inside `ALU` (control test commenting out `x` range check passes, proving `SAS-004` necessity).
  2. `mut_alu_lt_u_field_wrap` (`Exploit D1` on `OP_LT_U`): feeds `x = r - 1`, `y = 2`, claims `x < y` (`w = 1`, `y - 1 - x == 2 < 2^128 mod r`) -> MUST fail `x < 2^128` check.
  3. `mut_alu_mul_65bit_c1_overflow`: feeds `x = (2^64 - 1) * 2^64 + (2^64 - 1)`, `y = 2^64 + (2^64 - 1)` (`x_hi * y_hi` vs `m >= 2^128` where `c1_bit = 1, c1_lo = 0`) and attempts to claim `ovf = 0` -> MUST fail `H == 0` / `m` decomposition gate.
  4. `mut_alu_lt_s_unsigned_confusion` (`Exploit D2`): feeds `x = 2^128 - 1` (`-1`), `y = 1`, claims `LT_S(-1, 1) == 0` -> MUST fail `s_x != s_y => w == s_x` gate.
  5. `mut_alu_div_mod_s_int_min_neg1`: feeds `x = 2^127` (`i128::MIN`), `y = 2^128 - 1` (`-1`) with `ovf = 0` -> MUST fail `INT128_MIN / -1` guard.
  6. `mut_alu_div_noncanonical_rem`: feeds `rem = y` (`rem >= y`) on `OP_DIV_MOD_U` -> MUST fail `y - 1 - rem < 2^128` range check.

### 4.6 Lean Obligations (Including Evaluation of Writing `ALU` and `RANGE` Directly in Clean)

1. **Evaluation of Writing `ALU` and `RANGE` Directly in Clean (`ARCH-BRIEF.md:60`):**
   - **Repository Ground Truth (`clean/fork/README.md:3-6`, `clean/fork/lakefile.lean:5-10`):** `clean/fork` (`package sherlockSpec`, Lean `4.33.1`) imports **only Lean's `Init` library** (`no external dependencies`) and explicitly states: *"It does not contain the private Clean implementation or establish Rust, constraint or native refinement."* Furthermore, `midnight-proofs-0.8.3` (`MP:plonk/circuit.rs`) defines Halo2 PLONK regions imperatively in Rust over BLS12-381 `Fq` with zero automated Clean-DSL-to-Halo2 code extractor (upstream Clean targets AIR/Plonky3-style prime-field tables such as BabyBear/KoalaBear).
   - **What CAN and MUST Be Written Directly in `clean/fork/Clean/Sherlock/`:** Both `RANGE` and `ALU` are small, purely algebraic chips whose **exact polynomial gate equations over `Fin r` (`r = r_BLS12_381`)** can be transcribed 1-for-1 as `Clean.Sherlock.Modular.Module` instances (`clean/fork/Clean/Sherlock/ModularComposition.lean:37-40`) using Lean `4.33.1` `Init` (`Nat` / `Fin` / `omega`) with **zero external libraries**!
   - **Boundary Between Lean and `MockProver`:** `AluRange.lean` proves algebraic gate sufficiency over `Fin r` (limb reconstruction `< 2^128`, schoolbook `OP_MUL` `m < 2^129 < r` no-wrap, `H == 0 <-> x * y < 2^128`, two's-complement `LT_S` and `DIV_MOD_S` Euclidean uniqueness), while Halo2 physical cell assignment, `vsel` Lagrange degree `<= 6`, and `Rotation(-1, 0, +1)` wiring in `crates/mori-chip/src/{alu.rs, range.rs}` are verified by `MockProver` + the 6 negative mutants in §4.5.
2. **Theorems (`range_module_contract` and `alu_module_contract`):**
   - Create `clean/fork/Clean/Sherlock/AluRange.lean` + `AluRangeAxioms.lean`.
   - Prove `Contract SherlockValid sherlockLevel mRange` (`level = 0`) and `Contract SherlockValid sherlockLevel mAlu` (`level = 1`, requesting only `level = 0` `RANGE` tuples), discharging `ALU` and `RANGE` in `sherlock_leaf_modular_refinement` with **zero custom `axiom`s and zero `sorry`s**.

### 4.7 Code Skeleton

Owned files: `crates/mori-chip/src/range.rs` and `crates/mori-chip/src/alu.rs`.

```rust
// crates/mori-chip/src/range.rs
use midnight_proofs::{
    circuit::{AssignedCell, Region},
    plonk::{Advice, Column, ConstraintSystem, Error},
};
use crate::{tagged_table::TaggedTableConfig, F};

#[derive(Clone, Debug)]
pub struct Decomp128 {
    pub value: AssignedCell<F, F>,
    pub lo_u64: AssignedCell<F, F>,
    pub hi_u64: AssignedCell<F, F>,
    pub sign_bit: AssignedCell<F, F>,
}

#[derive(Clone, Debug)]
pub struct RangeCfg {
    pub tagged: TaggedTableConfig,
    pub acc_col: Column<Advice>,
    pub limb_bits: usize, // 12 (default, 4096 rows) or 14 (16384 rows)
}

impl RangeCfg {
    /// Requires: `limb_bits` in `{12, 14}` matching `tagged.range_bits`; `cs.phases().count() == 1`.
    /// Ensures: Configures running-sum decomposition gates using `Rotation::cur()` and `Rotation::next()` only.
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        tagged: TaggedTableConfig,
        acc_col: Column<Advice>,
    ) -> Self;

    /// Requires: `x < 2^128` on honest witnesses.
    /// Ensures: Constrains `0 <= x < 2^128` via `tag = 0` lookups and extracts verified `(lo_u64, hi_u64, sign_bit)` (`Exploit D2` helper).
    pub fn assign_u128_full(
        &self,
        region: &mut Region<'_, F>,
        row: &mut usize,
        x: u128,
    ) -> Result<Decomp128, Error>;
}

// crates/mori-chip/src/alu.rs
use midnight_proofs::{
    circuit::Layouter,
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed},
};
use crate::{bus::Bus, range::RangeCfg, F};

pub const OP_ADD: u8 = 1;
pub const OP_SUB: u8 = 2;
pub const OP_MUL: u8 = 3;
pub const OP_DIV_MOD_U: u8 = 4;
pub const OP_DIV_MOD_S: u8 = 5;
pub const OP_LT_U: u8 = 6;
pub const OP_LT_S: u8 = 7;
pub const OP_EQ: u8 = 8;
pub const OP_BITOPS: u8 = 9;

#[derive(Clone, Copy, Debug)]
pub struct AluRow {
    pub op: u8,
    pub sub_or_ovf: u8,
    pub x: u128,
    pub y: u128,
    pub w: u128,
}

#[derive(Clone, Debug)]
pub struct AluCfg {
    pub bus: Bus,
    pub range: RangeCfg,
    pub q_op_code: Column<Fixed>,
    pub cols: [Column<Advice>; 10],
}

impl AluCfg {
    /// Requires: `cs.phases().count() == 1`.
    /// Ensures: Configures `CHIP_ALU = 2` provider gates for `OP_ADD..OP_BITOPS` with degree <= 6, unconditional `x, y, w < 2^128` checks (`Exploit D1` fix), `c1_bit` schoolbook `MUL`, and signed MSB extraction (`Exploit D2` fix).
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        bus: Bus,
        range: RangeCfg,
        q_op_code: Column<Fixed>,
        cols: [Column<Advice>; 10],
    ) -> Self;

    /// Requires: `rows.len() <= capacity`; every active `AluRow` satisfies `op in 1..=9`.
    /// Ensures: Assigns `capacity` provider slots (gating unused slots `rows.len()..capacity` to `(0,0,0,0,0)`) and all backing `RANGE` decompositions.
    pub fn assign_batch(
        &self,
        layouter: &mut impl Layouter<F>,
        capacity: usize,
        rows: &[AluRow],
    ) -> Result<(), Error>;
}
```

### 4.8 Agent Context Boundary

- **MUST Read (`<= 5` files):**
  1. `crates/mori-chip/src/alu.rs:1-299` & `crates/mori-chip/src/range.rs:1-260`.
  2. `crates/mori-chip/src/bus.rs` & `crates/mori-chip/src/tagged_table.rs`.
  3. `crates/mori-chip/src/vsel.rs:1-120` (for packing `q_op_code`).
  4. `clean/fork/Clean/Sherlock/ModularComposition.lean:36-66` (for `AluRange.lean` contract signature).
- **MUST NOT Need to Read:**
  - `crates/mori-chip/src/{rfcore.rs, dec.rs, poseidon.rs, membackend.rs}`.
  - `crates/mori-core/` (all `/1` scalar arithmetic is fully captured by `OP_ADD..OP_BITOPS` above).

### 4.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-alu-range/spec.md`)

- **`ALU-001` (Self-Ranged 128-Bit Inputs and Outputs):** `ALU` (`CHIP_ALU = 2`) SHALL unconditionally range-check `0 <= x, y, w < 2^128` via `RANGE` (`tag = 0`) on every active provider slot (`Exploit D1`), and SHALL gate inactive slots to `(0, 0, 0, 0, 0)`. *(202 chars)*
  - **Scenario `S-ALU-001a` (`mut_alu_unranged_input_wrap`, `mut_alu_lt_u_field_wrap`):** **WHEN** a witness supplies `x >= 2^128` or `y >= 2^128` that wraps modulo `r` **THEN** `ALU`'s internal `RANGE` check rejects the witness regardless of caller constraints.
- **`ALU-002` (Overflow-Exact Schoolbook `MUL` and Signed `Int` Semantics):** `ALU` SHALL compute 64-bit schoolbook `OP_MUL` with boolean high-carry `c1_bit` and exact overflow flag `ovf <=> x * y >= 2^128`, and SHALL extract MSB sign bits for two's-complement `LT_S` and `DIV_MOD_S` (`Exploit D2`). *(222 chars)*
  - **Scenario `S-ALU-002a` (`mut_alu_mul_65bit_c1_overflow`, `mut_alu_lt_s_unsigned_confusion`, `mut_alu_div_mod_s_int_min_neg1`, Lean `alu_module_contract`):** **WHEN** `c1_bit = 1` on `OP_MUL`, `-1 < 1` on `OP_LT_S`, or `INT128_MIN / -1` on `OP_DIV_MOD_S` is misreported **THEN** `MockProver` rejects, and `AluRange.lean` proves soundness over `Fin r`.

---

## 5. Module `HASH` — Unconditional 70-Row Poseidon Permutation Engine and Capacity-IV Domain Registry

### 5.1 Purpose

`HASH` executes width-3 BLS12-381 `Fq` Poseidon permutations (`8` full + `60` partial = `68` rounds, `70` rows/block in `crates/mori-chip/src/poseidon.rs:5, 57-59`) over shared advice columns `a[0..5]` (`+0` dedicated advice columns), serving both direct copy-constrained fixed ports (`HashFixedPort`) and dynamic `CHIP_HASH = 6` bus requests. Every allocated 70-row block executes all 68 rounds unconditionally via keygen-fixed selectors (`poseidon.rs:410-414`, closing `Exploit E3` disabled-round forgery), while a frozen 64-bit capacity-IV registry (`dense.rs:60-62`) isolates every hash domain across the entire VM at zero extra permutations.

### 5.2 Frozen Interface

#### A. Slim 70-Row Permutation Block (`crates/mori-chip/src/poseidon.rs:57-59, 148-276`)

`HASH` standardizes on the slim 5-advice-column `PoseidonCfg` (`poseidon.rs:148-174`), retiring the 16-column stdlib wrapper `HashChipCfg` (`crates/mori-chip/src/hashchip.rs:38-46`, which inflated advice/fixed columns and gated rounds on dynamic regions):
- **Geometry:** `ROWS_PER_PERM = 70` (`poseidon.rs:59`):
  - Row `0` (`Absorb`): `one_abs` code (`1` = fresh sponge `st = [in0, in1, absorb_len + iv(dom, sub)]`; `2` = chained sponge `st = [prev_st0 + in0, prev_st1 + in1, prev_st2]` linked via `constrain_equal` to row `69` of the preceding block; `poseidon.rs:202-246`).
  - Rows `1..=4` (`4` initial full S-box rounds `x^5`, degree `5` + `1` fixed selector = `6`).
  - Rows `5..=64` (`60` partial rounds: S-box `x^5` on `st[0]` only, linear MDS on `st[1..2]`).
  - Rows `65..=68` (`4` terminal full S-box rounds).
  - Row `69` (`Output` / Bus Provider): holds post-permutation `[st'[0], st'[1], st'[2]]`; `out_d = st'[0]` (`poseidon.rs:265-275`).
- **Unconditional Round Execution (`Exploit E3` Defense):** On every allocated 70-row block `b in 0..P_total`, `q_full` (rows `1..=4, 65..=68`) and `q_part` (rows `5..=64`) and round constants `rc[0..3]` are **fixed at keygen (`1` unconditionally)**. No witness cell can skip or zero an internal round.
  - **Fixed-Port Blocks (`0..P_fixed`):** `q_prov = 0` on row `69`. Callers (`BOUNDARY COMMITMENT`, `MEM` flush/tree, `ROM` Merkle) connect inputs/outputs on row `0` and row `69` via `constrain_equal` (`poseidon.rs:433-460`). Unused fixed blocks (e.g., unexecuted `MEM` flushes) assign `in0 = 0, in1 = 0` and compute honest `PoseidonPerm(0, 0, iv)` without affecting any bus channel.
  - **Dynamic Bus Blocks (`P_fixed .. P_total`, `CHIP_HASH = 6`):** Row `69` sets `q_prov = 1` and assigns `(act * 6, act * hash_key, act * in0, act * in1, act * st'[0])` (`act in {0, 1}`), where `in0, in1, hash_key` on row `69` are `constrain_equal`-copied from row `0`, and row `0` sets `st[2] = absorb_len + iv_from_key(hash_key)`. When `act == 0`, row `0..68` still runs all 68 Poseidon rounds on `(0, 0, iv_default)` while row `69` outputs `(0, 0, 0, 0, 0)` on the bus (`BUS` Universal Rule 2).

#### B. Complete Frozen Capacity-IV Domain Registry (`dense.rs:54-83`, Extended VM-Wide)

Initial capacity state is `st[2] = absorb_len + (((dom << 16) | sub) << 32)` (`dense.rs:60-62`, `poseidon.rs:101-110`), packing three non-overlapping integer fields into `64` bits (`< 2^64 << r`):
- Bits `0..=31`: `absorb_len` (`0 <= absorb_len < 2^32`, exact count of absorbed field elements).
- Bits `32..=47`: `sub` (`0 <= sub < 2^16`, Merkle height `h`, `ROM` table ID, schema version, or claim sub-tag).
- Bits `48..=63`: `dom` (`1 <= dom <= 12`, strictly non-zero so `st[2] != 0` even at `absorb_len == 0`, preventing collision with raw `0`-capacity permutations).

| `dom` | Constant | `sub` (`16` bits) | `absorb_len` | Port Mode | Exact Consumer & Preimage Schema |
|---|---|---|---|---|---|
| `1` | `DOM_REC` | `0` | `4` (`2` blocks) | Fixed (`MEM`) | Heap record leaf digest `H_REC(kind, f0, f1, f2)` (`dense.rs:54, 65-69`). |
| `2` | `DOM_PAD` | `0` | `2` (`1` block) | Sentinel const | Unwritten tail slot sentinel `pad_d() = H(DOM_PAD, 0, [0, 0]) != 0` (`dense.rs:55, 71-74`). |
| `3` | `DOM_EMPTY` | `0` | `2` (`1` block) | Sentinel const | Empty 8-cell segment root `EMPTY_SEG = H(DOM_EMPTY, 0, [0, 0]) != 0` (`dense.rs:56, 76-79`). |
| `4` | `DOM_NODE` | `h in 1..=66` | `2` (`1` block) | Fixed (`MEM`) | Height-tagged binary Merkle node `H_NODE(h, left_d, right_d)` (`h = 1..=3` intra-segment `Tree8`, `h = 4..=66` top tree; `dense.rs:57, 81-83`). |
| `5` | `DOM_PROG` | `(tbl_id << 8) \| h` (`h in 1..=29`) | `2` (`1` block) | Fixed (`ROM`) | Static program table Merkle node authenticating table `tbl_id in 1..=14` into `prog_d`. |
| `6` | `DOM_TAIL8` | `h in 1..=3` | `2` (`1` block) | Fixed (`MEM` / Boundary) | Partial 8-cell register-tail fold (`rec_d` for `idx < rem`, `pad_d()` for `idx >= rem`), kept domain-distinct if committed alongside flushed `Tree8`. |
| `7` | `DOM_LEAF8` | `tbl_id in 1..=14` | `24` (`12` blocks) | Fixed (`ROM`) | Static 8-row x 3-column `ROM` cache-line leaf digest under `prog_d`. |
| `8` | `DOM_STATE` | `1` (schema v1) | `ARCH_STATE_CELLS` (`41`, `21` blocks) | Fixed (`BOUNDARY`) | `PoseidonBoundary` canonical leaf state digest `pre_d` / `post_d` (`M8a`). |
| `9` | `DOM_FRAME` | `0` (empty) / `1` (link) | `2` (`0` sentinel) or `5` (`3` blocks) | Dynamic (`CHIP_HASH = 6`) | `EMPTY_STACK_FRAME = H(DOM_FRAME, 0, [0, 0]) != 0`; link `fp_hash' = H(DOM_FRAME, 1, [fp_hash, f0, f1, f2, f3])`. |
| `10` | `DOM_TASK` | `0` (empty) / `1` (link) | `2` (`0` sentinel) or `5` (`3` blocks) | Dynamic (`CHIP_HASH = 6`) | `EMPTY_STACK_TASK = H(DOM_TASK, 0, [0, 0]) != 0`; link `tp_hash' = H(DOM_TASK, 1, [tp_hash, t0, t1, t2, t3])`. |
| `11` | `SHK_VAL` | `val_tag in 1..=7` | `2` or `2 * k` | Dynamic (`CHIP_HASH = 6`) | Canonical `sherlock-value/1` digest (`Unit, Bool, Int, Nat, String, List, Ctor` recursive fold; `GP-001`, `GP-002`). |
| `12` | `DOM_CLAIM` | `1=OUTCOME, 2=RUN, 3=RULES, 4=INPUT` | `2..=4` | Dynamic (`CHIP_HASH = 6` / `DECIDER`) | Sealed outcome/claim/input wrappers (`outcome_d`, `claim_d = H(DOM_CLAIM, 2, [prog_d, input_d, outcome_d, n_steps])` for `GP-002` consumption). |

**Non-Zero Sentinel Verification (`AUDIT.md:63`):** `pad_d()`, `EMPTY_SEG`, `EMPTY_STACK_FRAME`, and `EMPTY_STACK_TASK` are deterministic compile-time constants evaluated via `poseidon_perm` (`poseidon.rs:85-98`); an init assertion `assert_ne!(sentinel, F::ZERO)` runs at keygen and in unit tests.

### 5.3 Invariants

**Guaranteed by `HASH`:**
1. **Unconditional Full-Round Execution (`Exploit E3` Closed):** Every allocated 70-row block satisfies exact 68-round BLS12-381 Poseidon transition equations (`st' = PoseidonPerm(st)`), regardless of witness values or `act` flags.
2. **Pairwise Disjoint Domain Separation:** For any two distinct triples `(dom_1, sub_1, len_1) != (dom_2, sub_2, len_2)` in the registry (`dom in 1..=12`, `sub < 2^16`, `len < 2^32`), initial capacity elements satisfy `st_1[2] != st_2[2]` in `Fq` (`0 < st[2] < 2^64 < r`).
3. **Dynamic Bus Binding & Inactive Zero-Gate:** On every dynamic `CHIP_HASH = 6` block (or multi-block sponge chain), `hash_key` on row `69` binds `(dom, sub, absorb_len)` used in row `0`'s `st[2]`, `in0, in1` equal absorbed inputs, `out_d == st'[0]`, and `act == 0` gates row `69`'s bus tuple to `(0, 0, 0, 0, 0)`.

**Assumed from Other Modules:**
- `BUS`: `LookupSound` for `CHIP_HASH = 6`.
- Fixed-port callers (`BOUNDARY COMMITMENT`, `MEM`, `ROM`): wire `AssignedCell<F, F>` inputs and outputs via `constrain_equal` to row `0` and row `69` of their owned `HashFixedPort` handle.

### 5.4 Row Budget (`ESTIMATE`)

- **Per-Permutation Cost:** Exactly `70` advice rows (`poseidon.rs:59`) over shared columns `a[0..5]`.
- **Leaf Circuit Rows (`S = 16` and `S = 64`):**
  - *Boundary commitment (`PoseidonBoundary`, `pre_d` + `post_d`, `21` perms each):* `42 * 70 = 2,940` rows (constant across `S = 16` and `S = 64`; drops to `0` if Horner `BOUNDARY COMMITMENT` is selected in Area 2).
  - *Dynamic step hashes (`CHIP_HASH = 6` for `FRAMES`, `TASKS`, `SHK_VAL`, `DOM_CLAIM`):*
    - `S = 16` (`P_dyn = 48` blocks, e.g. `16` 3-block frame/task spills or value folds): `48 * 70 = 3,360` rows.
    - `S = 64` (`P_dyn = 192` blocks): `192 * 70 = 13,440` rows.
  - *Memory & `ROM` fixed-port hashes (`Tree8` + Merkle paths / `ZM-LogUp`):*
    - With `ZM-LogUp` (`MEMORY BACKEND` B) + `ROM` (`K_rom = 4` lines x `12 + 16 = 28` perms = `112` blocks at `S = 16`; `K_rom = 8` -> `224` blocks at `S = 64`): `7,840` rows (`S = 16`, total leaf `HASH` = **`ESTIMATE ~14,140` rows**, `21.6%` of `k_leaf = 16`) and `15,680` rows (`S = 64`, total leaf `HASH` = **`ESTIMATE ~32,060` rows**, `48.9%` of `k_leaf = 16` or `24.5%` of `k_leaf = 17`).
    - With Dense Depth-61 Merkle (`MEMORY BACKEND` A, `W = 2` flushes + `K_page = 4` reads = `6 * 64 = 384` perms): `26,880` rows (`S = 16`, total leaf `HASH` = **`ESTIMATE ~40,980` rows**, fitting `k_leaf = 16` `65,530` rows!) and at `S = 64` (`W = 8, K_page = 8` -> `1,024` perms = `71,680` rows, total leaf `HASH` = **`ESTIMATE ~103,740` rows**, fitting `k_leaf = 17` `131,066` rows!).
- **Column & Lookup Delta:**
  - Advice columns: `0` extra (reuses `a[0..5]`, `poseidon.rs:148-151`).
  - Fixed columns: `4` round-constant columns `rc[0..3]` + `1` `one_abs`/round selector (`poseidon.rs:152-156`, already included in E8 `9..10`-fixed-column baseline; `-6` advice and `-8` fixed columns vs legacy `HashChipCfg`, `hashchip.rs:38-46`).
  - Lookups: `0` new (uses `Bus` `CHIP_HASH = 6`).
  - **1-Leaf `ChipNode` and 2-Leaf `RunLift` `R` Impact:** **`ESTIMATE +0` Node rows** at `k_leaf = 16` (`+4` Node rows / `+8` `R` rows if `k_leaf = 16 -> 17` at `S = 64`).

### 5.5 Verification

- **Quint Model (`specs/quint/sherlock_hash_domains.qnt`, new ~150 LOC):**
  - Models `iv(dom, sub)` + `absorb_len` across all 12 domains, proving `InvDomainInjectivity` (`iv_cap(d1, s1, l1) == iv_cap(d2, s2, l2) => (d1, s1, l1) == (d2, s2, l2)`), `InvNonZeroCapacity` (`iv_cap > 0`), and sponge chain integrity across multi-block dynamic requests.
- **`MockProver` Positive Tests (`crates/mori-chip/src/poseidon.rs`):**
  - `poseidon_matches_midnight_stdlib_kat`: verifies bit-for-bit equality between 70-row `PoseidonCfg` and `midnight-circuits-7.3.0` reference Poseidon (`poseidon.rs:750-795`) across 1-block, 3-block (`DOM_FRAME`), 12-block (`DOM_LEAF8`), and 21-block (`DOM_STATE`) inputs.
  - `hash_mixed_fixed_and_dynamic_bus_ok`: runs fixed-port blocks (`q_prov = 0`) and active + idle dynamic `CHIP_HASH = 6` blocks (`act = 1` and `act = 0`) in one region.
- **Named Negative Mutants (`SAS-003`, `SAS-004`):**
  1. `mut_hash_skip_internal_round_e3` (`Exploit E3`): corrupts state at round `34` on an active or idle block -> MUST fail unconditional `q_part` gate on row `35`.
  2. `mut_hash_cross_domain_substitution`: proves a `DOM_REC = 1` digest and attempts to consume it as `DOM_FRAME = 9` on `CHIP_HASH = 6` -> MUST fail `hash_key <-> st[2]` binding gate.
  3. `mut_hash_sponge_length_truncation`: feeds a 2-element prefix digest for a `len = 5` `DOM_FRAME` request -> MUST fail `absorb_len` field in `st[2]` and final block link.
  4. `mut_hash_inactive_dynamic_bus_leak`: sets `act = 0` on a `CHIP_HASH = 6` block with `out_d != 0` on row `69` -> MUST fail `(1 - act) * c == 0` zero-gate.

### 5.6 Lean Obligations

- **Theorem (`hash_domain_injective_and_contract`):**
  1. Prove over `Fin r` (`r > 2^64`) that `capInit (dom : Fin 13) (h_pos : dom.val > 0) (sub : Fin 65536) (len : Fin 4294967296) : Fin r` is strictly injective and strictly non-zero (`capInit != 0`), so under an Ideal-Permutation / collision-resistance hypothesis `PermCollisionResistant` on `PoseidonPerm`, digests from distinct domains (`DOM_REC..DOM_CLAIM`), distinct Merkle heights `sub = h`, or distinct lengths are collision-free (discharging tree-splicing / cross-domain non-aliasing for Area 1 stacks, Area 2 Merkle/ROM trees, and Area 3 claims).
  2. Prove `Modular.Contract SherlockValid sherlockLevel mHash` (`level = 1`, `requests = []`, `clean/fork/Clean/Sherlock/ModularComposition.lean:63-66`).
- **Lean File Extended / Created:** Creates `clean/fork/Clean/Sherlock/HashDomains.lean` + `HashDomainsAxioms.lean`.
- **Premises / Axioms Relied On:** Zero custom `axiom`s; cryptographic collision resistance is carried as explicit structure premise `HashCollisionResistant` (matching `Aggregation.lean:64-105`).

### 5.7 Code Skeleton

Owned file: `crates/mori-chip/src/poseidon.rs` (extended with full domain registry & `CHIP_HASH = 6` dynamic port; `crates/mori-chip/src/hashchip.rs` deprecated/re-exported as compatibility shim only).

```rust
// crates/mori-chip/src/poseidon.rs
use midnight_proofs::{
    circuit::{AssignedCell, Layouter, Region},
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed},
};
use crate::{bus::Bus, F};

pub const ROUNDS: usize = 68;
pub const ROWS_PER_PERM: usize = ROUNDS + 2; // 70

pub const DOM_REC: u16 = 1;
pub const DOM_PAD: u16 = 2;
pub const DOM_EMPTY: u16 = 3;
pub const DOM_NODE: u16 = 4;
pub const DOM_PROG: u16 = 5;
pub const DOM_TAIL8: u16 = 6;
pub const DOM_LEAF8: u16 = 7;
pub const DOM_STATE: u16 = 8;
pub const DOM_FRAME: u16 = 9;
pub const DOM_TASK: u16 = 10;
pub const DOM_SHK_VAL: u16 = 11;
pub const DOM_CLAIM: u16 = 12;

/// Requires: `dom in 1..=12`.
/// Ensures: Returns non-zero `F::from((((dom as u64) << 16 | sub as u64) << 32) | len as u64)`.
pub fn domain_cap_iv(dom: u16, sub: u16, len: u32) -> F;

/// Ensures: Returns non-zero compile-time sentinels `pad_d()`, `empty_seg()`, `empty_stack_frame()`, `empty_stack_task()`.
pub fn empty_stack_frame() -> F;
pub fn empty_stack_task() -> F;

#[derive(Clone, Debug)]
pub struct HashFixedPort {
    pub in_cells: Vec<AssignedCell<F, F>>,
    pub out_cell: AssignedCell<F, F>,
}

#[derive(Clone, Debug)]
pub struct PoseidonCfg {
    pub st: [Column<Advice>; 3],
    pub aux: [Column<Advice>; 2],
    pub rc: [Column<Fixed>; 3],
    pub q_round: Column<Fixed>,
    pub bus: Bus,
}

impl PoseidonCfg {
    /// Requires: `cs.phases().count() == 1`; `st` and `aux` alias `a[0..5]`.
    /// Ensures: Configures unconditional 68-round Poseidon gates (`Exploit E3` fix) with degree <= 6 and rotations in `{-1, 0, +1}`, plus `CHIP_HASH = 6` output row gate.
    pub fn configure(
        cs: &mut ConstraintSystem<F>,
        a_prefix: [Column<Advice>; 5],
        rc: [Column<Fixed>; 3],
        q_round: Column<Fixed>,
        bus: Bus,
    ) -> Self;

    /// Requires: `dom in 1..=12`; `inputs.len() >= 1`.
    /// Ensures: Assigns `ceil(inputs.len() / 2)` unconditional 70-row blocks (`q_prov = 0`) and returns `HashFixedPort` for `constrain_equal` wiring.
    pub fn assign_fixed_sponge(
        &self,
        region: &mut Region<'_, F>,
        row_cursor: &mut usize,
        dom: u16,
        sub: u16,
        inputs: &[F],
    ) -> Result<HashFixedPort, Error>;

    /// Requires: `requests.len() <= block_budget`; each request has `dom in {9, 10, 11, 12}`.
    /// Ensures: Assigns `block_budget` unconditional 70-row blocks, serving active `CHIP_HASH = 6` tuples on `Bus` and zero-gating unused blocks to `(0,0,0,0,0)`.
    pub fn assign_dynamic_bus_pool(
        &self,
        layouter: &mut impl Layouter<F>,
        block_budget: usize,
        requests: &[DynamicHashReq],
    ) -> Result<(), Error>;
}
```

### 5.8 Agent Context Boundary

- **MUST Read (`<= 4` files):**
  1. `crates/mori-chip/src/poseidon.rs:1-470, 740-797`.
  2. `crates/mori-chip/src/dense.rs:50-90` (`DOM_*` constants and `iv()` encoding).
  3. `crates/mori-chip/src/bus.rs` (`CHIP_HASH = 6` tuple layout).
  4. `crates/mori-chip/src/hashchip.rs:1-140` (solely to verify deprecation parity).
- **MUST NOT Need to Read:**
  - `crates/mori-chip/src/{rfcore.rs, dec.rs, alu.rs, range.rs, membackend.rs}`.
  - `crates/mori-core/` or `crates/mori-recursion/`.

### 5.9 OpenSpec 1.17.0 Capability Spec (`specs/sherlock-hash/spec.md`)

- **`HASH-001` (Unconditional 70-Row Poseidon Execution):** `HASH` SHALL execute all `68` Poseidon rounds (`8` full + `60` partial) unconditionally via keygen-fixed selectors on every allocated `70`-row block (`Exploit E3`), sharing `a[0..5]` with zero extra advice columns, and zero-gating unused `CHIP_HASH = 6` bus outputs to `(0, 0, 0, 0, 0)`. *(282 chars)*
  - **Scenario `S-HASH-001a` (`poseidon_matches_midnight_stdlib_kat`, `mut_hash_skip_internal_round_e3`, `mut_hash_inactive_dynamic_bus_leak`):** **WHEN** any internal round witness is altered on an active or idle block **THEN** `MockProver` rejects; **WHEN** honest inputs run **THEN** outputs match `midnight-circuits` Poseidon KATs.
- **`HASH-002` (Injective Capacity-IV Domain Isolation):** `HASH` SHALL initialize capacity `st[2] = absorb_len + (((dom << 16) | sub) << 32)` across frozen domains `dom in 1..=12`, guaranteeing non-zero pairwise-disjoint IVs and non-zero sentinels (`pad_d()`, `EMPTY_SEG`, `EMPTY_STACK_*`). *(234 chars)*
  - **Scenario `S-HASH-002a` (`mut_hash_cross_domain_substitution`, Quint `InvDomainInjectivity`, Lean `hash_domain_injective_and_contract`):** **WHEN** a witness substitutes a digest from another `(dom, sub, absorb_len)` triple **THEN** `MockProver` rejects, and `HashDomains.lean` proves IV injectivity over `Fin r`.

---

## 6. Post-Prototype OpenSpec 1.17.0 Change Outline for Area 1 (`ARCH-BRIEF.md:62-68`)

After gating prototype measurements complete, Area 1 ships inside a new OpenSpec 1.17.0 change **`openspec/changes/sherlock-modular-vm-20261115/`** (separate from `sherlock-vm-consensus-20261107`).

### 6.1 `proposal.md` Scope (Area 1 Portion)

- **Why:** Replace the monolithic C6 dispatch / E8 spike leaf (`c6_dispatch_step.rs`, `rfcore.rs`) with five isolated, bus-coupled Area 1 processor modules (`BUS`, `CORE`, `DEC`, `ALU + RANGE`, `HASH`) satisfying `MODULARITY.md:1-12` and preserving `GP-1` through `GP-4` (`general-purpose.md:7-53`, `requirements.md:56-59`) verbatim without narrowing.
- **What Changes (Area 1):**
  1. Freeze `Bus` (`chip in {0, 2, 3, 4, 5, 6}`) + 2-column `TaggedTableConfig` (`3` total leaf lookups, `-1` lookup and `-1` `TableColumn` vs E8).
  2. Ship `DEC` covering all `121` active `/1` step rules + `2` startup rules with `Exploit C1`-proof bijective `ctrl_word` unpacking.
  3. Ship self-ranged `ALU + RANGE` (`Exploit D1` + `Exploit D2` + `c1_bit` schoolbook `MUL`) with `AluRange.lean` algebraic soundness proof.
  4. Ship unconditional 70-row `HASH` (`Exploit E3` closed) with the 12-domain capacity-IV registry and `HashDomains.lean`.
  5. Ship parameterized `CORE` (`LeafShape` `S = 16` / `S = 64`), top-of-stack `Frame`/`Task` caches, `v0..v2` per-slot isolation (`SRC-023`), and `SherlockBusContract.lean` + `CoreRefinement.lean` modular composition proof.
- **Capabilities Created (Area 1):** `specs/sherlock-bus/spec.md`, `specs/sherlock-core/spec.md`, `specs/sherlock-dec/spec.md`, `specs/sherlock-alu-range/spec.md`, `specs/sherlock-hash/spec.md`.

### 6.2 `tasks.md` Build-Order Checklist (Area 1 Module Kits)

Each task is ticked **only** when its `MockProver` positive tests, `SAS-004` negative mutants (including weakened-constraint controls), Quint invariant checks, and `lake -d clean/fork build` zero-custom-axiom checks pass:

- [ ] **Task 1.1 (`BUS` + `TaggedTableConfig` Kit):** Implement `crates/mori-chip/src/{bus.rs, tagged_table.rs}`; pass `bus_four_requests_per_step_ok`, `tagged_table_range_and_dec_coexist_ok`, and all 5 mutants (`mut_bus_*`, `mut_tagged_*`); check `sherlock_bus.qnt`; verify `Clean/Sherlock/SherlockBusContract.lean` (`#print axioms`).
- [ ] **Task 1.2 (`HASH` Kit — Level 1 Leaf Provider, Zero Dependencies on `DEC`/`ALU`):** Implement `crates/mori-chip/src/poseidon.rs` 12-domain registry + `CHIP_HASH = 6` pool; deprecate `hashchip.rs`; pass KAT + 4 `mut_hash_*` mutants; check `sherlock_hash_domains.qnt`; verify `Clean/Sherlock/HashDomains.lean`.
- [ ] **Task 1.3 (`ALU + RANGE` Kit — Level 0/1 Providers):** Implement `crates/mori-chip/src/{range.rs, alu.rs}`; pass boundary vectors + 6 `mut_alu_*` mutants (with `Exploit D1` weakened control); check `sherlock_alu128.qnt`; verify `Clean/Sherlock/AluRange.lean`.
- [ ] **Task 1.4 (`DEC` Kit — Level 0/1 Dispatch):** Implement `crates/mori-chip/src/dec.rs` (`121 + 2` `/1` rules, 54-bit `ctrl_word`); pass 123-rule roundtrip + 4 `mut_dec_*` mutants (with `Exploit C1` weakened control); check `sherlock_dec_bijection.qnt`; verify `Clean/Sherlock/DecDispatch.lean`.
- [ ] **Task 1.5 (`CORE` Kit — Level 2 Spec Module & Area 1 Integration):** Implement `crates/mori-chip/src/{rfcore.rs, rf.rs, vsel.rs}` 7/8-row `Arch::Narrow` slot; pass `S = 16` vs `S = 4` partition test + `h = 2^64 - 1` boundary + 7 `mut_core_*` mutants; check `sherlock_core_slots.qnt`; verify `Clean/Sherlock/CoreRefinement.lean` + `PartitionIndependence.lean`; run differential trace oracle against `owner_runtime.rs` across all `121 + 2` rules (`SAS-005`).

### 6.3 Gating Pre-Specification Experiments and Decision Thresholds (`P1`, `P3`, `P4`)

Before freezing `openspec/changes/sherlock-modular-vm-20261115/design.md`, three bounded prototype measurements MUST complete:

| Experiment | What Is Measured | Exact Pass / Selection Threshold | Fallback Triggered If Threshold Fails |
|---|---|---|---|
| **`P1` (`TaggedTableConfig` + `vsel` Node Fit)** | Build a synthetic leaf with final `(N_adv = 12..15, N_fix, N_eq = 14, N_lookup = 3, max_deg = 6)` and measure exact `VerifierGadget` row count in 1-leaf `ChipNode`, 2-leaf `RunLift` `R`, and `Decider` (`Pow2Range`). | **Pass:** 1-leaf `ChipNode <= 900,000` rows (`>= 50,000` margin below `950,000` gate, confirming `>= 24,000` row drop from `4 -> 3` lookups); `Decider` circuit rows `<= 1,000,000` AND `Pow2Range <= 1,040,000` (`>= 8,570` headroom below `1,048,570`); if `RunLift` `R <= 950,000` rows at `k = 20`, unlock 2-leaf tree node (`GP-4`). | If `RunLift` `R > 950,000` rows at `k = 20`: either trim 2 advice/eq columns (`-27,000` `R` rows) or adopt split accumulation (`general-purpose.md:57`) for `GP-4` arity-2 node rather than bumping `k = 20 -> 21`. |
| **`P3` (`S = 16` vs `S = 64` & `RANGE` `12`-Bit vs `14`-Bit Leaf Utilization)** | Measure exact leaf advice row sum (`CORE + DEC + ALU + RANGE + HASH + MEM`) and wall-clock leaf + node proving time at `(S = 16, k_leaf = 16)` vs `(S = 64, k_leaf = 16/17)` under `B = 12` and `B = 14`. | **Select `B = 12`:** if total leaf rows at target `S` fit `<= 58,000` (`k_leaf = 16`) or `<= 118,000` (`k_leaf = 17`), preserving `k = 13` fast unit tests. **Select `B = 14`:** if `B = 12` overflows `65,530` (`k_leaf = 16`) or `131,066` (`k_leaf = 17`) and `-18%` `RANGE` rows keeps `k_leaf` one power of two smaller. **Select `S = 64` post-M4:** if end-to-end proving time per 1,000 VM steps drops by `>= 2.0x` vs `S = 16` with `ChipNode` delta `<= +8` rows. | Keep `S = 16, k_leaf = 16, B = 12` if `k_leaf = 17` causes memory pressure above 48 GB heavy-runner cap (`clean/fork/README.md:16`). |
| **`P4` (Area 2 Memory Backend & Boundary Interaction with `HASH`)** | Measure leaf `HASH` block demand under Area 2 Option A (`DensePageBackend` depth-61 Merkle, `membackend.rs`) vs Option B (`ZM-LogUp` single-phase memory) and `PoseidonBoundary` (`42` perms) vs Horner boundary under real `VerifierGadget`. | **Select `ZM-LogUp`:** if it passes `VerifierGadget` (`cs.phases().count() == 1`) with `<= +2` advice/eq columns (`ChipNode <= 920,000` rows), cutting leaf `HASH` rows by `~26,880` (`S = 16`) / `~71,680` (`S = 64`). **Select `DensePageBackend`:** if `ZM-LogUp` requires `Phase(1)` challenges (blocked by `MC:verifier/verifier_gadget.rs:310`) or pushes `ChipNode > 935,000` rows. | `DensePageBackend` (`membackend.rs:1-460`) already compiles single-phase today and fits `k_leaf = 16` (`S = 16`) and `k_leaf = 17` (`S = 64`) at `+0` Node columns. |

---

## 7. Open Decisions in Area 1 (`ARCH-BRIEF.md:35`)

### OD-1: `RANGE` Limb Width — `12`-Bit (`4,096` Rows) vs `14`-Bit (`16,384` Rows)

- **Option A (`12`-bit, `4,096` rows, `11` limbs/u128):** Current spike width (`crates/mori-chip/src/range.rs:30-35`). Fits `k = 13` (`8,192` rows) for sub-second `MockProver` unit/mutant runs; cleanly splits at bit `64` (`5 * 12 = 60` + `4`-bit nibble of `l_5`) for schoolbook `OP_MUL`.
- **Option B (`14`-bit, `16,384` rows, `9` limbs + `2` boolean bits/u128):** Cuts `RANGE` advice rows by `~18%` (`9` table lookups vs `11`) and gives free boolean MSB sign bit `b_{127}` (`l_9 = b_{126} + 2*b_{127}`) at `+0` fixed columns and `+0` Node rows, but forces every unit test loading `TaggedTableConfig` to `k >= 15` (`32,768` rows, `~4x` slower `MockProver` iteration).
- **Recommendation:** **Parameterize `RangeCfg` (`limb_bits: usize`) and default to Option A (`12`-bit) for M4** (`~1,152` `RANGE` rows at `S = 16` is `< 2%` of `k_leaf = 16`, so `14`-bit saves zero `k_leaf` powers while slowing unit tests `4x`; switch constant to `14`-bit only if `P3` shows `S = 64` sits on the `65,530`-row cliff).

### OD-2: `DEC` Rule Coverage — `121 + 2` Complete `/1` Inventory (`"120 + 2"`) vs `108` `c6_dispatch_step.rs` Subset

- **Option A (`108` rules from `c6_dispatch_step.rs:363-409`):** Omits the `13` `XAdtHelper{Build,Plan}*` rules (`shared.rs:522-534`) and `2` startup rules (`KAdminStartup*`). Fails `GP-001` acceptance (*"builds a constructor of more than 8 fields across a leaf boundary"*, `general-purpose.md:16`, `requirements.md:56`), which directly exercises `XAdtHelperBuild*` and `XAdtHelperPlan*`!
- **Option B (`121` active `/1` step rules + `2` startup rules = `123` enabled `/1` rules):** Covers 100% of non-disabled, non-extension `RuleIdC` variants in `owner_runtime.rs:70-108, 1346` within `<= 2,048` static `TaggedTableConfig` rows (`+0` columns, `+0` Node rows vs `108` rules).
- **Recommendation:** **Adopt Option B (`121 + 2` complete `/1` rules) unconditionally.** Narrowing `/1` by dropping `XAdtHelper*` directly violates `GP-001` (`general-purpose.md:9, 16, 58`).

### OD-3: `CORE` Step Geometry — `Arch::Narrow` (`12` Advice Cols, `7..8` Rows/Step) vs `Arch::Wide` (`20..24` Advice Cols, `2..3` Rows/Step)

- **Option A (`Arch::Narrow`, `12` shared advice cols, `7..8` rows/step):** Consumes only `112..128` (`S = 16`) or `448..512` (`S = 64`) leaf rows (`< 0.8%` of leaf rows) while keeping leaf advice + equality column counts at minimum (`12..15` adv).
- **Option B (`Arch::Wide`, `20..24` advice cols, `2..3` rows/step, `rfcore.rs:150-167`):** Saves ~350 leaf rows that are already free under `HASH`-dominated `k_leaf = 16..17`, while adding `+8..+10` advice + equality columns = **`ESTIMATE +123,896 .. +154,870` 1-leaf `ChipNode` rows** (`design.md:134`), blowing past the `950,000` `k = 20` Node gate (`823,466 + 123,896 = 947,362 .. 978,336`) and destroying `RunLift` `R` at `k = 20`.
- **Recommendation:** **Adopt Option A (`Arch::Narrow`) unconditionally** and remove `Arch::Wide` from production `rfcore.rs`.

### OD-4: Steps per Leaf `S` — `S = 16` (`k_leaf = 16`, M4) vs `S = 64` (`k_leaf = 17`, Post-M4)

- **Option A (`S = 16`, `k_leaf = 16`):** Matches existing `SRC-014` wording (`requirements.md:18`) for M4 milestone closure.
- **Option B (`S = 64`, `k_leaf = 17`):** Cuts recursive `ChipNode` folds (the dominant `~95%` proving bottleneck) by **`4x`** across multi-leaf runs while adding only **`+4` rows** to 1-leaf `ChipNode` and **`+0` rows** to `Decider` (`design.md:135`).
- **Recommendation:** **Parameterize `CoreConfig` via `LeafShape { s_max }` (`SRC-023` verified across `S`), qualify M4 at `S = 16` if `SRC-014` remains unamended, and amend `SRC-014` to `1..=S` (`S = 64`) immediately post-M4** (gated by `P3`).

---

## 8. Conflicts with Existing Spec (`design.md`, `requirements.md`, `dissent.md`) or Studies (`ARCH-BRIEF.md:36`)

1. **`C-1` — Legacy Spike `CHIP_*` IDs vs Frozen 6-Chip Registry:**
   - *Conflict:* `crates/mori-chip/src/bus.rs:30-31`, `crates/mori-chip/src/hashchip.rs:32`, and `crates/mori-chip/src/poseidon.rs:53` define `CHIP_ALU = 1`, `CHIP_MEM = 2`, `CHIP_HASH = 3`, whereas `research/sherlock-lookup-cache-20261111/CONSENSUS.md:197-205` freezes `0 = NOP`, `2 = ALU`, `3 = MEM_ALLOC`, `4 = MEM_READ`, `5 = ROM`, `6 = HASH` (leaving `1` unassigned).
   - *Resolution:* Update `bus.rs`, `alu.rs`, `membackend.rs`, and `poseidon.rs` in Task 1.1 to the frozen `{0, 2, 3, 4, 5, 6}` registry.
2. **`C-2` — `ALU` Caller-Ranged Assumption (`Exploit D1`):**
   - *Conflict:* `crates/mori-chip/src/alu.rs:8-9, 111-125` and `openspec/changes/sherlock-vm-consensus-20261107/design.md:96` state that `ALU` assumes inputs `x, y < 2^128` as *"the requester's invariant"* and range-checks only residual `w`. Any unranged witness path into `ALU` wraps modulo `r` and breaks `OP_ADD`/`OP_LT` soundness (`Exploit D1`).
   - *Resolution:* `ALU` (`Section 4.2.A`) unconditionally range-checks `x, y, w < 2^128` inside `ALU`, superseding `design.md:96` and `alu.rs:8-9`.
3. **`C-3` — E8 Baseline Column Counts in `design.md` vs `rfcore.rs` and Lookup-Cache Consensus:**
   - *Conflict:* `openspec/changes/sherlock-vm-consensus-20261107/design.md:125` reports the leaf as *"12 advice, 13 equality and 20 fixed columns, 4 lookups"*, whereas `crates/mori-chip/src/rfcore.rs:833-843` and `research/sherlock-lookup-cache-20261111/CONSENSUS.md:129, 133` report the E8 measured leaf as `15` (or `16`) advice, `14` equality, `9..10` fixed columns, `4` lookups (`823,466` Node rows).
   - *Resolution:* Experiment `P1` logs exact `cs.num_advice_columns()`, `cs.num_fixed_columns()`, `cs.num_selectors()`, and `cs.lookups.len()` from `ConstraintSystem` so `design.md` uses a single ground-truth tuple.
4. **`C-4` — `/1` Enabled Rule Count (`121 + 2` Exact vs `"120 + 2"` Study Shorthand vs `108` C6 Emitter):**
   - *Conflict:* `research/sherlock-lookup-cache-20261111/CONSENSUS.md:208, 250` cites `"120 + 2"` `/1` rules, and `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:363-409` implements `107` rules in `supported_rules()` + `1` (`BBookkeepingTaskRelease`) in `dispatch_supported` = `108`. Direct count of `RuleIdC` (`crates/mori-core/src/sherlock/helper_machine/shared.rs:400-534`, `134` variants) minus `7` `is_disabled` (`owner_runtime.rs:70-80`) minus `4` `is_extension` (`owner_runtime.rs:82-90`) minus `2` `EventKind::Startup` (`owner_runtime.rs:1346`) equals **`121` active `/1` step rules + `2` startup rules = `123`** (`108` + `4` `XAdtHelperBuild*` + `9` `XAdtHelperPlan*` = `121`; `CONSENSUS.md` undercounted `XAdtHelperPlan*` by 1).
   - *Resolution:* `DEC` (`Section 3.2.A`) includes all `121 + 2 = 123` enabled `/1` variants (`<= 2,048` rows handles both effortlessly).
5. **`C-5` — Poseidon Round Count Typo in Lookup-Cache Consensus:**
   - *Conflict:* `research/sherlock-lookup-cache-20261111/CONSENSUS.md:204` refers to `"unconditional 63-round Poseidon2/Poseidon"`, whereas `crates/mori-chip/src/poseidon.rs:5, 57-59` (matching `midnight-circuits-7.3.0` BLS12-381 Poseidon) has **`ROUNDS = 68` (`8` full + `60` partial) and `ROWS_PER_PERM = 70`**.
   - *Resolution:* Use exact `68` rounds / `70` rows per block (`poseidon.rs:57-59`) everywhere in Area 1 row budgets.
6. **`C-6` — Fixed `1..=16` Leaf Steps in `SRC-014` vs Post-M4 `S = 64`:**
   - *Conflict:* `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:18` (`SRC-014`) and `design.md:44` hardcode `"1..=16"` steps per leaf, conflicting with `S = 64` amortization (`CONSENSUS.md:26, 224`).
   - *Resolution:* Keep `s_max = 16` for M4 `SRC-014` compliance and parameterize `SRC-014` to `1..=S` in the new OpenSpec change.
7. **`C-7` — Scratch Lanes `v0..v2` Persistence vs `SRC-023` (`AUDIT.md:67-69`):**
   - *Conflict:* `crates/mori-chip/src/rf.rs:34, 65-67` excludes `v0..v2` from boundary digests (`NDIG`) without forbidding cross-slot scratch reads; `research/sherlock-runlength-consensus-20261111/AUDIT.md:67-69` (Finding 9) proves slot-0-only zeroing breaks `SRC-023` if any rule reads prior-step `v0..v2`.
   - *Resolution:* `CORE` Invariant 4 (`Section 2.3`) enforces strict per-slot write-before-read / zeroing on `v0..v2` with zero cross-slot links.
8. **`C-8` — `verifier_gadget.rs` Path Citation (`AUDIT.md:89`):**
   - *Conflict:* Earlier run-length study drafts cited `crates/mori-recursion/src/verifier_gadget.rs` (non-existent; `AUDIT.md:89` Finding 14).
   - *Resolution:* All citations in this architecture reference `MC:verifier/verifier_gadget.rs:308-341, 604-610` and `MC:verifier/expressions/mod.rs:49`.
9. **`C-9` — Single `CHIP_MEM` vs Split `CHIP_MEM_ALLOC` / `CHIP_MEM_READ` and Open Node Dissents:**
   - *Conflict:* `crates/mori-chip/src/membackend.rs:56-58, 258` uses a single `CHIP_MEM = 2` with `8 * addr + kind`, whereas `CONSENSUS.md:201-202` splits `CHIP_MEM_ALLOC = 3` (append `addr == len_curr`) from `CHIP_MEM_READ = 4` (`addr < len_curr`); separately, `openspec/changes/sherlock-vm-consensus-20261107/dissent.md:82-118` records open tension between 1-leaf `ChipNode` and 2-leaf `RunLift` `R` at `k = 20` (`GP-4`).
   - *Resolution:* `BUS` (`Section 1.2.B`) adopts the safer `3`/`4` split so read steps cannot trigger allocation side-effects (`GP-001`), and `TaggedTableConfig` (`-51,204..-51,662` `R` rows `ESTIMATE`) maximizes `k = 20` headroom for `RunLift` `R` (`P1`).

---

## 9. What Was Not Checked (`ARCH-BRIEF.md:38`)

1. **No Live Compilation, Test Execution, Proving, Quint, or Lean Builds:** Per operational rules (`ARCH-BRIEF.md:42-43`), `cargo check`, `cargo test`, `MockProver`, real Halo2/KZG `ChipNode`/`Decider` keygen, `quint`, and `lake -d clean/fork build` were **not** executed in this drafting session. All row deltas off E8 (`design.md:130-135`, `rfcore.rs:833-843`) are linear-fit calculations marked `ESTIMATE` pending `P1/P3/P4`.
2. **Exact Populated Row Count of `build_sherlock_v1_dec_table()`:** All `134` `RuleIdC` variants and their `121 + 2` `/1` enabled subset (`shared.rs:400-534`, `owner_runtime.rs:70-90, 1346`) were verified by source inspection, but the exact cartesian expansion of boolean guards (`ESTIMATE 1,180 .. 1,850` rows inside the `<= 2,048` ceiling) was not generated by running a Rust table-builder binary.
3. **Exact `VselPacker` Bin-Packing Output on Full 123-Rule Micro-Op Set:** `vsel.rs:90-119` was read and verified (`MAX_DEG = 6`), but the exact packed fixed-column count (`ESTIMATE 4..5` columns) across all `121 + 2` rules awaits Task 1.5 execution.
4. **Area 2 and Area 3 Internal Circuit Implementations:** `crates/mori-recursion/` (`ivc.rs`, `run_decider.rs`), Compact ledger-9 verifier contracts (`sherlock-run.compact`), and sibling drafts in `research/sherlock-final-design-20261111/drafts/` were intentionally not read (`MODULARITY.md:7-10` and strict independence rules).
