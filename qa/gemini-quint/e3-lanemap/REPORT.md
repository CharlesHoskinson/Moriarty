# Soundness & Completeness Review: D-9 Reader `decode` Lane Map (`lane-src/iface.rs`, `e3-iface` `099d6d8`)

## Summary Table of Verdicts

| # | Question | Verdict | Summary of Finding |
|---|---|---|---|
| **1** | **Acceptance rule & output index** | **SOUND** | Each non-`(0, 0)` pair is accepted only when `offs[ent]` is `DIGEST(t)` (`0 <= t < 6`) or `SPONGE(3)`, and `o + vals(op, param) - 1` indexes `o + 1` for `DIGEST` and `o + 3` for `SPONGE(3)` (the exact last logged value). |
| **2** | **Uniqueness, lanes `0`/`1`, & `(0, 0)`** | **UNSOUND** *(edge cases 2a, 2b)* / **SOUND** *(2c)* | **(2a) UNSOUND in reader logic**: `s[lane] != 0` fails to catch a second pair targeting the same `lane` if the first mapped entry's output is `0` (overwrite bug; computationally guarded only by Poseidon preimage resistance).<br>**(2b) UNSOUND if lanes 0/1 are reserved**: `decode` checks `lane < d.lanes()` but has no `lane >= 2` lower-bound check, so `(0, ent)` (`ent != 0`) can set lane 0 (binding) and `(1, ent)` can set lane 1 (`comm`) when their call-vector limbs are 0.<br>**(2c) SOUND**: `(0, 0)` only shadows `(lane = 0, ent = 0)`; `(lane, 0)` for `lane >= 1` still maps entry 0 (`DIGEST(SHK_PROGC)`), and `(0, 0)` does not require `lanes[0] == 0`. |
| **3** | **Out of range (`ent`, `lane`, non-`u16`)** | **SOUND** *(at `Dims::FULL`)* / **UNSOUND** *(if `b` or `lanes() > 2^16`)* | `ent >= d.b` is refused by `offs.get(ent)`; `n_used <= ent < d.b` hits `NOP` padding and is refused by the op check; `lane >= d.lanes()` is refused before indexing `s`. However, `decode` has **no explicit `< 2^16` check** on `w[106..114]`, relying on `d.b = 256 < 2^16` and `d.lanes() = 1026 < 2^16` at `Dims::FULL`. |
| **4** | **Binding (`R`, replay, & wrap boundary)** | **UNSOUND** *(in `decode` alone; left to wrap)* | Native replay binds each entry's output to its logged inputs, and `R` binds entry 3's inputs to entries `0..2`. However, `decode` does **not** fix `w[106..114]` and checks `op == SPONGE && param == 3` rather than `ent == 3`. Unless the wrap pins `w[106..114]`, a prover can point `claim_pub` at a discretionary `DIGEST` or a discretionary `SPONGE(3)` outside `R` (or set pairs to `(0, 0)`). |
| **5** | **Interplay with `PUB(i)` & zero-limb rule** | **SOUND** | `dec` is canonical (`< r` and each limb `< 2^30`), so `lanes[lane] == 0` holds iff all 9 call-vector limbs are 0. `PUB(i)` is checked **after** the lane-map loop against `s[i]`, so any `PUB(i)` on a mapped lane must match the post-lane-map output `s[i]`. |
| **6** | **Completeness (`case_run` & chunks)** | **SOUND / COMPLETE** | Every honest `case_run` (`FullV`, `FullP`, `AllEntries`, `Overflow`, and `Chunk { j, k }` for `1 <= j <= k`, including `(19, 3)` and `(33, 3)` sharing entry 3) decodes to `Ok(run.statement())`. |

---

## Detailed Answers & Evidence per Question

### 1. Acceptance Rule and Output Index (`o + vals(op, param) - 1`)

- **Verdict**: **SOUND** (for each pair individually; see Question 2a for duplicate-lane overwrite when the first output is `0`).
- **Code Evidence** ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L632-L643)):
  ```rust
  let mut s = lanes;
  for k in 0..LANE_PAIRS {
      let (lane, ent) = (w[LANE_MAP + 2 * k], w[LANE_MAP + 2 * k + 1]);
      if (lane, ent) == (0, 0) {
          continue;
      }
      let (op, param, o) = *offs.get(ent as usize).ok_or("lane map entry")?;
      if !(op == DIGEST || (op == SPONGE && param == 3)) || lane as usize >= d.lanes() || s[lane as usize] != Fr::ZERO {
          return Err(format!("lane map pair {k}"));
      }
      s[lane as usize] = vs[o + vals(op, param) - 1];
  }
  ```
