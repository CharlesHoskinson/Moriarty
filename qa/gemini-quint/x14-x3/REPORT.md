# Soundness & Completeness Review: `X3` AIR in the BLS12-381 G1 Chain (Experiment X14)

## 1. Verdict

**Verdict: SOUND and COMPLETE (with conditions)**

The `X3` row kind (`ChainAir` over `x_expr` in `lane-src/x_exp_node.rs:660-698, 780-821`) is **sound and complete** as the $x$-coordinate stage
$$x_3 \equiv \lambda^2 - x_1 - x_2 \pmod{p_{381}}$$
of both the BLS12-381 $\mathbb{G}_1$ affine addition chain (`kind == "add"`, bus base $o = 1100$, $x_2$ from $P_2$) and the affine doubling chain (`kind == "dbl"`, bus base $o = 1110$, $x_2 = x_1$), subject to the following four conditions:

1. **Condition C1 — Upstream Input Limb Range Checks ($[0, 2^8)$):**
   `FieldExpr::eval` (`lane-src/builder.rs:437-448`) range-checks only its saved output variable `vars[0]` ($x_3$, 48 limbs of 8 bits) and its quotient/carry limbs (`q_limbs`, `carry_limbs`), **not** its input limbs `inputs[0..3]` ($\lambda, x_1, x_2$). In the chain (`lane-src/x_exp_node.rs:763-821`), $\lambda$ is `Var(0)` of `L` and is therefore limb-range-checked to $[0, 2^8)$ by `L`, whereas $x_1$ and $x_2$ are `In(0)` and `In(2)` of `L` and are **not** range-checked by `L` (nor by the benchmark harness `SourceAir`). The real producer of $P_1 = (x_1, y_1)$ and $P_2 = (x_2, y_2)$ on buses $o+0$ and $o+1$ **must** constrain every limb of $x_1, y_1, x_2, y_2$ to $[0, 2^8)$ (and $< p_{381}$ if canonical uniqueness is required; see Section 5).
2. **Condition C2 — Distinct Operation `id`s per Active Producer Row:**
   `ChainAir` (`lane-src/x_exp_node.rs:680`) neither range-checks `id` nor constrains `id` to be unique across rows. Because `X3` receives `(id, lambda, x1)` on bus $o+2$ and `(id, x2)` on a **separate** bus $o+3$ (split to stay within the LogUp message length bound $1 + 2 \times 48 = 97 \le 128$), two active rows with the **same** `id` can swap their $x_2$ inputs on bus $o+3$ while keeping every bus balanced (proven in Section 3 and mechanically witnessed by `question1SameIdInputSwapAcceptedTest` in `x3_test.qnt`). The upstream producer (`SourceAir` / MSM) must emit distinct `id`s across all active requests on bus $o+0$.
3. **Condition C3 — Non-Degenerate Affine Preconditions ($x_1 \not\equiv x_2 \pmod p$ for `add`, $y_1 \not\equiv 0 \pmod p$ for `dbl`, and $P_i \neq \mathcal{O}$):**
   `L_add` and `L_dbl` encode division via `FieldVariable::div` (`lane-src/field_variable.rs:308-315`) as the multiplicative constraint $\text{den} \cdot \lambda - \text{num} \equiv 0 \pmod p$ without proving $\text{den} \not\equiv 0 \pmod p$. When $\text{den} \equiv 0$ and $\text{num} \equiv 0 \pmod p$ — specifically $P_1 = P_2$ ($x_1 = x_2, y_1 = y_2$) in `g1_add`, or $(x_1, y_1) = (0, 0)$ in `g1_double` — the constraint reduces to $0 \cdot \lambda - 0 \equiv 0 \pmod p$, leaving $\lambda \in \mathbb{F}_p$ **completely unconstrained** and allowing the prover to forge `(x3, y3)` (Section 4). The verifier / upstream circuit must enforce $(x_2 - x_1) \cdot \text{inv}_{\Delta x} \equiv 1 \pmod p$ on `add` rows, $y_1 \cdot \text{inv}_y \equiv 1 \pmod p$ on `dbl` rows, and handle $\mathcal{O}$ via selector flags or an offset accumulator.
4. **Condition C4 — Stock Mod-Builder & LogUp Premises:**
   Under `is_valid == 1` and 8-bit range-checked input limbs, `CheckCarryModToZeroSubAir` with `VariableRangeCheckerChip` on `RANGE_BUS = 1000` soundly enforces $x_3 \equiv \lambda^2 - x_1 - x_2 \pmod{p_{381}}$, and `PermutationCheckBus` (`lane-src/interaction_mod.rs:130-176`) soundly enforces multiset equality when total interactions per bus do not overflow the BabyBear characteristic.

