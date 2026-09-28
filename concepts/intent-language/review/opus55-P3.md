# MIL/1 review — P3 (compilation, proving, AI ergonomics)

**Reviewer:** Claude Opus 5.5, lens P3. Read-only. No compile, no prove, no network. Every number below is an **estimate** derived from `wiki/zkir/zkir-instruction-set.md`; I measured nothing.

## Verdict

The direction is right, and two ideas — the evidence class as a typing rule, the footprint as a per-operation obligation — genuinely cannot be retrofitted, so deciding them now is correct. But the document's central compile-time claim is wrong, not merely optimistic: `DESIGN.md:123` calls Φ "total, decidable", and `:187` then requires the compiler to decide *satisfiability* of Φ. Those are different problems, and for Φ as written the second is undecidable. The rule at `:187` also rejects the design's own flagship example at `:251-262` under both of its clauses. Separately there is no compilation story: nothing says whether a stage circuit is specialised per intent or is a universal Φ interpreter, and that one unanswered question dominates every cost estimate. Before freeze I would (1) split Φ into a decidable *static* fragment and a larger *evaluable* fragment, (2) restate `:187` as a post-deadline obligation plus an in-circuit one-shot terminal tombstone, (3) shrink the U0 item from "the whole language" to the wire format and the stage public-input schema. With those edits this is freezable; without them U0 inherits an undecidable checker and U2 inherits a VK it will have to change.

## Findings

### 1. The decision procedures, named

| Check | Fragment | Procedure | Complexity | Solver? |
|---|---|---|---|---|
| Hole polarity (`:201`) | any Φ | syntax-directed polarity propagation, one declared polarity per operator argument (`−` antitone right, `mulDiv` antitone in its divisor, `⊆` antitone left) | **O(n)**, one pass | **No** |
| Footprint disjointness, ground cells (`:113-119`) | ground `Cell` | sort/hash both sets, intersect | **O((n+m) log(n+m))** | **No** |
| …with hole-valued account/asset indices | `Cell` over holes | congruence closure with disequalities (QF\_UF), union-find | near-linear | **No**, but a decision procedure you must implement and certify |
| `release ∧ refund` unsat (`:187`) | Φ minus products of two symbolic terms | DPLL(T): SAT over the Boolean skeleton + simplex with branch-and-bound / Omega | **NP-complete**; theory solver superexponential in variable count | **Yes** |
| …restricted to difference/UTVPI atoms, bounded atom count | `x − y ≤ c`, `t ≤ c` | enumerate Boolean assignments, Bellman–Ford negative-cycle check each | **O(2^A·V·E)**; A ≤ 16, V ≤ 32 ≈ 10^8 ops, sub-second (estimate) | **No** |
| …full Φ as written | QF-NIA (`term·term ≤ term·term`, `:134`; `mulDiv`, `:130`) | none | **undecidable** (Matiyasevich) | a solver returns `unknown`; no complete procedure exists |
| `release ∨ refund ∨ after(deadline)` valid | dual | same, on the negation | same | same |
| Escrow exit reachability (`:187`) | as above | not graph reachability: the nine states at `:185` are derived predicates, so this is satisfiability of `release` and of `refund` | same | same |

Two consequences the design does not draw. **Propositional abstraction alone is not enough.** Treating comparisons as opaque Booleans is sound both ways (abstraction UNSAT ⟹ real UNSAT; abstraction valid ⟹ real valid) and needs only a truth table at §8's bounded node count — but it rejects every escrow whose exhaustiveness rests on arithmetic trichotomy (`price ≥ strike` vs `price < strike`), which is how the covered call at `CATEGORY-MAP.md:105-109` is written. The minimum useful procedure is atom normalisation plus a theory solver. And **the cheap and expensive checks separate cleanly**: polarity and ground disjointness are linear and need nothing; escrow exhaustiveness is the only solver-shaped check and the only one whose answer can be `unknown`.

