I've drafted my independent MIL/3 proposal for the formal-core and language-architecture lens. Everything in it is specified-only. I edited no files and ran no tests, proofs or compilers. I also didn't read any other agent's MIL/3 output.

Startup: I read the develop skill and ran `status --json`. It reports capability `SP01.6 loan-swap-subset`, next action `sp01-loan-report`, missing evidence (stale SP01 inputs, missing current accounting, unavailable live state for `sp01-loan-swap-grok-01`), and no pending transactions. None of that blocks read-only research.

Tag convention: **[checked]** means verified against a file or by construction here. **[obligation]** must be proved or executed before freeze. **[deferred]** is not claimed at MIL/3. Rules without a tag are proposed normative text; every rule is specified-only. External sources are marked *comparative* and are not Moriarty evidence.

---

# MIL/3 draft — formal core and language architecture

## 1. Design thesis and scope

**Thesis.** Most of MIL/2's defects come from one conflation: it uses one language (Φ) for four different jobs.
1. The signer's promise (what the owner accepts).
2. The program's rule (what the pool, vault or loan contract enforces).
3. Branch selection (which escrow transition fires).
4. An authoring-time proof that the guards cover every case.

These jobs need different state access, different decision procedures and different trust bases. MIL/3 separates them into strata and layers. Two further rules follow:
- The authoring-time decision procedure is **not in the safety path**. Safety comes from evaluating each concrete stage in full. Exhaustiveness becomes a certificate-backed claim that only liveness and recovery claims depend on.
- Categories get **profiles**: registered cell kinds, certified operations, machine templates and evidence policies. They do not get new core primitives.

**In scope (this lens):**
- Layers and sorts.
- Intent Core grammar.
- The meaning of a template and the rule for filling its holes.
- Typing, evaluation and stage judgments.
- The pre/post strata.
- Alias-closed footprints.
- Failure classes.
- `ProgramValid`.
- The corrected escrow check.
- Category hooks.

**Out of scope, left to other lenses or deferred:**
- Numeric width selection (u112/u126/two-limb).
- Liquidation economics, oracle aggregation and foreign verifiers.
- The native statement layout, n-party clearing, concentrated liquidity, flash-loan traces and in-circuit parent verification (U4).

**Preserved without change:**
- All U0 evidence items ([ROADMAP.md:21](ROADMAP.md)).
- The U1 native-certificate gate and the U2 source-to-Midnight gate (ROADMAP.md:22–23).
- MIL/2's six pre-freeze obligations (DESIGN-MIL2.md §17) and the TeX obligations O1–O6 and (C1).

Nothing here closes any of them.

## 2. Layers

```
L0  Canonical encoding + version header + caps                (U0)
L1  MIL/3 surface syntax  ──elaborate──▶  L2
L2  Intent Core IC/3: signed template T, holes, Φ-strata, machines,
    evidence policy, authority, footprint declarations
L3  Program Core/4 (existing successor Core) + protected operations
    + certified operations Ω (profile-registered)
L4  Stage relation  Stage(T,σ,P,h,…)  — conjunction of 9 judgments
L5  Native statement (ZKIRv3 public inputs + ledger checks)   — OPEN (U1/U2)
─────────────────────────────────────────────────────────────
Profiles 𝒫_k (k ∈ 8 categories): cell kinds, alias maps, Ω entries,
machine templates, evidence policies, lints. Profiles cannot add
Φ syntax, weaken a core judgment, or register a failure class.
```

**Change from MIL/2.** An intent never applies effects directly. An `on_release {…}` block elaborates into a generated Core/4 program P_T, so every stage has exactly one effect route: `ProgramValid` (§4.9).
- This matches the Core rule that financial effects arise only when protected operations prepare descriptors ([semantic-contract.md:187,234](experiments/moriarty-language/spec/successor/semantic-contract.md)).
- The first profile admits exactly **one program invocation per stage**. Stages that compose several programs (intent machine plus pool, or multi-hop) are [deferred]. They wait for a measured Midnight multi-contract call interface; this is the AMM hop-boundary dissent.

## 3. Sorts

Base index sets: 𝔻 domains, 𝔸 nominal asset ids, 𝒦 clocks, ℙ parties, 𝕀 instance ids (pools, vaults, obligations, escrows, claims), 𝕆 observation ids.

| Sort | Meaning | Notes |
|---|---|---|
| `Qty⟨a⟩` | unsigned, width w_Q (profile; u128 default), smallest unit of a | MIL/2 §3.2 |
| `Delta⟨a⟩`, `Position⟨j⟩` | signed balance/supply change; signed exposure in instrument j | never spendable |
| `Price⟨b,q,s⟩` | b per q, scale 10^s | orientation is part of the type |
| `Share⟨p,c⟩` | claim on instance p, class c | conversion via Ω only |
| `Instant⟨c⟩`, `Duration⟨c⟩` | clock-indexed | c ∈ 𝒦 |
| `Bool` | | |
| `Cell⟨κ⟩` | name of a state cell of kind κ ∈ 𝒦ind | first-class **only** in footprints |
| `Obs⟨T,d⟩` | observation of a T-value, origin domain d | the trust class is **not** in the type; see below |
| `Route`, `Prog` | effect-structure holes | never Φ sorts [checked, MIL/2 §4.3] |