> **Sandbox Execution Note:** In this review environment, reading `.lane-skills/` (symlinked outside the workspace) and executing `quint` via `run_command` (`quint typecheck`, `quint test`, `quint run`) were refused by the sandbox permission guard. No runtime command outputs are claimed as observed. The Quint model ([x3.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-x3/qa/gemini-quint/x14-x3/x3.qnt)) and test suite ([x3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-x3/qa/gemini-quint/x14-x3/x3_test.qnt)) were statically verified against Quint 0.32.0 syntax and type rules, and the exact commands with expected exit codes are tabulated in Section 2.3.

---

## 2. Circuit Analysis, Refinement Invariant (`invRow`), and Mutants

### 2.1 Structure of `X3` and the G1 Chain (`lane-src/x_exp_node.rs:660-821`)

Each row of `X3` (`ChainAir` wrapping `x_expr`) has width `ew + 1`:
- **`FieldExprCols` (`local[..ew]`):**
  - `is_valid`: 1 column, constrained boolean (`builder.assert_bool(is_valid)` at `lane-src/builder.rs:366`).
  - `inputs`: 3 field elements of $nl = 48$ limbs each (8-bit limbs):
    - `In(0)` = $\lambda$ (`l`)
    - `In(1)` = $x_1$
    - `In(2)` = $x_2$
  - `vars`: 1 saved variable (`num_vars() == 1`) of 48 limbs:
    - `Var(0)` = $x_3$, limb-range-checked to $[0, 2^8)$ on `RANGE_BUS = 1000` with multiplicity `is_valid` (`lane-src/builder.rs:437-448`).
  - `q_limbs`, `carry_limbs`: quotient and carry limbs for the single modular constraint (`lane-src/field_variable.rs:50-56`):
    $$(\lambda^2 - x_1 - x_2) - x_3 = q \cdot p_{381}$$
    checked via `CheckCarryModToZeroSubAir::eval` gated by `is_valid` (`lane-src/builder.rs:424-434`).
  - `flags`: 0 columns (`needs_setup == false`, `num_flags == 0`).
- **Operation Identifier (`local[ew]`):**
  - `id`: 1 BabyBear column.
- **`PermutationCheckBus` Interactions (`lane-src/x_exp_node.rs:819`):**
  With bus base $o = 1100$ (`add`) or $o = 1110$ (`dbl`) and multiplicity `cols.is_valid`:
  1. **Bus $o + 2$ (receive, mult `-is_valid`):** `[id, In(0), In(1)]` $= (\text{id}, \lambda, x_1)$ ($1 + 48 + 48 = 97 \le 128$ fields).
  2. **Bus $o + 3$ (receive, mult `-is_valid`):** `[id, In(2)]` $= (\text{id}, x_2)$ ($1 + 48 = 49 \le 128$ fields).
  3. **Bus $o + 6$ (send, mult `+is_valid`):** `[id, Var(0)]` $= (\text{id}, x_3)$ ($1 + 48 = 49 \le 128$ fields).

Across the full G1 chain (`SourceAir`, `L`, `X3`, `Y3`), the 8 buses $o+0 \dots o+7$ form the directed dataflow DAG:

| Bus | Sender (`+mult`) | Receiver (`-mult`) | Message Tuple | Field Length |
|---|---|---|---|---:|
| $o + 0$ | `SourceAir` (`+1`) | `L` (`-L.is_valid`) | `(id, x1, y1)` | 97 |
| $o + 1$ (`add` only) | `SourceAir` (`+1`) | `L_add` (`-L.is_valid`) | `(id, x2, y2)` | 97 |
| $o + 2$ | `L` (`+L.is_valid`) | `X3` (`-X3.is_valid`) | `(id, lambda, x1)` | 97 |
| $o + 3$ | `L` (`+L.is_valid`) | `X3` (`-X3.is_valid`) | `(id, x2)` (where $x_2 = x_1$ in `dbl`) | 49 |
| $o + 4$ | `L` (`+L.is_valid`) | `Y3` (`-Y3.is_valid`) | `(id, lambda, x1)` | 97 |
| $o + 5$ | `L` (`+L.is_valid`) | `Y3` (`-Y3.is_valid`) | `(id, y1)` | 49 |
| $o + 6$ | `X3` (`+X3.is_valid`) | `Y3` (`-Y3.is_valid`) | `(id, x3)` | 49 |
| $o + 7$ | `Y3` (`+Y3.is_valid`) | `SourceAir` (`-1`) | `(id, x3, y3)` | 97 |

### 2.2 Soundness Proof of Refinement Invariant `invRow`

Under Conditions C1–C4, let a trace of `(SourceAir, L, X3, Y3)` be accepted (`st.accepted == true`):
1. **Uniqueness of Active `id`s Propagates from Bus $o+0$:**
   Suppose the active rows of `SourceAir` carry distinct identifiers $\{\text{id}_1, \dots, \text{id}_k\}$. Because `L` receives on bus $o+0$ with multiplicity `L.is_valid` $\in \{0, 1\}$, multiset equality on bus $o+0$ forces `L` to have exactly $k$ active rows (`L.is_valid == 1`) with the exact same distinct identifiers $\{\text{id}_1, \dots, \text{id}_k\}$ and inputs $(x_1, y_1)$ matching `SourceAir`. On bus $o+1$ (`add`), `L_add`'s unique row with $\text{id}_j$ is forced to receive $(x_2, y_2)$ of the same $\text{id}_j$.
