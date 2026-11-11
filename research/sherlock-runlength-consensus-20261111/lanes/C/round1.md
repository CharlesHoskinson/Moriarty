# Sherlock Recursion Run Length, Memory Ceiling, and Continuations — Lane C Round 1

**Study:** `sherlock-runlength-consensus-20261111` (`research/sherlock-runlength-consensus-20261111/BRIEF.md:1-79`)  
**Lane:** C — Specification, GP-1..GP-4 Semantics, C6 Outcome-Only Publication & Continuations  
**Repository Snapshot:** `c57351dd4` (`main`, 2026-11-11)

---

## Executive Summary

1. **Q1 (`2^64`, GP-1, and GP-001):** `2^64` is **not** a circuit size, trace allocation, or target computation. Every Sherlock circuit has a small, constant row count (`6,511` rows at `k = 13` for the `S = 16` chip leaf spike, `823,466` rows at `k = 20` for the recursive node, and `929,166` rows at `k = 20` for the run decider; `openspec/changes/sherlock-vm-consensus-20261107/design.md:129-131`). Prover work scales linearly `O(N)` with the actual step count `N` executed (`research/sherlock-runlength-consensus-20261111/BRIEF.md:14`). What `2^64` actually is: the **64-bit unsigned integer register width** of the step counter `h` (`H_BITS = 64` in `crates/mori-recursion/src/ivc.rs:79,359-376`), enforced by `SRC-004` (`openspec/changes/sherlock-vm-consensus-20261107/requirements.md:29`) so field addition `prev_h + steps` modulo the 255-bit BLS12-381 scalar prime `r` cannot wrap around to `0` and forge `is_genesis` mid-run (`clean/fork/Clean/Sherlock/Aggregation.lean:1196-1213`). In a fixed-VK zkVM, **"unbounded run length"** honestly means: **(i)** proving keys, verifying keys, and the on-chain verifier contract contain zero bound on step count, program size, or heap size; **(ii)** architectural counter and Merkle address widths sit above any physically realizable computation (matching the 64-bit step counter); and **(iii)** assurance claims distinguish key universality from finite tested qualification bounds (`SAS-001`, `SAS-012`, `requirements.md:65,76`). Drop-in replacement texts for `general-purpose.md` GP-1, `requirements.md` `GP-001`, and `specs/sherlock-general-purpose/spec.md` `GP-001` are given in §1.2 without narrowing GP-1..GP-4 in any respect.
2. **Q2 (Per-Run Ceiling & Memory Architecture):** Do **not** keep D3's `17`-bit segment index (`< 2^17` leaves, `< 2^20` cells; Option (a)). Because M3 unconditionally advances `len_post = base + 8` on every leaf close—even when `0` cells are appended (`crates/mori-chip/src/m3.rs:22-23,209`)—Option (a) creates a **partition-dependent completeness failure** against `SRC-023` (`requirements.md:48`): a `200,000`-step run succeeds at `S = 16` (`12,500` leaves) and fails to prove at `1` step/leaf (`200,000 > 2^17` leaves), and reaches its ceiling in hours on a cluster (`ESTIMATE`). Instead, adopt **Option (d)**: **widen M3's top-tree segment index to `TOP = 61` (`2^64` cells, `2^61` segments) or `TOP = 64` (`2^67` cells, `2^64` segments, matching `h < 2^64` 1-for-1 even at `1` step/leaf), add a 1-bit zero-append leaf bypass (`act[0] == 0 => len_post = base, post_root = pre_root`), and adopt Carbon's cons-linked slices `(TAG_SLICE, elem_ref, prev_link)`** (`dissent.md:198-202`; `tasks.md:86`). Widening `TOP` in `WalkConfig` (`crates/mori-chip/src/m3.rs:104-144`) adds **zero** advice columns, **zero** fixed columns, **zero** equality columns, and **zero** lookups (`ALU`/`RANGE` already operate on `u128`, `crates/mori-chip/src/alu.rs:1-9`), so the recursive node grows by only **`~4–12` rows** (`823,466 -> ~823,478`, `k = 20` unchanged, `ESTIMATE` from `research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md:27-35`), the decider grows by **`0` rows** (`929,166`, `k = 20` unchanged), and the full leaf stays at **`k = 15..16`** (`ESTIMATE`).
3. **Q3 (Continuations, GP-2, GP-4, and C6):** Distinguish **Layer A (intra-run recursive segment folding `SEG + SEG -> SEG`)** from **Layer B (cross-run continuations across separate runs)**. Once Q2(d) removes D3's `2^17` cap, Layer A **already** proves single runs of any physically realizable length in `O(1)` on-chain verification (linear chain or `GP-4` binary tree, `research/sherlock-gp-architecture-20261110/architecture.md:81-97`) while keeping `pre_d`/`post_d` internal (`design.md:140`). If cross-run continuations (Layer B) are also wanted for pausable multi-session proving or multi-epoch runs, Sherlock's source language cannot implement mid-execution continuations via a wrapper program because `/1` has no stack reification and `SRC-021` (`requirements.md:46`) forces empty frames/env/tasks at `h_pre = 0`. Layer B requires: **(i)** a deterministic logical step/fuel quantum cut (`quantum_steps`, never prover leaf boundaries, preserving `SRC-023` and `SVM-010`); **(ii)** an address-free canonical checkpoint digest `ckpt_d` (`sherlock-ckpt/1`) that replaces every `RefC`, `EnvRootC`, frame, slice, and helper task pointer with recursive `sherlock-value/1`-style digests (`vd`, `ed`, `sd`, `fd`, `hd`) plus `origin_in_d` and monotonic `(epoch, cum_steps)`; **(iii)** lazy `O(1)` in-circuit demand-unfolding at `Resume` so `SRC-021`'s no-scan kill criterion holds even on exponentially shared DAG values; and **(iv)** transitive `CONT-FOLD` cut-elimination inside `GP-2`/`GP-4` tree recursion so intermediate `ckpt_d` values are erased before the decider, publishing only `[prog_d, initial_input_d, final_outcome_d, total_steps]` and one C6 replay key `claim_d` on-chain.
4. **Q4 (Prior Art):** Across 10 production zkVMs (`RISC Zero`, `SP1 v5/v6`, `Jolt`, `Nexus`, `Valida`, `Cairo/Stwo`, `zkWasm`, `Miden`, `OpenVM`, `Ziren`) and 2025–2026 systems (`Twist & Shout`, `Jagged PCS`, `Airbender`, `ZisK`, `Lambda VM`, `Mangrove`, `Boundless`), fixed-VK systems pair `u64` (or unbounded multi-segment) cycle counters with `29–64`-bit address spaces and erase all segment counts and intermediate/final RAM commitments before the L1 SNARK verifier.
5. **Q5 (Practical Run Length & Spec Target):** On this 20-core ARMv9 GB10 host (`120.3 GiB` unified LPDDR5X, `48 GiB` process / `100 GiB` aggregate cgroup caps; `research/sherlock-speed-replan-20261108/prover.md:15`, `requirements.md:41`), measured `k = 20` node folds take `59.6–65.0 s` at `16.6–17.4 GB` RSS (`qa/sherlock/runs.jsonl:617,648-649`). With an `S = 16` chip leaf (`k = 15..16`, `~3–9 s` leaf prove hidden behind the node, `ESTIMATE`), throughput is **`~3.9–4.3 s` per owner step** (`~840–920` steps/hour, `~18–20 GB` aggregate RSS, `ESTIMATE`): **`~1,700` steps in 2 hours**, **`~8,500` steps overnight (10 h)**, and **`~21,000` steps/day** (`ESTIMATE`). Keep `fact(3)` (`293` steps) and `list123` (`303` steps) plus a `>= 512`-step `P3` in the `< 2 h` `G-C` gate (`SRC-011`/`SRC-015`), and add a **`4,096`-step (`256`-leaf at `S = 16`, `~4.5–5.0 h` `ESTIMATE`)** endurance target for `GP-001` alongside a **`>= 64`-step** 3-partition `SRC-023` replay test (task `5.12`).

---

## Q1. GP-1, GP-001, What `2^64` Actually Is, and Honest "Unbounded" Semantics

### 1.1 Direct Answer to the Owner's Questions on `2^64`

The brief records two owner statements (`research/sherlock-runlength-consensus-20261111/BRIEF.md:6-7`):
- *"2^64 is physically impossible"*
- *"I'm not understanding what 2^64 is, is it a limit or a something we calculate? We cannot calculate a 2^64 circuit"*

Both physical observations are completely right: **nobody can build, store, or evaluate a `2^64`-row circuit, nor can any physical machine execute `2^64` VM steps** (at `4.1 s/step` `ESTIMATE`, `2^64` steps would take `~2.40 × 10^12` years `ESTIMATE`; even on a `5 GHz` silicon CPU with zero proving overhead, `2^64` clock cycles take `~117` years `ESTIMATE`).

The confusion comes entirely from how `general-purpose.md:14` (*"The run length is bounded only by the height field (below 2^64)"*) and `requirements.md:56` (*"for every run length below 2^64"*) phrased an **integer counter-width overflow check** as though it were a circuit dimension or workload specification. Separating the four underlying concepts makes the architecture exact:

#### Concept 1: What `2^64` Actually Is — A 64-Bit Counter Overflow Guard (`SRC-004`), NOT a Circuit Size
- **Fixed circuit sizes:** Every circuit in Sherlock has a small, constant power-of-two domain `2^k` fixed once at key generation:
  - Chip leaf (`S = 16` steps/leaf, E8 cost model): **`6,511` rows**, `k = 13` (`2^13 = 8,192` rows; `openspec/changes/sherlock-vm-consensus-20261107/design.md:129`).
  - Recursive IVC node (`SHKIVC06`): **`823,466` rows**, `k = 20` (`2^20 = 1,048,576` rows; `design.md:130`).
  - Run decider (`SHKRUND1`): **`929,166` rows**, `k = 20` (`2^20 = 1,048,576` rows; `design.md:131`).
- **Incremental linear proving (`O(N)` work, `O(1)` circuit size):** When a program runs for `N` steps (say `N = 6` for `entry 7;` or `N = 293` for `fact(3)`, `requirements.md:40`), Sherlock never unrolls `N` steps into one giant circuit. It slices the trace into `ceil(N / S)` fixed-size leaves (`1` leaf for `entry 7;`, `19` leaves at `S = 16` for `fact(3)`), folds them one by one through the fixed-size `k = 20` node, and produces one `6,432`-byte decider proof (`prover.md:10`). Unexecuted steps cost **zero** proving work (`BRIEF.md:14`).
- **Why `2^64` appears in `SRC-004` (`requirements.md:29`) and `ivc.rs`:** Each recursive node carries a step-count register `h` ("height" = cumulative steps proved so far) represented as an element of the BLS12-381 scalar field `Fq` of prime order `r ≈ 2^255` (`crates/mori-chip/src/lib.rs:39`; `clean/fork/Clean/Sherlock/Aggregation.lean:1183-1184`). At each fold, the node computes `h = prev_h + steps` in `Fq` (`crates/mori-recursion/src/ivc.rs:360-376`) and tests whether the segment starts at genesis via `is_genesis = is_zero(prev_h)` (`ivc.rs:312,343`; `design.md:166`).
  - **Without a bit-width range check on `prev_h` and `h`:** A malicious prover could supply `prev_h = r - 1` and `steps = 1`, making `(prev_h + steps) mod r = 0`. That wraps `h` back to `0` mid-chain, fires `is_genesis = 1` on a later node, and discards every prior proof obligation in the accumulator (`Aggregation.lean:1208-1213`, theorem `unchecked_prev_wraps_to_genesis`).
  - **With `SRC-004` (`H_BITS = 64` in `ivc.rs:79,359-376`):** The node enforces `prev_h < 2^64`, `1 <= steps <= S_MAX`, and `h < 2^64` using `assert_lower_than_fixed`. Because `2^64 + 2^64 = 2^65 <= r`, field addition equals exact natural-number addition with zero modular wraparound (`Aggregation.lean:1196-1206`, theorem `src004_height_bridge`).
  - **Circuit cost of this check:** Only **`30–50` rows** in the `k = 20` node (`0.006%` of the node; `research/sherlock-speed-replan-20261108/prover.md:181-186`, `ESTIMATE`).
  - **Exact hardware analogy:** `h < 2^64` is identical to giving a CPU a 64-bit cycle counter or 64-bit program counter (`u64`). It defines the width of one 64-bit counter wire so addition never wraps around; we never "calculate a `2^64` circuit."

#### Concept 2: Per-Run Architectural Bounds (Counter Width vs. Memory Tree Depth)
A single Sherlock run has three architectural widths in the current design documents:
1. **Step counter `h` (`u64`):** `h < 2^64` (`SRC-004`, `ivc.rs:79`). Physically unreachable.
2. **Fuel and scalar arithmetic (`u128`):** `fuel = fuel_lo + 2^64 * fuel_hi < 2^128` (`SRC-021`, `design.md:174`) and `ALU`/`RANGE` check 128-bit unsigned integers (`crates/mori-chip/src/alu.rs:1-9`, `range.rs:1-19`). Physically unreachable.
3. **D3 memory segment index (`17` bits — proposed, not adopted):** In `crates/mori-chip/src/m3.rs:51` (`TOP = 17`), each leaf consumes one aligned `8`-cell segment (`base / 8 < 2^17`, `m3.rs:26-28`), capping a single run at **`< 2^17` (`131,071` usable) leaves and `< 2^20` (`1,048,568` usable) cells** (`research/sherlock-processor-analogy-20261110/experiments-4.md:148-154`; `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:40`). Unlike `2^64`, **`2^17` leaves (`<= 2,097,136` steps at `S = 16`, or `131,071` steps at `1` step/leaf) IS physically reachable** and creates a real per-run ceiling conflict with GP-1 (`dissent.md:228-232`), analyzed in Q2.

#### Concept 3: Intra-Run Recursive Folding vs. Multi-Run Continuations
Sherlock has two distinct ways to scale execution length without changing keys:
- **Intra-run segment recursion (`SEG + SEG -> SEG`):** One logical run of `N` steps is decomposed into `ceil(N / S)` leaves folded in a chain (`ivc.rs`) or binary tree (`GP-4`, `research/sherlock-gp-architecture-20261110/architecture.md:81,97`), sealed once at the end into `[prog_d, input_d, outcome_d, n_steps]`. Once D3's 17-bit segment index is widened in Q2, **every physically possible single run completes via intra-run folding alone**.
- **Multi-run continuations (`GP-2` + `GP-4`):** Splitting a computation across multiple checkpointed runs `k = 0, 1, ..., M-1` where run `k+1` ingests a canonical commitment to run `k`'s suspended VM state, then recursively folding the `M` runs into one on-chain proof (worked out formally in Q3).