**Change from MIL/2.** The evidence class ε moves out of the observation type into a **verifier-assigned label**. Oracle synthesis rec 2: a witness cannot pick its own class. The typing judgment carries a source set L ⊆ 𝕆 of observation *ids* (the TeX's refinement). The admission step computes the label λ: 𝕆 → {anchored, imported(π), attested(π,k,n)} × 𝔻. A requirement such as "anchored on d" becomes a side condition on λ(L). This removes MIL/2's ambiguity between the class as a type index and the class as a witness-supplied tag.

## 4. Normative draft

### 4.1 Intent Core grammar (EBNF-style; specified-only)

```
template  ::= header signer clock validity assets* policy* hole* observe*
              machine* constraint* authority footprint residue
header    ::= "version" "moriarty-intent/3" profile-id caps-digest
hole      ::= "hole" x ":" sort ["!" srcbound] "where" φG        (* bound: stratum G *)
observe   ::= "observe" o ":" Obs<T,d> "policy" policy-id "require" adm-pred*
constraint::= "promise" φE                                      (* signer post-condition *)
machine   ::= "machine" m "of" template-id "(" args ")"          (* instance of profile template *)
            | "escrow" m escrow-body                             (* sugar for core template ESC *)
escrow-body ::= custody fund transitions priority ["exhaustive" ["under" φG]] deadline
                on-branch* residual
transitions ::= ("branch" b "when" φG)+                         (* guards: stratum G only *)
priority  ::= "priority" (b | "order" "[" b ("," b)* "]")
footprint ::= "footprint" "{" "reads" cellexp* "writes" cellexp* "}"
residue   ::= "residue" bytes                                    (* opaque, digested *)

(* Φ, stratified by k ∈ {G, E} *)
term_k    ::= lit | x | o.value | o.observedAt | τ
            | pre(cellexp)                                       (* both strata *)
            | post(cellexp)                                      (* only k = E *)
            | eff(sel)                                           (* only k = E: sum over effect lines *)
            | term_k ("+"|"-") term_k | lit "*" term_k | min(term_k,term_k) | max(term_k,term_k)
            | ω(term_k*)                                         (* ω ∈ Ω, certified op; result sort declared *)
atom_k    ::= term_k ("≤"|"<"|"=") term_k | "not" atom_k | x ∈ S
            | fresh(o, Duration) | after(term_k) | before(term_k)
            | status(o) "=" st                                   (* verifier-assigned status projection *)
φ_k       ::= atom_k | φ_k "and" φ_k | φ_k "or" φ_k | k_of_n(k, [φ_k…])
```

Differences from MIL/2 §4.3:
- `delivered`, `discharged`, `funded`, `terminal`, `exercised`, `challenged`, `seized` and `consumed` are removed as Φ primitives. Each becomes `pre(cell) …` or `post(cell) …` over a registered machine or receipt cell kind. For example, `delivered(E, B, ≥ q, owner)` is sugar for `post(credited(E,B,owner)) ≥ q`, and `terminal(E)` is sugar for `pre(state(E)) ∈ {released, refunded}`.
- This change fixes an undefinedness in MIL/2 (see §5, C4) and reduces the core atom set to comparisons, membership, time and status.
- `ω ∈ Ω` admits a certified operation: a narrow CPMM quote, a divmod, or a scaled price comparison. It does **not** admit general variable × variable. General Φ₁ stays [deferred].

### 4.2 Typing judgment and strata

The judgment is

  Σ; Γ; k; Π ⊢ t : S ! L

where:
- Σ is the bounded schema/profile registry.
- Γ is the typing context: holes, observations, machine parameters.
- k ∈ {G, E} is the stratum.
- Π is the intent policy environment: evidence policies and machine templates.
- S is a sort and L ⊆ 𝕆 is the source set.

This extends the TeX judgment (T) with the stratum k.

**Stratification rules:**

```
      Γ ⊢ c : Cell<κ>   κ readable                       Γ ⊢ c : Cell<κ>     k = E
 ───────────────────────────────── (PRE)      ───────────────────────────── (POST)
  Σ;Γ;k;Π ⊢ pre(c) : val(κ) ! ∅                 Σ;Γ;E;Π ⊢ post(c) : val(κ) ! ∅

  Σ;Γ;G;Π ⊢ post(c) : _    ⟹   reject TYPE_POST_IN_GUARD           (static)
  Σ;Γ;G;Π ⊢ eff(sel) : _   ⟹   reject TYPE_EFFECT_IN_GUARD         (static)
  Σ;Γ;G;Π ⊢ φ : Bool ! L   ⟹   Σ;Γ;E;Π ⊢ φ : Bool ! L              (weakening G ⊆ E)
```

- Hole bounds (`where`), branch guards, priority and the refund/recovery guards all type at stratum **G**.
- Promises and ProgramValid's Ensures type at **E**.
- This is the intent-level analogue of source/5's `TYPE_POST_SCOPE` ([static-semantics.md:96–98](experiments/moriarty-language/spec/successor/static-semantics.md)) [checked that the analogue exists].

**Why stratify (counterexample, [checked by construction]).** Suppose guards could read `post`. Take `branch release when post(balance(d,E.cust,A)) = 0` and `branch refund when post(balance(d,E.cust,A)) = 0`. Both branches drain custody, so each guard is true *because* its own branch fired. The guard constrains nothing, and "release on delivery" collapses into "release at will". Branch choice would be a fixed point of the choice itself. Stratum G removes the cycle.

**Provenance rules (unchanged from the TeX, restated):** every constructor unions its operands' L. Holes: `Γ(x) = S ! B` means σ(x) must carry L ⊆ B. `ω(t₁…tₙ) : S ! ⋃Lᵢ`. Both arms of `and`/`or` are statically typed and their L included, even though evaluation short-circuits ([semantic-contract.md:34](experiments/moriarty-language/spec/successor/semantic-contract.md), EX-D1).

**Anchoring side condition.** A position annotated `requires anchored@d` admits t only if ∀i ∈ L. λ(i) = (anchored, d) and i is bound to an authenticated read at the stage head. [obligation O2]

### 4.3 Total evaluation of Φ

The evaluation environment is ρ = (s, h, τ, O, σ, e, s′):
- s is the authenticated pre-state at ledger head h.
- τ is the authenticated stage time.
- O are the admitted observations.
- σ is the completion (hole filling).
- e and s′ are present only at stratum E.

⟦·⟧ρ maps a term to V ⊎ Rej(c) and a formula to {tt, ff} ⊎ Rej(c). Rejection is strict (it propagates out of every operand position), except in the short-circuit case of `and`/`or`:

```
 ⟦t₁⟧=v₁  ⟦t₂⟧=v₂  v₁+v₂ < 2^w           ⟦t₁⟧=v₁ ⟦t₂⟧=v₂ v₁ ≥ v₂        ⟦t₁⟧=v₁ ⟦t₂⟧=v₂ v₁<v₂
 ──────────────────────────           ───────────────────────        ──────────────────────
   ⟦t₁+t₂⟧ = v₁+v₂                       ⟦t₁−t₂⟧ = v₁−v₂                ⟦t₁−t₂⟧ = Rej(ARITH_UNDERFLOW)
 ... v₁+v₂ ≥ 2^w ⟹ Rej(ARITH_RANGE)     n·v ≥ 2^w ⟹ ⟦n*t⟧ = Rej(ARITH_RANGE)

 ⟦φ₁⟧=ff ⟹ ⟦φ₁ and φ₂⟧=ff      ⟦φ₁⟧=tt ⟹ ⟦φ₁ and φ₂⟧=⟦φ₂⟧      ⟦φ₁⟧=Rej(c) ⟹ ⟦φ₁ and φ₂⟧=Rej(c)
 (dual for or)                ⟦not a⟧ = ¬⟦a⟧ on {tt,ff}, Rej preserved
 k=0 ∨ k>n ⟹ ⟦k_of_n(k,Φ⃗)⟧ = Rej(KOFN_RANGE);  else count tt left→right, first Rej propagates
 ⟦after(t)⟧ = (⟦t⟧ < τ)   clock(t) ≠ clock(τ) is a static TYPE_CLOCK error;  t overflow ⟹ Rej(TIME_RANGE)
 ⟦fresh(o,δ)⟧ = (o.observedAt ≤ τ ∧ τ − o.observedAt ≤ δ);  o.observedAt > τ ⟹ Rej(OBS_FUTURE)
 ⟦ω(v⃗)⟧ = Ω_ω(v⃗) ∈ V ⊎ Rej(c_ω)         (each ω registers its full rejection list)
```

Evaluation terminates because the term grammar has no recursion and the caps bound depth and node count [obligation O1: totality, by structural induction plus a TS/K differential].

**Short-circuit hazard.** With a skipped right operand, a *malformed* observation named only in φ₂ never raises Rej. MIL/3 therefore moves every observation check to **admission** (§4.8, rule ADM), before any evaluation. Evaluation can then only yield Rej for arithmetic, time or Ω reasons. This follows oracle synthesis rec 3.

**Rej is never a truth value.** Any Rej reached during a stage makes the stage relation false (§4.8). In particular, Rej in a higher-priority guard does **not** fall through to a lower-priority branch (see §4.7 and hostile trace H2).

### 4.4 Template meaning and completion refinement

- A template T has canonical bytes ⟨T⟩ and digest δ_T = Poseidon(tag_T ‖ ⟨T⟩) (MIL/2 §11).
- A completion σ maps holes to values or effect-structure objects, with canonical bytes ⟨σ⟩ in hole-declaration order.
- `Fill(T,σ)` holds iff:
  1. dom σ = holes(T);
  2. each σ(x) has the declared sort and L_σ(x) ⊆ B_x;
  3. each hole bound φ_x evaluates to tt at stratum G under the stage's ρ. Bounds may mention earlier holes; they are evaluated in declaration order, and Rej ⇒ ¬Fill;
  4. the completed template's hole-erased canonical form equals ⟨T⟩ byte-for-byte.

  Item 3 makes Fill stage-dependent: a bound such as `amount ≤ pre(balance(…))` is checked at the head where the stage runs.
- The observable projection π_T maps a stage record to (effects, recipients, fees, liabilities, authority consumed, disclosures, machine states, tombstones), hiding only route-internal choices designated in T.

Meaning of a template:

  ⟦T⟧ = { π_T(r) | ∃σ. Fill(T,σ) ∧ Stage(T,σ,r) }, and ⟦T,σ⟧ fixes σ.

Refinement: ⟦T,σ⟧ ⊆ ⟦T⟧ holds by definition [checked, as the TeX notes]. The substantive properties are:
- **(C1)** Native acceptance of (δ_T, ⟨σ⟩, w) on ledger ℓ implies Fill(T,σ) ∧ π_T(ℓ) ∈ ⟦T⟧. [obligation O4]
- **(C2, new) Sequential completion.** If σ = σ₁ ⊎ σ₂ and σ₁ fills a prefix of the declaration order, then Fill(T,σ) ⟺ Fill(T,σ₁) ∧ Fill(T[σ₁],σ₂). Solver chains and partial fills need this. It fails if a later hole's bound could constrain an earlier hole, so the grammar only lets a hole's bound mention holes declared before it. [obligation]
- **Surplus (hook).** If a promise has the form `eff(to owner, b) ≥ q`, T must carry a `surplus` clause (to owner | to completer | split by a literal ratio) governing output above q. Otherwise the stage rejects with `SURPLUS_UNALLOCATED`. Whether to default instead of reject is decision D5.

### 4.5 Alias-closed footprints

**Symbolic cell expressions.** `cellexp` is a cell kind applied to arguments built from literals, parameters, holes and `pre(field)` projections of other cells.

**Resolution.** After completion, ρ resolves each cellexp to a concrete cell: res_ρ : cellexp → Cell ⊎ Rej(FOOT_UNRESOLVED).
- If a cellexp mentions `pre(c′.f)`, then c′ joins the **derived read set**. For example, the payee of an obligation's discharge depends on `pre(obligation(o).creditor)`.
- At the template level, a hole-dependent cellexp must range over a statically finite set (a hole with a finite `∈ S` bound). Otherwise the check is `FOOT_UNBOUNDED` (static).

**Alias map.** Each registered cell kind κ declares α_κ : Cell → 𝒫(Cell), the concrete cells it *stands for*. Core examples:
- α(escrow(E)) = {state(E), balance(d, E.cust, a), credited(E,·,·)}.
- α(pool(P)) = {version(P), balance(d, P.cust, aᵢ)ᵢ, supply(d, lp(P))}.
- α(balance(d,p,a)) ∋ lockTotal(d,p,a) whenever the cell is **debited** (the debit hook; lending synthesis rec 2).

The closure α* is the least fixed point of X ↦ X ∪ ⋃_{c∈X} α(c). It is finite because α is profile-registered and acyclic; the profile registry enforces this.

**Derived footprint.**
- Derived reads R_der = α*(res(cells read in all G- and E-formulas) ∪ dependency cells).
- Derived writes W_der = α*(res(cells written by the prepared effects of the *selected* branch and program)).
- Every written cell is also read, R_der ⊇ W_der, because the pre-state value is needed.

**FOOT judgment:**

  R_decl, W_decl resolved and α*-closed; R_decl ⊇ R_der; W_decl ⊇ W_der; |α*(R_decl ∪ W_decl)| ≤ cap_foot.

Violations reject FOOT_UNDECLARED, FOOT_CAP or FOOT_UNRESOLVED.

**Fork independence (for U3 joins).** Branches i ≠ j must satisfy α*(W_i) ∩ α*(R_j ∪ W_j) = ∅. Otherwise the fork must name a serialization (an order or a version). Shared affine budgets are one authenticated counter cell in *every* branch's W, so budget-sharing branches are never "independent". This is deliberate: it prevents double spend.

**MIL/2 showcase.** Under this rule, the showcase footprint fails in two ways. It omits both A recipient balances (as the TeX found), and `balance(owner,B)` is written by the counterparty's delivery stage, not by E's stages [checked by reading DESIGN-MIL2.md:316–317].

### 4.6 Failure classes

| Class | Where | State published | Example codes |
|---|---|---|---|
| F0 Ill-formed | static/admission of T | none; no stage exists | TYPE_*, CAP_*, UNKNOWN_TAG, FOOT_UNBOUNDED |
| F1 Inadmissible | stage admission | none | OBS_POLICY, OBS_STATUS, OBS_FUTURE, HEAD_STALE, SIG_BIND, FILL_* |
| F2 Evaluation reject | Φ/Ω/Core reduction | none | ARITH_*, TIME_RANGE, KOFN_RANGE, Ω codes, ENSURES_FAILED |
| F3 Relation false | check phase | none | PROGRAM_INVALID, PROMISE_FALSE, CONSERVE, AUTH_*, FOOT_*, PRIORITY_VIOLATION, SURPLUS_UNALLOCATED |
| F4 Accepted failure | accepted stage, failure branch per signed policy | retained effects and fees only | the U0 `failure` judgment ([judgments.json](deliverables/u0-semantic-contract-2026-09-23/judgments.json) index 5) |
| S Pending / unknown | state, not failure | machine stays non-terminal | escrow `pending`, bridge `unknown` |
| A Authoring unsupported | authoring checker | none; the author must restructure or drop the claim | EXH_UNSUPPORTED, EXH_COUNTEREXAMPLE |
| N Native reject | proof/ledger | none | must coincide with F1–F3 under (C1) [obligation] |

**Normative rules:**
- F0–F3 and N produce no ledger state.
- Only F4 is an accepted stage. It is reachable only when the signed failure policy names it as a phase outcome.
- An F2 in a guard or a promise is never converted into F4.
- Classes S and A are not stage outcomes.

### 4.7 Escrow as a core machine; corrected authoring check

**Core template ESC.**
- States: {unfunded, pending, t₁, …, t_m}. The terminal states t_b are tombstoned.
- Each branch b has a guard g_b at stratum G and effects P_T,b.
- The priority is a strict total order ≺ on branches. Two branches collapse to MIL/2's `release | refund`; `signed_order` becomes an explicit list.

**Choice function:**

  Choose_≺(ρ) = b iff ⟦g_b⟧ρ = tt ∧ ∀b′ ≺ b. ⟦g_{b′}⟧ρ = ff

If some ⟦g_{b′}⟧ρ = Rej(c) with b′ ≼ b, Choose is Rej(c). If no branch is enabled, Choose = none and the machine stays pending (S).

**Consequence.** A branch can win only when every higher-priority guard is *definitely false*. An unavailable observation in a higher-priority guard blocks all lower branches. This is the fail-closed choice. It conflicts with recovery liveness; see decision D2.

**The MIL/2 escrow-check mismatch.**
- MIL/2 §7 and §15 say the check `after(deadline) ⇒ release_when ∨ refund_when` is "difference logic, no solver".
- The guards, however, range over all of Φ₀: sums, literal coefficients, min/max, `k_of_n` and disjunction.
- QF_IDL admits only atoms of the form x − y ⋈ c; QF_LIA admits concrete linear coefficients (*comparative*: [SMT-LIB logics](https://smt-lib.org/logics-all.shtml), captured at `deliverables/mil2-deep-research-2026-09-29/source-text/smt-lib-logics.md` lines 722–792).

Three counterexamples [checked by construction]:
- (a) `refund when pre(x)+pre(y) ≥ pre(z)` has three variables and is not a difference atom.
- (b) The U3 discriminator `spent*20 ≤ received*11` has coefficients 20 and 11, so it is not QF_IDL either. MIL/2 §4.4 is right that *evaluating* it needs no solver, but *deciding validity* over it is a different problem.
- (c) Even a pure-difference guard with disjunction makes the negated check a Boolean combination. "Polynomial time via negative cycles" then holds per conjunctive branch, not for the formula. The case split is exponential in the worst case, bounded only by the caps.

**MIL/3 repair.** The fix has three parts: take exhaustiveness out of the safety path, keep a certificate checker in the trust base, and keep any solver outside it.

1. **Safety does not depend on exhaustiveness.** Per-witness exclusivity (Choose plus a Boolean-constrained branch bit plus a one-shot tombstone) gives terminal safety [obligation O5, local part].
2. `exhaustive [under ψ]` is an **optional signed claim**. It is **required** only if T claims a recovery guarantee (MIL/2 §7 recovery obligation). Its meaning, over all type-bounded valuations v of the G-stratum free variables (cells, observation values, τ, holes, with 0 ≤ q < 2^w and Instants in range):

   EXH(E,ψ) :⟺ ∀v. ψ(v) ∧ after(deadline)(v) ⇒ ⋁_b g_b(v)

   Here Rej-producing valuations are excluded by adding each operator's domain condition to the antecedent. Certified ops ω and status atoms are **abstracted** as fresh unconstrained variables. This abstraction is sound for validity: if the claim is valid with the op's results unconstrained, it is valid with any results. It is incomplete, and incompleteness surfaces as EXH_UNSUPPORTED.
3. **Decision by fragment, certificate-checked:**
   - **Φ₀ᴰ (difference fragment).** Every arithmetic atom is v, v + c, or v − v′ + c compared with a literal. The certificate for UNSAT of ¬EXH is a DPLL-style case tree whose leaves each carry a negative cycle in the constraint graph. Integer and rational infeasibility coincide here: from integer weights, Bellman–Ford yields integer potentials when no negative cycle exists [checked by construction]. The checker is linear in certificate size.
   - **Φ₀ᴸ (all of Φ₀).** The certificate is a case tree whose leaves each carry a Farkas combination proving rational infeasibility, **or** a bounded branch-and-bound or cutting-plane tree when only integer infeasibility holds. The certificate format and its checker are part of the language profile. The solver that finds certificates sits outside the trust base.
   - **No certificate within the cap:** EXH_UNSUPPORTED (class A). The author drops the claim or narrows the guards.

   This replaces MIL/2's "difference logic, no solver" with "no solver in the trust base; certificate checker in the trust base". It needs no owner decision on SMT because the solver never enters the trust base. It does need a measured certificate cap [obligation O7, new].

### 4.8 Stage relation and small-step rule

**Stage judgment:**

  Stage(T,σ,P,h,τ,O,e,s′) := ADM ∧ SEL ∧ ProgramValid ∧ PROMISE ∧ CONSERVE ∧ AUTH ∧ FOOT ∧ HIST ∧ FAIL

It maps to the U0 keys as follows:
- ADM and FOOT → `stage`.
- Fill and PROMISE → `intent`.
- ProgramValid and CONSERVE → `effect`.
- AUTH → `authority`.
- HIST → `history`.
- FAIL → `failure`.

**Small-step configurations.** Write ⟨φ | …⟩ for a phase φ with its environment, and let s = state@h.

```
(ADM)   hdr(T) = moriarty-intent/3 ∧ caps(T) ok ∧ sig binds δ_T ∧ h = current head(inst)
        ∧ ∀o∈O. verify_π(o) = (λ(o), status(o)) ∧ policy/status/replay ok at h
        ∧ Fill(T,σ) at stratum G
        ──────────────────────────────────────────────────────────────
        ⟨Adm | T,σ,h,τ,O⟩ → ⟨Sel | ρ_G⟩                     else → Rej_F1(code)

(SEL)   Choose_≺(ρ_G) = b                       Choose = Rej(c)      Choose = none
        ─────────────────────                  ──────────────       ──────────────
        ⟨Sel|ρ_G⟩ → ⟨Prep|ρ_G,b⟩               → Rej_F2(c)          → Rej_F3(NO_BRANCH)
        (non-machine stages: b = main)

(PREP)  Core/4: ⟨P_b, pre=s|R, args=σ, obs=O⟩ →* ExpressionPrepared(post,D,w,L)
        ∧ protected ops prepare D ↦ (e_P, s′_P)
        ──────────────────────────────────────────────────────────────
        ⟨Prep|ρ_G,b⟩ → ⟨Chk|ρ_E = ρ_G + (e_P, s′_P), b⟩     Rejected(code,…) → Rej_F2(code)

(CHK)   ProgramValid ∧ ⟦promises⟧ρ_E = tt ∧ CONSERVE ∧ AUTH ∧ FOOT ∧ HIST
        ────────────────────────────────────────────────────────────
        ⟨Chk|ρ_E,b⟩ → Acc(s′_P, e_P, tomb(b))    first failing conjunct in fixed order → Rej_F3(code)
                                                  (any ⟦·⟧ = Rej(c) → Rej_F2(c))
```

The fixed check order, which makes the failure code deterministic, is: ProgramValid, PROMISE, CONSERVE, AUTH, FOOT, HIST.

The FAIL judgment applies only when the branch b selected by SEL is a *signed failure branch* (F4). It then constrains e_P to that phase's retained effects and fees.

### 4.9 ProgramValid (the missing conjunct)

ProgramValid(T,σ,P,h,b,e,s′) :⟺
1. **Identity.** P = code(pre(version(inst))) at h, and P's identity is in the stage statement. For intent machines, P = P_{T,b} is generated from T and bound through δ_T.
2. **Reduction.** Core/4 reduction of P's entry on (s|R_der, σ-args, O) yields ExpressionPrepared, not Rejected ([semantic-contract.md:187,242](experiments/moriarty-language/spec/successor/semantic-contract.md)). This includes all of P's final-suffix `Ensure` conditions, such as a pool invariant.
3. **Preparation.** Protected operations prepare P's descriptors to (e_P, s′_P) without rejection.
4. **Exact correspondence.** The multiset of canonical effect lines in e equals e_P, s′ equals s′_P on W_der, and s′ equals s off W_der.

Part 4 is what stops "give the trader more by draining LP reserves" (AMM synthesis rec 2). A PROMISE alone cannot stop it, because the trader is never harmed.

## 5. Contradictions and counterexamples found (MIL/2, the TeX and the syntheses)

| # | Finding | Status |
|---|---|---|
| C1 | Authoring check claimed as difference logic, but guards are full Φ₀; even the U3 discriminator is outside QF_IDL (§4.7). | [checked by construction] |
| C2 | MIL/2 §4.4 claims polynomial time for Φ₀ escrow patterns. That holds only for *conjunctions* of difference atoms, not for disjunctive guards. | [checked by construction] |
| C3 | Showcase footprint omits A recipient balances and wrongly lists `balance(owner,B)` as written by E. | [checked, DESIGN-MIL2.md:316–317] |
| C4 | `delivered(B, ≥20 B, to owner, …)` has no defined reference state. If it reads the owner's current balance, an owner who already holds 20 B satisfies it at funding, and the counterparty takes 11 A for nothing. MIL/3 makes it an escrow-scoped `credited` counter that only E-tagged effects increment. | [checked by construction] |
| C5 | Guards in MIL/2 are unstratified: `pre`/`post` are admitted anywhere in Φ, which allows the self-fulfilling guard in §4.2. | [checked] |
| C6 | Class in the type (`Obs<T,ε,d>`, MIL/2 §5) versus the oracle synthesis's verifier-assigned label. The two are incompatible as written. MIL/3 chooses the label. | reconciled, specified-only |
| C7 | Pool, vault and priced-comparison arithmetic: MIL/2 places it at U4 (Φ₁). The AMM, lending, stablecoins, derivatives, oracles and staking syntheses all want a narrow certified op at U1/U2. MIL/3 admits Ω in Φ (sound for authoring only via abstraction). | **conflict; decision D1** |
| C8 | Priority by owner-signed order (MIL/2 §6) versus ledger version serialization (lending review 5) versus origination priority (staking review 3) for competing seizes. | **decision D3** |
| C9 | Fee meaning. AMM rec 4 counts LP fees retained in reserves against the signed fee cap. Under MIL/2 the fee cap covers explicit fee lines only. The two readings give different acceptance for the same trace (V2 below). | **decision D4** |
| C10 | Fail-closed Choose (§4.7) versus the MIL/2 §7 recovery-viability obligation and the bridges' nonreceipt rule. If release evidence is unavailable, refund can never be proved under `priority release`. | **decision D2** |
| C11 | MIL/2 cites `ROADMAP.md:39` for the late race. In this checkout the race text is at ROADMAP.md:40 and the U3 row at :24. Anchor drift. | [checked] |
| C12 | MIL/2 cites `lean/DefiKernel/Typed/Transition.lean:80,121` as *[df]* precedent. The file is not tracked in this checkout (`git ls-files` finds no DefiKernel). The precedent is unverifiable here. | [checked] |

## 6. Effect on the eight categories (profile hooks only; no new core syntax)

| Category | Core pieces used | Profile registers | Still open |
|---|---|---|---|
| AMM | ProgramValid (pool invariant as a P Ensure), α(pool), exact correspondence, surplus clause | Ω_cpmm (exact-input quote or divmod), cell kinds `pool`, `version`, `lp` | width (D1), fee meaning (D4), multi-program stages [deferred] |
| Lending | ESC-like `obligation` machine template, debit hook to `lockTotal`, AUTH `enforce` | obligation states (originate, accrue, pay, discharge, impair, forgive), Ω_health | liquidation economics, seize ordering (D3) |
| Stablecoins | CONSERVE with supply derived from mint/burn lines, `issue` authority | backing co-effect rule, `mode` cell (one-way) | unbacked class (policy), shutdown claimant completeness [deferred] |
| Derivatives | write-once `fixing` cell, one-shot `exercised` tombstone, persistent liability through FAIL | instrument kind, Ω_posPart, Ω_scaledPrice | margin, funding, social loss [deferred] |
| Oracles | verifier-assigned λ, ADM-level validation, identity-keyed L | policies (anchored scalar first), `round` cell | median/TWAP collections [deferred] |
| Governance | `policy(id)` cell with version pinning in HIST, `amend` as an exact effect | timelock-queue machine template, grant-epoch cell | pause as a right versus a parameter (open), voting library |
| Bridges | ESC with states {pending, unknown, received, nonreceiptProved, final}, imported λ | transfer-claim kind, verifier-mode policy, nullifier cell | foreign verifier (U4), reorg response; D2 is decisive here |
| Staking/yield | Ω_divmod with operation-fixed rounding, α(vault) ∋ managed and supply, debit hook | withdrawal-queue template, reward index and checkpoint cells | restaking allocation, slash priority (D3) |

## 7. Traces (expected results; nothing executed)

Common declarations: domain d = midnight.preview, A and B with `decimals 0` (to keep numbers readable), clock c, stage time τ, all quantities in units.

**V1 — valid escrow refund.**
- T: the §13 showcase, modified for MIL/3:
  - `branch release when pre(credited(E,B,owner)) ≥ 20`
  - `branch refund when after(validity.end)`
  - `priority [release, refund]`
  - `exhaustive`
  - deadline = validity.end + 3600
  - footprint reads/writes α*(escrow(E)) ∪ {balance(d,owner,A), balance(d,cp,A)}
- Pre-state at h: state(E) = pending, balance(d,E.cust,A) = 11, credited(E,B,owner) = 0, validity.end = 1000.
- Stage input: τ = 1001, no observations, σ = ∅.

Derivation:
```
ADM: header ok; δ_T bound; h current; O = ∅; Fill(T,∅)                      → ⟨Sel|ρ_G⟩
SEL: ⟦pre(credited)≥20⟧ = (0 ≥ 20) = ff;  ⟦after(1000)⟧ = (1000 < 1001) = tt
     Choose_[rel,ref] = refund                                              → ⟨Prep|refund⟩
PREP: P_{T,refund} ≡ transfer(d,A,E.cust,owner,11); tomb(E)
      e_P = {bal(E.cust,A): −11, bal(owner,A): +11};  s′: state(E)=refunded → ⟨Chk⟩
CHK: ProgramValid ✓ (e = e_P);  CONSERVE: −11+11 = 0 = Δsupply ✓;
     AUTH: refund needs no solver budget ✓;
     FOOT: W_der = α*{escrow(E), bal(owner,A)} ⊆ W_decl ✓;  HIST: pred = h ✓
     → Acc(s′, e_P, tomb(E))                                                 ACCEPT
```

Authoring certificate for `exhaustive` (difference fragment), with the time variable t:
- ¬EXH ≡ (end+3600 < t) ∧ ¬(credited ≥ 20) ∧ ¬(end < t).
- Integer difference atoms: `end − t ≤ −3601` and `t − end ≤ 0`.
- The cycle end → t → end has weight −3601 + 0 < 0, so ¬EXH is UNSAT and the claim is certified. The `credited` literal is irrelevant.

**V2 — valid single-hop AMM (ProgramValid is carrying the load).**
- Pool P: x = 1000 A, y = 2200 B, fee 997/1000, one program invocation.
- T promises: `eff(debit owner,A) ≤ 11`, `1000*eff(fee lines) + 3*eff(pool in,A) ≤ 1000*1` (the LP-inclusive reading of D4, still Φ₀ with literal coefficients), `eff(to owner,B) ≥ 20`, `surplus to owner`.
- σ: dx = 11, dy = 23.
- Pool Ensure (Uniswap v2 form; *comparative*: [UniswapV2Pair.sol](https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol)): (1000x′ − 3dx)(1000y′) ≥ 10⁶·x·y.
- With x′ = 1011 and y′ = 2177: 1,010,967 · 2,177,000 = 2,200,875,159,000 ≥ 2,200,000,000,000 ✓.
- Tightness at dy + 1 = 24 fails (next trace), so 23 is the exact quote.
- Fee check: 3·11 = 33 ≤ 1000 ✓. Net 23 ≥ 20 ✓. **ACCEPT.**

**H1 — hostile, well-formed envelope: one-unit over-delivery.**
- Same as V2, but the prover supplies dy = 24.
- ADM ✓, SEL (main) ✓, PREP: the Core Ensure evaluates 1,010,967 · 2,176,000 = 2,199,864,192,000 < 2.2·10¹², giving ENSURES_FAILED → **Rej_F2 at PREP**.
- If the prover instead fabricates e with dy = 24 while P prepares dy = 23, CHK ProgramValid part 4 fails → **Rej_F3(PROGRAM_INVALID)**.
- In either case PROMISE is *true* (the trader gets 24 ≥ 20). The rejection must come from ProgramValid, which is exactly the AMM synthesis's discriminator.

**H2 — hostile, well-formed: forced refund under release priority.**
- As V1, but credited(E,B,owner) = 20 at h (delivery happened), τ = 1001, and the prover submits a refund stage.
- SEL: ⟦g_release⟧ = tt, so Choose = release ≠ refund.
- The branch bit supplied is refund, so the native constraint b = Choose fails → **Rej_F3(PRIORITY_VIOLATION)**.
- Variant: the release guard reads an imported observation o with an unverifiable status. ADM rejects (F1) and no refund is possible. This is the liveness cost described in D2.

**H3 — hostile, static: self-fulfilling guard.**
- `branch release when post(balance(d,E.cust,A)) = 0` → **F0 TYPE_POST_IN_GUARD**.

**H4 — hostile, alias.**
- V1 with W_decl = {escrow(E)} under the MIL/2 reading, where escrow(E) does not alias balance cells.
- FOOT: bal(owner,A) ∈ W_der \ W_decl → **Rej_F3(FOOT_UNDECLARED)**.

## 8. Unresolved decisions (recommendation plus the evidence needed)

- **D1 — certified ops Ω before Φ₁.** Recommend admitting a closed U1 list (cpmm-quote, divmod, scaled-compare), each with width, rounding role and rejection list, abstracted in authoring checks. Evidence needed: a U1 native certificate per op, with valid and adversarial witnesses and cost. This changes MIL/2 decision 1's placement, so it needs a consequential-design vote.
- **D2 — choice under unavailable evidence.** Three alternatives:
  - (a) fail-closed Choose (this draft);
  - (b) restrict higher-priority guards to local `pre` state in U2/U3;
  - (c) require a positive nonreceipt proof for refund (bridges).

  Recommend (b) for U3 plus (c) for bridges. Evidence needed: the late-race and partition traces at U3.
- **D3 — ordering of competing seizes.** Owner-signed order, ledger version serialization, or origination priority. Recommend ledger version (an `enforce` stage binds encumbrance version v and rejects at v′ ≠ v), keeping the signed order for policy priority only. Evidence needed: a two-keeper concurrent trace.
- **D4 — fee meaning.** Explicit fee lines only, or LP-inclusive. Recommend making it a signed field (`fee_basis explicit | inclusive`) with no default. Evidence needed: a U3 discriminator decision; V2's result depends on it.
- **D5 — unallocated surplus.** Reject (this draft) or default to owner.
- **D6 — one program per stage.** Keep until the Midnight multi-contract call interface is pinned and measured.

## 9. Verification obligations

O1–O6 (TeX) and (C1) are all still open, plus:
- **O7:** a certificate format and checker for EXH, and its cap.
- **O8:** stratification soundness. G-formulas are invariant under any choice of e and s′.
- **O9:** alias-closure termination and acyclic profile registration.
- **O10:** (C2) sequential completion.
- **O11:** deterministic failure codes across TS, K and native.
- **O12:** ProgramValid part 4 bound to the native statement (U2).
- **O13:** Ω abstraction soundness for EXH.

**Deferred:** Φ₁, multi-program and multi-hop stages, n-party arity > 1, concentrated liquidity, flash traces, TWAP collections, foreign verifiers, in-circuit parent proofs, and the U4 recursion items.

These rules are language semantics only. They add no maintainer approval, campaign or review step to public compile, prove or deploy.

## 10. Anchors

**Repository:**
- DESIGN-MIL2.md §§4.3–4.4, 7, 9, 13 (lines 126–218, 239–250, 289–326)
- MIL2-PROPOSED-SEMANTICS.tex lines 101–106, 111–127, 131–138, 208–227
- RESEARCH-FINDINGS.md:7–13 (R1)
- static-semantics.md:96–98
- semantic-contract.md:34, 187, 234, 242
- judgments.json (six keys)
- ROADMAP.md:21–24, 38, 40
- The eight `opus55-*-recommendations/SYNTHESIS.md` files
- `source-text/smt-lib-logics.md` lines 722–792

**Comparative (not Moriarty evidence):**
- https://smt-lib.org/logics-all.shtml
- https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol
- https://eips.ethereum.org/EIPS/eip-4626
- https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md

This is one agent's independent proposal, not an integrated or adopted specification.