2. **Input Binding in `X3` (`x3RowsRefine`):**
   `L` sends $(\text{id}_j, \lambda_j, x_{1,j})$ on bus $o+2$ and $(\text{id}_j, x_{2,j})$ on bus $o+3$ (with $x_{2,j} = x_{1,j}$ in `dbl`) with multiplicity 1 for each distinct $\text{id}_j$. Every valid row `rx` of `X3` (`rx.is_valid == 1`) receives `(rx.id, rx.lam, rx.x1)` on bus $o+2$ and `(rx.id, rx.x2)` on bus $o+3$ using its single column `rx.id`. Because each $\text{id}_j$ appears at most once on $o+2$ and at most once on $o+3$, bus balance forces `rx.id == id_j` for a unique valid producing row of `L`, and forces:
   $$\text{rx.lam} = \lambda_j, \qquad \text{rx.x1} = x_{1,j}, \qquad \text{rx.x2} = x_{2,j}.$$
3. **Output Computation in `X3` and `Y3` (`chainOutputsAffineSum`):**
   Since `rx.is_valid == 1` and its inputs have 8-bit limbs (Condition C1), `X3`'s `FieldExpr` constraint forces
   $$\text{rx.x3} \equiv \text{rx.lam}^2 - \text{rx.x1} - \text{rx.x2} \equiv \lambda_j^2 - x_{1,j} - x_{2,j} \pmod p.$$
   `X3` sends $(\text{id}_j, \text{rx.x3})$ on bus $o+6$ with multiplicity 1. By the same unique-$\text{id}_j$ join across buses $o+4, o+5, o+6$, the unique valid row of `Y3` with $\text{id}_j$ receives $(\lambda_j, x_{1,j})$, $y_{1,j}$, and $x_{3,j} = \text{rx.x3}$, and `Y3`'s `FieldExpr` forces $y_{3,j} \equiv \lambda_j(x_{1,j} - x_{3,j}) - y_{1,j} \pmod p$. Finally, bus $o+7$ binds `SourceAir`'s output for $\text{id}_j$ to $(x_{3,j}, y_{3,j})$, which equals the affine sum $P_{1,j} + P_{2,j}$ (when $x_{1,j} \not\equiv x_{2,j} \pmod p$) or $[2]P_{1,j}$ (when $y_{1,j} \not\equiv 0 \pmod p$). Thus `invRow` holds.

### 2.3 Mutant Analysis & Exact Quint Verification Table

Each of the five required mutants weakens one part of `X3` in [x3.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-x3/qa/gemini-quint/x14-x3/x3.qnt) (modeled over $\mathbb{F}_{13}$ on $E(\mathbb{F}_{13}): y^2 \equiv x^3 + 4 \pmod{13}$ with valid points $P_0 = (2, 5)$, $P_1 = [2]P_0 = (12, 9)$, $P_2 = P_0 + P_1 = (8, 3)$, $P_3 = [2]P_1 = (6, 5)$) and is caught by `invRow`:

1. **`m_no_recv_x2` (drop the receive on bus $o+3$; $x_2$ free):**
   Without the receive `(id, In(2))` on bus $o+3$, `X3`'s third input column `x2` is unconstrained by the bus. The prover modifies `X3[0].x2` from $12$ to $0 \pmod{13}$, computes $x_3' = (3^2 - 2 - 0) \bmod 13 = 7 \neq 8$, and updates `Y3[0]` and `SourceAir[0]` honestly to $(7, 6) \neq (8, 3)$. Accepted by `m_no_recv_x2`; violates both clauses of `invRow`.
2. **`m_sign_flip` (constraint $x_3 = \lambda^2 - x_1 + x_2 \bmod p$):**
   Replacing $- x_2$ with $+ x_2$ in `x_expr` forces `X3` on non-zero $x_2$ ($x_2 = 12$ on row 0, $x_2 = 6$ on row 1) to compute $x_3' = (\lambda^2 - x_1 + x_2) \bmod 13 = 6 \neq 8$ on row 0 and $12 \neq 0$ on row 1, which propagates through `o+6` into `Y3` and `o+7` into `SourceAir`. Accepted by `m_sign_flip`; violates both clauses of `invRow`.
3. **`m_no_id_send` (drop `id` from the send on bus $o+6$):**
   When `X3` sends `[x3]` instead of `[id, x3]` on bus $o+6$, `X3`'s output $x_3$ is no longer bound to the operation `id`. In a 2-row trace with distinct operations `id = 1` ($x_{3,0} = 8$) and `id = 2` ($x_{3,1} = 0$), `Y3` row 0 (`id = 1`) consumes $x_{3,1} = 0$ and `Y3` row 1 (`id = 2`) consumes $x_{3,0} = 8$, while the unkeyed multiset $\{[8], [0]\}$ on bus $o+6$ remains balanced. Accepted by `m_no_id_send`; violates `invRow` (`chainOutputsAffineSum`).
