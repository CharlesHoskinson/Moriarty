# Soundness & Completeness Review: `HeaderOpenAir` (Experiment X10, FP-5)

## 1. Verdict

**Verdict: SOUND and COMPLETE (with conditions)**

`HeaderOpenAir` (`lane-src/x_exp_node.rs:75-145`) is **sound and complete** as a standalone STARK AIR proving the 42-word overwrite-mode sponge opening
$$\text{post\_d} = \text{PaddingFreeSponge}_{P, 16, 8, 8}(\text{SHK\_SD2}, \text{outcome\_tag}, \text{outcome\_payload}, \text{input\_d}, \text{deps\_d}, \text{view\_root}, \text{body\_d})$$
subject to the following conditions:

1. **Cryptographic & Sub-AIR Premises (Premise A5 / `header_open_sound`):**
   - `Poseidon2SubAir<BabyBear, 1>` soundly constrains `row[r].post == P(row[r].inputs)` on all 8 rows (`r in 0..8`).
   - The 16-word Poseidon2 permutation $P$ acts as a collision-resistant / injective permutation such that `PaddingFreeSponge<P, 16, 8, 8>` is collision-free on 42-word inputs prefixed by `SHK_SD2`.
2. **Enclosing Node Integration Conditions (Not in `HeaderOpenAir` itself):**
   - **Public-value linkage:** The enclosing recursive node (`VerifierSubCircuit` + `NODE INTEGRATION`) must equate `HeaderOpenAir`'s 8 public values `pvs[0..8]` with the verified child segment's authenticated `post_d` lane (`SEG` payload words `16..24`, envelope words `17..25`). In `lane-src/x_exp_node.rs:348-350`, `HeaderOpenAir` is benchmarked as an unlinked extra AIR whose `pvs` are supplied independently by the host prover.
   - **Opened-word port & SEAL predicates:** `HeaderOpenAir` has `num_interactions = 0` (`lane-src/x_exp_node.rs:94, 104-144`). A production `SEAL` / `SEAL-WITH-DEPS` node must export the 42 opened header words (`outcome_tag`, `outcome_payload`, `input_d`, `deps_d`, `view_root`, `body_d`) from rows `0..5` via a `PermutationCheckBus` port (or shared columns) and enforce the seal predicates (Section 5 below).
3. **Selector-Gated Extraction & One-Hotness:**
   - When extracting the opened header words via the row selectors $s[0..6]$ (i.e., $\sum_{r=0}^7 s[r][b] \cdot \text{row}[r].\text{inputs}[k]$ or bus interactions gated by $s[b]$), the one-hot sum constraint $\sum_{i=0}^7 s[r][i] = 1$ (`lane-src/x_exp_node.rs:119`) is strictly required so that padding rows 6 and 7 cannot activate $s[0]$ or $s[1]$ a second time.

> **Sandbox Execution Note:** In this review environment, shell command execution (`run_command` for `quint typecheck`, `quint test`, and `quint run`) was refused by the sandbox permission guard. No runtime command outputs are claimed as observed. The Quint model (`qa/gemini-quint/x10-header/header_open.qnt`) and test harness (`qa/gemini-quint/x10-header/header_open_test.qnt`) were statically verified against the repository's Quint 0.32.0 conventions (`models/sherlock/quint/aggregation.qnt`), and the exact commands with their expected exit codes are tabulated in Section 3.

---

## 2. Circuit & Refinement Analysis (`invOpen`)

### 2.1 Reference Sponge vs. AIR Trace Layout

The reference hash `sdk_digest` (`lane-src/x_exp_node.rs:195-198`, `lane-src/padding_free_sponge.rs:41-78`) runs `PaddingFreeSponge<P, WIDTH=16, RATE=8, OUT=8>` over $HW = 42$ words:
- Starts from the all-zero state $\text{state}^{(0)} = [0^{16}]$.
- For each full 8-word block $b \in \{0, 1, 2, 3, 4\}$ (words $8b \dots 8b+7$), overwrites the rate half $\text{state}[0..8]$ with block $b$, retains the capacity half $\text{state}[8..16]$, and applies $\text{state}^{(b+1)} = P(\text{state})$.
- For the final partial block $b = 5$ of length $\text{tail} = 42 - 5 \times 8 = 2$ (words $40, 41$), overwrites only $\text{state}[0..2]$ with $[w[40], w[41]]$, **retains both the remaining rate words $\text{state}[2..8]$ and the capacity words $\text{state}[8..16]$** from $\text{state}^{(5)}$, and applies $\text{state}^{(6)} = P(\text{state})$ (`lane-src/padding_free_sponge.rs:56-68`, since $i = 2 \neq 0$).
- Outputs $\text{state}^{(6)}[0..8]$.

