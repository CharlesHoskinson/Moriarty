# Opus lane 03: K semantics for the universal interpreter

Author: Opus 5.5 (high). Independent first-wave proposal; selects and approves nothing. No K, Quint, build or native command ran.

## Source facts
- **K [K, 2077-2231].** `strict` generates a heating rule per argument position, any of which may fire; `seqstrict` fixes left-to-right order. Either way the pending context sits on K's internal `<k>` stack. Several `owise` rules (priority 200) "can apply in any order".
- **Plutus [P]** uses explicit frames and a separate budget spender. **RISC Zero [R]** segments split a trace: evidence for segmentation, not settlement.
- **Repository observation.** The overflow-before-division staging is at `experiments/moriarty-language/formal/k/README.md:95-101`. The security draft cites `formal/k/README.md`, which does not exist here; that cite needs correcting.

## Recommendation (design inference)
**Use explicit CESK as the reference semantics; bytecode is a later refinement.** Suspension, waiting and statement projection need the continuation as serializable data, and a `strict`-built `<k>` context is in no digest. So control rules SHALL NOT use `strict`, `seqstrict`, `owise`, `?Fresh` or a recursive `[function]` evaluator. Frames are named constructors in a `<kont>` list; `<k>` holds at most one item.

Bytecode (lane 04) adds a layout/stack-signature verifier to the trusted base, so admit it only with a stated simulation relation to CESK. Counterargument: circuits may dispatch bytecode more cheaply than variable-arity AST nodes. Cost is not semantic evidence.

### Cells
- `<mode>`: `Eval`, `Ret`, `Wait(eid)`, `Halt(v)`, `Reject(code)`, `Exhausted`.
- `<control>` (expression or value), `<env>` (de Bruijn `List` of `Loc`), `<store>` (`Map Loc -> Obj`), `<nextLoc>`, `<kont>` (`List` of `Frame`).
- `<depth>`, `<maxDepth>`, `<fuel>`, `<used>`, `<step>`.
- `<program>`: the immutable code table and constructor declarations.
- `<working>`, `<committed>`, `<duties>`, `<pending>`.

Store objects are `U128(n)`, `Node(type, tag, List{Loc})`, `Clo(codeId, envLoc)` and `RecGroup(codeIds, captured)`. Allocation increments `nextLoc`; every child location is below its parent (D1 in the security draft).

### Rules (one semantic operation each)
Each rule is keyed by the triple (mode, head of control, head of kont). Rule labels carry versioned costs.
- **Lookup and closures.** `eval-const`, `eval-var`, `eval-lam` (allocates a `Clo`).
- **Calls.** `eval-app` pushes `FnK(arg, env)`; `ret-FnK` pushes `ArgK(fnVal)`; `ret-ArgK` enters the callee, pushing `RetK(callerEnv)` with `depth+1` for a non-tail call and nothing for a tail call. `ret-RetK` pops exactly one frame and restores the caller's environment.
- **Recursion.** `letrec` admits only function right-hand sides; it allocates the `RecGroup` before the closures pointing to it.
- **Constructors and matching.** `eval-ctor` evaluates fields left to right through `CtorK(type, tag, done, rest, env)`, then allocates (`ctor-0` for arity zero). `eval-case` pushes `CaseK(T, arms, env)`; `ret-CaseK` binds fields by declared index.
- **u128 primitives.** `prim-add`, `prim-sub`, `prim-mul`, `prim-div`, `prim-lt`, `prim-eq`.
- **Effects.** `emit` (appends to `<working>`), `commit` (the financial checkpoint), `await`, `resume`, `timeout`, `halt`.
- **Fuel.** `exhaust`: one rule per label, guarded by `fuel < cost(label)`.

### Exact u128, errors and fuel
Primitives compute in K's unbounded `Int`, with no wrap, saturation or field sort (UISEC-K-04). `Reject` and `Exhausted` are distinct terminal modes with no successor. Fuel is charged in the same rule, before any store, effect or pending change (UISEC-K-05), and counts logical work only, not circuit rows or DUST.

### Financial checkpoint vs proof chunk
A **proof chunk** is an arbitrary cut of the step sequence made by the statement layer. No K rule mentions it; it commits nothing and resets no fuel. A **financial checkpoint** is the program-visible `commit` rule, appending `<working>` to `<committed>`. On `Reject` or `Exhausted`, only `<working>` (including tentative duty changes) is dropped; `<committed>`, committed duties, `<pending>` and `<used>` remain. Ledger phases stay a separate adapter (product contract `:47`).