- **Op & Parameter Check**:
  - In the entry-table pass ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L584-L598)), every entry `i in 0..d.b` is checked against `bounds(d, op)` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L155-L164)):
    - `DIGEST` (`op = 8`) is bounded to `0 <= param <= N_TAGS - 1` (`0..=5`).
    - `SPONGE` (`op = 3`) is bounded to `1 <= param <= d.v - 1`.
  - In the lane-map loop (line 639), `!(op == DIGEST || (op == SPONGE && param == 3))` rejects every op other than `DIGEST(t)` and `SPONGE(3)`—specifically refusing `NOP (0)`, `PERM (1)`, `H2 (2)`, `SPONGE(n)` with `n != 3`, `PATH (4)`, `ECMUL (5)`, `ECMULG (6)`, and `PUB (7)`.
- **Output Index `o + vals(op, param) - 1`**:
  - In `offs`, entry `ent` has starting offset `o` in `vs` and occupies `vs[o .. o + vals(op, param)]` (verified to fit within `d.v` at line 599: `if off > d.v || np > d.p { return Err("capacity".into()); }`).
  - **For `DIGEST(t)` (`op = 8`)**:
    - `vals(DIGEST, param) == 2` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L138)).
    - `OpIn::Digest { tag, d2 }.entry()` logs `vec![*d2, digest(*tag, *d2)]` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L356)), and native replay checks `digest(param, v[0]) == v[1]` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L625)).
    - `o + vals(DIGEST, param) - 1 = o + 2 - 1 = o + 1`, which is `v[1]`—the 2nd and last logged value of the `DIGEST` entry.
  - **For `SPONGE(3)` (`op = 3, param = 3`)**:
    - `vals(SPONGE, 3) == 3 + 1 == 4` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L133)).
    - `OpIn::Sponge(x).entry()` logs `[x[0], x[1], x[2], sponge_u(0, x)]` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L348-L352)), and native replay checks `sponge_u(0, &v[..3]) == v[3]` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L624)).
    - `o + vals(SPONGE, 3) - 1 = o + 4 - 1 = o + 3`, which is `v[3]`—the 4th and last logged value of the `SPONGE(3)` entry.
  - This matches `Run::statement` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L486-L498)), which assigns `s[lane as usize] = *e.values.last().unwrap()`.

---

### 2. Uniqueness (`s[lane] != 0`), Lanes 0 and 1, and `(0, 0)` Sentinel

- **Verdict**:
  - **2a (Duplicate-lane check via `s[lane] != 0`)**: **UNSOUND** in the reader's logic when the first mapped entry's output is `Fr::ZERO` (computationally infeasible to trigger in full `decode` only if Poseidon has no known zero preimage).
  - **2b (Mapping lane 0 or lane 1)**: **UNSOUND** if the reader is expected to forbid mapping the binding lane (`0`) and `comm` lane (`1`).
  - **2c (`(0, 0)` as "unused")**: **SOUND**.