The 42 header words are laid out across rows $0..5$ as follows (`ARCHITECTURE.md` §1.11 FP-5, `lane-src/x_exp_node.rs:152-164`):

| Header Field | Word Indices | AIR Row & Input Columns | Width (words) |
|---|---|---|---|
| `SHK_SD2` (domain tag) | `w[0]` | `row[0].inputs[0]` | 1 |
| `outcome_tag` | `w[1]` | `row[0].inputs[1]` | 1 |
| `outcome_payload` | `w[2..10]` | `row[0].inputs[2..8] ++ row[1].inputs[0..2]` | 8 |
| `input_d` | `w[10..18]` | `row[1].inputs[2..8] ++ row[2].inputs[0..2]` | 8 |
| `deps_d` | `w[18..26]` | `row[2].inputs[2..8] ++ row[3].inputs[0..2]` | 8 |
| `view_root` | `w[26..34]` | `row[3].inputs[2..8] ++ row[4].inputs[0..2]` | 8 |
| `body_d` | `w[34..42]` | `row[4].inputs[2..8] ++ row[5].inputs[0..2]` | 8 |

### 2.2 Soundness Proof of `invOpen`

Let `(tr, pv)` be any 8-row trace and 8-word public value vector accepted by `HeaderOpenAir::eval`:

1. **Unique Identity Selector Matrix ($s = I_8$):**
   - By `assert_bool(s)` and `assert_one(sum)` (`x_exp_node.rs:115-119`), every row $r \in 0..8$ has $s[r][i] \in \{0, 1\}$ with $\sum_{i=0}^7 s[r][i] = 1$, so each row $r$ is a standard basis vector $e_{c(r)}$.
   - On row $0$, `when_first_row().assert_one(ls[0])` (`x_exp_node.rs:120`) forces $s[0][0] = 1$, hence $s[0] = e_0 = [1,0,0,0,0,0,0,0]$.
   - On each transition $r \to r+1$ ($r \in 0..7$), `when_transition().assert_eq(ns[i + 1], ls[i])` for $i \in 0..7$ (`x_exp_node.rs:121-123`) forces $s[r+1][i+1] = s[r][i]$. By induction on $r$, $s[r][r] = 1$, and since $\sum_{i=0}^7 s[r][i] = 1$ with boolean entries, $s[r][i] = \delta_{r,i}$ for all $r, i \in 0..8$.
   - Consequently, selector-projected extraction $w_{b,k} = \sum_{r=0}^7 s[r][b] \cdot \text{row}[r].\text{inputs}[k]$ equals $\text{row}[b].\text{inputs}[k]$ for all $b \in 0..6$.

2. **Row 0 Domain and Initial Capacity (`s[0] == 1`):**
   - `when(ls[0]).assert_eq(lp.inputs[0], SHK_SD2)` (`x_exp_node.rs:125`) forces $w[0] = \text{row}[0].\text{inputs}[0] = \text{SHK\_SD2}$.
   - `when(ls[0]).assert_zero(lp.inputs[k])` for $k \in 8..16$ (`x_exp_node.rs:126-128`) forces $\text{row}[0].\text{inputs}[8..16] = [0^8]$, matching the initial sponge state $\text{state}^{(0)} = [0^{16}]$ after overwriting rate words $0..8$ with $w[0..8]$.