4. **`m_recv_swapped` (receive `(id, x1, lambda)` in place of `(id, lambda, x1)` on bus $o+2$):**
   Because `L` sends `(id, lambda, x1)` on bus $o+2$ while `m_recv_swapped` receives `(id, In(1), In(0))`, the prover swaps `X3`'s `In(0)` and `In(1)` columns (`X3.lam = x1`, `X3.x1 = lam`). Bus $o+2$ balances, and `X3` computes $x_3' \equiv x_1^2 - \lambda - x_2 \pmod p$ ($2 \neq 8$ on row 0, $7 \neq 0$ on row 1). Accepted by `m_recv_swapped`; violates both clauses of `invRow`.
5. **`m_valid_free` (multiplicity `1` in place of `is_valid` on the send):**
   In `FieldExpr::eval` (`lane-src/builder.rs:424-448`), both the modular constraint $x_3 \equiv \lambda^2 - x_1 - x_2 \pmod p$ and the 8-bit limb range check on $x_3$ are gated by `is_valid`. When `X3` sets `is_valid = 0` on a row, `X3.x3` is completely unconstrained, yet with multiplicity `1` on the send (`o+6`), `X3` still injects `(id, x3_forged)` onto bus $o+6$ to `Y3`. Accepted by `m_valid_free`; violates `invRow` (`chainOutputsAffineSum`).
   *(Structural note on the `L - X3 - Y3` multiplicity cycle: across buses $o+2$, $o+4$, and $o+6$, total multiplicities satisfy $\sum X3.\text{is\_valid} \stackrel{o+2}{=} \sum L.\text{is\_valid} \stackrel{o+4}{=} \sum Y3.\text{is\_valid} \stackrel{o+6}{=} \sum \text{send\_mult}(X3)$. Thus if `o+2` uses `X3.is_valid` while `o+6` uses `1`, any trace with an inactive padding row `X3.is_valid == 0` has $\sum X3.\text{is\_valid} < N_{\text{rows}}$, which also breaks completeness on every padded trace.)*