*[df]* reaches the same split by construction: `/home/charl/projects/defiformal/lean/DefiKernel/Parallel/Compatibility.lean:66-93` decides footprint disjointness by `firstOverlap` over lists, with `checkCompatibility_ok_iff` proving the decider equivalent to its spec; `Atomic/Policy.lean:36-70` renders coverage and uniqueness — the analogue of exhaustiveness and disjointness — as *finite enumeration over declared lanes*, not validity over an open state. Repo-wide: `decide` 1178, `omega` 66, `linarith` 49, SMT **zero**, `nlinarith` **zero**. That is the shape to copy: make the property finite before you make it logical.

### 2. Does anything need an SMT solver? Plainly: one thing, and it is avoidable

`release ∧ refund` unsatisfiability and `release ∨ refund ∨ after(deadline)` validity need an SMT-grade integer decision procedure for any Φ richer than difference logic, and are undecidable for Φ as written. Nothing else on the list does.

The project's stated position (`u0-study-2026-09-28/opus55-L3.md:127`: a solver on the U0 path "would add a trusted computing base member before any native path exists") is right for a reason the project already has evidence for — the Aeon `repay` model produced `principal = −1` precisely because range assumptions were dropped from the encoding (`UNIFIED-PROPOSAL.md:33,§10`). An SMT encoding of checked finite-width arithmetic with directed rounding is the artifact most likely to be silently wrong.

A compile-time-only use escapes the TCB **iff nothing downstream is omitted because the solver said so**. As written it does not: the disjointness proof is what licenses the acceptance relation to treat release and refund as mutually exclusive terminal outcomes. The escape is one line of circuit: an **in-circuit one-shot terminal tombstone on `escrow(Id)`**, so a second payout is unsatisfiable regardless of what the compiler proved. The escrow cell is already an authenticated read, so this adds one `test_eq` and one `assert` — under 100 rows, negligible against a Poseidon compression (estimate). Then the static check is an authoring diagnostic, the solver leaves the TCB, and `unknown` becomes a warning rather than a contradiction.

### 3. Canonical form, digest, and the compilation question nobody asked

ZKIRv3 has no control flow: a static linear instruction sequence in one flat scope, branches emulated by evaluating both paths and `cond_select` (`zkir-instruction-set.md:23,33`). A recursive predicate tree therefore compiles one of two ways and §8 chooses neither:

- **Per-intent specialised circuit.** Unroll the actual tree; cheap, linear in nodes. But the VK differs per intent, which kills permissionless authoring at settlement time, collides with ZR05 verifier identity, and makes U2's "generality argument covers the admitted grammar" unmeetable.
- **Universal Φ interpreter.** One VK; each of N node slots evaluates *every* opcode and selects. With ~20 opcodes, several range-checked (`less_than` does bit decomposition; a checked u128 add is `add` + `constrain_bits 128`), I estimate **300–1,000 rows per slot**: 32 slots ≈ **10k–32k rows**, 64 slots ≈ **20k–64k**.

Against k=17 = 131,072 rows — and `AGENTS.md:80` records R3 *exhausted* rows at k17 — that is the budget question. Full accounting for the §10 example, all estimates: digest by Poseidon over ~110 canonical field elements ≈ 55 permutations ≈ **3k–17k rows**; in-circuit Jubjub signature ≈ 2 variable-base scalar mults ≈ **4k–12k**; four authenticated cell reads at depth 32 ≈ 128 compressions ≈ **6k–38k**; interpreter **10k–32k**; effects and conservation ≈ **1k**. Total **~24k–100k rows**. One escrow fits at k=17 with thin margin; the two-escrow `CoveredCall` (`CATEGORY-MAP.md:101-112`) plausibly does not.

The dominant term is **authenticated reads, not the digest**, which inverts the §12 tension. It also exposes the missing bound: `:116` gives `Footprint ::= { reads: {Cell}, writes: {Cell} }` with no cardinality limit, and every declared cell is a Merkle path.

**There are two digests, not one.** `:205` names a single H. The wallet signs a Blake2b/SHA-256 digest; the circuit can only afford Poseidon (`persistent_hash` is SHA-256 via `std.sha2_256`, ~2,000–2,600 rows per 512-bit block, estimate). `UNIFIED-PROPOSAL.md §5 T1` already records the Blake2b-versus-Poseidon split as unresolved. §8 must name both digests with a pinned correspondence, or name one and accept its cost.