3. **Full-Block Capacity Chaining on Rows $0 \to 1 \dots 4 \to 5$ (`s[0..5]`):**
   - On every row $r$, `self.p2.eval(&mut sub)` (`x_exp_node.rs:105-106`) forces $\text{row}[r].\text{post} = P(\text{row}[r].\text{inputs})$.
   - For $i \in 0..5$ and $k \in 8..16$, `when_transition().when(ls[i]).assert_eq(np.inputs[k], lpost[k])` (`x_exp_node.rs:130-134`) forces $\text{row}[i+1].\text{inputs}[8..16] = \text{row}[i].\text{post}[8..16]$.
   - By induction across rows $0..4$, $\text{row}[r].\text{post} = \text{state}^{(r+1)}$ of `PaddingFreeSponge` after absorbing $w[0..8(r+1)]$.

4. **Partial Last Block Overwrite on Row $4 \to 5$ (`s[4] == 1`):**
   - For $k \in 2..8$, `when_transition().when(ls[4]).assert_eq(np.inputs[k], lpost[k])` (`x_exp_node.rs:136-139`) forces $\text{row}[5].\text{inputs}[2..8] = \text{row}[4].\text{post}[2..8]$.
   - Together with $\text{row}[5].\text{inputs}[8..16] = \text{row}[4].\text{post}[8..16]$ from step 3 and $\text{row}[5].\text{inputs}[0..2] = w[40..42]$, $\text{row}[5].\text{inputs}$ equals `overwritePrefix(state^(5), w[40..42], 2)`.
   - Thus $\text{row}[5].\text{post} = P(\text{row}[5].\text{inputs}) = \text{state}^{(6)}$.

5. **Output Binding on Row 5 (`s[5] == 1`) and Uniqueness:**
   - For $k \in 0..8$, `when(ls[5]).assert_eq(lpost[k], pvs[k])` (`x_exp_node.rs:141-143`) forces $\text{pv}[0..8] = \text{row}[5].\text{post}[0..8] = \text{state}^{(6)}[0..8] = \text{sponge}(w)$.
   - Therefore $\text{sponge}(w) = \text{pv}$ and $w[0] = \text{SHK\_SD2}$ (`invOpen` holds).
   - Because $P$ is injective on 16-word states (modeled in `header_open.qnt` by an algebraically proven injective linear map on $\mathbb{Z}^{16}$ and in Sherlock by premise A5 `P2Inj`), no two distinct 42-word headers $w \neq w'$ can produce the same $\text{pv}$ without a sponge collision; hence $w$ is the unique header committed by $\text{pv}$ (`header_open_sound`).

---

## 3. Mutant Analysis & Verification Table

### 3.1 Technical Findings on Mutants M1–M6

- **M1 `no_capacity_chain` (`hdr_no_capacity_chain`):**
  Dropping capacity chaining on transition $0 \to 1$ (`i = 0`) leaves `row[1].inputs[8..16]` unconstrained. The prover modifies `row[1].inputs[8]` by $+1$ and recomputes rows $1..5$ honestly, setting `pv = row[5].post[0..8]`. Because `row[1].inputs[8]` is a capacity word (not part of the 42 opened rate words $w$), `openedWords(tr)` remains the original $w$, while `pv != sponge(w)`. Caught by `invOpen`.
- **M2 `no_partial_keep` (`hdr_no_partial_keep`):**
  Dropping `when_transition().when(ls[4]).assert_eq(np.inputs[k], lpost[k])` for $k \in 2..8$ leaves the unabsorbed rate words `row[5].inputs[2..8]` of the partial block free. The prover modifies `row[5].inputs[2]` by $+1$ and recomputes `row[5].post` and `pv = row[5].post[0..8]`. The 42 opened words only include `row[5].inputs[0..2]`, so `sponge(w)` uses `row[4].post[2..8]` and differs from `pv`. Caught by `invOpen`.
- **M3 `no_domain` (`hdr_no_domain`):**
  Dropping `when(ls[0]).assert_eq(lp.inputs[0], SHK_SD2)` allows the prover to set `row[0].inputs[0] = SHK_SD2 + 1` and recompute rows $0..5$ honestly. The trace is accepted with $w[0] \neq \text{SHK\_SD2}$, breaking domain separation (`ARCHITECTURE.md` §1.11 FP-4). Caught by `invOpen`.
