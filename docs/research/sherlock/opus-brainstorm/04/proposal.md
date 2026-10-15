# Opus lane 04: Quint lifecycle and recursive-data abstraction

Author: Opus 5.5 (high). This is an independent first-wave proposal. It selects and approves nothing. No K, Quint, build or native command ran.

## Source facts
- **[Q:157,207-209]** Quint has "no recursive operators" and points to `map`, `filter`, `fold` and `foldl` instead. Sum types are `L_1(T_1)|...|L_n(T_n)` [Q:57].
- **Limitation.** The capture does not state the prompt's rule against recursive *types*. I design as if they were unavailable; the claim is unverified.
- **Repository observation.** `docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_lifecycle.qnt` is marked unexecuted. Its line 195 says "Positive targets only; no security invariant or adversarial test". Its `PendingState` is abstract, not the serialized continuation (line 21). It is prior art only.

## Recommendation (design inference)
**Model the machine, not the term.** K's recursive AST becomes integer addresses into three finite tables:
1. **Code table** (`pure val CODE: int -> Instr`). Sub-expressions are code ids, so `Instr` never contains `Instr`.
2. **Heap** (`int -> Obj`). It has `Obj = U(int) | Node({ty, tag, fields: List[int]}) | Clo({code, env: List[int]}) | RecGroup({codes, env})`. Fields are locations.
3. **Continuation** (`List[Frame]`). The frames are `FnK`, `ArgK`, `RetK`, `CtorK` and `CaseK`, and they hold only ints and int lists.

Iteration is the temporal unrolling of `step`, so no operator needs recursion. Lane 03's rule D1 says every child location is below its parent. That makes acyclicity local, `heap.keys().forall(l => fields(l).forall(c => c < l))`, with no transitive closure. Whole-heap checks use bounded `fold`s.

### Grouped state
Each variable is one record, assigned once per action, so partial updates are impossible.
- `var m: Machine`: `{mode, ctl: Expr(int)|Val(int), env, kont, heap, nextLoc, depth, maxDepth, fuel, used, stepNo, working, committed, duties, pending: int -> PState}`. The mode is one of `Eval | Ret | Wait(int) | Halt(int) | Reject(Code) | Exhausted`.
- `var world`: observations, chunk cuts and adversary submissions.
- `var ghost`: `initFuel`, `lastAction`, the chunk log, max depth seen and settlement counts. Ghost data never feeds `m`.

### Actions (one per K rule label)
- **`step`** is enabled only in `Eval` or `Ret`. It is an `any{}` over rules whose guards are disjoint on (mode, ctl head, kont head), with no `nondet`.
- **`resume(e, v)` and `timeout(e)`** are enabled only in `Wait(e)`. `v` comes from a finite shape domain and is allocated as **fresh** cells, so it can never alias an existing location.
- **`cut`** (the proof-chunk boundary) logs `proj(m)` to `ghost` and leaves `m` unchanged.
- **`idle`** is the terminal stutter. It preserves `m` and records `lastAction = Idle`.

### No magical proof flags
No `proofValid`, `verified` or `accepted: bool` state is allowed. Acceptance is a pure relation recomputed from bound data:

`accepts(s) = s.pre == proj(s.preM) and s.post == proj(s.postM) and stepRel(s.preM, s.postM)`

"The circuit enforces `stepRel` on the bound projection" is a **named hole**, not encoded state. Quint can check that the statement layout is *sufficient*. In a two-copy product machine, equal projections of reachable configurations must give equal successor sets. Dropping `pending`, `kont` or `used` from the projection fails this check (the ZR03 analog).