**The canonical form is not canonical.** `:205` fixes ordering and widths but no *normalisation*: `Price<B,Q,s>` with `s ∈ 0..18` gives one price 19 encodings; `Duration ::= u64 unit` gives `60s` and `1m`; `Asset` is a record with a recursive `Repr` and no stated total order. Identical intents get different digests, breaking AI property 9 and the replay detection built on it.

### 4. The Episode: no, it does not force U4 into U2 — it forces ZR09 into U3

Per-transaction recursive verification of a prior contract-call proof is not assumed available. The three possible compilations are one proof for the whole DAG (impossible for any Episode that waits for evidence, i.e. most), a recursive certificate (U4, blocked), and a proof per stage with a linking relation. The third is available now and the roadmap permits it: **ledger induction** — each stage's public inputs carry the intent digest, the predecessor commitment and the cumulative budget counters; the ledger enforces predecessor-equals-authenticated-head and monotone budget decrease. `ROADMAP.md` explicitly allows a scoped ledger-induced history profile while U4 is unresolved. A *linear* Episode is therefore U3 work with no recursion.

What does leak:

- **The public-input schema leaks into U0.** Adding the predecessor commitment or budget counters later changes the VK and invalidates every U2 negative control (`UNIFIED-PROPOSAL.md §5 T3`). The Episode forces ZR03's binding list into the freeze — interface work, not proving work.
- **Joins do force U4-class work into U3.** `:306` and `ROADMAP.md` put joins at U3; `docs/MORIARTY-BACKEND-REQUIREMENTS.md` assigns ZR09 (bounded multi-parent joins) to **U4**. A join is deliverable at U3 only if one Midnight transaction can atomically consume two contract-state heads. The design never asks. Resolution: split into **Episode-linear** (U3, ledger induction) and **Episode-DAG** (U4).
- **Cumulative budgets and parallel branches are incompatible as specified.** Two branches decrementing one counter against one head cell is ZR10's race. The Episode needs a **declared budget partition at each fork**.

The Episode also has **no syntax** — it appears at `:35` and `:41` and never again, not in §8's canonical form, not in §10. If it carries budgets and compensation, its bounds must be inside the signed digest, or a solver picks the DAG shape and ZR12's checked bounds have no signed operand.

### 5. Where each construct must be enforced

Release/refund: **in-circuit** (they bind effects). Exhaustiveness and disjointness: **compile-time plus in-circuit tombstone**. Per-`(domain, asset)` conservation: **in-circuit**, cheap. Polarity/monotonicity: **compile-time only** — a hole is filled before signing and the digest binds the filled form. Footprint declaration: **compile-time**; footprint *conformance* is **in-circuit** and is the expensive one. Evidence class: **compile-time typing**, provided §4 stops exposing `anchored` as a proposition (D5). Authority kind and consumption: **in-circuit plus ledger**. Replay: **ledger**. The only construct "a host-computed Boolean is never sufficient" makes genuinely expensive is footprint conformance, and it is affordable only if the footprint is bounded.

### 6. AI ergonomics, property by property