- **M4 `output_on_row7` (`hdr_output_on_row7`):**
  Checking `pvs[0..8]` against `lpost[0..8]` when `ls[7] == 1` (row 7, padding) instead of `ls[5] == 1` (row 5) completely decouples `pv` from the 6 sponge permutations on rows $0..5$. An honest trace on rows $0..5$ with padding row 7 set to `[0; 16]` and `pv = row[7].post[0..8]` is accepted while `pv = P([0; 16])[0..8] != sponge(w)`. Caught by `invOpen`.
- **M5 `selector_not_onehot` (`hdr_selector_not_onehot`):**
  *Key Structural Finding:* In `HeaderOpenAir::eval`, `when_first_row().assert_one(ls[0])` and `when_transition().assert_eq(ns[i + 1], ls[i])` already force the main diagonal $s[r][r] = 1$ for all $r \in 0..8$, even without `assert_one(sum)`. However, the shift constraint `ns[i + 1] == ls[i]` only constrains columns $1..7$ of the next row and **never constrains column $0$ (`ns[0]`) on rows $r \ge 1$**. When `assert_one(sum)` is dropped, a malicious prover can set $s[6][0] = 1$ on padding row 6 (which shifts to $s[7][1] = 1$ on padding row 7), while setting `row[6].inputs = [SHK_SD2, 0, ..., 0]` and `row[7].inputs[8..16] = row[6].post[8..16]` on the free padding rows so that every `when(ls[0])` and `when_transition().when(ls[0])` constraint is satisfied (`when_transition()` is false on row 7, so $s[7][1] = 1$ triggers no constraint at all). Consequently, selector $s[0]$ is active on both row 0 and row 6, and selector $s[1]$ is active on both row 1 and row 7, corrupting selector-gated extraction (`openedWords(tr)[0] = 2 * SHK_SD2 != SHK_SD2` and `sponge(openedWords(tr)) != pv`). Caught by `invOpen` and `invSelectorIdentity`.
- **M6 `first_capacity_free` (`hdr_first_capacity_free`):**
  Dropping `when(ls[0]).assert_zero(lp.inputs[k])` for $k \in 8..16$ allows the prover to set `row[0].inputs[8] = 1` (non-zero initial capacity) and recompute rows $0..5$ honestly with `pv = row[5].post[0..8]`. Because `sponge(w)` initializes capacity to $0$, `sponge(w) != pv`. Caught by `invOpen`.

### 3.2 Exact Quint Commands & Expected Exit Codes

