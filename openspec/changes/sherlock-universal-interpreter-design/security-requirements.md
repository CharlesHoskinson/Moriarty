# DRAFT security requirements: universal Moriarty interpreter

Author: Opus 5.5. All security judgments below are mine. Status: specified-only DRAFT. It does not amend or renumber ZR01–ZR16, MNR01–MNR08, OSR-1014-01–15 or any U/M/MC/SP/G exit, and closes none of them. Every V1 parent stays open. No hostile test was executed, and no source profile was rolled out.

Primary citations:
- `docs/MORIARTY-BACKEND-REQUIREMENTS.md` (sha `0e8ebd91…`): ZR at lines 15-30, MNR at lines 81-88.
- `openspec/.../security-reconciliation-2026-10-14.md` (sha `97edda3d…`): OSR.
- `docs/MORIARTY-PRODUCT-CONTRACT.md:43-47`: effects, duties, phases, pending states.
- `AGENTS.md:115-117`.
- The captured public IVC trait at `R/brainstorm/06/midnight-ivc.md:86-116,147` (sha `aaff38e9…`, upstream pin unverified).

Recursion route: user-supplied fact. The ledger-9 custom-decider route works without VerifyProof. **No clause requires ledger10 or VerifyProof.**

## Abstraction boundary and evidence classes

| Class | What it can establish | What it cannot |
|---|---|---|
| K rule semantics (draft; kprove blocked) | Rule-level type preservation, stack discipline, budget monotonicity, exact u128 for the K definition | Circuit soundness, hash binding, ZKIR correspondence |
| Quint bounded samples | Reachability of lifecycle states and no sampled invariant violation | Exhaustiveness, all u128 values, cryptography |
| Circuit correspondence (ZR06/ZR11) | Every accepted adversarial witness has a valid semantic realization | Currentness, liveness |
| Native-local proof + decider | Encoded relation and deferred obligations discharged | Ledger state, finality |
| Ledger acceptance (target-observed) | Present entitlement and unique consumption (ZR10) | Anything about a different target |

**K and Quint abstract the following, and prove none of it:** digest and collision resistance, heap-to-commitment binding, in-circuit range checks, field-wraparound witnesses, decider soundness, the prover's free choice of private witness, ledger phases and currentness, DUST/fees, privacy, liveness, and Preview/Fenrir identity. Each is an explicit premise. Following OSR-1014-02, no result transfers between classes without a stated differential predicate.

## Draft design choices (security-relevant)

- **Heap: D1, recommended.** Immutable acyclic store. Allocation is topological, so every child location is below its parent's.
- **Heap: D2.** A hash-consed DAG with digest references. Acyclicity here is only computational, so it needs an argument.
- **Heap: D3, cycles or mutable cells. Rejected for V1.** It breaks well-founded structural recursion and digest well-foundedness, the analogue of ZR04.
- **Sharing.** Sharing is allowed, but cost is charged per edge traversed, not per unique node. Otherwise 2ⁿ-tree amplification is free.
- **Equality.** Equality is structural and charged per traversal, unless a later digest-equality contract is adopted.

## EARS clauses

Notation: σ is the pre-state ⟨heap H, nextLoc, stack K, budget B, effects E, duties D, pending P⟩, and S is the step statement.

**UISEC-001 Code identity and dispatch.** When a step dispatches a call, the interpreter SHALL execute only code whose identity is in the program code table bound by S. It SHALL reject any other identity.
- *Predicate:* `S.codeTable = H(table(program)) ∧ codeId ∈ dom(table) ∧ arity(codeId) = |args|`.
- *Scenarios:* out-of-range index; a same-arity function from another program; a trivial-true body; a table-digest swap. All reject.
- *Reuses:* ZR05, MNR03.

**UISEC-002 Typed construction and matching.** When an ADT value is constructed or matched, the interpreter SHALL reject an undeclared tag, a wrong arity, a child of the wrong sort, a scalar presented as a reference, or a location that was never allocated. Matching SHALL be exhaustive or end in an explicit reject arm.
- *Predicate:* `∀n∈H: tag(n)∈ctors(type(n)) ∧ |ch(n)|=arity(tag(n)) ∧ ∀c: type(c)=field(tag,i)`.
- *Scenarios:* a tag from another type with the same arity (type confusion); a forged node from a witness; an altered inactive-arm value.
- *Reuses:* ZR06.

**UISEC-003 Acyclic immutable heap (D1).** While a value is live, its node SHALL remain unchanged. Each new node SHALL reference only locations below `nextLoc`.
- *Predicate:* `∀n, c∈ch(n): loc(c) < loc(n)`, and `H` restricted to `[0,nextLoc)` is unchanged by every step.
- *Scenarios:* a self-reference; a forward reference; overwriting an existing location; a shared 2ⁿ DAG traversal that must exhaust budget deterministically.