#### Concept 4: What "Unbounded" Can Honestly Mean in a Fixed-VK General-Purpose zkVM
In any concrete cryptographic circuit over a finite field, registers and Merkle paths have finite bit-widths, and physical provers have finite time, RAM, and disk. Saying a fixed-VK zkVM supports **"unbounded run length"** (or **"every practical run length with no key-imposed bound"**) honestly means three exact, verifiable engineering guarantees:
1. **Strict Key & Verifier Independence (`O(1)` keys, `O(1)` verifier, `O(N)` prover):** Neither the proving keys, the verifying keys, nor the on-chain contract encode any trace-length constant, program table cap, or heap capacity ladder (`general-purpose.md:9-13`). Proving `N` steps performs `O(N)` work by folding `S`-step chunks of constant circuit size; verification cost and final proof size (`6,432 B`, `prover.md:10`) are independent of `N`.
2. **No Reachable Architectural Ceiling:** Every in-circuit counter and address width (such as the 64-bit step counter `h < 2^64` in `SRC-004` and a matching memory tree depth in Q2) exists solely as a fixed-width integer non-overflow invariant whose threshold (`>= 2^61..2^64`) exceeds centuries of physical computation—and if execution is additionally segmented across multi-run epochs (Q3), even epoch counters compose under the same key set.
3. **Honest Separation Between Key Universality and Tested Assurance Bounds (`SAS-001`, `SAS-012`):** The specification states that *one key set* admits runs of arbitrary finite length without re-keying, while release receipts report the *actual finite step counts qualified on hardware* (`requirements.md:65,76`), never implying that a `2^64`-step trace or `2^64`-row circuit is computed.

---

### 1.2 Exact Drop-In Replacement Text (Preserving All Substance of GP-1..GP-4)

#### A. Replacement for `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md` GP-1

**Current text (`openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:9-16`):**
> Line 9: `One fixed key set proves the execution of every program of the Sherlock language, on every input, for every run length and every outcome. A new program needs no new key, no new contract and no new ceremony.`  
> ...  
> Line 14: `- The run length is bounded only by the height field (below 2^64).`  
> Line 16: `Acceptance: one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from, including one program larger than one leaf's carriers, one run with input, and one run of each outcome kind.`

**Drop-in replacement for `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md:14-16` (keeping lines 9–13 unchanged):**

```markdown
- Run length is not bounded by circuit size or by the key set. A run of N steps is proved incrementally by folding fixed-size S-step leaf proofs into a constant-size final proof (proving cost linear in the steps actually executed; no circuit is sized to the maximum run length). Within a single run, step and memory-address counters use fixed architectural integer widths (step height h < 2^64 in SRC-004, preventing field wraparound modulo r) that sit beyond any physically realizable trace; if a computation is split across checkpointed continuation runs, GP-2 and GP-4 compose those runs into one on-chain proof under the same key set.

Acceptance: one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from, including one program larger than one leaf's carriers, one multi-leaf run at or above the qualified long-run target (4,096 steps across at least 256 leaves at S = 16, with final proof size equal to a single-leaf run), one run with input, and one run of each outcome kind.
```

