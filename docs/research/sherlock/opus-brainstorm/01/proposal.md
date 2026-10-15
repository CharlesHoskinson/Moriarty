# Opus lane 01: recursive data, heap representation and binding security

Author: Claude Opus 5.5 (`claude-opus-5-5`), high effort, security author for this lane. 2026-10-15. Design proposal only: nothing adopted or executed (no K, Quint, build or native work), and no private material sent online. Labels: **[S]** source fact, **[I]** inference, **[L]** limitation.

## Source facts

- **[S]** CESK maps names to addresses and addresses to values. `letrec` extends the environment before evaluating right-hand sides ("lets and sets") (`brainstorm/01/cesk.md:110-120,576-620`).
- **[S]** OCaml `let rec` accepts cyclic data (`let rec l = 1 :: l`). Each right-hand side must be *statically constructive* and *not immediately linked* to the defined names (`captures/ocaml-letrecvalues.md:48-104`).
- **[S]** Lurk represents expressions, environments and continuations as Poseidon-hashed pointers; continuations form a tagged DAG (`brainstorm/01/lurk.md:100,397-402`). The page labels this a historical 2022 spec.
- **[S]** UPLC's `Constr` tag is an unqualified `Word64` (lane 07).

## Representations compared

| | A. Extensional K terms | B. Allocation-addressed heap | C. Content-addressed DAG |
|---|---|---|---|
| Sharing | Invisible | Explicit aliasing | Automatic |
| Cycles | Impossible (finite terms) | Via `letrec` or `set!` | Only through a hash collision |
| Bad references | None | Dangling pointers, address reuse | Missing preimage |
| Canonical statement | Encode the term | Depends on allocation order | The digest |
| Circuit fit | Poor (copying) | Good | Good (one hash per node) |

**[I] Recommendation.** Use **A as the K reference semantics** and **C as the commitment contract**, linked by `decode : Digest × Store ⇀ Value`. Leave B out of the first draft: allocation nondeterminism breaks canonical statements (ZR03), and B adds use-after-free and cycle obligations that ADTs, lists and trees do not need.

## First-draft shapes [I]

- `Value ::= uint(w,n) | bool | bytes | data(TypeId,Tag,Values) | clo(CodeRef,Env) | recclo(GroupId,Index,Env)`.
- `Env ::= Map{Id ↦ Value}`: an immutable snapshot with no addresses.
- `Kont ::= .Frames | Frame⋅Kont`, with defunctionalised frames (`Arg`, `Let`, `Match`, `PrimL/R`, `Ret(savedEnv)`). `Kont` is not a `Value` sort, so there is no call/cc in v1.
- **`letrec` binds only λs.** This is stricter than OCaml: `let rec x = Cons(1,x)` is forbidden. Calling `recclo(G,i,ρ)` unfolds member *i* under ρ plus `recclo(G,j,ρ)` for every *j*, as LAMBDA++ does with `muclosure`. No environment ever contains itself.
- Commitment: `H(sortTag‖typeId‖tag‖arity‖height‖childDigests…)`, with distinct sort tags for Value, Env, Frame, AST and Code.

## Security obligations [I]

1. **Type-qualified tags.** `(TypeId,Tag)` determines arity and field sorts, read from a declaration table bound into code identity (ZR03, ZR05).
2. **Injective, domain-separated encoding** across all sorts. This prevents a frame or AST from being passed off as data (ZR03, ZR06).
3. **Ranked well-foundedness.** Decoding checks `height(child) < height(parent)`, so acyclicity does not rest on collision resistance alone (ZR04).
4. **Unobservable sharing, with sharing-aware cost.** A self-pair nested 64 levels deep is 64 nodes but 2⁶⁴ logical leaves. Equality, traversal and encoding must charge per visit, and admission must bound both DAG nodes and logical size (ZR12, ZR13).
5. **Closed lexical binding.** Lookup is total, the innermost binding wins, captured environments are immutable, and no `letrec` name is read before its group is installed.
6. **Continuation integrity.** A return pops exactly one frame. Data cannot forge or inspect frames. A suspension commits to the tuple `(codeId, control, env, kont, budget, declsHash)` (ZR11, ZR14).

## EARS clauses (draft)

- **UISEC-DATA-001.** When a constructor value is built, matched or decoded, the interpreter shall validate `(TypeId,Tag)`, arity and field sorts against the bound declaration table, and shall fault on mismatch. *Acceptance:* `Nil:List` and `None:Option` (both tag 0) are never interchangeable.
- **UISEC-DATA-002.** The canonical encoding shall be injective across the value, environment, frame, AST and code sorts, with a distinct domain tag per sort.
- **UISEC-DATA-003.** If a decoded input graph has a cycle, a missing child or a non-decreasing height, then the interpreter shall refuse it before the first step, with no effects.
- **UISEC-DATA-004.** The interpreter shall expose no address or pointer-identity operation. Results shall be identical for shared and unshared encodings of equal values.
- **UISEC-DATA-005.** While traversing, comparing or encoding values, the interpreter shall charge budget per logical visit, or refuse admission when logical size exceeds the declared bound, and shall never wrap a counter.
- **UISEC-DATA-006.** When a `letrec` group is admitted, the compiler shall reject any right-hand side that is not a λ. The interpreter shall bind the whole group before evaluating any body.
- **UISEC-DATA-007.** The interpreter shall keep frames outside the user value sort. Each return shall pop exactly one frame and restore its saved environment.
- **UISEC-DATA-008.** When resuming a suspension, the interpreter shall verify the full tuple commitment and refuse on any mismatch.

## Model scenarios (design only)

**Normal**

- N1: `let t=Leaf(3) in sum(Node(t,t)) = 6`, with equal logical cost in shared and unshared form.
- N2: `letrec even/odd` on 4 returns `true`.
- N3: snapshot partway through `sum(Node(Leaf(2),Node(Leaf(3),Leaf(4))))`, then resume to get 9.

**Adverse** (each must refuse or fault, with no effects)

- A1: a self-referential or swapped-child DAG (003).
- A2: tag 0 under the wrong `TypeId` (001).
- A3: a frame digest supplied as a constructor (002).
- A4: a 64-level sharing bomb (005).
- A5: `let rec x = Cons(1,x)` (006).
- A6: `let rec f = f 1` (006).
- A7: a resume with substituted `kont` or `budget` (008).

## Tooling limits [L]

- **K.** Cells `<control> <env> <kont> <decls> <budget> <status>`. K terms are finite by construction, so K **cannot exercise** obligations 2–4.
- **Quint.** Abstract a bounded digest store with sort tags and heights, and sample A1–A4 on small instances only.
- **Neither tool proves** collision resistance, field-encoding injectivity, circuit correspondence (ZR06, ZR11) or ledger acceptance.
- **Recursion route.** Proof recursion uses the existing ledger 9 decider route, with no VerifyProof prerequisite (`recursion-research-20261015/SECURITY-CIRCUIT-ADDENDUM.md:7-12`). Source recursion is a separate layer.

## Work split and counterargument

- **Work split.** Decoding and height checks are host and codegen work, plus per-node hashing inside S_P. No new circuit family is proposed.
- **Counterargument.** Reference semantics defined directly over C would remove the A/C refinement gap, at the cost of a less readable K model. Keep both options open for review.
- **[L]** I captured one new primary source. I did not inspect Lurk's current implementation or OCaml's internal `letrec` compilation.