**UISEC-004 Explicit frames.** When a call pushes a frame, `|K|` SHALL NOT exceed the declared `maxDepth`. A return SHALL pop exactly the frame on top, of the matching code identity and expected result type. Overflow SHALL yield an explicit `Exhausted` outcome.
- *Predicate:* well-bracketed call/return trace; `|K| ≤ maxDepth`; `S` commits `H(K)`.
- *Scenarios:* return on an empty stack; a frame forged in the witness that does not match the committed stack digest; depth + 1.

**UISEC-005 Exact u128.** The interpreter SHALL compute every scalar result exactly in [0, 2¹²⁸−1]. Any intermediate overflow or underflow, or division by zero, SHALL reject with a named code. There is no wrap and no saturation. Rounding happens only under an explicit mode.
- *Predicate:* `r = ⟦op⟧(a,b) ∧ 0 ≤ r ≤ MAX`, otherwise `Reject(code)`. Every witness-supplied u128 is range-checked in-circuit.
- *Scenarios:* MAX+1; 0−1; an intermediate MAX·2 before division; field value p−1 presented as a u128.
- *Precedent:* the K staging (overflow guard before division) at `formal/k/README.md:95-101`.

**UISEC-006 Step and total budget.** Before any effect of a step, the interpreter SHALL charge that step's declared cost. It SHALL reject when the cost exceeds `stepCap` or cumulative use exceeds `S.totalBudget`. Counters SHALL be checked against overflow and monotone across recursion. At joins, budget is partitioned and never duplicated.
- *Predicate:* `B' = B − cost ∧ cost ≤ stepCap ∧ B' ≥ 0`. The remaining budget is bound in S.
- *Scenarios:* counter wrap; a budget reset by relabelling a recursive step as genesis; a split/join that duplicates budget; cost charged after the effect.
- *Reuses:* ZR09, ZR12, ZR13. Logical work, circuit cost and DUST stay separate quantities.

**UISEC-007 Complete effects and residual duty.** When a step completes, exhausts or rejects, S SHALL carry the ordered complete effects and the full duty frame. Each liability SHALL satisfy `open + accrue − discharge = close`. No outcome, partial or failed, SHALL delete a duty. Refunds never reduce gross debit, and fees count against net.
- *Scenarios:* a duty dropped on `Exhausted`; unpaid discharge; a hidden third-party debit.
- *Reuses:* OSR-1014-05, OSR-1014-10, AGENTS 115-117.

**UISEC-008 Pending events.** While an external effect is unresolved, its unique event identity SHALL remain in P, in state Pending, Partial or Unknown. A timeout SHALL move it to Unknown, never to non-execution. Release SHALL require an authenticated non-execution observation. An event SHALL settle at most once.
- *Scenarios:* omitting P from the state digest; double settlement; timeout followed by refund.
- *Reuses:* product contract line 47, OSR-1014-13.

**UISEC-009 Canonical step statement.** For each step or segment, S SHALL injectively bind program, semantics and code table; the pre- and post-digests of σ (heap root, `H(K)`, B, E, D, P); the step index; and the domain. Every field SHALL be labelled public, private or constant.
- *Predicate:* encoding is injective or computationally binding (`midnight-ivc.md:147`).
- *Scenarios:* mutate each field alone (the ZR03 test); noncanonical encodings; ambiguous concatenation.
- *Reuses:* ZR03, OSR-1014-02.

**UISEC-010 Native recursion and ledger-9 decider.** When steps are chained by native recursion:
- genesis SHALL be an authenticated base case;
- every predecessor SHALL verify under an authorized relation identity;
- final acceptance SHALL run the decider, which checks every heap and state summary against its full data and discharges every deferred obligation (`midnight-ivc.md:103-116`);
- an outer-only success SHALL NOT be reported as final-valid.

*Scenarios:* a recursive step relabelled as base; the decider skipped; a summary inconsistent with its data; a substituted step relation; mock or dummy mode.
- *Reuses:* ZR04, ZR05, ZR08, MNR02, MNR05, ZR16.

**UISEC-011 Currentness is separate.** Where interpreter history verifies, the result SHALL NOT be reported as present entitlement until ledger state binding and unique consumption are observed. Of two valid successors, at most one SHALL consume.
- *Evidence:* blocked-external.
- *Reuses:* ZR10, MNR04.

**UISEC-012 Evidence-class labelling.** Every interpreter claim SHALL name its class from the table above, with tree, profile and premises. A Kore emission, a Quint sample or a K trace SHALL NOT be cited as correspondence, proof or ledger acceptance.
- *Reuses:* OSR-1014-02, MNR05.

## Not claimed

No soundness, correspondence, cost or privacy result. No architecture is selected. This draft is not independent product approval.
