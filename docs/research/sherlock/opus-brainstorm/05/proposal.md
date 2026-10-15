# Opus lane 05: specialized circuits, one VM or finite families, and the outcome optimizations must preserve

Author: Claude Opus 5.5 (`claude-opus-5-5`), high effort, security owner. This is a design-phase brainstorm that approves nothing. I read sources at `9c63048a` and executed nothing. Because nothing was measured, it makes no cost or soundness claim. Labels: **[S]** source fact, **[R]** repository observation, **[I]** inference.

## 1. What general recursion actually needs

**[I]** "Universal" should describe the **semantics**, not necessarily a circuit. General recursive programs over recursive data need one reference machine **M** with:

- committed immutable code (`prog_id`);
- `(pc, locals)`;
- an explicit frame stack;
- an immutable typed ADT arena;
- exact u128 arithmetic;
- per-step and total budgets;
- working and committed financial state, the ordered effect log, residual duties and the wait set;
- a terminal status.

Lanes 03 and 05 propose the same state independently (R/brainstorm/03/proposal.md:9-15, 05/proposal.md:7). **[R]** Proof recursion chains evidence but supplies no source frames (SECURITY-CIRCUIT-ADDENDUM.md:31). Ledger 9 recursion needs no VerifyProof (:7-12).

There are three ways to realize M's step relation:

- **A, per-program step S_P.** Identity is the step VK; reproducible compiler output is trusted per program. New surface: S_P VK authorization (SECURITY-RESEARCH.md:64, M1/M2). One VK per program.
- **B, one VM step U.** Identity is a code commitment as data; U is trusted once. New surface: in-circuit code fetch, dispatch completeness, code/data confusion. Frozen VKs.
- **C, finite family {C_1..C_m} by opcode group.** The wrapper selects each step's VK from a pinned table. B's surface plus opcode-class-to-VK binding. Frozen VKs.

**[S]** SuperNova says universal-circuit steps cost "at least … the sum of sizes of circuits representing each supported instruction". Its non-uniform IVC pays only for the instruction invoked (raw/supernova-eprint-2022-1758.md:45). **[I]** Midnight is Halo2/PLONK-KZG, not folding (product contract :23), so C is the native analogue: J_k's per-slot VK table carries the dispatch (addendum :40). Whether C beats B (wrapper rows per proof versus dispatch rows per step) is **unmeasured**.

**Recommendation [I]:** freeze M and **Obs** (section 3) first. Treat A, B, C, block fusion and segmentation as *realizations* of M, each a jet that must "preserve values, rejection behavior, effects and the declared resource semantics" (contract :27, :31). Start demonstrators on A, which matches the Fenrir route and has no code-as-data surface. Admit B or C only under UISEC-OPT-004.

## 2. Missing semantics and unnecessary CPU work

1. **Totality.** Each well-encoded state has exactly one successor or terminal status; an unconstrained stuck state is a soundness hole.
2. **Terminal classes.** `Returned(v)`, `Failed(reason)`, `Exhausted(fuel|frames|arena)`, `Suspended(k)` and `Unsupported` stay distinct. Hitting a capacity limit is not a semantic failure.
3. **u128.** The scalar field holds u128 sums but not u128 products. Overflow, underflow and division by zero are explicit, range-checked failures, never field wrap-around.
4. **Arena.** Node ids must be valid, tags must match their arity and type, and acyclicity holds by construction (child id < parent id). Equality is **structural only**, so sharing or hash-consing never makes node identity observable.
5. **Dispatch.** CALL targets only the committed code table.
6. **Work.** Logical work W is defined on M. A fused step charges the W of the M steps it replaces. Circuit rows are not W.

**Not needed [I]:** mutable byte memory and its consistency argument, a register file, syscalls, privileged modes, floats, a RISC-V toolchain, or any verifier inside a step circuit (verification stays in R, J and D, addendum :44). Arena reads still need membership against a committed map, at unmeasured cost.

## 3. Observable outcome frozen for the optimization pass

Inputs: admitted program P, typed input I (signed intent, initial committed state) and declared caps B (fuel, frames, arena, per step).

`Obs(P,I,B) = (status class, canonical structural v, ordered committed effects with commit boundaries, final folded financial state, residual duties, W, canonical continuation commitment if Suspended)`

**Not observable:** rows, proof count, segment boundaries, arena layout and sharing, which realization ran, prover time. The artifact tuple and VK identities are **bound but not semantic**; they still enter `stmt` (ZR01/MNR01, addendum :66-73).

Tightening B is a profile change, never an optimization.

## 4. Proposed EARS: UISEC-OPT (Opus-owned draft)

- **UISEC-OPT-001.** The specification SHALL define one reference machine M, and SHALL state each realization as a refinement of M with named state encodings and a step mapping.
  *Accept:* both are listed, plus the gaps K and Quint leave.
- **UISEC-OPT-002.** WHEN any optimization is applied, the realization SHALL produce Obs(P,I,B) identical to M's within B.
  *Accept:* equality on an independently frozen corpus. The corpus must include factorial, a non-tail tree sum (9), a list sum (5) and the two-segment 100/40 ledger example (05/proposal.md:15).
- **UISEC-OPT-003.** IF M yields Failed or Exhausted, or no run at all, THEN no realization SHALL yield a decider-accepted `Returned` statement.
  *Accept:* each of the following is refused at D: wrapped u128, an unallocated or mistyped node read, a forged frame pop, a skipped fuel charge, and a witnessed base flag. A node or R proof is not a verdict (MNR02).
- **UISEC-OPT-004.** WHERE program identity is data, each step SHALL prove that the instruction at pc belongs to the `prog_id` commitment. WHERE a family VK is selected, the wrapper SHALL prove that the VK belongs to the pinned root and that its opcode class matches the fetched instruction.
  *Accept:* a swapped VK, a swapped instruction, and a trivially true circuit with a matching shape are all refused.
- **UISEC-OPT-005.** The step relation SHALL be total over well-encoded states. IF a realization cannot represent a state that M reaches within B, THEN it SHALL report `Unsupported` before proving, and SHALL NOT report Failed or silently shrink B.
  *Accept:* a test at the boundary of every cap dimension.
- **UISEC-OPT-006.** Arena values SHALL be immutable, typed and acyclic by construction, and equality SHALL be structural.
  *Accept:* shared and unshared encodings of the same tree yield identical Obs.
- **UISEC-OPT-007.** WHEN execution is segmented or batched, effects, commit boundaries, duties and W SHALL equal those of the unsegmented run. Each boundary SHALL bind `prev.post == next.pre` and the counter.
- **UISEC-OPT-008.** The design SHALL NOT adopt an optimization on a cost or soundness claim unless a named predicate was measured or proved under authorized execution. Logical work, rows, proving time and fees SHALL be reported separately. K, Quint and tests SHALL NOT alone establish circuit correspondence or ledger acceptance.

## Limitations and counterargument

Section 1 is analysis, not measurement. **Counterargument:** A spreads compiler trust across every program, while B concentrates it in one place, so a certified U may ultimately be safer. UISEC-OPT keeps that path open. The V1 parents and ZR/MNR requirements stay unchanged.