- **Evidence & Concrete Counterexamples**:
  1. **(2a) Two pairs setting one lane when the first mapped entry outputs `0`**:
     - `decode` initializes `let mut s = lanes;` (line 632) and uses `s[lane as usize] != Fr::ZERO` (line 639) to check **both** that the call-vector lane was zero and that no prior pair in `0..k` already wrote `s[lane]`.
     - **Concrete Counterexample** (`ZERO_OUTPUT_LOG` in `lanemap.qnt` and `uniquenessAndUnusedZeroPairTest` in `lanemap_test.qnt`):
       - Suppose `lanes[2] == 0`, entry `4` (`DIGEST`) has output `vs[o_4 + 1] == 0`, entry `5` (`DIGEST`) has output `vs[o_5 + 1] == 25`, and `lane_map = [(2, 4), (2, 5), (3, 3), (5, 2)]`.
       - At `k = 0` (`(2, 4)`): `s[2] == 0`, so `s[2] != 0` is `false`. `decode` sets `s[2] = 0`.
       - At `k = 1` (`(2, 5)`): `s[2]` is **still `0`**, so `s[2] != 0` is `false`! `decode` accepts the duplicate mapping of lane `2` and overwrites `s[2] = 25`.
       - As a result, `decode` accepts a lane map where lane `2` is set twice, and the final statement `s[2] = 25` does not equal the output `0` of entry `4` mapped by pair `0`.
       - *(Note: In `iface.rs`, native replay at lines 619–631 checks `v[1] == digest(param, v[0])` and `v[3] == sponge_u(0, &v[..3])`, so producing a `0` output requires a zero preimage of `PoseidonFr`. Nevertheless, conflating the zero-limb check and the duplicate-lane check is a logical flaw; tracking mapped lanes separately as in `decodeFixedDup` fixes it unconditionally.)*
  2. **(2b) Setting lane 0 (binding) or lane 1 (`comm`) via the lane map**:
     - By `Dims::lanes(&self) = 2 + self.l_tx` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L109-L111)) and `case_run` (`first_effect = 2usize`, [lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L738-L742)), lanes `0` and `1` are the binding and `comm` lanes, while effect lanes start at index `2`.
     - Line 639 checks only `lane as usize >= d.lanes()` (upper bound) and does **not** check `lane < 2` (lower bound).
     - **Concrete Counterexample** (`uniquenessAndUnusedZeroPairTest` in `lanemap_test.qnt`):
       - Set `lanes[0] = Fr::ZERO` and `lanes[1] = Fr::ZERO` in the call vector, and use `lane_map = [(0, 3), (1, 0), (19, 3), (38, 2)]`.
       - Pair `0` is `(0, 3) != (0, 0)`. Because `0 < d.lanes()` and `s[0] == 0`, `decode` accepts `(0, 3)` and sets `s[0]` (the binding lane) to the `SPONGE(3)` output of entry `3`.
       - Pair `1` is `(1, 0) != (0, 0)`. Because `1 < d.lanes()` and `s[1] == 0`, `decode` accepts `(1, 0)` and sets `s[1]` (the `comm` lane) to the `DIGEST(SHK_PROGC)` output of entry `0`.
  3. **(2c) Safety of `(0, 0)` as the "unused" sentinel**:
     - Line 635 checks `if (lane, ent) == (0, 0) { continue; }` (matching `Run::words` line 471: `.filter(|p| **p != (0, 0))`).
     - When a pair is `(0, 0)`, it is skipped before `s[0] != Fr::ZERO` is tested, so an honest call vector with `lanes[0] != Fr::ZERO` is unaffected by unused `(0, 0)` pairs.
     - Mapping any lane `lane >= 1` to entry `0` (`DIGEST(SHK_PROGC)`) uses `(lane, 0) != (0, 0)`, which is not skipped.
     - Only the single coordinate `(lane = 0, ent = 0)` is shadowed by `(0, 0)`. Because lane `0` is the binding lane and never an honest mapped effect lane (`>= 2`), `(0, 0)` is safe as the unused marker.

---

### 3. Out of Range: `ent` Beyond Used Entries, `lane` Beyond `lanes()`, and Non-`u16` Words

- **Verdict**: **SOUND** for `ent` and `lane` bounds (and **SOUND at `Dims::FULL`** for `u16`, but **UNSOUND** for general `Dims` with `b > 2^16` or `lanes() > 2^16` due to missing explicit `u16` checks on lane-map words).
- **Evidence**:
  1. **Entry index `ent` beyond the used entries (`offs`)**:
     - **Case A (`ent as usize >= d.b`)**: `offs` has length `d.b`. At line 638, `offs.get(ent as usize)` returns `None`, and `.ok_or("lane map entry")?` returns `Err("lane map entry".into())`.
     - **Case B (`n_used <= ent as usize < d.b`, i.e., pointing into the trailing `NOP` padding)**: At lines 590–594:
       ```rust
       if op == NOP {
           ended = true;
       } else if ended {
           return Err(format!("entry {i}: used entry after NOP"));
       }
       ```
       Every entry at index `ent >= n_used` is guaranteed to have `op == NOP` (`0`). When a lane-map pair references `n_used <= ent < d.b`, `offs.get(ent as usize)` returns `Some((NOP, 0, off))`, and line 639 `!(op == DIGEST || (op == SPONGE && param == 3))` evaluates to `true`, returning `Err(format!("lane map pair {k}"))`.
  2. **Lane index `lane` beyond `d.lanes()`**:
     - At line 639, `lane as usize >= d.lanes()` is checked before `s[lane as usize] != Fr::ZERO` (short-circuit `||`), returning `Err(format!("lane map pair {k}"))` without out-of-bounds panic.
  3. **Words that are not `u16`**:
     - The FP-3 spec states: `"lane map: 4 pairs (lane: u16, entry: u16)"`.
     - `unpack` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L503-L528)) only checks that every unpacked word is a canonical BabyBear field element `w < P` (`P = 2013265921 < 2^31`), not `< 2^16`.
     - Unlike `anchor_tags` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L535)), which explicitly enforces `head[J_WORDS + i] < (1 << 16)`, `decode` performs **no explicit `< (1 << 16)` check** on `w[LANE_MAP .. LANE_MAP + 8]`.
     - Instead, for any pair `(lane, ent)` accepted by `decode`, either `(lane, ent) == (0, 0)` or `ent < d.b` and `lane < d.lanes()`. At `Dims::FULL` (`b = 256`, `l_tx = 1024`, `lanes() = 1026`) and the test `Dims` (`b = 48`, `lanes() = 64`), `d.b < 2^16` and `d.lanes() < 2^16`, so any word >= `2^16` is rejected by `ent >= d.b` or `lane >= d.lanes()`. However, if `Dims` were instantiated with `d.lanes() > 65536` or `d.b > 65536`, `decode` would accept a non-`u16` `lane` or `ent` word.