| Check / Target | Module | Invariant / Suite | Exact Quint Command | Expected Exit Code | Expected Outcome |
|---|---|---|---|---:|---|
| Typecheck model | — | — | `quint typecheck qa/gemini-quint/x10-header/header_open.qnt` | `0` | Typecheck succeeds |
| Typecheck tests | — | — | `quint typecheck qa/gemini-quint/x10-header/header_open_test.qnt` | `0` | Typecheck succeeds |
| Unit & mutant test suite | `header_open_test` | 13 `run` tests | `quint test qa/gemini-quint/x10-header/header_open_test.qnt --main=header_open_test --max-samples=100 --seed=0x10` | `0` | All 13 tests pass |
| Soundness (`invOpen`) | `hdr_sound` | `invOpen` | `quint run qa/gemini-quint/x10-header/header_open.qnt --main=hdr_sound --invariant=invOpen --max-steps=10 --max-samples=200 --seed=0x10` | `0` | `[ok] No violation found` |
| Non-vacuity 1 (honest opening reached) | `hdr_sound` | `invNoAcceptedHonest` | `quint run qa/gemini-quint/x10-header/header_open.qnt --main=hdr_sound --invariant=invNoAcceptedHonest --max-steps=10 --max-samples=200 --seed=0x10` | `1` | `[violation]` (witnesses accepted honest opening) |
| Non-vacuity 2 (zero-payload opening reached) | `hdr_sound` | `invNoAcceptedZeroPayload` | `quint run qa/gemini-quint/x10-header/header_open.qnt --main=hdr_sound --invariant=invNoAcceptedZeroPayload --max-steps=10 --max-samples=200 --seed=0x10` | `1` | `[violation]` (witnesses accepted zero-payload opening) |
| Mutant M1 (`no_capacity_chain`) | `hdr_no_capacity_chain` | `invOpen` | `quint run qa/gemini-quint/x10-header/header_open.qnt --main=hdr_no_capacity_chain --invariant=invOpen --max-steps=10 --max-samples=200 --seed=0x10` | `1` | `[violation]` (`invOpen` violated; mutant caught) |
| Mutant M2 (`no_partial_keep`) | `hdr_no_partial_keep` | `invOpen` | `quint run qa/gemini-quint/x10-header/header_open.qnt --main=hdr_no_partial_keep --invariant=invOpen --max-steps=10 --max-samples=200 --seed=0x10` | `1` | `[violation]` (`invOpen` violated; mutant caught) |
| Mutant M3 (`no_domain`) | `hdr_no_domain` | `invOpen` | `quint run qa/gemini-quint/x10-header/header_open.qnt --main=hdr_no_domain --invariant=invOpen --max-steps=10 --max-samples=200 --seed=0x10` | `1` | `[violation]` (`invOpen` violated; mutant caught) |
| Mutant M4 (`output_on_row7`) | `hdr_output_on_row7` | `invOpen` | `quint run qa/gemini-quint/x10-header/header_open.qnt --main=hdr_output_on_row7 --invariant=invOpen --max-steps=10 --max-samples=200 --seed=0x10` | `1` | `[violation]` (`invOpen` violated; mutant caught) |
| Mutant M5 (`selector_not_onehot`) | `hdr_selector_not_onehot` | `invOpen` | `quint run qa/gemini-quint/x10-header/header_open.qnt --main=hdr_selector_not_onehot --invariant=invOpen --max-steps=10 --max-samples=200 --seed=0x10` | `1` | `[violation]` (`invOpen` violated; mutant caught) |
| Mutant M6 (`first_capacity_free`) | `hdr_first_capacity_free` | `invOpen` | `quint run qa/gemini-quint/x10-header/header_open.qnt --main=hdr_first_capacity_free --invariant=invOpen --max-steps=10 --max-samples=200 --seed=0x10` | `1` | `[violation]` (`invOpen` violated; mutant caught) |

---

## 4. Padding Rows & Completeness

### 4.1 Are padding rows 6 and 7 free, and can they change the opened words?

- **Are padding rows 6 and 7 free?**
  **Yes, their 16 input words `row[6].inputs[0..16]` and `row[7].inputs[0..16]` are completely free**, subject only to `Poseidon2SubAir` (`row[6].post == P(row[6].inputs)` and `row[7].post == P(row[7].inputs)`) and the fixed selector columns `s[6] = e_6`, `s[7] = e_7`.
  In `HeaderOpenAir::eval` (`lane-src/x_exp_node.rs:129-143`):
  - Capacity chaining loops over `i in 0..NPERM - 1` (`0..5`), which gates transitions on `ls[0]..ls[4]` (linking rows `0 -> 1`, `1 -> 2`, `2 -> 3`, `3 -> 4`, `4 -> 5`). It does **not** gate `ls[5]` (`5 -> 6`) or `ls[6]` (`6 -> 7`).
  - Partial-block overwrite gates on `ls[NPERM - 2] = ls[4]` (`4 -> 5`).
  - Output equality gates on `ls[NPERM - 1] = ls[5]` (row 5).
  - Neither `ls[6]` nor `ls[7]` appears in any sponge constraint. In `header_trace` (`x_exp_node.rs:179-181`), the honest prover pads `inputs[6]` and `inputs[7]` with `[F::ZERO; 16]`, but any 16 field elements per padding row with valid Poseidon2 round columns satisfy the AIR.
- **Can they change the opened words?**
  **No.** In the sound circuit, the selector constraints force $s[6] = [0,0,0,0,0,0,1,0]$ and $s[7] = [0,0,0,0,0,0,0,1]$. Both rows have $s[r][0..6] = 0$, and no transition constraint flows backward from rows $6..7$ into rows $0..5$. Therefore, varying `row[6].inputs` and `row[7].inputs` has zero effect on `openedWords(tr)` or `pv` (verified by `paddingRowsFreeAndPreserveOpeningTest` in `header_open_test.qnt`).