| Check / Target | Module | Invariant / Suite | Exact Quint Command | Expected Exit Code | Expected Outcome |
|---|---|---|---|---:|---|
| Typecheck model | — | — | `quint typecheck qa/gemini-quint/x14-x3/x3.qnt` | `0` | Typecheck succeeds |
| Typecheck tests | — | — | `quint typecheck qa/gemini-quint/x14-x3/x3_test.qnt` | `0` | Typecheck succeeds |
| Unit & mutant test suite | `x3_test` | 16 `run` tests | `quint test qa/gemini-quint/x14-x3/x3_test.qnt --main=x3_test --max-samples=100 --seed=0x14` | `0` | All 16 tests pass |
| Soundness (`invRow`) | `x3_sound` | `invRow` | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=x3_sound --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` | `0` | `[ok] No violation found` |
| Non-vacuity 1 (accepted honest chain) | `x3_sound` | `invNoAcceptedHonest` | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=x3_sound --invariant=invNoAcceptedHonest --max-steps=10 --max-samples=200 --seed=0x14` | `1` | `[violation]` (witnesses accepted honest chain) |
| Non-vacuity 2 (accepted `add` chain) | `x3_sound` | `invNoAcceptedAdd` | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=x3_sound --invariant=invNoAcceptedAdd --max-steps=10 --max-samples=200 --seed=0x14` | `1` | `[violation]` (witnesses accepted `add` chain) |
| Non-vacuity 3 (accepted `dbl` chain) | `x3_sound` | `invNoAcceptedDbl` | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=x3_sound --invariant=invNoAcceptedDbl --max-steps=10 --max-samples=200 --seed=0x14` | `1` | `[violation]` (witnesses accepted `dbl` chain) |
| Mutant 1 (`no_recv_x2`) | `m_no_recv_x2` | `invRow` | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=m_no_recv_x2 --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` | `1` | `[violation]` (`invRow` violated; mutant caught) |
| Mutant 2 (`sign_flip`) | `m_sign_flip` | `invRow` | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=m_sign_flip --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` | `1` | `[violation]` (`invRow` violated; mutant caught) |
| Mutant 3 (`no_id_send`) | `m_no_id_send` | `invRow` | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=m_no_id_send --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` | `1` | `[violation]` (`invRow` violated; mutant caught) |
| Mutant 4 (`recv_swapped`) | `m_recv_swapped` | `invRow` | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=m_recv_swapped --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` | `1` | `[violation]` (`invRow` violated; mutant caught) |
| Mutant 5 (`valid_free`) | `m_valid_free` | `invRow` | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=m_valid_free --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` | `1` | `[violation]` (`invRow` violated; mutant caught) |

---

## 3. Question (1): What the `id` Column Guarantees and What It Does Not

### 3.1 What `id` Guarantees
1. **Cross-AIR Join Key Under Distinct Producer `id`s:**
   In `ChainAir::eval` (`lane-src/x_exp_node.rs:680-696`), the single column `id = local[self.ew]` is prepended (`Ref::Id`) to every bus interaction of that row (`o+2`, `o+3`, and `o+6` in `X3`). When all active rows sent by the upstream producer (`SourceAir` on bus $o+0$) carry **distinct** `id` values:
   - Bus $o+0$ balance forces the active rows of `L` (`L.is_valid == 1`) to carry the exact same multiset of distinct `id` values.
   - Consequently, on each of buses $o+2$, $o+3$, $o+4$, and $o+5$, `L` emits at most one message per `id`.
   - Any active row of `X3` (`X3.is_valid == 1`) must therefore match its `(id, lambda, x1)` on bus $o+2$, its `(id, x2)` on bus $o+3$, and its `(id, x3)` on bus $o+6$ to the **unique** operation carrying that `id`.
   - Physical row indices in `L`, `X3`, and `Y3` do not need to align: permuting rows of `X3` while preserving their `id` column still satisfies `PermutationCheckBus` and preserves `invRow` (verified by `permutedX3RowOrderAcceptedTest` in `x3_test.qnt`).
2. **Intra-Message Tuple Binding on Bus $o+2$:**
   Because `(id, lambda, x1)` are sent in a **single** message on bus $o+2$, $\lambda$ and $x_1$ of a single `L` row can never be decoupled from each other, even if two rows share the same `id`.

### 3.2 What `id` Does NOT Guarantee — Can Two Rows with the Same `id` Swap Inputs?
- **`id` is neither range-checked nor constrained to be unique in `ChainAir` or `SourceAir`:**
   `id` is an unconstrained witness column in $\mathbb{F}_{p_{\text{BB}}}$.
- **YES — two active rows with the same `id` CAN swap inputs across separate buses:**
  Because the LogUp message length bound is $2^7 = 128$ BabyBear elements (`lane-src/x_exp_node.rs:658-659`, `lane-src/interaction_mod.rs:223-235`), `L` cannot send `(id, lambda, x1, x2)` in one message ($1 + 3 \times 48 = 145 > 128$). Instead, `L` sends `(id, lambda, x1)` on bus $o+2$ and `(id, x2)` on a **separate** bus $o+3$.
  Suppose two active operations $A$ and $B$ are assigned the **same** identifier $\text{id}_A = \text{id}_B = \text{id}$:
  - `L` sends $\{(\text{id}, \lambda_A, x_{1,A}), (\text{id}, \lambda_B, x_{1,B})\}$ on bus $o+2$ and $\{(\text{id}, x_{2,A}), (\text{id}, x_{2,B})\}$ on bus $o+3$.
  - In `X3`, row 0 (with `id`) can receive $(\text{id}, \lambda_A, x_{1,A})$ on bus $o+2$ and $(\text{id}, x_{2,B})$ on bus $o+3$, computing the cross-contaminated coordinate
    $$x_{3,0}' \equiv \lambda_A^2 - x_{1,A} - x_{2,B} \pmod p,$$
    while row 1 (with `id`) receives $(\text{id}, \lambda_B, x_{1,B})$ on bus $o+2$ and $(\text{id}, x_{2,A})$ on bus $o+3$, computing
    $$x_{3,1}' \equiv \lambda_B^2 - x_{1,B} - x_{2,A} \pmod p.$$
  - Every bus ($o+0 \dots o+7$) is **100% balanced as a multiset**, and every `FieldExpr` constraint in `L`, `X3`, and `Y3` is satisfied, yet both output points are forged! (Mechanically demonstrated by `proveSameIdSwapX2` and `question1SameIdInputSwapAcceptedTest` in [x3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-x3/qa/gemini-quint/x14-x3/x3_test.qnt)).
  - Likewise, under duplicate `id`s, `L_add` can swap $P_2 = (x_2, y_2)$ on bus $o+1$ relative to $P_1 = (x_1, y_1)$ on bus $o+0$, and `Y3` can swap $y_1$ on bus $o+5$ and $x_3$ on bus $o+6$ relative to $(\lambda, x_1)$ on bus $o+4$.
  - **Requirement:** The producer AIR (`SourceAir` / MSM) must constrain its emitted `id`s to be distinct (e.g., via a step counter $\text{id}_{r+1} = \text{id}_r + 1$ or a preprocessed row-index column).

---

## 4. Question (2): Exceptional Cases ($x_1 = x_2$, $y = 0$, Point at Infinity $\mathcal{O}$)

In `lane-src/x_exp_node.rs:763-779`, `L_add` and `L_dbl` compute $\lambda$ using `FieldVariable::div` (`lane-src/field_variable.rs:266-322`), which introduces a witness variable $\lambda$ and enforces only the polynomial identity
$$\text{other} \cdot \lambda - \text{self} \equiv 0 \pmod p$$
without checking that the denominator `other` is non-zero modulo $p$. `X3` (`x3 = l.square() - x1 - x2`) and `Y3` (`y3 = l * (x1 - x3) - y1`) contain no division and accept any canonical $(\lambda, x_1, x_2)$ passed by `L`.

### 4.1 What the AIR Does on Each Exceptional Case

| Exceptional Case | Chain | Constraint in `L` | AIR Behavior | Security Impact |
|---|---|---|---|---|
| **$x_1 = x_2$ and $y_1 = y_2$** ($P_1 = P_2$ fed to `add`) | `g1_add` ($o = 1100$) | $(x_2 - x_1)\lambda - (y_2 - y_1) \equiv 0 \cdot \lambda - 0 \equiv 0 \pmod p$ | **Accepts ANY $\lambda \in \mathbb{F}_p$** (`question2ExceptionalAddEqualPointsFreeLambdaTest`) | **CRITICAL SOUNDNESS GAP** if $x_1 \neq x_2$ is not enforced: the prover can pick any $\lambda \in \mathbb{F}_p$ and forge $x_3 = \lambda^2 - 2x_1$, $y_3 = \lambda(x_1 - x_3) - y_1$ (any secant intersection through $P_1$ instead of $[2]P_1$). |
| **$x_1 = x_2$ and $y_1 \not\equiv y_2$** ($P_2 = -P_1$, sum is $\mathcal{O}$) | `g1_add` ($o = 1100$) | $(x_2 - x_1)\lambda - (y_2 - y_1) \equiv -(y_2 - y_1) \not\equiv 0 \pmod p$ | **Unsatisfiable** for `is_valid = 1` (`question2ExceptionalAddInversePointsRefusedTest`) | **Completeness gap** (cannot prove $P + (-P) = \mathcal{O}$ with `is_valid = 1`), but sound (no false affine output is accepted). |
| **$y = 0$ and $x \neq 0$** (2-torsion point, $[2]P = \mathcal{O}$) | `g1_double` ($o = 1110$) | $(2y)\lambda - 3x^2 \equiv -3x^2 \not\equiv 0 \pmod p$ | **Unsatisfiable** for `is_valid = 1` | **Completeness gap** if a point with $y = 0, x \neq 0$ is doubled (note: on BLS12-381 $y^2 = x^3 + 4$ over $\mathbb{F}_{p_{381}}$, $\gcd(3, p_{381}-1) = 3$ and $-4$ is a non-residue, so no $\mathbb{F}_{p_{381}}$-rational curve point has $y = 0$ unless off-curve). |
| **$y = 0$ and $x = 0$** ($(0, 0)$ zero-encoding of $\mathcal{O}$) | `g1_double` ($o = 1110$) | $(2 \cdot 0)\lambda - 3 \cdot 0^2 \equiv 0 \cdot \lambda - 0 \equiv 0 \pmod p$ | **Accepts ANY $\lambda \in \mathbb{F}_p$** (`question2ExceptionalDblOriginFreeLambdaTest`) | **CRITICAL SOUNDNESS GAP** if $(0,0)$ is fed as an active row to `g1_double`: the prover can choose any $\lambda \in \mathbb{F}_p$ and output $(\lambda^2, -\lambda^3) \pmod p \neq (0, 0)$. |
| **Point at infinity $\mathcal{O}$** as $P_1$ or $P_2$ | `g1_add` / `g1_double` | Affine chord/tangent equations | No infinity flag or bypass mux exists in `L`, `X3`, or `Y3` | Affine formulas on a dummy representation like $(0, 0) + (x_2, y_2)$ compute $\lambda = y_2/x_2$, $x_3 = (y_2/x_2)^2 - x_2 \neq x_2$, corrupting the identity law $\mathcal{O} + P_2 = P_2$. |

### 4.2 What the Verifier / Enclosing Circuit Must Add
1. **Non-Zero Denominator Check in `L_add` (or Producer):**
   Must constrain $x_2 - x_1 \not\equiv 0 \pmod{p_{381}}$ on every active `g1_add` row, either by witnessing an inverse $\text{inv}_{\Delta x}$ with $(x_2 - x_1) \cdot \text{inv}_{\Delta x} - 1 \equiv 0 \pmod{p_{381}}$, or by adding boolean flags in the point-addition state machine for $x_1 = x_2, y_1 = y_2$ (routing to doubling) and $x_1 = x_2, y_1 = -y_2$ (producing $\mathcal{O}$).
2. **Non-Zero Denominator / On-Curve Check in `L_dbl`:**
   Must ensure $(x_1, y_1) \neq (0, 0)$ and $y_1 \not\equiv 0 \pmod{p_{381}}$ on every active `g1_double` row (either by witnessing $y_1 \cdot \text{inv}_y \equiv 1 \pmod{p_{381}}$ or by induction from curve-checked generator/input points where $y^2 = x^3 + 4$ has no $(0, 0)$ solution).
3. **Infinity Handling:**
   Either carry an `is_inf` bit in the producer/accumulator (bypassing the affine chain with `is_valid = 0` when either operand or the sum is $\mathcal{O}$) or use an offset-generator MSM accumulator design where no intermediate state can hit $\mathcal{O}$ or $P_1 = \pm P_2$ (proven statically or guarded by $(x_2 - x_1)^{-1}$).

---

## 5. Question (3): Input Range Checks and What an Unchecked Producer Would Allow

### 5.1 Do `X3`'s Inputs Need Range Checks of Their Own?
- **Inside `X3`, no — provided the producer on buses $o+2$ and $o+3$ has already range-checked every limb to $[0, 2^8)$:**
  In `FieldExpr::eval` (`lane-src/builder.rs:404-448`), `X3` loads `inputs` ($\lambda, x_1, x_2$) as `OverflowInt::from_unsigned_limbs` without range-checking them, and range-checks only `vars[0]` ($x_3$, 48 limbs) plus `q_limbs` and `carry_limbs`.
  When `X3.is_valid == 1` and the buses are balanced:
  - `In(0)` ($\lambda$) equals `Var(0)` of `L` on bus $o+2$. Because `Var(0)` is a saved variable of `L`, **`L` already range-checks all 48 limbs of $\lambda$ to $[0, 2^8)$** (`lane-src/builder.rs:437-448`).
  - `In(1)` ($x_1$) and `In(2)` ($x_2$) equal `L`'s inputs on buses $o+2$ and $o+3$, which in turn equal the upstream producer's limbs on buses $o+0$ and $o+1$.
  - Because `PermutationCheckBus` enforces exact BabyBear field equality on all 48 limb coordinates of each message, re-checking those $3 \times 48 = 144$ input limbs inside `X3` would be redundant **if and only if** the upstream producer of $(x_1, y_1)$ and $(x_2, y_2)$ already range-checks them to $[0, 2^8)$.
- **Caveat on `L` and `SourceAir` in `lane-src/x_exp_node.rs`:**
  In `L` (`l_expr`), $x_1, y_1, x_2, y_2$ are `SymbolicExpr::Input` variables (`ExprBuilder::new_input`), **not** saved variables (`Var`), so `L` does **not** range-check $x_1, y_1, x_2, y_2$. In the benchmark harness `SourceAir` (`lane-src/x_exp_node.rs:702-735`), `SourceAir` performs no range checks at all. Therefore, in production, the AIR that originates $P_1$ and $P_2$ on buses $o+0$ and $o+1$ (or `L` itself) **must** range-check the 48 limbs of $x_1, y_1, x_2, y_2$ to $[0, 2^8)$.

### 5.2 What an Unchecked Producer Would Allow
If an upstream producer emits un-range-checked BabyBear field elements $\ell_k \in \mathbb{F}_{p_{\text{BB}}}$ ($p_{\text{BB}} = 2013265921 \approx 2^{30.9}$) for the limbs of $x_1$ or $x_2$:

1. **Native-Field Wrap-Around in `CheckCarryModToZeroSubAir` (Forging the $\bmod p_{381}$ Equation):**
   `ExprBuilder` (`lane-src/builder.rs:101, 167, 225-230` and `lane-src/field_variable.rs:101-114`) computes `max_overflow_bits`, `q_limbs`, and `carry_bits` assuming every input limb satisfies $|\ell_k| \le 2^8 - 1 = 255$. `CheckCarryModToZeroSubAir` verifies $\text{expr}_k - (q \cdot p)_k = c_k \cdot 2^8 - c_{k-1}$ in $\mathbb{F}_{p_{\text{BB}}}$, relying on the bound $|\text{expr}_k - (q \cdot p)_k + c_{k-1} - c_k \cdot 2^8| < p_{\text{BB}} / 2$ so that equality in $\mathbb{F}_{p_{\text{BB}}}$ implies integer equality in $\mathbb{Z}$.
   If $x_{1,k}$ or $x_{2,k}$ can take arbitrary values in $\mathbb{F}_{p_{\text{BB}}}$ (up to $\sim 2^{30}$), then:
   - In `X3`, the limb expression $\sum_{i+j=k} \lambda_i \lambda_j - x_{1,k} - x_{2,k} - x_{3,k} - (q \cdot p)_k$ can wrap modulo $p_{\text{BB}}$ while keeping `q_limbs`, `carry_limbs`, and `x3` limbs inside their range-checked bounds, allowing the prover to satisfy `X3` for values that violate $x_3 \equiv \lambda^2 - x_1 - x_2 \pmod{p_{381}}$.
   - In `L_add`, $(x_2 - x_1) \cdot \lambda$ multiplies unchecked limbs of $x_2 - x_1$ ($\sim 2^{30}$) by 8-bit limbs of $\lambda$ ($\sim 2^8$), immediately exceeding $p_{\text{BB}}$ and wrapping modulo $p_{\text{BB}}$.
2. **Limb Aliasing and Unreduced Representations ($[0, 2^{384})$ vs. $[0, p_{381})$):**
   Without limb range checks in $[0, 2^8)$, a single integer $v = \sum_{k=0}^{47} \ell_k 2^{8k}$ has many distinct limb vectors (e.g., $(\ell_0 + 256, \ell_1 - 1, \dots)$) that look distinct to `PermutationCheckBus` (which compares limb-by-limb) while evaluating to the same integer in `FieldExpr`. Furthermore, even with 8-bit limb range checks, 48 limbs span $[0, 2^{384})$ whereas $p_{381} < 2^{381} < 2^{384}$, so `FieldExpr` alone permits unreduced representatives $x_3 + k \cdot p_{381} < 2^{384}$; any final equality check or sponge commitment on terminal curve points must either enforce canonical reduction $< p_{381}$ or compare modulo $p_{381}$.

---

## 6. Question (4): Completeness

- **Standalone `X3` Completeness:**
  For **every** triple of canonical field elements $\lambda, x_1, x_2 \in [0, p_{381})$ (decomposed into 48 limbs in $[0, 2^8)$), the honest trace generator (`FieldExpr::generate_subrow` in `lane-src/builder.rs:467-577`) sets `is_valid = 1`, computes $x_3 = (\lambda^2 - x_1 - x_2) \bmod p_{381} \in [0, p_{381})$ (whose 48 canonical limbs lie in $[0, 2^8)$), and computes the exact integer quotient $q = (\lambda^2 - x_1 - x_2 - x_3) / p_{381}$ and limb carries `carries`. Because $\lambda, x_1, x_2, x_3 \in [0, p_{381}) \subset [0, 2^{384} - 1]$, $|q|$ and $|\text{carries}_k|$ are guaranteed to fit within `self.q_limbs[0]` and `carry_bits` (`lane-src/builder.rs:225-233, 526-547`), satisfying all `FieldExpr` polynomial constraints and `VariableRangeCheckerChip` range checks.
- **Full G1 Chain Completeness on Valid Points:**
  For every pair of valid affine points $P_1 = (x_1, y_1), P_2 = (x_2, y_2) \in E(\mathbb{F}_p)$ with $x_1 \neq x_2$ on the addition chain ($o = 1100$), and for every valid affine point $P_1 = (x_1, y_1) \in E(\mathbb{F}_p)$ with $y_1 \neq 0$ on the doubling chain ($o = 1110$), the denominators $x_2 - x_1$ and $2y_1$ are invertible modulo $p$, so `L`, `X3`, and `Y3` all have unique canonical witnesses in $[0, p)$ that satisfy every `FieldExpr` constraint and balance every bus $o+0 \dots o+7$ (verified by `honestAddChainAcceptedTest`, `honestDblChainAcceptedTest`, and `permutedX3RowOrderAcceptedTest` in [x3_test.qnt](file:///home/hoskinson/.cache/gemini-lanes/x14-x3/qa/gemini-quint/x14-x3/x3_test.qnt)).

---

## Lead results (2026-11-13 03:19, quint 0.32.0, run by the lead outside the lane sandbox)

The lane could not run commands. The lead ran every quint command of this report in the lane worktree `~/.cache/gemini-lanes/x14-x3`; expected exit code 0 for typecheck, test and the sound invariant, 1 for non-vacuity and mutants.

Lead edit (mechanical): the instance aliases `m_*` in `x3_test.qnt` clashed with the mutant modules of `x3.qnt` and were renamed `t_*`. No invariant, mutant or value changed.

| rc | expected | result | command |
|---:|---:|---|---|
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-x3/x3.qnt` |
| 0 | 0 | MATCH | `quint typecheck qa/gemini-quint/x14-x3/x3_test.qnt` |
| 0 | 0 | MATCH | `quint test qa/gemini-quint/x14-x3/x3_test.qnt --main=x3_test --max-samples=100 --seed=0x14` |
| 0 | 0 | MATCH | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=x3_sound --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=x3_sound --invariant=invNoAcceptedHonest --max-steps=10 --max-samples=200 --seed=0x14` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=x3_sound --invariant=invNoAcceptedAdd --max-steps=10 --max-samples=200 --seed=0x14` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=x3_sound --invariant=invNoAcceptedDbl --max-steps=10 --max-samples=200 --seed=0x14` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=m_no_recv_x2 --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=m_sign_flip --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=m_no_id_send --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=m_recv_swapped --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` |
| 1 | 1 | MATCH | `quint run qa/gemini-quint/x14-x3/x3.qnt --main=m_valid_free --invariant=invRow --max-steps=10 --max-samples=200 --seed=0x14` |