Holding as claimed: 2 (declared search space), 5 (outcome vocabulary — and `unknown` being *necessary* is itself an admission that §6's checks are not decidable), 9 (once normalisation is fixed).

**Property 4 splits.** For *plan* rejection it holds and is cheap: Φ evaluation is total and bounded, so one bottom-up evaluation plus one top-down blame pass yields the falsified leaves with a concrete witness in **O(n)**. For *compile-time* rejection it does not hold as written: disjointness fails in the SAT direction, so the object is a **prime implicant**, not a "minimal unsatisfied clause"; exhaustiveness fails in the UNSAT direction, where an MUS is right and deletion-based extraction costs |atoms| solver calls (QuickXplain O(k log(n/k))) — tens of extra calls (estimate). The design promises the wrong artifact for the compile-time case.

**Property 6 is statically computable.** Φ has no loops and no data-dependent iteration, so cost is a syntactic function of the node. Caveats: an authenticated read's cost depends on state-tree depth, a deployment parameter that must be pinned; and under a universal interpreter every intent pays the *maximum* node count, so the useful number is the declared bound, not the per-clause sum.

**Property 7 holds only for ground footprints.** Route freedom and static disjointness are in direct conflict: if the route is a hole, the write set is unknown until completion. The only resolution is an *over-approximate* declared footprint that completion is checked against — exactly what *[df]* does (`Typed/Expr.lean:146-161` includes both `ite` branches in `stateReads`, with `Expr.eval_congr:179` as the soundness theorem). MIL/1 needs that theorem stated. But an over-approximation for a 3-hop route over 8 approved venues is perhaps 32–50 cells against a concrete plan's 4–6 (estimate), so two such intents almost always collide and property 7 rarely fires for route-free intents. Also, `replay(Id)` is a cell (`:114`), so unless replay ids are per-intent, *no* two operations are ever independent.

**Property 10, residue.** It does **not** weaken the acceptance relation, and a program **cannot** hide a real effect in it: effects come only from `Effect ::=` (`:211-216`), each conserved and bound, and residue sits inside the digest so it is signed. What it widens is the authorized set, honestly — its purpose. Two problems remain: it is the only unbounded-length uninterpreted field in an encoding that must be node-bounded and hashed in-circuit; and free text is where a model will dump what it should have refused to sign. Make it a bounded list of registry codes plus a hash of free text, and let an intent declare a residue class that must be empty.

## Defects

- **D1 (`:187`, load-bearing).** Both clauses reject the design's own §10 example. Validity: at t = now, `delivered(...)` is false, `after(validity.end) or failed(route)` is false, `after(deadline)` is false — the disjunction is not valid. Disjointness: delivery after `validity.end` satisfies release and refund together, so the rule forbids exactly the late-success/refund race `ROADMAP.md` makes a U3 exit. The intended rule is `after(deadline) → (release ∨ refund)` plus a priority order between the exits. As stated, exit reachability is also a liveness property, which `:24` declares a non-goal.
- **D2 (`:123` vs `:187`).** "Decidable" is true of Φ *evaluation* and false of Φ *satisfiability*; the document never distinguishes them.
- **D3 (`:79-80`).** On BLS12-381, r ≈ 2^254.9. `mulDiv : Qty × Share × Share → Qty` and the cross-multiplied comparison "in a wider intermediate" both form products of two u128 values ≈ 2^256, which **overflows the native field**. The U0 numeric profile is u128 (`checked-add-u128`), and ZKIRv3's foreign-field types serve secp/curve25519, not generic 256-bit integers. Either cap operand widths so products stay under 2^254, or specify a two-limb multiply with carry range checks — an estimated ~10× the cost of a native `mul`. The ERC-4626 shape `floor(a·S/Va)`, the one worked family→constructor precedent in the repo, overflows as specified.
- **D4 (`:133-143`).** Φ has **no negation**, yet `CATEGORY-MAP.md:108` writes `not exercised`, `:48-49` write `never`/`none`, `:111` writes `equity(p, mark)` and `maintenance`, `DESIGN.md:257` writes `failed(route)`, and `CATEGORY-MAP.md:174` writes a 4-ary `delivered`. AI property 1 is false of the design's own examples. Worse, without negation you cannot write `refund = not release`, the natural way to get disjointness and exhaustiveness at once.
- **D5 (`:96` vs `:141`).** `anchored(obs)` is both a typing rule and a Φ proposition. As a proposition it can appear under `or`: `anchored(o) or k_of_n(1, [attested(...)])`. That turns trust laundering into a satisfiable formula rather than a type error and voids AI property 8. Remove `anchored`/`final` from `prop`, or admit them only in top-level conjunctive position.
- **D6 (`:201`).** `Φ_guarantee[σ] ⟹ Φ_guarantee` is not well-formed — the consequent has an unfilled hole. The intended property is `σ ≤ σ' ⟹ Adm(Φ[σ']) ⊆ Adm(Φ[σ])`. And `hole venue : Program` cannot be polarity-checked at all: monotonicity with respect to a program filling is not a syntactic property of the intent. It is sound only via `venue ∈ approved_set` — an allowlist, i.e. trust, sitting awkwardly beside goal 2's "no privileged solver".
- **D7 (`:205`).** Canonicalisation specifies ordering, not normalisation. See §3.
- **D8 (`:297`).** The proposed fallback saves the *cheap* half (the digest) and keeps the *expensive* half (reads and interpreter). It is also the discarded-Boolean pattern `docs/MORIARTY-CONSOLIDATED-DESIGN.md` forbids: an unevaluated release-adjacent clause means accepting a plan that violates a signed condition. The right fallback is to **shrink the declared bounds**, not split the tree.
- **D9 (`CATEGORY-MAP.md:39`).** n-party clearing "as an Episode with a join over disjoint footprints" cannot work: two participants clearing against the same pool write the same cells. The honest statement is that clearing needs a multi-signer stage, which the canonical stage statement structurally excludes.

## Missing

**Must be in MIL/1:** a cardinality bound on `Footprint`; the in-circuit terminal tombstone; a defined compiler behaviour on `unknown` (reject as `unsupported`, per the §11.5 vocabulary); the read-footprint soundness statement (*[df]* `Expr.eval_congr`); residue bounded and typed; negation or an explicit complement form in Φ; a declared `episode { max_stages, max_fanin, depth }` block inside the signed digest; and an explicit separation between the static and the evaluable Φ fragments.

**Can be a library:** aggregation and TWAP; fee tiers; the shape registry of pre-compiled condition templates I would use at U2 instead of a universal interpreter.

**Later milestone:** MUS / prime-implicant extraction (U1); the universal Φ interpreter (U6); multi-parent joins and cross-domain episodes (U4).

## Disagreements

1. **`:310`, "The U0 item is the whole of it."** No. Φ is versioned by the digest, so adding operators later is not a breaking change; the wire format and the stage public-input schema are. §13 has the urgency exactly inverted.
2. **`:297`'s fallback** — see D8; I would replace it outright.
3. **`CATEGORY-MAP.md:72`, flash loans.** Wrong diagnosis. Stage atomicity is what would make a flash loan *sound*; what excludes it is the absence of an intra-stage call to an unknown program.
4. **The liquidation latency bound** (`CATEGORY-MAP.md:249`) is not open — it is liveness, a declared non-goal. Move it.
5. **`:123`'s one-language claim.** Judgments are evaluated over a concrete instance; escrow conditions must additionally be *reasoned about* statically. Different requirements; the static fragment should be the smaller one.

## Top three changes

1. **Replace `:187` with a decidable rule plus an in-circuit backstop.** (a) `after(deadline) → (release ∨ refund)` over the **static fragment** of Φ — comparisons, difference constraints on instants, membership in enumerated sets, Boolean structure, `k_of_n` — decided by Boolean enumeration plus Bellman–Ford at a bounded atom count, no solver. (b) A declared priority between release and refund resolving the late-success race. (c) A mandatory one-shot terminal tombstone on `escrow(Id)` in-circuit. Add that clauses outside the static fragment are `unsupported` and reject conservatively. This removes the undecidability, removes the solver from the TCB, unblocks the U3 race requirement, and makes the design's own §10 example legal.
2. **Add a compilation section to §8** that (a) chooses the universal-interpreter model and names the shape registry as the U2 strategy; (b) declares the two digests and their pinned correspondence; (c) specifies canonical *normalisation* for `Price`, `Duration`, `Asset` and every set; (d) bounds footprint cardinality and Φ node count with concrete numbers, and states that every intent pays the maximum.
3. **Rewrite §13.** Keep in U0 only: the canonical encoding with its version tag and unknown-field-rejects rule; the digest and hash correspondence; the stage public-input schema (predecessor commitment, budget counters, terminal tombstone, replay id); the authority right-kind tag with `issue`, `enforce` and an extension point; the footprint field with its bound; the evidence class in the observation type; domain-qualified `Instant`. Defer to U1–U3 with a reserved encoding slot each: the full Φ operator set, holes and polarity checking, `mulDiv`/`Share`/`Signed`, residue, and the Episode entirely. Split the Episode row into Episode-linear (U3) and Episode-DAG (U4). Estimate: §13 as written is 4–8 engineer-months; this subset is 3–6 engineer-weeks.