### Waiting continuation
`await(e, shape, timeoutArm)` requires an empty `<working>`, so tentative effects never span an external wait. It sets `pending[e] = Pending`, enters `Wait(e)` and persists `<kont>`, `<env>` and `<store>`. `resume` requires `Obs(e, v)` with `pending[e] = Pending` and `v` matching `shape`. `timeout` sets `Unknown` (never "not executed") and enters `timeoutArm`. A second or unknown settlement gives `Reject(EventState)`.

### Smallest honest executable fragment
**F0 (pure):** `Const u128`, `Var`, `Lam`, `App`, function-only `LetRec`, one `List = Nil | Cons(u128, List)`, `Case`, add/sub/lt and `If`, with outcomes Halt, Reject or Exhausted. It claims nothing about effects. Frozen expectations: non-tail `sum [2,3,4]` gives `Halt(9)`; `0-1` gives `Reject(Underflow)`; `MAX+1` gives `Reject(Overflow)`; tail-recursive sum keeps `|kont|` bounded; fuel below the trace total gives `Exhausted` with `<store>` unchanged by the faulting step. Cost totals stay unfrozen until a cost table exists. **F1** adds `emit`, `commit`, `await`, `resume` and `timeout` over abstract event ids, with no ledger.

## Draft EARS clauses
- **UISEC-K-01 Determinism.** While the mode is `Eval` or `Ret`, at most one rule SHALL apply to any well-formed configuration. Rules affecting control SHALL NOT use `strict`, `seqstrict`, `owise`, `?Fresh` or priorities.
  *Obligation:* rule-key disjointness table and critical-pair review.
- **UISEC-K-02 Explicit frames.** When a non-tail call, field evaluation or case scrutinee is entered, the rule SHALL push exactly one named frame to `<kont>` with `depth <= maxDepth`, otherwise `Exhausted`. Each return SHALL pop exactly one frame. A tail call SHALL push none. `<k>` SHALL hold at most one item.
- **UISEC-K-03 Constructor/match.** When `ret-CaseK` fires, it SHALL require `store[loc] = Node(T, tag, fs)` with `T` equal to the declared scrutinee type, `tag` among the arms and `|fs| = arity(tag)`. Otherwise the result SHALL be `Reject(MatchShape)`.
- **UISEC-K-04 Exact u128.** When a primitive fires, the rule SHALL store the mathematical result only if `0 <= r < 2^128`, otherwise `Reject(Overflow|Underflow)`; `div`/`mod` by 0 SHALL give `Reject(DivZero)`.
- **UISEC-K-05 Fuel first.** When a rule with cost `c` fires, it SHALL require `fuel >= c` and decrement atomically with its effect. Otherwise the machine SHALL reach `Exhausted`, with `store`, `kont`, `committed`, `duties` and `pending` unchanged.
- **UISEC-K-06 Checkpoint is not chunk.** Only `commit` SHALL move `<working>` to `<committed>`. `Reject` and `Exhausted` SHALL drop only `<working>`. No rule SHALL observe chunk boundaries.
- **UISEC-K-07 Waiting.** When `await` fires, `<working>` SHALL be empty and the continuation SHALL persist. Resume SHALL require `pending[e] = Pending` and a shape-matching observation. Timeout SHALL yield `Unknown`. Each event SHALL settle at most once.
- **UISEC-K-08 Projection completeness.** The step-statement projection SHALL be total over configurations. It SHALL include every cell except `<program>`, which is bound by identity. *Scenario:* omitting `<pending>` or `<kont>` (the ZR03 analog).

**Further obligations (proposed only):** invariants for well-bracketed frames, `loc(child) < loc(parent)`, in-range `U128` store values and constant `fuel + used`; snapshot-resume equal to the uninterrupted run over full configurations; a Quint lifecycle projection of modes and pending states with stated gaps.

## Not proved, even if the model runs
Determinism of the generated K backend, circuit correspondence and in-circuit range checks, digest binding, bytecode refinement, cost calibration, termination beyond fuel, ledger phases and currentness, privacy, and chunk invariance inside a circuit all stay unestablished. Quint samples are bounded; kprove remains blocked.

[K] https://kframework.org/docs/user_manual/ (captured `k-user-manual.md`). [P] and [R] are the lane 04 and lane 05 captures, hashed in `sources.json`.