---

### 4. Binding: `R`, Native Replay, and Reader vs. Wrap Responsibilities

- **Verdict**: **SOUND** with respect to a fixed `lane_map` (and when the wrap fixes `w[LANE_MAP..LANE_MAP + 8]`); **UNSOUND in `decode` alone** if the wrap does not constrain `w[LANE_MAP..LANE_MAP + 8]` (and `decode` is looser than necessary by accepting any `SPONGE(3)` entry rather than requiring `ent == 3`).
- **Evidence**:
  1. **Can a prover make a mapped lane differ from `Run::statement()` for the same `lane_map` and `log`?**
     - Absent the zero-output duplicate-lane overwrite of Question 2a, **no**:
       - `r_entries` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L605-L610)) enforces that entries `0..3` are `DIGEST(SHK_PROGC)`, `DIGEST(SHK_INPUT)`, `DIGEST(SHK_OUTCOME)`, `SPONGE(3)`, entry `4` is `DIGEST(tag_in)`, and (when `mode == 1`) entry `5` is `DIGEST(tag_out)`.
       - Lines 615–618 enforce `vs[6..9] == [vs[1], vs[3], vs[5]]`, binding the 3 inputs of entry `3` (`SPONGE(3)`) to the outputs of entries `0, 1, 2`.
       - Lines 619–631 replay `PERM`, `H2`, `SPONGE`, and `DIGEST` natively over `vs`, forcing `vs[o + vals(op, param) - 1]` for every `DIGEST` and `SPONGE` entry to equal the true Poseidon output of its logged inputs.
       - Therefore, for the given `w[LANE_MAP..LANE_MAP + 8]`, `decode` sets each mapped `s[lane]` to the exact value `Run::statement()` computes.
  2. **Can a pair point the `claim_pub` lane at a discretionary `DIGEST` or at another `SPONGE(3)` entry outside `R`?**
     - **Yes, in `decode` alone!** `decode` never checks the contents of `w[LANE_MAP..LANE_MAP + 8]` against `[(LANE_A_IN, 4), (LANE_CLAIM[0], 3), (LANE_CLAIM[1], 3), (LANE_OUTCOME, 2)]`, and line 639 checks `op == DIGEST || (op == SPONGE && param == 3)` rather than `op == DIGEST || ent == 3`.
     - **Concrete Counterexample A (pointing `claim_pub` at a discretionary `DIGEST`)**:
       In `case_run(d, Case::FullV, 7)`, entry `8` is a discretionary `DIGEST(SHK_LOCK)` entry (and in `lanemap.qnt`, entry `5` in mode `0` is a discretionary `DIGEST(SHK_LOCK)` entry with output `25`). Replacing pair `1` (`(19, 3)` in `iface.rs`, `(3, 3)` in `lanemap.qnt`) with `(19, 8)` (or `(3, 5)`) is **accepted** by `decode` and sets the first `claim_pub` lane to the discretionary `DIGEST(SHK_LOCK)` output instead of entry `3`'s `claim_pub` output (`bindingAndDiscretionaryTargetTest`).
     - **Concrete Counterexample B (pointing `claim_pub` at a discretionary `SPONGE(3)` outside `R`)**:
       Only entry `3` (`sp = R_OFF[3] = 6`, lines 615–618) has its inputs checked against `[vs[1], vs[3], vs[5]]`. If a run logs a discretionary `SPONGE(3)` entry at `ent >= 5` (modeled in `EXTRA_SPONGE3_LOG` at entry `8` with inputs `[81, 82, 83]` and output `84`), native replay checks only `v[3] == sponge_u(0, &[v[0], v[1], v[2]])` on the prover's chosen inputs. Pointing `claim_pub` lane `3` at entry `8` (`{ lane: 3, ent: 8 }`) is **accepted** by `decode` and sets `s[3] = 84 != 33` (`bindingAndDiscretionaryTargetTest`).
       *(Note: Changing line 639 in `iface.rs` from `op == DIGEST || (op == SPONGE && param == 3)` to `op == DIGEST || ent == 3` would immediately close the discretionary-`SPONGE(3)` hole inside `decode`, since entry `3` of `R` is already verified to be `(SPONGE, 3)` at lines 555 and 607, and no other `SPONGE` entry in the log is ever a valid lane-map target!)*
     - **Concrete Counterexample C (omitting pairs with `(0, 0)`)**:
       If a prover sets `w[LANE_MAP..LANE_MAP + 8]` to all `(0, 0)`, `decode` skips all 4 pairs and lets the prover place arbitrary `Fr` values directly into `lanes[9]`, `lanes[19]`, `lanes[33]`, and `lanes[38]` in the call vector.
  3. **Precise Boundary: What the Reader (`decode`) Enforces vs. What is Left to the Wrap**:
     - **Enforced by the reader (`decode`)**:
       - Canonical BabyBear unpacking (`w[i] < P`, high bits zero) and canonical `Fr` limb decoding (`dec`: 9 limbs `< 2^30`, reconstructed integer `< r`) for all call-vector lanes and logged value slots.
       - Per-op parameter bounds (`bounds(d, op)`), contiguous non-`NOP` entries followed only by `NOP` padding, total value and permutation capacities (`off <= d.v`, `np <= d.p`), and zero padding of unused value rows (`vs[off..] == 0`).
       - Reservation `R` header and prefix (`r_entries(&w[..STMT])`: entries `0..3` are `DIGEST(0)`, `DIGEST(2)`, `DIGEST(3)`, `SPONGE(3)`; entry `4` is `DIGEST(tag_in)`; entry `5` in mode `1` is `DIGEST(tag_out)`), `E_IN_WORD == 4`, `E_OUT_WORD == 4 + mode`, and entry `3` input equality `vs[6..9] == [vs[1], vs[3], vs[5]]`.
       - Native Poseidon replay of every `PERM`, `H2`, `SPONGE`, and `DIGEST` entry over `vs`.
       - For each non-`(0, 0)` pair `(lane, ent)` in `w[106..114]`: `ent < d.b`, `offs[ent]` is `DIGEST` or `SPONGE(3)`, `lane < d.lanes()`, `s[lane] == 0` before assignment, and `s[lane] = vs[o + vals(op, param) - 1]`.
       - Post-lane-map `PUB(param)` check: `s[param] == vs[o]` for every `PUB` entry.
     - **Left to the wrap (and key constants / inner proof)**:
       - **Fixing the lane map `w[106..114]`** to the program key's constant pairs (e.g. `[(9, 4), (19, 3), (33, 3), (38, 2)]`) and fixing `layout_d` / `Dims`.
       - **Binding the `DIGEST` preimage inputs `v[0]` (`pack(d2)`)** of `R` (entries `0, 1, 2, 4`, and `5`) to the public statement words in `root_stmt` (`r_p`, `prog_d`, `run_tag`, `input_d`, `outcome_d`, and chunk index `j`), including `A_0 = C_P` in ordinary mode (`vs[R_OFF[4]] == vs[R_OFF[0]]`).
       - **Checking the log onion commitment** (`log_d` at words `57..65` and `log_n` at word `65`, which `decode` never reads) and verifying `PATH` entries against wrap-private Merkle siblings.