### 4.2 Is the opening complete (every honest header accepted, including all-zero digests and a zero payload)?

**Yes.** For every 42-word BabyBear array $w \in \mathbb{F}^{42}$ satisfying $w[0] = \text{SHK\_SD2}$, `header_trace(&w)` (`x_exp_node.rs:167-192`) constructs an 8-row matrix and 8-word digest `d = state[0..8]` after 6 permutations that satisfies every constraint of `HeaderOpenAir::eval` and equals `sdk_digest(&w)`:
- `HeaderOpenAir` places **no non-zero or range constraints** on $w[1..42]$.
- **Zero payload (`w[2..10] = [0; 8]`):** Occurs when `outcome_tag` has no value digest payload or wraps a zero-padded code/kind (`ARCHITECTURE.md` §1.11 FP-4, FP-5; `x_exp_node.rs:158-161, 402`). Accepted and matches `sdk_digest` (`honestZeroPayloadAcceptedTest` and negated reach invariant `invNoAcceptedZeroPayload`).
- **All-zero digests after domain (`w[1..42] = [0; 41]`):** Tested by `"all_zero_after_domain"` in `x_exp_node.rs:410-414` and `honestAllZeroAfterDomainAcceptedTest` in `header_open_test.qnt`. Because `PaddingFreeSponge` in `padding_free_sponge.rs:54-74` counts absorbed input elements via `input.next()` rather than inspecting whether field elements are zero, absorbing 41 zeros after `SHK_SD2` still executes all 6 permutations (`i = 2 != 0` on the final 2-word block) and produces the exact same digest in both `HeaderOpenAir` and `PaddingFreeSponge`.

---

## 5. What Part of the Seal Is NOT in `HeaderOpenAir`, and What a Node Must Add

In `lane-src/x_exp_node.rs:333-350`, `HeaderOpenAir` is appended as a standalone AIR to `VerifierSubCircuit<N>::airs()` with `num_interactions = 0`. It proves only that its own 8 public values `pvs[0..8]` are the `PaddingFreeSponge` digest of *some* 42-word preimage starting with `SHK_SD2`. It does **not** connect either end of that opening to the rest of the node proof.

To realize a sound `SEAL` or `SEAL-WITH-DEPS` node (`ARCHITECTURE.md` §1.11 FP-2, FP-4, FP-5; §2 kits `BOUNDARY COMMITMENT` and `NODE INTEGRATION`), the node circuit must add:

1. **Link `HeaderOpenAir`'s `pv` (`post_d`) to the Verified Child's Authenticated `post_d` Lane:**
   - The recursive verifier subcircuit authenticates child slot 0's `SEG` statement `(prog_d, pre_d, post_d, h_lo, h_hi)` (`FP-2` payload offsets `0, 8, 16, 24, 28`).
   - The node must constrain `HeaderOpenAir`'s output digest (`lpost[0..8]` on row 5 where `s[5] == 1`, or its 8 public values `pvs[0..8]`) to equal the 8 words of child slot 0's authenticated `post_d`. Without this equality, a prover can run `HeaderOpenAir` on an arbitrary fabricated header $w'$ and supply $\text{sponge}(w')$ as `HeaderOpenAir`'s `pvs`, completely unlinked to the child execution segment.

2. **Export the 42 Opened Header Words (`StateOpeningCells`) via a Port / Bus:**
   - `HeaderOpenAir` holds the 42 header words in `row[0..4].inputs[0..8]` and `row[5].inputs[0..2]`, which are private main-trace columns.
   - The node must send these words (gated by selectors `s[0]..s[5]`) over a `PermutationCheckBus` port (`port_send` / `port_recv` of interface `I-8` / `I-11`) or share columns with the SEAL AIR to expose the typed `StateOpeningCells`:
     - `outcome_tag = w[1]` (`row[0].inputs[1]`)
     - `outcome_payload = w[2..10]` (`row[0].inputs[2..8] ++ row[1].inputs[0..2]`)
     - `input_d = w[10..18]` (`row[1].inputs[2..8] ++ row[2].inputs[0..2]`)
     - `deps_d = w[18..26]` (`row[2].inputs[2..8] ++ row[3].inputs[0..2]`)
     - `view_root = w[26..34]` (`row[3].inputs[2..8] ++ row[4].inputs[0..2]`)
     - `body_d = w[34..42]` (`row[4].inputs[2..8] ++ row[5].inputs[0..2]`)