*(Why this preserves and strengthens GP-1 without narrowing: Line 9's universal quantifier—"every program of the Sherlock language, on every input, for every run length and every outcome" under "One fixed key set"—stays intact word for word. Line 14 now states plainly why no `2^64` circuit exists, what `h < 2^64` actually does, that neither circuit size nor memory segments cap practical runs, and how multi-run continuations compose if used. Line 16 adds an explicit multi-leaf run-length acceptance check at constant proof size.)*

#### B. Replacement for `openspec/changes/sherlock-vm-consensus-20261107/requirements.md` `GP-001` (Strict EARS)

**Current text (`openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56`):**
> `| GP-001 | U | GP-1 universal VM. One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every run length below 2^64 and every outcome (Halted(v) for every value type, as a digest of a canonical value encoding; Rejected(code); Exhausted(kind)); the public statement SHALL carry prog_d and input_d as data, not circuit constants; capacity SHALL NOT depend on the key (state larger than one leaf's carriers runs against committed memory); a new program SHALL need no new key, contract or ceremony. (Added 2026-11-10 by the C6 review, review.md:166. The text above is unchanged.) When a supported run exceeds one leaf's storage or hashing capacity, the prover SHALL continue through authenticated memory and recursive decomposition under the same key set, subject only to the run-length bound of GP-1. | One key set and one deployed verifier contract accept proofs of at least three programs the key set was not built from, including one program larger than one leaf's carriers, one run with input, and one run of each outcome kind (row A33). | general-purpose.md GP-1; SRC-017; C6 review 2026-11-10 |`

**Drop-in replacement row for `openspec/changes/sherlock-vm-consensus-20261107/requirements.md:56`:**

```markdown
| GP-001 | U | GP-1 universal VM. One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every outcome (`Halted(v)` for every value type, as a digest of a canonical value encoding; `Rejected(code)`; `Exhausted(kind)`), and across any finite run length by folding fixed-size `S`-step segment proofs into a constant-size final proof with proving work proportional to the steps executed (`O(1)` circuit and proof size, no trace-length circuit constant). The public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants; capacity and run length SHALL NOT depend on the key (state larger than one leaf's carriers runs against committed memory, and no per-run memory-segment index SHALL cap execution below the 64-bit non-wrap step-counter width `h < 2^64` of SRC-004 unless continued across runs through GP-002 and GP-004 under the same key set); a new program SHALL need no new key, contract or ceremony. When a supported run exceeds one leaf's step, storage or hashing capacity, the prover SHALL continue through authenticated memory and recursive decomposition under the same key set without changing logical execution or resource accounting. | One key set and one deployed verifier contract accept proofs of at least three programs the key set was not built from, including one program larger than one leaf's carriers, one multi-leaf run of at least 4,096 owner steps whose final proof byte length equals that of a single-leaf run, one run with input, and one run of each outcome kind (row A33). | `general-purpose.md` GP-1; SRC-004; SRC-017; C6 review 2026-11-10; runlength consensus 2026-11-11 |
```

#### C. Matching Replacement for `openspec/changes/sherlock-vm-consensus-20261107/specs/sherlock-general-purpose/spec.md:7-15`

```markdown
### Requirement: GP-001 Universal VM
One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every outcome (`Halted(v)` for every value type as a digest of a canonical value encoding, `Rejected(code)`, `Exhausted(kind)`), and across any finite run length by folding fixed-size `S`-step segment proofs into a constant-size final proof with proving work proportional to the steps executed. The public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants. Capacity and run length SHALL NOT depend on the key: a program whose state does not fit one leaf's carriers runs against committed memory, and no per-run memory-segment index SHALL cap execution below the 64-bit non-wrap step-counter width `h < 2^64` of SRC-004 unless continued across runs through GP-002 and GP-004 under the same key set. A new program SHALL need no new key, contract or ceremony.

When a supported run exceeds one leaf's step, storage or hashing capacity, the prover SHALL continue through authenticated memory and recursive decomposition under the same key set without changing logical execution or resource accounting.

#### Scenario: Programs and multi-leaf runs the keys were not built from
- **WHEN** one key set and one deployed verifier contract receive proofs of at least three programs the key set was not built from, including one program larger than one leaf's carriers, one multi-leaf run of at least 4,096 owner steps, one run with input, and one run of each outcome kind.
- **THEN** every proof is accepted with no new key and no new contract, and the final proof byte length is equal across single-leaf and multi-leaf runs (row A33).
```

---

### 1.3 Downstream Impact Audit on Existing EARS Requirements

| Requirement | Location | Impact under Q2(d) (Widened Single-Run Memory) | Additional Impact if Q3 Layer B (Cross-Run Continuations) Is Also Enabled |
|---|---|---|---|
| **`SRC-004`** | `requirements.md:29` | **Unchanged in logic.** Clarify in `design.md` §3.2(1) that `prev_h, steps, h < 2^64` is a 64-bit non-wrap range check on scalar field lanes (`Aggregation.lean:1201`), costing `~30–50` node rows (`ESTIMATE`). | Unchanged per run (`n_steps_k < 2^64`); if `CONT-FOLD` sums steps across runs, carry cumulative steps as `u128` (`(steps_lo, steps_hi)` each `< 2^64`). |
| **`SRC-006`** | `requirements.md:31` | **Unchanged.** Decider still verifies only terminal outcomes (`Halted`, `Rejected`, `Exhausted`), refuses `Running`, and exposes `[prog_d, input_d, outcome_d, n_steps]`. | Keep on-chain decider refusing `Running` and `Suspended`; internal recursion seals a continuation segment to a `CONT` claim (`tag = 3`, `ckpt_d`) that is eliminated by `CONT-FOLD` before the on-chain decider (§3.5). |
| **`SRC-007`** | `requirements.md:32` | **Unchanged** (`claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`; GP-1 contract later moves `prog_d` from sealed constructor state to call data as already noted at `requirements.md:32`). | On-chain contract still sees only the outer terminal `claim_d = Poseidon(SHK_RUN, 1, prog_d, initial_input_d, final_outcome_d)` with zero state digests stored on-chain. |
| **`SRC-021`** | `requirements.md:46` | **Unchanged** (still requires resolving `[OPEN]` lazy input mechanism for `val_d = vd(x)`). | Add a domain-separated `Resume(prog_d, cont_input_d)` branch at `h_pre = 0` when `input_d` opens with `dom("SHK_CONT_IN")`, plus lazy `SEALED_VAL` demand-unfolding (§3.2–3.3). |
| **`SRC-022`** | `requirements.md:47` | **Unchanged** (`outcome_d` and `sherlock-value/1` `vd` stay identical). | Internal continuation claims add `H(O, 1, 3, ckpt_d)` over canonical address-free `sherlock-ckpt/1` (`ed`, `sd`, `fd`, `hd` extending `vd`), never accepted directly by `acceptRun`. |
| **`SRC-023`** | `requirements.md:48` | **Strengthened (completeness gap closed).** Removing D3's `2^17` leaf cap ensures that *all* partitions (`S = 16`, `S = 1`, mixed) are provable—not just equal when both happen to fit `< 2^17` leaves. | Requires continuation cuts to be triggered deterministically by a logical `quantum_steps` parameter in `input_d`, never by prover-chosen leaf boundaries (§3.4). |
| **`SVM-010`** | `requirements.md:18` | **Unchanged and satisfied.** Widening `TOP` + zero-append bypass ensures pure-compute steps never exhaust physical segments. | Reinforces that physical proving limits never emit `Exhausted`; deterministic continuation cuts emit `Suspended(ckpt_d)` only at declared logical `quantum_steps`. |
| **`SAS-001` / `SAS-012`** | `requirements.md:65,76` | **Directly aligned.** Release reports record finite `tested bounds` (e.g. `4,096` steps) separately from proved `Aggregation.lean` height-invariance theorems. | Same. |

---

## Q2. Per-Run Memory & Length Ceiling: Comparison of Designs (a), (b), (c), and (d)

### 2.1 Exact Baseline Cost Equations from `crates/mori-chip` and E2/E7/E8

All row estimates below are anchored to the measured chip-leaf and node equations in the repository:
1. **Poseidon-only HASH chip (`crates/mori-chip/src/poseidon.rs:54-60`):** Width 3, rate 2, 8 full + 60 partial rounds = **`70` leaf rows per permutation**, absorbing 2 field elements per permutation (`ceil(n / 2) * 70` rows for `n` inputs, `poseidon.rs:113-115`).
2. **E8 Spike Leaf (`S = 16`, `C = 8`, `TOP = 17`, `crates/mori-chip/src/m3.rs:50-54`, `rfcore.rs:1204-1228`, `experiments-4.md:57-63,155-159`):**
   - Shape: **`12` advice columns**, **`13` equality columns**, **`20` fixed columns**, **`4` lookups**, max degree `6`, unit rotations `{-1, 0, +1}`.
   - Non-hash rows: `221` (Narrow core `13 * 17`) + `192` (1-limb running-sum 128-bit `RANGE`+`ALU`, `12 * 16`) + `8` (`SegMemConfig`) = **`421` rows** (fixed `RANGE` table = `4,096` rows, `range.rs:34`).
   - M3 hash rows (`PERMS = 87`, `m3.rs:54`): `16` perms (`8` records `H(act,a,b,c)`) + `7` perms (depth-3 segment subtree) + `2 * TOP = 34` perms (old + new depth-17 walks sharing siblings and boolean bits) + `30` perms (`pre_d`, `post_d` over 29 regs + `mem_root`) = `87 * 70` = **`6,090` rows**.
   - Spike total: `421 + 6,090` = **`6,511` rows (`k = 13`)** (`design.md:129`).
   - Full-leaf baseline add-ons (`ESTIMATE`, needed by every option once `PAGE`, `PROG`, and `vd` are wired; `design.md:196-210`, `review.md:126-141`):
     - Let `P_mem = 8` be the per-leaf old-cell `PAGE` read capacity and `P_prog = 8` the depth-12 `PROG` read capacity (census E0 observed `<= 6` distinct control-word `PROG` rows per 16 steps and `0.20–0.22` appends/step, max `3`/step; `design.md:119`).
     - One `PAGE` opening in a cell tree of total depth `D = TOP + 3` hashes the 4-word tagged record (`2` perms) plus `D` Merkle compressions = `(TOP + 5) * 70` rows (`1,540` rows/read at `TOP = 17`, `ESTIMATE`; cf. `design.md:209` `20 * 70 = 1,400` walk-only rows).
     - `P_prog = 8` openings at depth 12: `8 * (12 + 2) * 70 = 7,840` rows (`ESTIMATE`).
     - Worst-case `8` `sherlock-value/1` `vd` hashes (`~3.5` perms avg incl. constructors): `~1,960` rows (`ESTIMATE`; census average `+736` rows, `review.md:139`).
     - Full-leaf baseline at `TOP = 17` (`P_mem = 8`): `6,511 + 8 * 1,540 + 7,840 + 1,960` = **`~28,631` rows (`k = 15`, domain `32,768`)** (`ESTIMATE`; at `P_mem = 4`, **`~22,471` rows, `k = 15`**, `ESTIMATE`).
3. **Node & Decider Row Response (`design.md:130-134`; `experiments/e2-e4.md:27-35`):**
   - `node_rows ≈ 570,957 + 7,298 * adv + 8,189 * eq + 6,004 * fixed + 22,600 * lookup` (`design.md:134`, `ESTIMATE`), measured at **`823,466` rows (`k = 20`, `126,534` headroom below `950,000` gate)** over E8 (`design.md:130`).
   - Raising leaf `k` with **unchanged columns and lookups** adds **only `+4` node rows per `+1` increase in `k_leaf`** (`741,708` at `k = 13` to `741,736` at `k = 20`, measured in E2, `experiments/e2-e4.md:27-34`) and **`0` decider rows** (`929,166` ordinary / `1,032,467` table rows at `k = 20`, invariant across all leaf keys, `experiments/e2-e4.md:28-35`, `review.md:213`).

---

### 2.2 Detailed Comparison of Options (a), (b), (c), and (d)

| Criterion | (a) Keep D3 (`TOP = 17`) + GP-2 Continuations | (b1) Widen D3 to 32-Bit Seg (`TOP = 32`, `2^35` cells) | (b2) Widen D3 to 64-Bit Seg (`TOP = 64`, `2^67` cells) | (c1) Dense Unaligned Batch (`M2`: `len += used`) | (c2/c3/c4) Paged / Growing-Depth / Height-Keyed | **(d) Recommended: Widened M3 (`TOP = 61..64`) + Empty-Leaf Bypass + Linked Slices** |
|---|---|---|---|---|---|---|
| **Max leaves / run** | `131,071` (`2^17 - 1`) | `4.29 × 10^9` (`2^32 - 1`) | `2^64 - 1` (matches `SRC-004`!) | Unbounded by empty leaves; `2^64` cells | `(c2)` `2^48..2^64`; `(c3)` `2^64`; `(c4)` `2^64` steps | **`2^64 - 1` allocating leaves; `∞` empty leaves (`h < 2^64` only bound)** |
| **Max cells / run** | `1,048,568` (`< 2^20`) | `3.43 × 10^10` (`2^35`) | `1.47 × 10^20` (`2^67`, fits `u128` ALU) | `2^64` (`D = 64`) | `2^48..2^67` | **`2^64` (`TOP = 61`) or `2^67` (`TOP = 64`)** |
| **Spike leaf rows (`S=16`)** | `6,511` (`k = 13`) | `8,611` (`k = 14`, `ESTIMATE`) | `13,091` (`k = 14`, `ESTIMATE`) | `~18,000–21,000` (`k = 15`, `ESTIMATE`) | `(c3)` `13,091+`; `(c4b)` `~20,500` (`ESTIMATE`) | **`12,671` (`TOP=61`) / `13,091` (`TOP=64`), `k = 14` (`ESTIMATE`)** |
| **Full leaf rows (`P_mem=8`)** | `~28,631` (`k = 15`, `ESTIMATE`) | `~39,131` (`k = 16`, `ESTIMATE`) | `~61,531` (`k = 16`, `ESTIMATE`) | `~74,000` (`k = 17`, `ESTIMATE`) | `(c2)` `~45k–65k`; `(c4b)` `~69k` (`k=17`, `ESTIMATE`) | **`~58,591` (`TOP=61`) / `~61,531` (`TOP=64`), `k = 16` (`ESTIMATE`)** |
| **Added leaf cols / lookups** | `0` cols, `0` lookups | **`0` cols, `0` lookups** | **`0` cols, `0` lookups** | `+1..3` cols or `+1` lookup (`ESTIMATE`) | `+1..3` cols / muxes (`ESTIMATE`) | **`0` advice, `0` eq, `0` fixed cols (pack code 4), `0` lookups (`ESTIMATE`)** |
| **Node rows (`k = 20`)** | `823,466` (+ extra `CONT` circuit!) | **`~823,470` (`+4` rows, `ESTIMATE`)** | **`~823,470` (`+4` rows, `ESTIMATE`)** | `~838k–865k` (`+15k..42k`, `ESTIMATE`) | `~830k–865k` (`ESTIMATE`) | **`~823,470` (`+4` rows, `k = 20`, `126.5k` headroom, `ESTIMATE`)** |
| **Decider rows (`k = 20`)** | `929,166` | **`929,166` (`+0` rows)** | **`929,166` (`+0` rows)** | `929,166` (`+0` rows) | `929,166` (`+0` rows) | **`929,166` (`+0` rows)** |
| **Cross-run state tax** | **Severe:** `O(|live heap|)` per `2^17` leaves | None for `< 2^32` leaves | **None for `< 2^64` steps** | None | None | **None for `< 2^64` steps** |
| **`SI-12..SI-14` impact** | Breaks `SI-12`; needs linked slices / map | Breaks `SI-12`; needs linked slices / map | Breaks `SI-12`; needs linked slices / map | **Preserves dense `SI-12`** (if per-table or deterministic) | `(c2)` dense possible; `(c4)` breaks `SI-12` | **Solved cleanly via Carbon's cons-linked slices (`+0` map walks)** |
| **`SRC-023` (Partition Indep.)** | **FAILS completeness** (`1` step/leaf vs `S=16`!) | Holds up to `2^32 - 1` steps; fails above | **100% sound & complete for all `h < 2^64`** | **100% sound & complete** (even `mem_root` equal!) | `(c4b)` 100% (`mem_root` equal!); `(c3)` 100% | **100% sound & complete for all `h < 2^64` (`TOP = 64`)** |

#### Analysis of Option (a): Keep D3 (`2^17` Leaves, `2^20` Cells) + GP-2 Continuations — **REJECT**
1. **Violates `SRC-023` partition-independent completeness well below `2^17` steps:** In M3 (`crates/mori-chip/src/m3.rs:22-23,209`), every leaf unconditionally sets `len_post = base + 8` and consumes one top-tree segment slot—even when `used = 0` (`qa/gemini-quint/chipleaf-pairrow-m3/verdict.md:41`). Consider a pure-compute program of `200,000` steps (`0` heap allocations). Proved with full `S = 16` leaves, it uses `12,500 < 2^17` segments and **succeeds**. Proved at `1` step/leaf (explicitly required by `SRC-023`'s three-partition acceptance test, `requirements.md:48`, task `5.12`), it needs `200,000 > 131,071` segments and **fails to prove**.
2. **Partition-dependent continuation cuts violate `SVM-010` and `SRC-023`:** Under `SVM-010` (`requirements.md:18`), running out of physical segment slots cannot produce `Exhausted`, and leaf closes cannot alter logical counters. If Option (a) triggers a continuation run when segments run out, the `S = 16` prover cuts at step `~2,097,136` while the `S = 1` prover cuts at step `131,071`, yielding *different* intermediate continuation claims for the same `(prog_d, input_d)`. Making continuation cuts partition-independent under D3 forces cutting every run at the *worst-case* 1-step/leaf bound (`<= 131,071` steps, or fewer when `PAGE`/`PROG` caps close leaves early!), throwing away `93.75%` of D3's nominal `2M`-step window.
3. **Prohibitive cross-run heap serialization tax:** Under C6 (`SRC-006`, `GP-002`, `requirements.md:31,57`), run `k` cannot pass a raw physical `post_d` or `mem_root` to run `k+1`. Serializing a `100,000`-cell live heap into a canonical digest at the end of run `k` costs `~300,000` Poseidon permutations = `21,000,000` leaf rows (`~360` extra `k = 16` leaves + `~360` extra node folds = **`~6.1` hours** of proving overhead at every continuation boundary, `ESTIMATE`) unless lazy demand-unfolding (§3.3) and incremental digesting are built into every step anyway.
4. **Physically reachable ceiling:** On a 32-worker parallel prover fleet (under GP-4 tree recursion), `131,071` leaves at `S = 16` (`2.09M` steps) would hit D3's hard wall in **`~2.5–3` hours** (`ESTIMATE`).

#### Analysis of Option (b): Widen D3's Segment Index (`32` or `64` Bits)
Inspecting `crates/mori-chip/src/m3.rs:104-144` and `crates/mori-chip/src/alu.rs:1-9` reveals a crucial structural fact: **widening `TOP` requires zero new advice columns, zero new fixed columns, zero new equality columns, and zero new range-check lookups**:
- **Range checks are already free:** `WalkConfig` (`m3.rs:113-139`) decomposes `idx = base / 8` across the `TOP` absorb rows of the old walk via boolean bit constraints `b * (1 - b) = 0`, `acc_in = 2 * acc_out + b`, `acc_out[TOP - 1] = 0`, and `8 * acc_in[0] = base`. Because the BLS12-381 scalar field has `r > 2^254`, `8 * sum_{l=0}^{TOP-1} b_l 2^l < 2^(TOP+3) << r` never wraps modulo `r` for any `TOP <= 125`, and `ALU`/`RANGE` (`alu.rs:1-9`, `range.rs:1-19`) **already decompose 128-bit unsigned integers** (`u128`) for `addr < len` and `len + 1`!
- **Selector columns are unchanged:** `WalkConfig` uses `code_column(meta, 3)` (`codes 1, 2, 3` on one fixed column, `m3.rs:98,105`), which works identically whether `TOP` is `17`, `32`, `61`, or `64`.
- **Exact row scaling (`+1` bit of `TOP`):** Adds `+2` Poseidon permutations (`+140` rows) to M3's old+new segment walks plus `+1` permutation (`+70` rows) per `PAGE` old-cell read slot:
  - **`TOP = 29` (32-bit cell address `base < 2^32`, `2^29 = 536.8M` leaves):** `+12` levels -> `+1,680` walk rows + `8 * 840 = +6,720` `PAGE` rows (`P_mem = 8`) = `+8,400` leaf rows (`~37,031` total, **`k = 16`**, `ESTIMATE`). Node: **`+4` rows** (`~823,470`, `k = 20`); Decider: **`+0` rows** (`929,166`, `k = 20`) (`ESTIMATE`).
  - **`TOP = 32` (32-bit segment index `seg < 2^32`, `2^35` cells):** `+15` levels -> `+2,100` walk rows + `+8,400` `PAGE` rows = `+10,500` leaf rows (`~39,131` total, **`k = 16`**, `ESTIMATE`). Node: **`+4` rows** (`k = 20`); Decider: **`+0` rows** (`ESTIMATE`).
  - **`TOP = 61` (64-bit cell address `base < 2^64`, `2^61` leaves):** `+44` levels -> `+6,160` walk rows + `+24,640` `PAGE` rows = `+30,800` leaf rows (`~59,431` total, **`k = 16`**, `ESTIMATE`). Node: **`+4` rows** (`k = 20`); Decider: **`+0` rows** (`ESTIMATE`).
  - **`TOP = 64` (64-bit segment index `seg < 2^64`, `base < 2^67 << 2^128` cells):** `+47` levels -> `+6,580` walk rows + `+26,320` `PAGE` rows = `+32,900` leaf rows (`~61,531` total, **`k = 16`**, fitting inside `2^16 = 65,536` and meeting the `design.md:129` `k_leaf <= 16` gate! `ESTIMATE`). And because `SRC-004` requires `steps >= 1` per leaf and `h < 2^64`, a run has **at most `2^64 - 1` leaves**—matching `2^64 - 1` usable segments (`1 .. 2^64 - 1`, with segment `0` reserved) **1-for-1 under every possible leaf partition!**

#### Analysis of Option (c): Non-Per-Leaf Segment Designs
1. **`(c1)` Allocate only on append (`M1` per-append vs. `M2` dense unaligned frontier vs. Conditional `M3`):**
   - **M1 (per-append Merkle walk, `experiments-2.md:260-269`):** `8` appends at `D = 64` require `16 * 64 = 1,024` walk perms = `71,680` walk rows alone (`k = 17..18`, `ESTIMATE`), strictly worse than widened M3 (`128` walk perms = `8,960` rows).
   - **M2 (dense unaligned append log, `len_post = len_pre + used`, `experiments-2.md:270-273,291-293`):** Because `len` advances *only* by actual `used in 0..8` appends, there are **zero padding addresses**—making `len`, `fp`, `tp`, `RefC::Data(addr)`, and even `mem_root` **100% partition-independent** and preserving `SI-12` contiguous slices (`start + k`)! However, because `len_pre .. len_pre + used` (`used <= 8`) is unaligned, an append batch can straddle two adjacent aligned 8-cell subtrees (`seg` and `seg + 1`), requiring **two** old-to-new Merkle walks (`4 * TOP` perms = `17,920` walk rows at `TOP = 64`, `ESTIMATE`), pre-image witness records for the partially filled prefix of `seg`, and delicate cross-subtree carry/routing gates (+1–2 columns -> `+14k..25k` node rows, `ESTIMATE`) with a substantially larger soundness surface (`experiments-2.md:291-293`).
   - **Conditional M3 (empty-leaf bypass on `act[0]`):** In `SegMemConfig` (`crates/mori-chip/src/mem.rs:148-184`), `act[0] in {0, 1}` is *already* constrained to `1` iff `used >= 1` (`act` is a boolean prefix). Gating the M3 root update on `act[0]` (`act[0] == 0 => post_root = pre_root` and `len_post = base`; `act[0] == 1 => old_walk` starts at `0`, `post_root = new_walk`, `len_post = base + 8`) takes **3 degree-<=4 polynomial equations** on existing columns, prevents pure-compute leaves from consuming segments, and keeps M3's single aligned walk!
2. **`(c2)` Paged / Hierarchical Sparse Merkle Memory (RISC Zero / OpenVM style):**
   - E.g. a depth-40 directory of `1,024`-cell pages (`2^50` cells), caching `active_page_root` in core registers across leaves until the page fills. While this amortizes top-tree walks across leaves, every `PAGE` read of an older cell still traverses all `50` levels (`3,500` rows/read, `ESTIMATE`), and page-boundary rollover requires conditional directory-flush constraints in every leaf anyway (since PLONK circuits are worst-case sized per leaf). Thus worst-case leaf rows are **larger** than single-walk M3 while adding flush-state soundness obligations.
3. **`(c3)` Growing-Depth Merkle Tree:**
   - Tracks `tree_depth` in a register and increments `tree_depth` (`root' = H(root, empty[d])`) when `seg == 2^tree_depth`. In PLONK/Halo2, circuit domain `2^k` is fixed at keygen, so the leaf still pays `MAX_DEPTH` rows (`64` levels), while dynamic stop-level selectors on both `WalkConfig` and `PAGE` add extra fixed/advice columns (`+6k..15k` node rows, `ESTIMATE`) and create root-confusion attack surfaces if `PAGE` reads are not clamped to the exact current `tree_depth`. Strictly dominated by fixed `TOP = 64`.
4. **`(c4)` Height-Keyed Address Space (`addr = (h, sub_op)` or `(h >> 4, step_slot, sub_op)`):**
   - Keying memory cells by logical step height `h < 2^64` (`addr = 8 * h + j`, `j < 8`) makes every cell's address **100% independent of leaf partitioning** (`S = 1` and `S = 16` assign the exact same address `8 * h + j` to step `h`'s `j`-th append, preserving `SI-13`/`SI-14` `child < owner`!). However, in an `S = 16`-step leaf, step heights `h_pre .. h_pre + steps - 1` span up to **two** aligned `16`-step blocks (`h_pre >> 4` and `(h_pre >> 4) + 1` when `h_pre` is not `0 mod 16` under mixed partitions!), requiring `2` block subtrees of `16 * 4 = 64` cells (`128` record hashes + `126` subtree hashes + `4 * 60` top walks = `622` perms = `43,540` hash rows before `PAGE`, pushing the leaf to `k = 17`, `ESTIMATE`), and still breaks `SI-12` `start + k` across steps that allocate `0` fields. Because C6 (`SRC-006`, `SRC-022`) already erases physical addresses from `outcome_d` and `claim_d`, paying `43,540` rows to make internal `mem_root` partition-invariant is unnecessary.

#### Analysis of Option (d) — **RECOMMENDED DESIGN**: Widened M3 (`TOP = 61` or `64`) + Empty-Leaf Bypass (`act[0]`) + Carbon's Cons-Linked Slices
Combine three orthogonal, minimal-surface components:
1. **Set `TOP = 64` (or `TOP = 61` paired with the empty-leaf bypass):**
   - At `TOP = 64` (`SEG = 8`), `base = 8 * seg < 2^67 << 2^128` fits directly in `crates/mori-chip/src/alu.rs` (`u128`), `WalkConfig` (`m3.rs:104-144`) adds **zero** columns and **zero** lookups, Node stays at **`~823,470` rows (`k = 20`, `+4` rows `ESTIMATE`)**, Decider stays at **`929,166` rows (`k = 20`, `+0` rows)**, and full leaf rows (`~61,531` at `P_mem = 8`, `~42,171` at `P_mem = 4`, `ESTIMATE`) fit **`k = 16`** (`<= 16` gate of `design.md:129`; `4x–8x` smaller than `K_LEAF_MAX = 20` of `SRC-014`). *(If `k = 15` is desired with `P_mem = 8`, `TOP = 45` gives `2^48` cells = `281` trillion cells—matching SP1 v6 Hypercube's 48-bit address space—at `~48,231` rows, or `k = 15` at `P_mem = 4` with `~33k` trimmed to `< 32,768`, `ESTIMATE`; paired with the empty-leaf bypass below, only allocating leaves consume segments, so `2^45` allocating leaves >= `3.5 × 10^13` steps.)* Between `TOP = 61` (with empty-leaf bypass) and `TOP = 64` (with empty-leaf bypass), **`TOP = 64` + empty-leaf bypass** is cleanest because even `2^64 - 1` single-append leaves cannot exhaust segments before `SRC-004`'s `h < 2^64` fires.
2. **Gate M3 segment consumption on `act[0]` (1-bit empty-leaf bypass):**
   - In `m3.rs:125-134`, `act[0]` from `SegCells` (`mem.rs:234`) is already boolean and equals `1` iff at least one cell was appended in the leaf (`mem first` gate at `mem.rs:175-179` enforces `(1 - act[0]) * (addr0 - len_end) = 0`).
   - Replace `s * x = 0` and `s * (lp - base - 8) = 0` in `m3 walk first` (`m3.rs:129-133`) with:
     - `s * act0 * x = 0` (when `act0 = 1`, slot `base / 8` must be empty `0` in `pre_root`),
     - `s * (lp - base - 8 * act0) = 0` (`len_post = base + 8` when `act0 = 1`, `len_post = base` when `act0 = 0`),
     - On `post_d`'s root input: select `act0 ? new_root : pre_root` (one degree-2 constraint `root_post - pre_root - act0 * (new_root - pre_root) = 0`).
   - Result: Any leaf with `0` memory appends leaves `len` and `mem_root` **bit-for-bit unchanged** (`SVM-011`), burning **zero** segments regardless of leaf partitioning!
3. **Resolve `SI-12..SI-14` (`dissent.md` D-1) via Carbon's Cons-Linked Slices (Task `4.18`):**
   - Replace contiguous `SliceC { start, len }` (`addr + k` indexing in `owner_runtime.rs:2990,4292`, `owner_adt_runtime.rs:1585,1601`, `families/structural.rs:637`) with immutable cons-linked cells `(TAG_SLICE, elem_ref, prev_link)` (`prev_link < cell_addr`), and inline child references directly into the 3-word `MEM` cell for `<= 2`-arity forms (`Leaf`, `Cons`, `Branch`, `UInt`, `Bool`, `Nil`).
   - **Why this beats Astra's `logical_id -> m3_addr` map:** Astra's translation table costs an extra depth-`TOP` old+new walk (`2 * 64 * 70 = +8,960` rows per write, `+4,480` rows per read at `TOP = 64`, `ESTIMATE`; `+2,380` rows even at `TOP = 17`, `review.md:54`). Linked slices cost **zero** extra Merkle maps, keep `SI-13` (`child < owner`) and `SI-14` (`previous < cell_addr`) strictly satisfied as `<` inequalities on monotone addresses, replace `fresh_id == active.*` (`owner_runtime.rs:3021-3028`) with `fresh_addr == running_len` inside the core step, and keep logical allocation counters (`alloc.data_nodes`, `alloc.data_fields`) strictly partition-independent for `P6` exhaustion (`SVM-010`).
4. **Effect on `SRC-023` and `GP-2..GP-4`:**
   - Every valid run `h < 2^64` is provable under **every** leaf partition (`S = 16`, `S = 1`, mixed), producing identical `[prog_d, input_d, outcome_d, n_steps]` and `claim_d` (`SRC-023`).
   - `GP-2` (`(prog_d', input_d', outcome_d')` consumption), `GP-3` (foreign proofs), and `GP-4` (binary/DAG PCD over `SEG` and `SET` statements, `architecture.md:81-97`) compose with **zero** cross-run heap serialization overhead.


---

## Q3. Continuations, GP-2, GP-4, and C6 Outcome-Only Publication

### 3.1 Two Continuation Layers — Intra-Run Segment Folding vs. Cross-Run Continuations

To reason cleanly about continuations in Sherlock, we must separate **two architectural layers** that are often conflated under the word "continuation" in zkVM literature:

1. **Layer A — Intra-Run Segment Recursion (`SEG + SEG -> SEG`, Already in Sherlock's Architecture):**
   - A single logical run `(prog_d, input_d)` of `N` steps is split across `L = ceil(N / S)` chip leaves.
   - Each leaf proves `[prog_d, input_d, pre_d, post_d, h_pre, steps, status, out_d]` internally (`openspec/changes/sherlock-vm-consensus-20261107/design.md:140`).
   - Either a linear IVC chain (`crates/mori-recursion/src/ivc.rs:360-376`) or a `GP-4` binary tree (`research/sherlock-gp-architecture-20261110/architecture.md:81,97`; `clean/fork/Clean/Sherlock/Aggregation.lean:50`) merges adjacent segments by enforcing `left.post_d == right.pre_d` and `left.h_lo + left.steps == right.h_lo`, then **erases** the intermediate `post_d` (`design.md:152`).
   - Once the merged span covers `[0 .. N)` and ends in a terminal status (`Halted`, `Rejected`, `Exhausted`), the Run Decider (`SRC-006`, `requirements.md:31`) seals the run to `[prog_d, input_d, outcome_d, n_steps = N]`, hiding `pre_d` and `post_d`.
   - **Key consequence of Q2(d):** With `TOP = 64` + the zero-append leaf bypass, **Layer A alone proves any physically realizable single run (`N < 2^64`) in `O(1)` on-chain verification with zero cross-run heap serialization**.

2. **Layer B — Cross-Run Continuations (`RUN_0 ; RUN_1 ; ... ; RUN_{M-1}` Composed via `GP-2` + `GP-4`):**
   - Needed **only** if a user or proving market wants to pause an in-progress run across separate proving sessions/epochs (e.g., checkpointing every `10,000` steps across days or untrusted workers without keeping raw `sherlock-gp-bundle/1` transcripts, or chaining multi-epoch stateful computations) *or* if a small per-run memory cap were retained.

#### Why Current Sherlock Specs and User-Space Wrapper Programs Cannot Do Mid-Execution Layer B Continuations
Checking the exact `/1` VM, recursion, and ISA specifications shows **three hard barriers** against naive or source-level continuations:
1. **`SVM-009` (`requirements.md:17`) and `owner_runtime.rs` forbid `/1` suspension:** In `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:4514-4523`, `/1` executes only `Halted`, `Rejected`, and `Exhausted`, explicitly rejecting `ControlC::Await`, `OutcomeC::Waiting`, and `ExhaustReason::LogicalCap` (`code 7`). Meanwhile, `/2`'s host checkpoint (`crates/mori-core/src/sherlock/compile_artifact/suspension2.rs:1-20,109-119`, magic `SHK2CKP1`) is an unproved host-side SHA-256 byte dump, not an in-circuit canonical commitment.
2. **Source-language wrapper programs cannot capture mid-execution stack/helper state:** Sherlock's expression ISA (`crates/mori-core/src/sherlock/helper_machine/shared.rs:1155-1208`) has no `call/cc`, no exception handler over fuel exhaustion, and no primitive to reify `frames: Vec<FrameRowC>`, `env_root: EnvRootC`, or active structural helper tasks (`tasks: Vec<HelperTaskC>`, e.g., mid-way through `BHelperEqual` or `Fold`). And `SRC-021` (`requirements.md:46`, `design.md:172-218`) forces `h_pre = 0` to start at `ctl = Eval(ENTRY)` with `fp = NIL`, `frame_count = 0`, `env = None`, `helper = None`, and `tasks = []`. Therefore, a user-space wrapper `step_state(s) -> Either(NextState, FinalVal)` works **only** when the guest program itself is hand-rewritten into an explicit top-level state machine that returns `Halted(s_{k+1})` before fuel runs out—**never** for arbitrary general-purpose recursive programs (`GP-1`).
3. **`SRC-006` (`requirements.md:31`) and `GP-002` (`requirements.md:57`) forbid passing physical state digests across runs:** Under C6, `outcome_d` cannot expose `post_d`, `fin_d`, `mem_root`, or `len`, and `GP-002` restricts cross-run proof composition strictly to settled `(prog_d', input_d', outcome_d')` claims.

Therefore, supporting true general-purpose Layer B continuations requires a **VM Continuation Mode** (`Suspended(ckpt_d)` + `Resume(prog_d, cont_input_d)`) paired with **`CONT-FOLD` cut elimination** inside `GP-2`/`GP-4` recursion. Below is its complete formal design.

---

### 3.2 Complete VM State Inventory & Canonical Partition-Independent Checkpoint Digest (`sherlock-ckpt/1`)

#### Exact Sherlock `/1` Dynamic Machine State
From `MachineImage` (`crates/mori-core/src/sherlock/helper_machine/shared.rs:1500-1526`), `sherlock-c6-phys/1` (`c6_phys.rs:48-74`, `c6_phys_data.rs:38-44`, `c6_phys_helper.rs:63-70`), `Regs32` (`crates/mori-chip/src/rf.rs:32-67`), and D7 (`design.md:100`), a live `/1` machine state `S` at an owner or helper step consists of six components:

| # | State Component | Reference Rust Carrier (`mori-core`) | Chip Leaf Carrier (`mori-chip` + D7) | Contains Physical `MEM` Addresses? |
|---|---|---|---|---|
| 1 | **Active Control** | `control: ControlC`, `control_origin: usize` (`shared.rs:944-998,1503-1504`) | `ctl_tag`, `ctl_a`, `ctl_b`, `ctl_c` (`rf.rs:34-37`) | **Yes** (`RefC::Data(addr)` in `Value(r)`, `ApplyValue`, slices, etc.) |
| 2 | **Lexical Environment** | `env_root: EnvRootC` -> immutable chain `EnvCellC { binder, value: RefC, previous }` (`shared.rs:1093-1098,1505,1522`) | `env` register (`rf.rs:38`) + `MEM` cells `(TAG_ENV, binder_val, prev_env)` | **Yes** (`EnvRootC(id)` and `value: RefC`) |
| 3 | **Continuation Frames** | `frames: Vec<FrameRowC>` (`<= stack_limit`, `shared.rs:883-942,1506,1523`) | `top` slots (`rf.rs:39-42`) + `fp` linked list in `MEM` + `frame_count` (`design.md:100`) | **Yes** (`fp` link, saved `EnvRootC`, saved `RefC`s & slices) |
| 4 | **Helper Sub-Machine** | `helper: Option<HelperControlC>`, `result: Option<HelperResultC>`, `tasks: Vec<HelperTaskC>` (`shared.rs:758-881,1019-1039,1517-1524`) | `helper`/`task` slots (`rf.rs:43-50`) + `tp` linked list in `MEM` + `task_count` (`design.md:100`) | **Yes** (`tp` link, `RefC`s & slices in `HelperControlC` / `HelperTaskC`) |
| 5 | **Value Heap** | `data: Vec<RawDataRow>`, `data_fields: Vec<RefC>`, `value_slots: Vec<RefC>` (`shared.rs:1070-1091,1520-1525`) | Append-only `MEM` Merkle root `mem_root` + frontier `len` (`m3.rs:47`) | **Yes** (`len` and `mem_root` include D3 leaf-close padding!) |
| 6 | **Resource Counters** | `initial_fuel`, `remaining_work`, `consumed_work`, `helper_work`, `helper_visits`, `reference_cursor`, `alloc: AllocCounters` (`shared.rs:1059-1068,1508-1514`) | `fuel_lo/hi`, `work_lo/hi`, `alloc_*` (`rf.rs:51-62`) | **No** (`SVM-010` mandates logical counters are strictly partition-independent!) |

#### Canonical Partition-Independent Digest `ckpt_d` (`sherlock-ckpt/1`)
In `openspec/changes/sherlock-universal-interpreter-design/interfaces.md:179,240`, Sherlock's semantic equivalence at an A4 technical cut is defined via `project(S)`, which erases unreachable garbage and concrete arena indices (`RefC::Data`, `EnvRootC`, `fp`, `tp`). We instantiate `project(S)` in-circuit using `midnight-circuits 7.3.0`'s width-3, rate-2 Poseidon sponge (`crates/mori-chip/src/poseidon.rs:96-110`), extending `sherlock-value/1` (`vd`, `design.md:175-188`, `SVM-012` at `requirements.md:20`) with four recursive structural digests:

1. **Value Digest `vd(r)` (`dom("SHK_VALUE_V1")`, already in `design.md:175-188`):**
   - `Unit -> H(V, 1, 0)`; `Bool(b) -> H(V, 1, 1, b)`; `Int(z) -> H(V, 1, 2, sign, mag_lo, mag_hi)`; `Bytes/String -> H(V, 1, 3|4, byte_len, packed_words...)`; `Tuple/List/Ctor/Record/Closure`: recursively hashes child `vd(f_i)` (and closure `code_pc` + `ed(closure_env)`).
   - **Soundness check on DAG sharing erasure:** Does replacing `RefC::Data(addr)` with tree-extensional `vd(r)` alter future execution if two variables originally aliased the same `Data` cell? Verified in `crates/mori-core/src/sherlock/helper_machine/families/structural.rs:153-209`: `/1`'s structural equality (`BHelperEqual`) and comparison (`BHelperCompare`) **never short-circuit on pointer equality `lhs == rhs`**; they unfold every `RefC::Data` node field-by-field regardless of aliasing. Because all `/1` values are immutable and observationally pure, erasing `Data(addr)` into `vd(r)` preserves **100% identical control paths, `consumed_work`, and `helper_visits`**.

2. **Environment Chain Digest `ed` (`dom("SHK_ENV_V1")`):**
   - `ed(None) = H(E, 1, 0)`
   - `ed(Some(cell)) = H(E, 1, 1, cell.binder, vd(cell.value), ed(cell.previous))`
   - **Incremental `O(1)` maintenance:** Because `EnvCellC` is immutable and append-only (`SI-14`), every `EnvPut` computes `ed_new` in **`3` Poseidon permutations (`210` rows `ESTIMATE`)** from `(binder, vd_val, ed_prev)` stored alongside the cell!

3. **Cons-Linked Slice Digest `sd` (`dom("SHK_SLICE_V1")`):**
   - `sd(nil) = H(S, 1, 0)`
   - `sd(cons(elem, prev)) = H(S, 1, 1, vd(elem), sd(prev))` (`3` permutations = `210` rows `ESTIMATE` per slice cons).

4. **Frame Stack Digest `fd` (`dom("SHK_FRAME_V1")`) & Helper/Task Stack Digest `hd` (`dom("SHK_TASK_V1")`):**
   - `fd(empty) = H(F, 1, 0)`
   - `fd(push(frame, rest)) = H(F, 1, 1, frame_tag, prog_pc, arity_idx, ed(saved_env), vd_or_sd_fields..., fd(rest))` — maintained in **`O(1)` (`3–4` perms = `210–280` rows `ESTIMATE`)** on every `PUSH` (and restored from the popped `MEM` frame record on `POP`).
   - `hd` folds `tasks` identically (`H(T, 1, 1, task_tag, vd_lhs, vd_rhs, sd_slice, hd_rest)`) and wraps the active `helper: Option<HelperControlC>` + `result: Option<HelperResultC>` at the top (`H(H, 1, helper_canon, result_canon, task_stack_d)`).

5. **Full Checkpoint Digest `ckpt_d` (`dom("SHK_CKPT_V1")`):**
   ```text
   counters_d  = H(dom("SHK_CNT_V1"), 1,
                   initial_fuel_lo, initial_fuel_hi,
                   remaining_work_lo, remaining_work_hi,
                   consumed_work_lo, consumed_work_hi,
                   helper_work, helper_visits,
                   stack_limit, frame_count, task_count,
                   alloc.data_nodes, alloc.data_fields, alloc.value_slots,
                   alloc.env_cells, alloc.max_frames, alloc.max_tasks)

   ctl_canon_d = H(dom("SHK_CTL_V1"), 1,
                   ctl_tag, control_origin, ctl_canon_a, ctl_canon_b, ctl_canon_c)
                 // where any RefC / EnvRootC / SliceC in ctl is replaced by its vd / ed / sd

   ckpt_d      = H(dom("SHK_CKPT_V1"), 1,
                   origin_in_d,          // genesis input_d[0] of the chain
                   epoch,                // 0-indexed continuation segment counter + 1
                   cum_steps_lo, cum_steps_hi, // cumulative steps < 2^128 across segments
                   ctl_canon_d,
                   counters_d,
                   ed,                   // canonical lexical environment digest
                   fd,                   // canonical continuation frame stack digest
                   hd)                   // canonical helper & task stack digest
   ```

**Proof of `SRC-023` Partition Independence of `ckpt_d`:** Every input to `ckpt_d` is a logical counter (`SVM-010`), a `PROG` index (`control_origin`, `prog_pc`), or a recursive algebraic hash (`vd`, `ed`, `sd`, `fd`, `hd`) over logical tags and scalars. Zero `MEM` addresses (`RefC::Data`, `EnvRootC`, `fp`, `tp`), zero `len` frontiers, and zero `mem_root` digests appear in `ckpt_d`. Therefore, whether run `k` was proved at `S = 16`, `S = 1`, or mixed leaf sizes, **stopping at the same logical step produces the bit-for-bit identical `ckpt_d`**.

---

### 3.3 `O(1)` Lazy Demand-Unfolding at `Resume` (Simultaneously Solving `SRC-021`'s `[OPEN]` Input Mechanism)

How does run `k + 1` ingest `ckpt_d` at `h_pre = 0` without scanning or hashing the entire live heap?
- **Why eager preloading fails:** A live checkpoint may reference `100,000` heap cells—or a 31-node shared binary DAG `t30` (`t_{i+1} = Branch(t_i, t_i)`) whose tree-unfolded size is `2^31 - 1` nodes! Eagerly populating `MEM` at `h_pre = 0` would either violate `SRC-021`'s kill criterion (*"No full-table or full-memory scan at startup or shutdown"*, `design.md:172,216`) or blow up exponentially on shared DAGs.
- **Solution — Sealed Digest Handles + On-Demand `UNFOLD` Provider:**
  1. **`O(1)` `Resume(prog_d, cont_input_d)` at `h_pre = 0` (`~20` Poseidon perms = `1,400` rows `ESTIMATE`):**
     - Verifies `cont_input_d == H(dom("SHK_CONT_IN"), 1, ckpt_d, quantum_steps)` and opens `ckpt_d` into `(origin_in_d, epoch, cum_steps, ctl_canon_d, counters_d, ed, fd, hd)`.
     - Starts with an **empty fresh segment memory** (`mem_root = EMPTY_MEM_ROOT`, `len = 8`), restores `counters_d`, `ed`, `fd`, `hd`, and `ctl_canon_d` directly into core registers, and represents any unmaterialized value/env/frame/task reference as a **sealed handle** `(TAG_SEALED_VAL, vd_x, 0)` (and `TAG_SEALED_ENV(ed)`, `TAG_SEALED_SLICE(sd)`, `TAG_SEALED_FRAME(fd)`, `TAG_SEALED_TASK(hd)`).
  2. **Lazy 1-Level `UNFOLD` on Dereference (`~3–4` perms = `210–280` rows per touched node `ESTIMATE`):**
     - Whenever an owner or helper step inspects a sealed handle `(TAG_SEALED_VAL, vd_x, 0)` (or pops `TAG_SEALED_FRAME(fd)` when `top` is empty), the witness supplies the immediate 1-level preimage `(val_tag, arity, child_vd_0, child_vd_1, ...)`, hashes it via `HASH` to assert `== vd_x`, and appends a fresh `MEM` cell holding `(val_tag, SEALED(child_vd_0), SEALED(child_vd_1))`!
     - Untouched heap subtrees and buried stack frames cost **zero** rows in run `k + 1`!
     - **DAG sharing preserved:** The prover maintains a segment-local memo map `vd_x -> local_addr`; if run `k + 1` touches the same `vd_x` twice (or constructs a node with `vd_x`), `PAGE` reads the already-materialized `local_addr` (verified to carry `vd_x`), so `t30` unfolds at most **`31` unique `vd` digests**, never `2^31`!
  3. **Bonus — Closes `SRC-021` (`design.md:190-191`) `[OPEN]` Input Mechanism:** At genesis `Init(prog_d, input_d)` for non-Unit program input `val_d = vd(x)`, binding initial argument register `arg0 = (TAG_SEALED_VAL, val_d, 0)` uses the **exact same `UNFOLD` gadget** in `O(1)` startup rows!

---

### 3.4 Exact Public Statements (`[prog_d, input_d, outcome_d, n_steps]`) & the Deterministic Cut Rule

#### Mandatory Deterministic Cut Rule (`quantum_steps`)
> **Critical Soundness & `SRC-023` Invariant:** Cross-run suspension MUST NOT be triggered at arbitrary prover-chosen steps or when physical leaf slots fill up. If `(prog_d, input_d)` allowed suspending at *any* step `1 <= t < N`, a single deterministic program+input would have `N - 1` distinct valid `outcome_d` digests (`Suspended(ckpt_1)`, ..., `Suspended(ckpt_{N-1})`), breaking outcome determinism (`SVM-002`) and `SRC-023`!

Instead, `input_d` commits a logical step quantum `quantum_steps: u64`:
- `quantum_steps == 0`: standard single-run mode (never suspends; runs to terminal `Halted`/`Rejected`/`Exhausted`, proved via Layer A).
- `quantum_steps == Q > 0`: run executes up to `Q` logical steps (`h_local` from `0` to `Q`).
  - If a terminal state (`Halted`, `Rejected`, `Exhausted`) is reached at `n_steps <= Q`, the run emits that terminal `outcome_d` (`tag in {0, 1, 2}`).
  - If and only if `h_local == Q` and the machine is still `Running`, a deterministic `P0` quantum-cut fires and emits internal suspension outcome `tag = 3` (`Suspended(ckpt_d)`), with `n_steps = Q`.

#### Exact Field Equations Across an `M`-Segment Continuation (`k = 0 .. M - 1`)

| Segment Role | `prog_d` | `input_d[k]` | `outcome_d[k]` | `n_steps[k]` | Internal `GP-2` Dependency (`deps_d`) |
|---|---|---|---|---|---|
| **Single Run (`M = 1`, `Q = 0`)** | `prog_d` | `H(SHK_IN, 1, val_d, fuel_lo, fuel_hi, stack_limit, 0)` | `H(O, 1, 0\|1\|2, payload)` | `N` (`1 <= N < 2^64`) | `EMPTY` |
| **Initial Segment (`k = 0`, `M > 1`)** | `prog_d` | `in_0 = H(SHK_IN, 1, val_d, fuel_lo, fuel_hi, stack_limit, Q_0)` | `H(O, 1, 3, ckpt_d[1])` | `Q_0` | `EMPTY` |
| **Middle Segment (`1 <= k < M - 1`)** | `prog_d` | `in_k = H(SHK_CONT_IN, 1, ckpt_d[k], Q_k)` | `H(O, 1, 3, ckpt_d[k+1])` | `Q_k` | `{(prog_d, *, H(O, 1, 3, ckpt_d[k]))}` |
| **Final Segment (`k = M - 1`, `M > 1`)** | `prog_d` | `in_{M-1} = H(SHK_CONT_IN, 1, ckpt_d[M-1], Q_{M-1})` | `out_final = H(O, 1, 0\|1\|2, payload)` | `1 <= n_{M-1} <= Q_{M-1}` | `{(prog_d, *, H(O, 1, 3, ckpt_d[M-1]))}` |

*(Note compatibility with current `SRC-021`: setting `quantum_steps = 0` or keeping `SHK_IN` 6-ary for non-continuing runs and using `SHK_IN_Q` when `Q > 0` leaves existing single-run `input_d` test vectors untouched.)*

---

### 3.5 Folding `M` Continuation Runs via `GP-2` + `GP-4` Tree Recursion into One `O(1)` On-Chain Proof

In `research/sherlock-gp-architecture-20261110/architecture.md:81-97,131-133`, the unified 2-slot `GP-4` node already carries `kind in {SEG, SET}` plus an assumption/dependency bag `deps_d`. Adding **one transitive cut-elimination rule (`CONT-FOLD`)** to `SET + SET -> SET` folds an `M`-segment continuation chain in a **balanced binary tree of depth `ceil(log2 M)`**:

1. **Leaf-run sealing (`SEG -> CONT_SET`):** Each segment `k in 0 .. M - 1` folds its intra-segment leaves (`Layer A`) to `CONT_SET(prog_d, in_k, out_k, s_k)`. Note that all `M` segments can be proved **in parallel** once native host execution pre-computes `ckpt_d[1 .. M-1]` (`GP-4` parallel proving)!
2. **Binary `CONT-FOLD` rule in the `k = 20` node (`~5` Poseidon perms + `u128` add = `~250–350` node rows `ESTIMATE`):**
   Given left child `CONT_SET(prog_d, in_L, out_mid, s_L)` and right child `CONT_SET(prog_d, in_R, out_R, s_R)` plus witness `(ckpt_d, Q_R)`:
   - Enforce `left.prog_d == right.prog_d`;
   - Recompute `H(O, 1, 3, ckpt_d) == out_mid` (`2` perms) and `H(SHK_CONT_IN, 1, ckpt_d, Q_R) == in_R` (`2` perms);
   - Range-check `s_total = s_L + s_R < 2^128` (`(s_lo, s_hi)` in `u64` limbs);
   - Emit merged `CONT_SET(prog_d, in_L, out_R, s_total)` — **completely erasing `ckpt_d`, `out_mid`, and `in_R` inside the recursion circuit!**
3. **Final Run Decider (`SRC-006`, `requirements.md:31`):**
   - Accepts root `CONT_SET(prog_d, input_d[0], final_outcome_d, total_steps)` **if and only if** `input_d[0]` has genesis domain `SHK_IN` (`epoch == 0`), `final_outcome_d` has **terminal tag `0, 1, or 2`** (strictly rejecting `Running` and `Suspended` tag `3`!), and `deps_d == EMPTY`.
   - Emits **one** `6,432`-byte KZG proof verified in **`O(1)`** by `acceptRun`, exposing **only** `[prog_d, initial_input_d_with_Q0_erased_or_bound, final_outcome_d, total_steps]`. *(Even better: if `Initial` binds `origin_in_d = H(SHK_IN, 1, val_d, fuel_lo, fuel_hi, stack_limit)` without `Q_0` and places `Q_0` only in the internal `CONT_SET` header, then even `Q_0 .. Q_{M-1}` are 100% erased at the decider—so an `M`-run continuation produces the **exact same `input_d`, `outcome_d`, and `claim_d`** as a 1-run Layer A proof!)*

#### Layer A vs. Layer B Cost Comparison (`ESTIMATE`)
| Metric | Layer A (`SEG + SEG -> SEG` with Q2(d) `TOP = 64`) | Layer B (`CONT-FOLD` across `M` Continuation Runs) |
|---|---|---|
| **Extra leaf rows / step** | `0` (already in `~61.5k` `k = 16` leaf, `ESTIMATE`) | `+1,000..2,500` rows/leaf for incremental `ed`/`fd`/`hd` & `UNFOLD` (`k = 16`, `ESTIMATE`) |
| **Boundary cost per cut** | `0` extra leaves (`post_d == pre_d` is 1 scalar check) | `1` `Resume` leaf (`O(1)`) + lazy `UNFOLD` for touched working set (`~1–5` leaves) + `1` extra `CONT-FOLD` node (`~64 s`, `ESTIMATE`) |
| **Verifier cost & proof size** | `O(1)`, `6,432 B` (`prover.md:10`) | `O(1)`, `6,432 B` |

---

### 3.6 Interaction with C6 Outcome-Only Publication & Red-Team Analysis of `claim_d`

Under C6 (`SRC-007`, `requirements.md:32`; `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:19-30`), the Compact contract computes:
```text
claim_d = persistentHash([dom("SHK_RUN"), 1, prog_d, input_d, outcome_d])   // or Poseidon equivalent
assert(!settled.member(claim_d));
settled.insert(claim_d);
```
Testing Layer B continuations against every C6 replay, loop, and splicing vector shows how each hazard behaves and how the design above closes it:

1. **Zero On-Chain State Leakage & `O(1)` Ledger Growth:** Because `CONT-FOLD` eliminates every `ckpt_d[1 .. M-1]` before the Run Decider, intermediate segment claims **never touch `settled`**. A 1,000-epoch computation inserts **one** 32-byte `claim_d` into `settled`, identical to a 6-step program.
2. **Deterministic Loops / Identical Functional State Replay (`ckpt_d[k] == ckpt_d[j]`?):**
   - *Attack scenario:* Suppose a guest runs a polling or arithmetic loop that visits the same `(ctl, env, frames, heap)` twice—or consider `/2`'s documented hazard (`docs/sherlock/design/suspension-and-finance.md:380-393`) where a zero-fuel `Await` transition could return to the identical state. If `ckpt_d` hashed only `(ctl, ed, fd, hd)`, then `ckpt_d[k] == ckpt_d[j]` (`k < j`), allowing a malicious prover to **short-circuit `j - k` loop iterations** in `CONT-FOLD` (or collide `deps_d` if deduplicated as a set)!
   - *Why `sherlock-ckpt/1` is immune:* Every `/1` step strictly decrements `remaining_work` by `>= 1` (`P5` fires at `0`), increments `consumed_work`, and every continuation cut increments `epoch` (`k -> k + 1`) and `cum_steps` (`+ Q_k >= 1`). Because `ckpt_d` commits `(origin_in_d, epoch, cum_steps_lo, cum_steps_hi, counters_d)`, **`ckpt_d` forms a strictly DAG-acyclic sequence**: `ckpt_d[k] == ckpt_d[j]` for `k != j` implies a Poseidon collision. Neither loop-skipping nor `deps_d` collision can occur.
3. **Fuel, Allocation, and Stack Rejuvenation Attacks:**
   - *Attack scenario:* A guest given `fuel = 500` needs `1,000` steps to halt (`Halted(7)` instead of `Exhausted(Fuel)`). Can a prover run `400` steps, checkpoint, and resume with fresh `fuel = 500`?
   - *Defense:* `Resume` loads `remaining_work`, `stack_limit`, `frame_count`, and `alloc.*` **exclusively from `ckpt_d`'s `counters_d`**, never from fresh prover witness inputs. Total fuel and allocation caps across `M` continuation segments equal a single uninterrupted run (`SVM-010`).
4. **Cross-Input Splicing Attack:**
   - *Attack scenario:* Inputs `x_1 != x_2` both evaluate `f(x_1)` and `f(x_2)` to `0` at step `Q_0`, reaching identical `(ctl, ed, fd, hd)` and identical `counters_d` at `epoch = 1`. Could an off-chain prover splice `RUN_0(x_1)` with `RUN_1(x_2)` or confuse off-chain proof caches?
   - *Defense:* `ckpt_d` hashes `origin_in_d = input_d[0]`. Even when two distinct inputs converge to the exact same machine state and fuel count, their `ckpt_d` digests remain distinct (`ckpt_d^{(1)} != ckpt_d^{(2)}`), binding every continuation proof cryptographically to its genesis input.
5. **Premature Settlement of `Suspended(ckpt_d)` On-Chain:**
   - *Attack scenario:* Prover tries to pass an un-folded `Suspended(ckpt_d)` (`tag = 3`) proof directly to `acceptRun` to lock `(prog_d, input_d, Suspended(ckpt_d))` in `settled`.
   - *Defense:* `SRC-006` constrains the Run Decider to `tag in {0, 1, 2}` (`Halted`, `Rejected`, `Exhausted`); `tag = 3` is valid only inside `CONT_SET` node folding and has no valid Run Decider witness.


---

## Q4. Prior Art: How Production zkVMs Bound and Extend Run Length and Memory

Every URL cited in this section was fetched and verified live during this study (`HTTP 200`).

### 4.1 Architectural Taxonomy & Universal L1 Public-Input Erasure

Across 10 production zkVMs and 2025–2026 research systems, **run-length and memory scaling** divide into **five cross-segment memory paradigms**, while **on-chain public-input design** has converged on near-universal erasure of internal trace and memory state:

1. **Paged / Block Sparse Algebraic Merkle Tree Across Segments + Offline Intra-Segment Memory** (*RISC Zero v3, OpenVM v2.0, Nexus 1.0/2.0 IVC, SP1 v6 inter-chunk*): Touched pages/blocks open and update against a Poseidon2/Poseidon Merkle root at segment boundaries; recursion joins adjacent segments via `left.post_root == right.pre_root`.
2. **Tree-Less Cross-Shard Multiset Accumulators** (*SP1 v5 Turbo, SP1 v6 intra-chunk, Ziren v2.0, Valida, ZisK, ZKsync Airbender*): Replaces per-boundary Merkle hashing with a global offline memory permutation—either **elliptic-curve point addition over a degree-7 extension ("septic") curve** (`global_cumulative_sum` in SP1 and Ziren, avoiding Fiat-Shamir ordering dependencies across shards) or a persistent cross-segment LogUp sum (`Valida`, `ZisK`, `Airbender`).
3. **Write-Once Continuous Relocated Memory + Public Memory Pages** (*Cairo / Stwo*): Write-once cells `(a' - a)(a' - a - 1) = 0`; sub-runs (`Cairo PIE`s) relocate into continuous address spans or compose recursively through the Cairo Bootloader and `publicMemoryPages`.
4. **Witness-Column KZG Commitment Chaining** (*zkWasm*): Commits pre/post memory tables directly as BN254 KZG G1 points and enforces `slice[i].post_img_col == slice[i+1].pre_img_col` inside Halo2 recursion.
5. **Monolithic Multi-Pass Streaming / Prove-and-Retire** (*Jolt RV64 + Twist & Shout + Dory streaming; Lambda VM Prove-and-Retire*): Streams witness generation and sumcheck/FRI in two passes within `< 2 GB` RAM to emit a single monolithic proof over arbitrary trace length `T` with zero cross-segment recursion.

**Universal L1 Verifier Consensus:** Every general-purpose zkVM shipping an EVM SNARK verifier (**RISC Zero, SP1 v5/v6, OpenVM v2.0, Ziren v2.0, Nexus 1.0/2.0**) **erases segment counts, cycle counts, intermediate PCs, intermediate memory commitments, and even the final post-halt memory root** before the L1 verifier—matching Sherlock's **C6 outcome-only publication** (`SRC-006`, `SRC-007`) almost 1-for-1 (in fact, Sherlock's `SRC-006` currently exposes `n_steps` alongside `[prog_d, input_d, outcome_d]`, whereas RISC Zero, SP1, OpenVM, and Ziren hide even `n_steps` on-chain!).

---

### 4.2 Cross-System Comparison Matrix

| System | Per-Segment / Shard Limit | Architectural Run-Length Limit | Address Space & Cross-Segment Memory | Continuation Folding / Aggregation | Final On-Chain Verifier Public Inputs (Memory / Run-Length Exposed?) |
|---|---|---|---|---|---|
| **1. RISC Zero v3** | `2^po2` cycles: default `2^20` (`1M`), range `2^13..2^24` | `u64` host cycles; **unbounded** via `SystemSplit` | 32-bit (`4 GiB`; `~3 GiB` user), `1 KiB` pages, depth-22 **Poseidon2 Merkle tree** | `Segment` -> `lift` -> binary `join` -> `identity_p254` -> BN254 Groth16 | `[CONTROL_ROOT_{0,1}, claim_{0,1}, BN254_CONTROL_ID]` from `(imageId, journalDigest)`. **`0` cycle/segment/RAM data** (`postState` zeroed on halt). |
| **2. SP1 v5 / v6 Hypercube** | v5: `2^22` cycles. v6: `MAX_SHARD_SIZE = 2^24`, chip `<= 2^22` rows | `u64` timestamp (`4 x 16`-bit limbs); **unbounded** | v5: 32-bit, **septic EC sum**. v6: **48-bit** (`256 TiB`; `24 GiB` default), septic EC intra-chunk + **Poseidon2 Merkle** inter-chunk | `core` (Jagged PCS) -> `normalize` -> `compress` -> `shrink` -> gnark BN254 Groth16/Plonk | v5: `[vkey, pvDigest]`. v6: `[vkey, pvDigest, exitCode=0, vkRoot, nonce]`. **`0` cycle/shard/RAM data**. |
| **3. Jolt (RV64IMAC)** | Monolithic streaming; default `max_trace_length = 2^24` | Arbitrary power-of-2 `T` (`< 2 GB` streaming target) | 64-bit (`RAM_START = 0x8000_0000`, `32 MiB` default heap); **Twist & Shout** sumcheck + **Dory** PCS | Single `~50 KB` proof via streaming sumcheck + BlindFold (recursion WIP) | Native verifier checks `(inputs, outputs, panic, MemoryLayout, T)` via `OutputSumcheck`; EVM verifier WIP. |
| **4. Nexus (v1/2 vs v3)** | v1/2: 1 step (or small batch) / IVC fold. v3: single Stwo trace `<= 2^28` | v1/2: **unbounded** IVC. v3: M31 `< 2^31 - 1` (`<= 2^28` rows) | v1/2: 32-bit **Poseidon Merkle tries**. v3: 32-bit 2-pass Harvard + **LogUp** | v1/2: HyperNova + CycleFold PCD -> Spartan/Groth16. v3: single Stwo proof | v1/2: 1 scalar `u.x = H(vk, (F^Xi, x, y))` (**hides `n` & RAM tries**). v3: `(ELF, pub_in, pub_out, ec)` + FRI log sizes. |
| **5. Valida (Lita)** | `SegmentMachine` `max_trace_height` (BabyBear power-of-2) | **Unbounded** across segments | `< (p-1)/2 ≈ 2^30 B` (`1 GiB`); **persistent cross-segment LogUp bus** | `MultiSegmentMachineProof` (`Vec<SegmentProof>`, no EVM SNARK wrapper in repo) | Off-chain verifier sees per-segment `(pc, fp, output, log_degree)`; no Solidity contract in repo. |
| **6. Cairo / Stwo** | Stwo M31: `N * B^2 <= 2^30` (`N <= 2^28` rows at `B = 2`) | **Unbounded** via `Cairo PIE` + recursive **Cairo Bootloader** | `LOG_MEMORY_ADDRESS_BOUND = 29` (`2^29` felts in `stwo-cairo`); **write-once continuous memory** + public pages | SHARP recursive `stwo_cairo_verifier` tree under Cairo Bootloader | L1 `GpsStatementVerifier` sees top-level `(pc, ap, builtin ptrs, publicMemoryPages)` & registers `(progHash, outHash)`; inner PIEs hidden. |
| **7. zkWasm** | Halo2 KZG `2^k` rows (`k = 18` default, `k = 22` slice) | `u64` global `eid`; **unbounded** slices | 32-bit Wasm (`64 KiB` pages, `<= 4 GiB`); **KZG witness column equality** | `continuation-batcher` Halo2 KZG recursion over BN254 | BN254 Solidity verifier sees `[kzgAcc, init/final image commit, pub I/O]`; hides slice count & mid-states. |
| **8. Miden v1.0-rc.1** | Single VM run: `64 .. 2^29` cycles (`2^12` fragments) | `2^29` cycles/run; scales via deferred precompiles + tx/batch/block kernels | Per-context `[0, 2^32)` (`2^28` default live felts) LogUp **Memory chiplet** + depth-64 **Advice SMT/MMR** | `TransactionKernel` -> `BatchKernel` -> `BlockKernel` + deferred precompile STARKs | VM `ExecutionClaim` = 40 felts `(P, K, StackIn[16], StackOut[16])` (**`0` cycle/RAM roots**); L1/AggLayer sees `BlockHeader` roots. |
| **9. OpenVM v2.0** | Metered height (`< 2^29`), `15 GiB` RAM, bus cells; `t < 2^29` | `u64` host `instret`; **unbounded** across segments | `addr_space_height = 3`, `POINTER_MAX_BITS = 29` (`512 MiB` RV32 RAM); **Poseidon2 Merkle tree** (`CHUNK = 4`) | `app` -> `leaf` -> `internal-for-leaf` -> `internal-recursive` -> `root` -> `static` (Halo2 BN254) | `OpenVmHalo2Verifier` checks `[kzgAcc[12], appExeCommit, appVmCommit, publicValues]`. **`0` cycle/segment/final_root data**. |
| **10. Ziren v2.0** | `shard_size`, `clk < 2^26`, chip `< 2^22` rows, delay-slot safe | `< 2^26` shards (`> 10^14` insns) | 32-bit MIPS32r2 (`4 GiB`, regs `0..35`); **KoalaBear septic EC multiset sum** | `prove_core` (WHIR + Jagged PCS) -> `compress` -> `shrink` -> `wrap_bn254` -> gnark Groth16/PLONK | `IZKMVerifier` feeds **3** scalars `[programVKey, sha256(pv) & ((1<<253)-1), VK_ROOT]`. **`0` cycle/shard/RAM data**. |

---

### 4.3 Per-System Technical Breakdown & Primary Citations

1. **RISC Zero (v3.x / RV32IM Circuit v2):**
   - *(a) Limits:* Segments execute `2^po2` cycles (`MIN_CYCLES_PO2 = 13` to `MAX_CYCLES_PO2 = 24`, default `DEFAULT_SEGMENT_LIMIT_PO2 = 20` = `1,048,576` cycles split across `user + paging + reserved`; `RV32IM_SEAL_VERSION = 3` pins `totalCycles == 2^po2`), with `u64` session counters and unbounded `ExitCode::SystemSplit` continuations (`https://raw.githubusercontent.com/risc0/risc0/main/risc0/zkp/src/lib.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/circuit/rv32im/src/execute/mod.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/zkvm/src/host/server/exec/executor.rs`).
   - *(b) Memory:* 32-bit RV32IM (`4 GiB`, user `[0x0001_0000, 0xc000_0000)`) divided into `1 KiB` pages (`2^22` leaves) in a depth-22 BabyBear Poseidon2 sparse Merkle tree; segments `PAGE_IN` touched pages from `pre_state.merkle_root` and `PAGE_OUT` dirty pages to `post_state` (`https://raw.githubusercontent.com/risc0/risc0/main/risc0/circuit/rv32im/src/execute/platform.rs`, `https://raw.githubusercontent.com/risc0/risc0/main/risc0/binfmt/src/image.rs`).
   - *(c–d) Folding & On-Chain:* `lift` -> binary `join` (`left.post_state == right.pre_state`) -> `resolve` (guest composition assumptions, directly analogous to Sherlock's `GP-2`!) -> `identity_p254` -> BN254 Groth16 (`https://dev.risczero.com/api/recursion`). On `Halted(0)`, RISC Zero **overwrites `postStateDigest` with `SYSTEM_STATE_ZERO_DIGEST` (`pc = 0, merkle_root = 0`)** so `RiscZeroGroth16Verifier.sol` checks only `[CONTROL_ROOT_{0,1}, claim_{0,1}, BN254_CONTROL_ID]` reconstructed from `(imageId, journalDigest)` (`https://raw.githubusercontent.com/risc0/risc0/main/risc0/binfmt/src/lib.rs`, `https://raw.githubusercontent.com/risc0/risc0-ethereum/main/contracts/src/groth16/RiscZeroGroth16Verifier.sol`). Only incomplete `Paused` receipts verified via `verifyIntegrity` expose a non-zero `postStateDigest` (`https://raw.githubusercontent.com/risc0/risc0-ethereum/main/contracts/src/IRiscZeroVerifier.sol`).

2. **SP1 (v4/v5 Turbo & v6 Hypercube):**
   - *(a) Limits:* v5 (RV32IM, BabyBear) shards at `2^22` cycles; v6 Hypercube (RV64IM, KoalaBear, multilinear sumcheck + Jagged PCS) bounds shards by `MAX_SHARD_SIZE = 2^24` cycles, `HEIGHT_THRESHOLD = 2^22` rows/chip, and `ELEMENT_THRESHOLD = 3 * 2^27` trace cells, with 64-bit global timestamps (`W3 = [F; 4]` 16-bit limbs) and unbounded sharding (`https://docs.succinct.xyz/docs/sp1/hypercube/shards`, `https://raw.githubusercontent.com/succinctlabs/sp1/dev/crates/core/executor/src/opts.rs`, `https://raw.githubusercontent.com/succinctlabs/sp1/dev/crates/hypercube/src/air/public_values.rs`).
   - *(b) Memory:* v5 checks 32-bit RAM across shards with zero Merkle trees by mapping `(shard, clk, addr, val)` boundary reads/writes onto the septic curve `y^2 = x^3 + 2x + 26z^5` over `Fp[z]/(z^7 - 2z - 5)` (`SepticDigest`) and asserting the global point sum cancels `MemoryInitialize`/`MemoryFinalize` (`https://docs.succinct.xyz/assets/files/SP1_Turbo_Memory_Argument-b042ba18b58c4add20a8370f4802f077.pdf`, `https://raw.githubusercontent.com/succinctlabs/sp1/v5.0.0/crates/stark/src/air/public_values.rs`). **Notably, SP1 v6 upgraded to a 48-bit address space (`MAXIMUM_MEMORY_SIZE = (1 << 48) - 1` = `256 TiB`, `DEFAULT_MEMORY_LIMIT = 24 GiB`) and reintroduced Poseidon2 Merkle roots (`prev_merkle_root`, `merkle_root`) across `2^24`-entry trace chunks** while keeping septic sums within chunks (`https://raw.githubusercontent.com/succinctlabs/sp1/dev/crates/primitives/src/consts.rs`).
   - *(c–d) Folding & On-Chain:* `core` -> `normalize` -> 3/4-ary `compress` -> `shrink` -> gnark BN254 Groth16/Plonk (`https://docs.succinct.xyz/docs/sp1/hypercube/recursion`, `https://l2beat.com/zk-catalog/sp1hypercube`). `SP1VerifierGroth16.sol` checks `[vkey, pvDigest]` in v5 (`2` scalars) and `[vkey, pvDigest, exitCode=0, vkRoot, nonce]` in v6 (`5` scalars), exposing **zero** shard/timestamp/memory state (`https://raw.githubusercontent.com/succinctlabs/sp1-contracts/main/contracts/src/v5.0.0/SP1VerifierGroth16.sol`, `https://raw.githubusercontent.com/succinctlabs/sp1-contracts/main/contracts/src/v6.0.0/SP1VerifierGroth16.sol`).

3. **Jolt (a16z — RV64IMAC + Twist & Shout):**
   - Upgraded in Oct 2025 to 64-bit `RV64IMAC` (`DEFAULT_MAX_TRACE_LENGTH = 2^24`, `DEFAULT_HEAP_SIZE = 32 MiB`, `https://a16zcrypto.com/posts/article/64-bit-proving-jolt/`, `https://raw.githubusercontent.com/a16z/jolt/main/common/src/constants.rs`). Replaces segment recursion and Merkle trees with **Twist & Shout** one-hot `d`-dimensional increment sumchecks (`< 35` field muls/cycle, `https://eprint.iacr.org/2025/105`, `https://a16zcrypto.com/posts/article/jolt-6x-speedup/`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/how/twist-shout.md`) and **streaming Dory + streaming sumcheck** targeting arbitrary `T` in `< 2 GB` RAM (`https://raw.githubusercontent.com/a16z/jolt/main/book/src/how/dory.md`, `https://raw.githubusercontent.com/a16z/jolt/main/book/src/roadmap/streaming.md`). Native verifier checks `JoltDevice` I/O via `OutputSumcheck` (`https://raw.githubusercontent.com/a16z/jolt/main/book/src/how/architecture/ram.md`, `https://raw.githubusercontent.com/a16z/jolt/main/common/src/jolt_device.rs`); recursion and EVM verifier remain on the roadmap (`https://raw.githubusercontent.com/a16z/jolt/main/book/src/roadmap/on-chain-verifier.md`).

4. **Nexus (v1.0/2.0 IVC vs. v3.0 Stwo):**
   - v1.0/2.0 folds unbounded RV32I steps via HyperNova + CycleFold PCD over Poseidon Merkle memory tries (`Tz`), compressing at termination to a SNARK with **1 scalar public input** `u.x = H(vk, (F^Xi, x, y))` that hides step count `n` and all memory roots (`https://assets.nexus.xyz/whitepaper.pdf` §3.2, §4.5). v3.0 (March 2025) switched to a single-proof Stwo Circle STARK over `M31` (`N <= 2^28` rows) with two-pass compact 32-bit Harvard memory profiling and LogUp offline memory (`https://raw.githubusercontent.com/nexus-xyz/nexus-zkvm/main/specification/zkvm-spec-3.0.pdf` §1.2.4, §2.2, §6–7).

5. **Valida (Lita):**
   - Registerless Harvard stack/RAM VM (`PC`, `FP` only) over BabyBear Plonky3; splits traces at `max_trace_height`, bounds RAM to `< (p - 1) / 2 ≈ 1 GiB` to prevent mod-`p` delta wrap, and links segments via a persistent cross-segment LogUp sum (`cumulative_persistent_sum`) bundled directly in `MultiSegmentMachineProof` without SNARK recursion (`https://arxiv.org/html/2505.08114v1`, `https://raw.githubusercontent.com/lita-xyz/valida-vm/main/machine/src/persistence.rs`, `https://raw.githubusercontent.com/lita-xyz/valida-vm/main/machine/src/proof.rs`).

6. **Cairo / Stwo (StarkWare):**
   - `stwo` over `M31` (`M31_CIRCLE_LOG_ORDER = 31`) requires `N * B^2 <= 2^30` (`N <= 2^28` rows at `B = 2`) and bounds per-proof write-once relocated memory to `LOG_MEMORY_ADDRESS_BOUND = 29` (`2^29` `felt252` words; `https://eprint.iacr.org/2021/1063`, `https://eprint.iacr.org/2024/278`, `https://raw.githubusercontent.com/starkware-libs/stwo/dev/crates/stwo/src/core/circle.rs`, `https://raw.githubusercontent.com/starkware-libs/stwo/dev/.agents/papers/llm/Stwo_Whitepaper.llm.md`, `https://raw.githubusercontent.com/starkware-libs/stwo-cairo/main/crates/common/src/memory.rs`). Unbounded runs execute as `Cairo PIE`s folded recursively by `stwo_cairo_verifier` inside the Cairo Bootloader (`https://zksecurity.github.io/stark-book/cairo/bootloader.html`); L1 `GpsStatementVerifier.sol` checks top-level `cairoAuxInput` (`pc`, `ap`, builtin pointers, `publicMemoryPages`) and registers child `(programHash, keccak256(output))` facts while hiding inner recursive PIE traces (`https://raw.githubusercontent.com/starkware-libs/starkex-contracts/master/evm-verifier/solidity/contracts/gps/GpsStatementVerifier.sol`).

7. **zkWasm (Delphinus Lab):**
   - Slices 32-bit Wasm (`64 KiB` pages) into Halo2 BN254 KZG tables (`-k 18..22`) with `u64` global event IDs, chaining `pre_img_col == prev.post_img_col` KZG G1 commitments in `continuation-batcher` so the BN254 Solidity verifier hides slice count and intermediate states (`https://raw.githubusercontent.com/DelphinusLab/zkWasm/main/README.md`, `https://raw.githubusercontent.com/DelphinusLab/continuation-batcher/main/README.md`, `https://zkproof.org/wp-content/uploads/2024/11/Formal-Verification-of-zkWasm.pdf`).

8. **Miden (v1.0-rc.1):**
   - Goldilocks Plonky3 VM bounding a single run to `MAX_CYCLES = 2^29` (`2^12` fragments), isolating per-context `[0, 2^32)` LogUp `Memory` chiplets from persistent depth-64 `Advice` SMTs/MMRs (`mtree_get`/`mtree_set`), offloading heavy hashes/signatures to `DeferredRoot` precompile STARKs, and scaling horizontally via `TransactionKernel -> BatchKernel -> BlockKernel` (`https://raw.githubusercontent.com/0xMiden/miden-vm/next/processor/src/execution_options.rs`, `https://raw.githubusercontent.com/0xMiden/miden-vm/next/docs/src/design/chiplets/memory.md`, `https://raw.githubusercontent.com/0xMiden/miden-vm/next/docs/src/design/deferred/semantics.md`, `https://raw.githubusercontent.com/0xMiden/miden-base/next/crates/miden-block-prover/src/lib.rs`). Standalone `ExecutionClaim` is a fixed 40-felt tuple `(P, K, StackInputs[16], StackOutputs[16])` with zero cycle/RAM data (`https://raw.githubusercontent.com/0xMiden/miden-vm/next/core/src/program/claim.rs`).

9. **OpenVM (v2.0):**
   - No-CPU BabyBear bus architecture cutting segments on `height < 2^29`, `15 GiB` prover RAM, or bus interactions (`t < 2^29` per segment, `u64` host `instret`), persisting 512 MiB RV32 RAM (`POINTER_MAX_BITS = 29`) across segments via a depth-30 Poseidon2 Merkle tree (`CHUNK = 4`; `https://openvm.dev/whitepaper.pdf`, `https://docs.openvm.dev/specs/architecture/continuations/`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/arch/config.rs`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/arch/execution_mode/metered/segment_ctx.rs`, `https://raw.githubusercontent.com/openvm-org/openvm/main/crates/vm/src/system/memory/mod.rs`). Its `Root` circuit verifies a Merkle proof of `user_public_values` against `final_root` and **drops `final_root`, `final_pc`, and `recursion_depth`** before the Halo2 BN254 `OpenVmHalo2Verifier.sol` wrapper (`https://raw.githubusercontent.com/openvm-org/openvm-solidity-sdk/main/src/v2.0-base/OpenVmHalo2Verifier.sol`).

10. **Ziren / zkMIPS (v2.0, Oct 2026):**
    - MIPS32r2 over KoalaBear using LogUp-GKR + Jagged PCS + WHIR; cuts shards at `clk < 2^26`, chip `< 2^22` rows, and `< 2^26` shards (carrying `(pc, next_pc)` for delay slots), sums 32-bit RAM boundaries on KoalaBear septic curve `y^2 = x^3 + 3zx - 3` over `Fp[z]/(z^7 + 2z - 8)`, and feeds **only 3 BN254 inputs** `[programVKey, sha256(publicValues) & ((1<<253)-1), VK_ROOT]` to gnark on L1 (`https://eprint.iacr.org/2026/2330`, `https://docs.zkm.io/design/continuation.html`, `https://docs.zkm.io/design/memory-checking.html`, `https://docs.zkm.io/design/prover-architecture/prover-architecture.html`, `https://docs.zkm.io/dev/verifier.html`).

11. **2025–2026 Continuation & Memory Advances:**
    - **Jagged PCS (`https://eprint.iacr.org/2025/917`, `https://blog.succinct.xyz/sp1-hypercube/`):** Eliminates multi-table power-of-2 padding waste in SP1 v6 & Ziren v2.
    - **ZKsync Airbender (`https://raw.githubusercontent.com/matter-labs/zksync-airbender/main/docs/philosophy_and_logic.md`, `https://raw.githubusercontent.com/matter-labs/zksync-airbender/main/docs/subarguments_used.md`):** M31 RV32IM chunking `~2^22`-cycle shards up to `2^36` cycles (`2 x 19`-bit timestamp limbs) via global LogUp + CSR delegation, explicitly guarding total multiplicities `< |F|` against characteristic-`p` wraparound (directly parallel to `SRC-004`!).
    - **ZisK (`https://raw.githubusercontent.com/0xPolygonHermez/zisk/main/README.md`, `https://0xpolygonhermez.github.io/zisk-docs/`) & Lambda VM (`https://blog.alignedlayer.com/lambda-vm-continuations/`):** ZisK slices each sub-chip trace independently across cuts; Lambda VM's *Prove-and-Retire* regenerates witnesses in 2 passes to emit a monolithic proof in `O(1)` chunk RAM.
    - **Mangrove & Sonobe (`https://eprint.iacr.org/2024/416`, `https://raw.githubusercontent.com/ethereum/sonobe/main/README.md`):** Uniformized folding IVC with commit-and-fold memory (`~800 MB` RAM at `2^32` gates).
    - **Boundless / `RiscZeroSetVerifier.sol` (`https://raw.githubusercontent.com/boundless-xyz/boundless/main/README.md`, `https://raw.githubusercontent.com/risc0/risc0-ethereum/main/contracts/src/RiscZeroSetVerifier.sol`):** Aggregates cross-application `ReceiptClaim`s into an MMR under one L1 SNARK proof (analogous to Sherlock `GP-4` DAG batching).

### 4.4 Direct Design Takeaways for Sherlock
1. **Why tree-less septic-curve memory (`SP1 v5`, `Ziren`) is NOT a fit for Sherlock:** Offline multiset memory requires end-of-run `MemoryInitialize`/`MemoryFinalize` sweeps proportional to total distinct cells touched (which forced even SP1 v6 Hypercube to reintroduce Poseidon2 Merkle roots across `2^24`-entry chunks for 48-bit RAM!), whereas Sherlock's append-only immutable `MEM` + widened `M3` (`TOP = 64`) achieves **`O(1)` startup and `O(1)` shutdown with zero finalization sweep (`SRC-021`)**.
2. **Counter non-wrap guards (`SRC-004`) are universal best practice:** SP1 v6 (`u64` in `4 x 16`-bit limbs), Airbender (`2 x 19`-bit limbs + `< |F|` multiplicity check), OpenVM (`t < 2^29`), Ziren (`clk < 2^26`), and Valida (`addr < (p-1)/2`) all exist specifically to prevent modular field wraparound—never to size a circuit.
3. **C6 outcome-only publication matches industry state of the art:** Erasing `pre_d`, `post_d`, and `ckpt_d` before the L1 decider matches RISC Zero (`SYSTEM_STATE_ZERO_DIGEST`), OpenVM (`Root` dropping `final_root`), SP1, Ziren, and Nexus.

---

## Q5. Practical Run Length on This Hardware and Spec Tested Target

### 5.1 Host Hardware & Measured Prover Baseline

- **Host (`research/sherlock-speed-replan-20261108/prover.md:15-16`, `qa/sherlock/runs.jsonl:598-713`):** 20-core `aarch64` NVIDIA GB10 (`10× Cortex-X925 + 10× Cortex-A725`), `126,120,948 kB` (`120.3 GiB`) unified LPDDR5X (`273 GB/s`), CPU Halo2/KZG prover over BLS12-381 (`midnight-proofs 0.8.2`).
- **Cgroup Resource Budgets (`SRC-016`, `requirements.md:41`; `qa/sherlock/budgets.json:11-22`):** `48 GiB` (`51,539,607,552 B`) single-process RSS cap; `100 GiB` aggregate slice cap; `G-A <= 1,200 s` (`20 min`); `G-C <= 7,200 s` (`2 h`).

| Circuit / Mode | Domain `k` | Measured Wall Time | Peak RSS | Proof Bytes | Source Citation |
|---|---|---|---|---|---|
| **M2 leaf** | `k = 13` | `2.55 s` | `< 1 GiB` | `6,304 B` | `prover.md:11` |
| **ZKIR leaf (solo, 20t)** | `k = 20` | `66.2 s` | `11.14 GB` | `~10.3 KB` | `experiments/e2-e4.md:10-12`, `runs.jsonl:709` |
| **IVC Node `SHKIVC06` (solo, 20t)** | `k = 20` | `59.62 – 64.95 s` (`61.07 s` med) | `16.58 – 17.35 GB` | `9,888 B` | `runs.jsonl:617,648,649`, `c0-first-profile.md:39` |
| **ZKIR leaf + Node concurrent** | `k = 20 + 20` | `69.1 s` leaf / `90.2 s` node | `17.6 GB` agg | — | `experiments/e2-e4.md:15-19`, `runs.jsonl:711` |
| **3× Node concurrent (`3 × 6t`)** | `3 × (k = 20)` | `128.09 – 131.50 s` (`43.8 s`/node) | `47.73 GB` agg | `9,888 B` | `c0-first-profile.md:57`, `runs.jsonl:618` |
| **Run Decider `SHKRUND1` (solo)** | `k = 20` | `67.12 – 77.88 s` (`70.35 s` med) | `17.35 GB` | `6,432 B` | `runs.jsonl:648,649`, `prover.md:11` |

---

### 5.2 Projected Step Throughput, RSS, and Disk Scaling (`ESTIMATE`)

1. **Per-Leaf & Per-Step Proving Time (`S = 16` steps/leaf under Q2(d) `k = 16` leaf):**
   - Scaling from `k = 13` (`2.55 s`) and `k = 20` (`66.2 s`, `128×` domain -> `26×` time), a `k = 16` chip leaf (`65,536` rows, `16×` smaller than `k = 20`) takes **`~5–9 s` solo** (`~3–5 s` at `k = 15`, `ESTIMATE`) and **`< 1.5 GB` RSS** (`ESTIMATE`).
   - **Critical Topology Fact — Pipelined Linear IVC vs. Binary Tree on 1 CPU Host:**
     - **Pipelined Linear Fold (`L` leaves -> `L` node folds):** Overlapping leaf `i + 1` (`~7 s`, 2 threads) with node `i` (`~62–66 s`, 18 threads) hides leaf proving behind the `k = 20` node, yielding **`~64–68 s` per 16-step leaf = `4.0 – 4.25 s / step`** (`~850 – 900` steps/hour) at **`~18 – 20 GB` aggregate RSS** (`< 42%` of the `48 GiB` single-process cap! `ESTIMATE`). *(For dense pure-control phases averaging `16` full steps/leaf; if rare `PAGE`/`PROG`-heavy phases average `12` steps/leaf, **`~5.3 s / step`**, `ESTIMATE`.)*
     - **2-Slot Binary Tree (`GP-4` `SEG + SEG -> SEG`) on 1 Host:** Merging `L` leaves in a binary tree requires `L` leaf lifts (`SEG`) + `L - 1` internal binary merges = **`2L - 1` `k = 20` node proofs**! Even running 3 concurrent nodes (`43.8 s` amortized/node at `47.7 GB` RSS, `runs.jsonl:618`), `(2L - 1) * 43.8 s ≈ 87.6 s / leaf` = **`~5.5 s / step`** (`1.35×` slower and `2.5×` higher RSS than pipelined linear IVC on one box, `ESTIMATE`). Binary tree aggregation wins on **wall-clock latency** when distributed across **multiple machines** (or once GB10 unified-memory ICICLE GPU acceleration lands `1.5–2.5×` node speedup to `~25–40 s`/node, `prover.md:74-75`, `ESTIMATE`).
2. **One-Time Setup / Key Cache Overhead:** `k = 20` PK/VK load + Run Decider (`~70 s`) adds a constant **`~95 – 110 s`** fixed tail per run (`ESTIMATE`).
3. **Disk Footprint:**
   - Streaming linear IVC without GP bundle retention: **`< 50 MB`** working disk (`2` `9,888 B` accumulators + `6,432 B` final proof, `ESTIMATE`).
   - Retaining `sherlock-gp-bundle/1` (`SRC-019`, `requirements.md:44`, `<= 16 KB`/leaf, `~10.3 KB` measured): **`~644 B / step` (`~10.3 KB / 16` steps)** = **`~2.6 MB` at `4,096` steps**, **`~644 MB` at `1,000,000` steps** (`ESTIMATE`), plus cached `k = 20` PKs on disk (`~4–8 GB` one-time cache, `ESTIMATE`).

#### Practical Single-Host Run-Length Budget Table (`S = 16`, CPU Baseline, `ESTIMATE`)

| Wall-Clock Budget | Max `S = 16` Leaves (`L`) | Practical Owner Steps (`~14–16` steps/leaf) | Peak RSS (`ESTIMATE`) | `gp-bundle/1` Disk (`ESTIMATE`) | Representative Workload |
|---|---|---|---|---|---|
| **`~2.3 min`** (`140 s`) | `1` leaf | `1 – 16` steps | `~18 GB` | `~11 KB` | `entry 7;` (`6` steps, `G-A` well inside `20 min`) |
| **`20 min`** (`G-A` cap) | `~16` leaves | `~225 – 256` steps | `~19 GB` | `~170 KB` | Small functional / ADT kernels |
| **`~23 min`** (`1,380 s`) | `19 – 20` leaves | `293 – 303` steps | `~19 GB` | `~205 KB` | `fact(3)` (`293` steps) or `list123` (`303` steps) |
| **`2 hours`** (`G-C` cap) | `~105 – 110` leaves | **`~1,500 – 1,760` steps** | `~19 GB` | `~1.1 MB` | Full `G-C` gate (`fact(3)` + `list123` + `512`-step `P3` combined!) |
| **`4.5 – 5.0 hours`** | **`256` leaves** | **`4,096` steps** | `~19 GB` | `~2.7 MB` | **Recommended `GP-001` endurance soak target** |
| **`10 hours` (overnight)** | `~540` leaves | **`~7,600 – 8,600` steps** | `~19 GB` | `~5.6 MB` | Overnight regression soak |
| **`24 hours` (1 day)** | `~1,320` leaves | **`~18,500 – 21,100` steps** | `~19 GB` | `~13.6 MB` | Full-day single-host run |
| **`7 days` (1 week)** | `~9,300` leaves | **`~130,000 – 148,000` steps** | `~19 GB` | `~96 MB` | Multi-day endurance ceiling on 1 CPU host |

---

### 5.3 What Tested Target the Spec Should Name

A single number cannot serve both a fast `< 2 h` CI gate and a multi-hundred-segment endurance proof on CPU (`4.1 s/step` `ESTIMATE`). The spec should name **three explicit, honest qualification tiers** in `requirements.md` (`SRC-011`, `SRC-015`, `GP-001`, `SRC-023`, `SAS-012`) and `assurance.md`:

1. **Routine Release Gate `G-C` (`SRC-011`, `SRC-015` — `< 2 h` Total Wall Budget):**
   - Keep `fact(3)` (`293` steps, `~22 min` `ESTIMATE`) + `list123` (`303` steps, `~23 min` `ESTIMATE`) + one unseen post-freeze `P3` program of **`>= 512` steps (`>= 32` leaves at `S = 16`, `~36 min` `ESTIMATE`)**, finishing all three end-to-end inside `~82 min < 120 min` (`ESTIMATE`).
2. **GP-1 Multi-Segment Endurance Soak (`GP-001` Row `A33` — Dedicated Release Milestone Gate):**
   - Name **`>= 4,096` owner steps across `>= 256` leaves (allocating `> 256` heap cells so state exceeds single-leaf carriers by `> 32×`)**, completing in **`~4.5 – 5.0 hours` (`ESTIMATE`)** on this host (or `~2–3 h` with 3-way parallel tree workers / GPU, `ESTIMATE`) with `proof.len() == 6,432 B` identical to `entry 7;`. `256` leaves exercises 8+ binary tree levels (`2^8`), multi-hundred-step accumulator folding, non-trivial M3 subtree transitions, and real heap paging without pretending to run millions of CPU hours.
3. **`SRC-023` Partition-Independence Replay Gate (Task `5.12`, Row `A38`):**
   - Name a **`>= 64`-step allocating program** proved under all three required partitions (`S = 16` -> `4` leaves; `S = 1` -> `64` leaves; mixed -> e.g. `12` leaves), total `80` node folds = **`~90 min` (`ESTIMATE`)**, asserting byte-for-byte equality of `[prog_d, input_d, outcome_d, n_steps]` and `claim_d`.

---

## What Was Not Checked

In compliance with Round 1 read-only rules and workspace constraints, the following items were not executed or remain open outside this study:
1. **No builds, tests, circuit synthesis, or prover runs were executed:** All `cargo test`, `MockProver`, `lake build`, Quint verification, and Halo2 benchmark invocations were deliberately skipped per `BRIEF.md:67`; measured numbers come strictly from committed artifacts (`qa/sherlock/runs.jsonl`, `prover.md`, `c0-first-profile.md`, `experiments/e2-e4.md`), and all projected row counts and timings are marked `ESTIMATE`.
2. **Full 108-rule chip leaf (`PAGE` + `PROG` + `vd`) is not yet implemented in `crates/mori-chip`:** Only the E8 narrow-core + M3 + `ALU`/`RANGE` + `HASH` spike (`6,511` rows, `crates/mori-chip/src/rfcore.rs:1204-1228`) exists in Rust today; full-leaf row totals (`~58.6k–61.5k` at `TOP = 61..64`) are analytical `ESTIMATE`s over the measured `70`-row Poseidon chip.
3. **Concrete `UNFOLD` and `CONT-FOLD` circuit gadgets & `GP-3`/`GP-4` 2-slot node (`tasks.md` `4.14`):** `crates/mori-recursion/src/ivc.rs` currently implements linear 1-accumulator IVC; the 2-slot tree/DAG node and Layer B's `UNFOLD`/`CONT-FOLD` constraints are specified mathematically in Q3 but not yet synthesized in Halo2 or mechanized in `clean/fork/Clean/Sherlock/Aggregation.lean`.
4. **`D-2` (`ty_d` / `decl_schema_d` canonical type normal form in `dissent.md`):** Orthogonal to run length and memory ceiling; left at its existing specification status (`SVM-012` / `SRC-022`).
5. **Live Midnight Preview / Mainnet `LedgerParameters` (`tasks.md` `6.8`) & `midnight-proofs` `0.8.2` vs `0.8.3` pin (`c6-outcome-ledger9-review/review.md:207-237`):** Network RPC parameters and upstream `midnight-ledger` crate reunification were not queried or built.