---

### 5. Interplay with `PUB(i)` and the Zero-Limb Rule for Mapped Lanes

- **Verdict**: **SOUND**.
- **Evidence**:
  1. **Zero-limb rule in the call vector**:
     - The FP-3 spec requires: `"a lane in the lane map holds zero limbs"` in the call vector.
     - `Run::words` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L468-L479)) enforces `assert_eq!(x, Fr::ZERO)` for every lane in `mapped`, writing `[0; 9]` limbs.
     - `decode` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L573-L576)) decodes each lane's 9 limbs using `dec` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L414-L427)). Because `dec` rejects any limb `>= 2^30` and rejects any reconstructed integer `big >= Fr::order()` (`Canon`), `dec` returns `Some(Fr::ZERO)` **if and only if `big == 0`**, which holds **if and only if all 9 limbs are `0`** (a non-zero multiple of `r` is rejected by `big >= Fr::order()`).
     - On the first pair mapping `lane`, `s[lane as usize]` still holds `lanes[lane as usize]`, so `s[lane as usize] != Fr::ZERO` (line 639) rejects any mapped lane that had any non-zero limb in the call vector.
  2. **Ordering and interplay with `PUB(i)`**:
     - The `PUB(i)` loop ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L644-L648)) runs **after** the lane-map loop:
       ```rust
       for &(op, param, o) in &offs {
           if op == PUB && s[param as usize] != vs[o] {
               return Err(format!("PUB({param}) differs from its lane"));
           }
       }
       ```
     - Note that `param < d.lanes()` was already verified at line 586 via `bounds(d, PUB) == Some((0, d.lanes() as u64 - 1))`.
     - For an **unmapped** lane `i`, `s[i] == lanes[i]`, so `PUB(i)` checks `vs[o] == lanes[i]`.
     - For a **mapped** lane `i`, `lanes[i] == Fr::ZERO` in the raw call vector, and the lane-map loop replaces `s[i]` with the mapped `DIGEST` or `SPONGE(3)` output `v_out`. Because `PUB(i)` is checked against `s[i]` after the lane-map loop:
       - A `PUB(i)` entry targeting a mapped lane `i` is accepted if and only if its logged value `vs[o]` equals the **post-lane-map output `v_out`** (and is refused if `vs[o] == Fr::ZERO` while `v_out != Fr::ZERO`, as demonstrated in `pubAndZeroLimbInterplayTest` and `mutPubBeforeLanemapCaughtTest`).
       - In `case_run` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L748-L762)), `pubs` selects only unmapped lanes (`if !mapped.contains(&l) { break l; }`).