### Invariants
- **Lifecycle.** A terminal mode has only `idle` as successor. `Wait(e)` implies `pending[e] == Pending` and an empty `working`. Each event settles at most once.
- **Frames.** `depth == kont.filter(isRetK).length() <= maxDepth`. Each step pushes or pops at most one frame.
- **Heap.** `heap.keys() == 0.to(nextLoc-1)`. Children are below parents. Every `U(n)` has `0 <= n < 2^128`; Quint `int` is unbounded, so the real bound is used and only inputs are sampled.
- **Resources.** `fuel + used == initFuel`. `used` never decreases.
- **Checkpoints.** `committed` is append-only. `Reject` and `Exhausted` change only `working`.

### Nonvacuous witnesses
Each implication invariant gets a `val wit*` predicate, and checking its negation must *fail*. The proposed witnesses are `witDeepReturn` (max depth ≥ 2, now 0), `witExhaustInFrame`, `witWaitResumeHalt`, `witTimeoutUnknown`, `witRejectAfterCommit`, `witHeapFull`, `witDuplicateSettleRefused` and `witCutMidCall`. An invariant whose witness is never reached is reported **vacuous**, not passed.

## K→Quint projection π and holes
π maps `<k>` to `ctl` via a code-addressing map α, `<store>` to `heap`, `<program>` to `CODE`, and every other cell to its same-named field.

**Holes, each unestablished:**
- **H1.** α (AST→ids) has no proof of totality or injectivity. Generate it from the same frozen source and add a round-trip test.
- **H2.** Results are bounded samples, not proof.
- **H3.** Rule correspondence rests only on differential replay of K traces as Quint `run` tests.
- **H4.** K backend determinism is lane 03's obligation.
- **H5.** Out of scope: circuit soundness, digest binding, field encoding, privacy, ledger phases and currentness, and cost calibration.
- **H6.** Liveness is not checked. Termination comes from fuel only.

## Draft EARS (UISEC-Q)
- **Q-01 No recursion leak.** The model SHALL represent recursive data only as integer addresses into `CODE`, `heap` or `kont`. No type SHALL contain itself, and no operator SHALL call itself.
- **Q-02 Deterministic step.** While the mode is `Eval` or `Ret`, `step` SHALL have at most one enabled rule and SHALL use no `nondet`.
- **Q-03 Terminal finality.** When the mode is `Halt`, `Reject` or `Exhausted`, only `idle` SHALL be enabled, and it SHALL preserve `m`.
- **Q-04 Fresh, single resume.** When `resume(e, v)` fires, `pending[e]` SHALL be `Pending` and `v` SHALL occupy only locations `>= nextLoc`. A second settlement SHALL yield `Reject(EventState)`, and `timeout` SHALL yield `Unknown`.
- **Q-05 Resource conservation.** `fuel + used` SHALL equal `initFuel`. `used` SHALL NOT decrease on `cut`, `resume`, `timeout` or any terminal transition.
- **Q-06 Chunk invisibility.** `cut` SHALL leave `m` unchanged. Runs that differ only in cuts SHALL reach equal `m`.
- **Q-07 Derived acceptance.** No acceptance boolean SHALL exist unless it is computed by `stepRel` over bound projections. Equal reachable projections SHALL imply equal successor sets.
- **Q-08 Nonvacuity.** No invariant SHALL be reported held unless its witness was observed reachable.

## Security scenarios (specified only)
- **S1.** The projection omits `pending` or `kont`. Expected detection: a Q-07 product counterexample.
- **S2.** Duplicate or late settlement. Expected detection: Q-04.
- **S3.** Fuel is refreshed at resume or cut. Expected detection: Q-05.
- **S4.** `Reject` after `commit` erases `committed`. Expected detection: the checkpoint invariant.
- **S5.** A resume value aliases an existing location or forms a cycle. Expected detection: Q-04 and the heap invariant.
- **S6.** A timeout is treated as not executed. Expected detection: Q-04.
- **S7.** A model where `step` is never enabled. Expected detection: a Q-08 witness gap.
- **S8.** An adversary submits a mismatched post state. Expected detection: `accepts` is false.

A surviving seeded mutant is a finding.

## Counterargument
α is a new trusted mapping. A term-level model reads more easily but cannot exist here. Product checks square the state space.