3. **Enforce the SEAL / SEAL-WITH-DEPS Predicates on the Opened Words (`NODE INTEGRATION`):**
   - **Genesis & Terminal checks (SRC-006):** Assert `seg.h_lo == 0` and assert that the opened `outcome_tag` is a terminal tag (`Halted`, `Rejected`, or `Exhausted` — never `Running`).
   - **Dependency commitment check:**
     - In `SEAL`: assert that the opened `deps_d == EMPTY_DEPS` (`P2(SHK_DEPS)`).
     - In `SEAL-WITH-DEPS`: assert that slot 1's `DL.chain_d ==` opened `deps_d` (and verify `DL.in_link` when set).
   - **Program Admission (`ADM`) and Certified Input View (`VIEW`, FP-5 / AD-17(b)):**
     - Re-hash a witnessed `input_d` preimage `(SHK_IN, salt, val_d, fuel_lo, fuel_hi, stack_limit)` and assert equality with the opened `input_d` to authenticate `val_d`.
     - Open slot 1's `prem_root`: if `val_d == VAL_NONE` (absent input), require `prem_root == ADM(P2(SHK_ADM, 1, seg.prog_d))` and `opened.view_root == VIEW_NONE`; otherwise require `prem_root == JOIN(ADM(adm_d), VIEW(vw_d))` where `adm_d = P2(SHK_ADM, 1, seg.prog_d)` and `vw_d = P2(SHK_VIEW, 1, val_d, opened.view_root)` — binding the opened `view_root` header digest to the certified `VIEW` claim.
   - **Claim Digest, SET Head Construction, and Physical State Erasure:**
     - Compute `outcome_d = P2(SHK_OUTCOME_V2, 2, opened.outcome_tag, opened.outcome_payload)`.
     - Compute `claim_d = P2(SHK_RUN, 1, seg.prog_d, opened.input_d, outcome_d)`.
     - Compute `set_root = P2(SHK_SET_LEAF, claim_d, seg.h_hi)` (for `SEAL`, with `n_steps = seg.h_hi` range-checked `< 2^64`) or `P2(SHK_SET_DEP, claim_d, seg.h_hi, DL.cover_root, DL.in_link)` (for `SEAL-WITH-DEPS`).
     - Publish `stmt_tag = SET` with `set_root` in the node's 74-word public statement envelope while dropping `pre_d`, `post_d`, and `body_d` so no physical memory/register state leaks into the sealed claim.

---

## Lead results (2026-11-12 18:45, quint 0.32.0, lead run outside the lane sandbox)

The lane could not run commands. The lead ran every command of section 3.2 in the lane worktree (`~/.cache/gemini-lanes/x10-header`). All 12 exit codes match the expected column:

| Command | rc | Outcome |
|---|---:|---|
| typecheck header_open.qnt | 0 | ok |
| typecheck header_open_test.qnt | 0 | ok |
| quint test header_open_test (13 tests, seed 0x10) | 0 | all passed |
| hdr_sound invOpen | 0 | No violation found |
| hdr_sound invNoAcceptedHonest (non-vacuity) | 1 | violation (reach witnessed) |
| hdr_sound invNoAcceptedZeroPayload (non-vacuity) | 1 | violation (reach witnessed) |
| M1 hdr_no_capacity_chain invOpen | 1 | caught |
| M2 hdr_no_partial_keep invOpen | 1 | caught |
| M3 hdr_no_domain invOpen | 1 | caught |
| M4 hdr_output_on_row7 invOpen | 1 | caught |
| M5 hdr_selector_not_onehot invOpen | 1 | caught |
| M6 hdr_first_capacity_free invOpen | 1 | caught |

Note: after the lane read `lane-src/x_exp_node.rs`, the lead changed only ownership in `HeaderOpenAir::eval` (`.clone()` on non-Copy `AB::Var`, compile fix). No constraint changed.