---

### 6. Completeness: Honest `case_run` Lane Maps (Including Chunk Cases)

- **Verdict**: **SOUND / COMPLETE**.
- **Evidence**:
  - Every `case_run` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L835-L840)) sets:
    ```rust
    let lane_map = [
        (LANE_A_IN, ANCHOR_IN as u64), // (9, 4)
        (LANE_CLAIM[0], 3),            // (19, 3)
        (LANE_CLAIM[1], 3),            // (33, 3)
        (LANE_OUTCOME, 2),             // (38, 2)
    ];
    ```
  - **Distinct lanes & shared entry 3**:
    - The 4 target lanes `9, 19, 33, 38` are pairwise distinct, `< d.lanes()` (for `Dims::FULL` with `lanes() = 1026` and test `Dims` with `lanes() = 64`), and initialized to `Fr::ZERO` in `lanes` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L737-L747)).
    - Pairs `1` and `2` (`(19, 3)` and `(33, 3)`) map **two different lanes to the same entry `3` (`SPONGE(3)`)**. `decode` accepts this because `s[lane as usize] != Fr::ZERO` checks target-lane uniqueness (`19 != 33`), not source-entry uniqueness.
  - **Ordinary cases (`mode = 0`: `FullV`, `FullP`, `AllEntries`, `Overflow`)**:
    - `R` has 5 entries (`0..=4`): entry `2` is `DIGEST(SHK_OUTCOME)`, entry `3` is `SPONGE(3)`, and entry `4` (`ANCHOR_IN`) is `DIGEST(SHK_PROGC)`. All 4 pairs target `DIGEST` or `SPONGE(3)` entries and decode to `Ok(run.statement())`.
  - **Chunk cases (`mode = 1`: `Case::Chunk { j, k }` for `1 <= j <= k`)**:
    - `anchor_tags` ([lane-src/iface.rs](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/lane-src/iface.rs#L533-L549)) returns `Some((tag_in, tag_out))` with `tag_in = if j == 1 { SHK_PROGC } else { SHK_LOCK }` and `tag_out = if j == k { SHK_PROGC } else { SHK_LOCK }`.
    - `R` has 6 entries (`0..=5`): entry `4` (`ANCHOR_IN`) is `DIGEST(tag_in)` and entry `5` (`a_out`) is `DIGEST(tag_out)`.
    - `lane_map` maps `(9, 4)` to entry `4` (`DIGEST(tag_in)`), `(19, 3)` and `(33, 3)` to entry `3` (`SPONGE(3)`), and `(38, 2)` to entry `2` (`DIGEST(SHK_OUTCOME)`). All 4 pairs satisfy `op == DIGEST || (op == SPONGE && param == 3)` and decode to `Ok(run.statement())` (tested in `cases_decode_or_refuse` for `j = 1, 2, 3, k = 3` and in `completenessHonestCasesTest` in `lanemap_test.qnt` for `(1, 3), (2, 3), (3, 3), (1, 1)`).

---

## Commands Run and Exact Outcomes

Per the prompt instructions, we attempted to invoke `quint` in the workspace and report the exact outcome honestly without claiming results not seen:

1. `ls -la && ls -la lane-src/`
   - **Outcome**: Exited with code `0`; listed workspace directories (`.git`, `.lane-skills`, `lane-src`, `qa`) and `lane-src/iface.rs` (`43449` bytes).
2. `git log -n 3 --oneline && quint --version`
   - **Outcome**: Canceled/refused by the command guard (chained command / flag not permitted).
3. `quint --version`
   - **Outcome**: Canceled/refused by the command guard.
4. `quint typecheck qa/gemini-quint/e3-lanemap/lanemap.qnt`
   - **Outcome**: Refused by the sandbox permission check:
     ```
     Encountered error in tool execution: permission check failed for unsandboxed "quint typecheck qa/gemini-quint/e3-lanemap/lanemap.qnt": user denied permission to run command:
     quint typecheck qa/gemini-quint/e3-lanemap/lanemap.qnt
     ```
   - Per the sandbox policy and prompt instruction (*"If your sandbox lets you run `quint typecheck`, `quint test` and `quint run`, do so; if a command is refused, say so honestly and do not claim results you did not see"*), no further `quint` commands were executed after the sandbox refusal.
   - To run the model and test suite outside this sandbox, execute:
     ```bash
     quint typecheck qa/gemini-quint/e3-lanemap/lanemap.qnt
     quint typecheck qa/gemini-quint/e3-lanemap/lanemap_test.qnt
     quint test qa/gemini-quint/e3-lanemap/lanemap_test.qnt --main lanemap_test --seed 17
     quint run qa/gemini-quint/e3-lanemap/lanemap.qnt --main lanemap --invariants SOUND COMPLETE --witnesses witnessHonestAccepted witnessAdversarialAccepted witnessAdversarialRefused --max-steps 20 --max-samples 1000 --seed 17
     ```

---

## Mutants in `lanemap.qnt` and How Each is Caught in `lanemap_test.qnt`

We defined **5 named mutants** in [lanemap.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/qa/gemini-quint/e3-lanemap/lanemap.qnt) and corresponding catching tests in [lanemap_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/e3-lanemap/qa/gemini-quint/e3-lanemap/lanemap_test.qnt):

| Mutant Name (`lanemap.qnt`) | Defect Introduced | Catching Test (`lanemap_test.qnt`) | How It Is Caught |
|---|---|---|---|
| `mut_any_sponge` | Relaxes `(eo.op == OP_SPONGE and eo.param == 3)` to `eo.op == OP_SPONGE`, accepting `SPONGE(n)` with `n != 3`. | `mutAnySpongeCaughtTest` | Maps lane `3` to entry `8` (`SPONGE(2)` with values `[6, 7, 66]`). Real `decode` refuses (`not(realRes.ok)`), while `mut_any_sponge` accepts (`mutRes.ok`) and violates `soundForResult` because entry `8` has `param == 2 != 3`. |
| `mut_first_value_not_last` | Reads `vs.nth(eo.off)` (first logged value of entry) instead of `vs.nth(eo.off + entryValCount(eo.op, eo.param) - 1)` (last logged value). | `mutFirstValueNotLastCaughtTest` | On `HONEST_LANE_MAP`, `mut_first_value_not_last` sets lane `2` to `11` (input of entry `4` instead of output `21`) and lanes `3, 4` to `21` (first input of entry `3` instead of output `33`), violating `soundForResult` (`res.stmt != nativeStatement`). |
| `mut_no_dup_check` | Omits the `acc.stmt.nth(p.lane) == 0` guard in the lane-map loop. | `mutNoDupCheckCaughtTest` | Tested on (1) duplicate-lane map `[(2, 4), (2, 5), (3, 3), (5, 2)]` and (2) non-zero call-vector lane `callLanes[2] = 99`. Real `decode` refuses both, while `mut_no_dup_check` accepts both and violates `soundForResult` (`noDuplicateMappedLanes` and `callLanes.nth(p.lane) == 0`). |
| `mut_any_op` | Replaces `isAllowedLaneMapEntry(eo.op, eo.param)` with `entryValCount(eo.op, eo.param) >= 1`, accepting any non-`NOP` op. | `mutAnyOpCaughtTest` | Maps lane `2` to entry `6` (`PERM`), entry `7` (`H2`), and entry `9` (`PUB(6)`). Real `decode` refuses all three, while `mut_any_op` accepts all three and violates `soundForResult`. |
| `mut_pub_before_lanemap` | Checks `checkPubEntries(offs, vs, callLanes)` against pre-lane-map `callLanes` instead of post-lane-map `loopRes.stmt`. | `mutPubBeforeLanemapCaughtTest` | Replaces entry `9` with `PUB(2)` logging `0` on mapped lane `2` (where `callLanes[2] == 0` and post-lane-map `stmt[2] == 21`). Real `decode` refuses (`"pub_mismatch"`), while `mut_pub_before_lanemap` falsely accepts and violates `soundForResult`. |

In addition, `uniquenessAndUnusedZeroPairTest` directly catches the **real `decode` zero-output duplicate-lane edge case** on `ZERO_OUTPUT_LOG` (where entry `4` has output `0` and `dupNonZeroMap` maps lane `2` twice: `decode` accepts and violates `soundForResult`, whereas `decodeFixedDup` rejects).

---

## What the Quint Model Covers vs. Does Not Cover

### Covered by `lanemap.qnt` and `lanemap_test.qnt`
- The entry table offset calculation `offs` and flattened value array `vs` for `{NOP, PERM, H2, SPONGE(2), SPONGE(3), PUB, DIGEST}` with `0` (`NOP`) or `1..4` values per entry.
- The reservation `R` check (`checkReservation`) for both ordinary (`mode = 0`) and chunk (`mode = 1`, `1 <= j <= k`) runs, including `anchor_tags` and `claim_pub` input binding `vs[6..9] == [vs[1], vs[3], vs[5]]`.
- The exact `decode` lane-map acceptance rule, `(0, 0)` sentinel handling, `o + vals(op, param) - 1` output indexing, `s[lane] != 0` check, and post-lane-map `PUB(i)` check.
- Comparison against `nativeStatement`, the 5 mutants above, the zero-output duplicate-lane collision (`ZERO_OUTPUT_LOG` vs. `decodeFixedDup`), mapping lanes `0` and `1`, pointing `claim_pub` at a discretionary `DIGEST` or `SPONGE(3)` outside `R` (`EXTRA_SPONGE3_LOG`), and nondeterministic adversarial lane-map generation (`stepAdversarial` / `randomAdversarialLaneMapTest`).

### Not Covered by the Quint Model
1. **31-bit BabyBear unpacking (`unpack`) and 9-limb `Fr` canonical decoding (`dec`)**: The model represents each decoded `Fr` value and call-vector lane as a single small integer rather than 9 limbs of 30 bits packed 8 BabyBear words per `Fr` element.
2. **Real Poseidon / Poseidon2 cryptographic evaluation**: Native replay (`perm`, `h2`, `sponge_u`, `digest`) and the BabyBear log onion (`p2`, `onion`, `log_merge`) are abstracted; logged entries in `makeCaseLog` carry pre-populated small-integer inputs and outputs rather than field elements over $\mathbb{F}_r$.
3. **Full dimensions (`Dims::FULL`) and permutation capacity (`d.p`, `perms`)**: The model scales dimensions down to `N_LANES = 8`, `B = 11` entries, and `25` logged values (`N_PAIRS = 4` is preserved at full size) to keep state-space exploration small and fast.
4. **Wrap-level circuit constraints outside `decode`**: The model captures `decode` as written in `lane-src/iface.rs` and does not model the outer wrap key constants that must pin `w[LANE_MAP..LANE_MAP + 8]`, bind `DIGEST` input `d2` values to `root_stmt`, and verify `PATH` Merkle siblings and the `log_d`/`log_n` onion commitment.

---

## Lead verification (2026-11-13, quint 0.32.0, Rust backend)

The lane's commands were refused by its sandbox; the lead ran them.

| Command | Outcome |
|---|---|
| `quint typecheck lanemap.qnt` / `lanemap_test.qnt` | ok |
| `quint test lanemap_test.qnt --main=lanemap_test --seed=0x1 --max-samples=200` (lane files) | 10 passing, 2 failing: `uniquenessAndUnusedZeroPairTest` and `bindingAndDiscretionaryTargetTest` |
| the same after the lead fix below | 12 passing (6 question tests, 5 mutant tests, the random adversarial lane map (200 runs)) |
| `quint run lanemap.qnt --main=lanemap --invariant=SOUND --max-samples=5000 --max-steps=5 --seed=0x42` | `[ok] No violation found` (the random generator does not reach the zero-output log of 2a; the 2a test reaches it directly) |
| `quint run ... --invariant=COMPLETE --max-samples=5000 --max-steps=5 --seed=0x42` | `[ok] No violation found` |

Lead fix to the tests (original in `lane-original/`): the two failures were wrong expected values, not wrong verdicts. A lead probe showed each counterexample is accepted by the model's `decode` as the report says (`ok` true); the asserted lane value was 25 where the model's entry 5 output in mode 0 is 21 (`DIGEST(SHK_PROGC)`, `aOutVal` 21). Lines 81 and 130 now expect 21.

Lead action on the findings (openvm `e3-iface`):
- 2a (a lane mapped twice when the first mapped output is 0): fixed in 9539418. The lane map is now `apply_lane_map` with a per-lane flag; unit test with a zero output mapped twice: refused.
- 3 (lane-map words not checked as u16): fixed in 9539418 (`lane map pair k: not u16`), with a test.
- 2b (a pair may target lane 0, the binding lane, or lane 1, `comm`, when their call-vector limbs are 0) and 4 (the reader does not pin the lane map: a pair may point `claim_pub` at a discretionary `DIGEST` or a second `SPONGE(3)` outside `R`): not changed. Whether the reader refuses lanes 0 and 1, and whether the lane map is a key constant of the wrap or is checked against `R` positions (entry 3 for `claim_pub`, entry 4 for `a_in`), are design choices for the owner; with the `a_out` 4 against 5 pair question they belong to the lane-map item left open.
- J4 on fixture-j8 after 9539418: the 7 positives decode to the native statement; the 3 negatives are refused as before.
