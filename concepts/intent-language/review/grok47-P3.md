I'll review MIL/1 as a compilation and proving design: first the proposal and the constraints it has to satisfy, then the ZKIR cost model those claims sit on.I have the cost anchors and the ZKIR surface. One check on the conservation law MIL/1 would freeze against, then the review.# P3 review — compilation, proving, and AI ergonomics

**Verdict.** MIL/1 is the right shape for a signed intent, and a specialized single-stage escrow is buildable on the authoritative ZKIRv3 surface (34 straight-line instructions, 13 types, no loops). It is not sound or buildable as frozen. Three rules are wrong: the escrow meta-checks reject the design's own examples and the U3 race; the §12 fallback authenticates bytes while dropping clauses from the proved relation; and cross-multiplied `u128` arithmetic does not fit in the BLS12-381 scalar, so the obvious lowering is a field wrap. U0 should hash-bind a versioned grammar, widths, bounds, digest algorithm, and enforcement loci. The unsatisfiability checker, the in-circuit tree interpreter, and Episode proofs are later milestones. I did not compile, prove, or run a solver.

## Findings

### The digest is achievable. The expensive reading is the wrong one.

Φ is an inductive tree, not a recursive proof. With a version-level depth and node cap, canonical encoding (declared field order, fixed-width integers, sorted sets) is a bounded list of field elements, and `transient_hash` (Poseidon) can absorb it (`wiki/zkir/zkir-instruction-set.md:398-405`). ZKIR can express that because it can unroll a finite tree. It cannot loop over a witness-sized tree (`wiki/zkir/zkir-instruction-set.md:21`).

§8 (`DESIGN.md:203-207`) collapses three different circuits:

1. **Specialized stage.** Compile this intent's Φ to straight-line constraints. Put `digest` in the public inputs (`impact`). The ledger checks a signature over that same digest. Evaluation cost tracks the actual tree. Binding the digest is one public input. Measured Compact circuits already carry 8–25 public inputs at 452–2,507 rows (`wiki/benchmarks.md:101-106`).
2. **In-circuit hash of a witness tree, one VK.** The circuit recomputes Poseidon(encoding) and evaluates every node up to the maximum. Cost tracks the cap, on every transaction. The only Poseidon row figure I can derive: an IVC step of 1,000 Poseidon rounds measured 163,172 rows (`wiki/benchmarks.md:139`), about 160 rows per round including the wrapper. That is an upper estimate, not a gate count. A few hundred permutations is then on the order of 10⁴–10⁵ rows: the `k=14` band of the only financial prove on record (1.16–1.74 s, unverified, `wiki/benchmarks.md:142`) through the `k=17` region that has already failed (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:105`). Absorption rate is not in the sources I read, so this is an order-of-magnitude estimate.
3. **In-circuit signature verification.** Authoritative v3 has `ec_mul` on Curve25519 and hashes Poseidon, SHA-256, and Keccak. It has no SHA-512 and no Ed25519 verifier. `and`/`or`/`xor`/`sha512` are crate extensions, not the 34 (`wiki/zkir/zkir-instruction-set.md:488-505`). Curve types make a scalar multiplication expressible. They do not make a ledger signature check expressible. T6's "in-circuit is primary" (`deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md:199-200,248`) overclaims the instruction set.

The sound, affordable binding is (1) plus Poseidon, with the wallet signing the Poseidon digest the circuit recomputes, or the ledger checking a signature over the public digest. SHA-256 (`persistent_hash`) of the byte encoding is the costly hash and the wrong default.

`assert` is not a boolean constraint: in-circuit it enforces non-zero (`wiki/zkir/zkir-instruction-set.md:220`). Every Φ bit needs `constrain_to_boolean` before `assert`, or a witness of 2 satisfies a clause. That is a lowering rule the design has to own (ZR06, `docs/MORIARTY-BACKEND-REQUIREMENTS.md:20`).

### The fallback is not sound.

§12 (`DESIGN.md:296-297`) commits to the tree and evaluates only "the clauses that bind effects." A commitment makes the bytes signed. It does not put unevaluated clauses into the relation. The product contract requires the encoded relation to include the intended conditions (`docs/MORIARTY-PRODUCT-CONTRACT.md:27`). Freshness, `anchored`, authority windows, and hole-domain restrictions bind what may happen and are not effect lines. Drop them and a satisfying witness can be stale, imported-presented-as-anchored, or off-domain. A negative control that mutates only the evaluated subset cannot see the omitted clause. Recording the decision keeps the paper trail honest and the acceptance relation weaker than the signature. Those are different claims. The host-boolean rule (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:46`) forbids repairing the gap in the host.

### Where each construct has to be enforced

| Construct | Locus | Why this one |
|---|---|---|
| Escrow release / refund | Circuit evaluates Φ on the witnessed effects and ledger-supplied time; ledger applies the balance change | A host bit is the failure mode §8 exists to prevent. `after` reads the ledger clock (`docs/MORIARTY-BACKEND-REQUIREMENTS.md:51`). |
| Exclusive outcome | Circuit, per witness (`cond_select` on a boolean constrained branch bit) | This witness takes one branch. Universal disjointness is a different property. |
| Conservation | Circuit, checked adds, per `(domain, asset)` | Ledger conservation of the native coin does not cover issued tokens or fee lines. |
| Evidence class | Compile-time rejection, and a circuit binding from `anchored` to a ledger read | A private witness tag is forgeable. `imported` verification is a named trust premise until U4. |
| Authority kind | Circuit equality between the effect and the signed right; consumption against authenticated previous | `issue` on a supply line and `enforce` against a debtor-signed policy are ordinary constraints. |
| Replay | Ledger nullifier / nonce. Circuit exposes `replay(Id)` as a public input | A proof can be resubmitted. In-circuit non-replay cannot see other transactions. |
| Footprints | Ground set intersection after completion, `O(n log n)` | Concurrent reservation against one budget is ledger state (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:56`). |
| Monotonicity | Authoring polarity walk, re-run on the completed tree before signing | Universal over fillings. The chain sees the signed completion. |
| Digest | Circuit Poseidon plus ledger signature over the public digest | See above. |
| Residue | Inside the digest, outside Φ, unparsed | Below. |

U3's discriminator, `delivered(B, ≥ 20)` and a deadline (`ROADMAP.md:38`), is a comparison plus a clock check. That is in the `expire`/`decide` band (hundreds to a few thousand rows). Affordable. Unaffordable on this surface: Ed25519-in-circuit, a universal interpreter at a few hundred nodes, `k-of-n` foreign signatures inside a U2 stage, and in-circuit verification of parent proofs.

### The compile-time checks

`release ∧ refund` unsatisfiable, and `release ∨ refund ∨ after(deadline)` valid (`DESIGN.md:187`), quantify over states. Φ has no quantifiers. Decidability comes only from finite widths. Unbounded integers with `mulDiv` and products are Hilbert-undecidable. The design never fixes the width.

Split the language:

- **Φ₀**, no product and no `mulDiv`. Escrow pattern (compare with a constant or a clock) is quantifier-free integer difference logic, decided by negative-cycle detection, polynomial in nodes. Sums and `min`/`max` lift it to quantifier-free linear integer arithmetic, NP-complete; `min`/`max` blow up by case split, so their count has to be capped. `k_of_n` expands binomially; cap `n` (8 is plenty).
- **Φ₁**, cross-multiply and `mulDiv`, is quantifier-free bitvector arithmetic. Decidable, NEXPTIME-complete, and in practice an SMT solver. This repo already measured the cost of that choice: one 128-bit division obligation, bitvector `rlimit` 242,607,369 against 1,978 for unbounded `Int`; at 256 bits the bitvector encoding exceeded 600 seconds (`openspec/changes/aeon-refinement-integration/design.md:27-31`). I did not re-run it.

Footprint disjointness on ground cells needs no solver. A hole inside a cell makes the pre-completion answer `unknown`, not `satisfied`.

Polarity is a linear variance walk, once each operator has a declared variance. It is not a solver. It is unsound where variance depends on a sign: in `a · h ≤ b · c`, increasing `h` tightens the atomic formula when `a > 0` and loosens it when `a < 0`. A walk that classifies the left factor as antitone accepts the clause. `Signed<A>` is in the language (`DESIGN.md:72`). The stated property, `Φ[σ] ⟹ Φ` (`DESIGN.md:201`), still contains the hole on the right. It is not a sentence.

"Reachable exit" names a transition system §6 does not define. On the episode DAG it is graph reachability and ignores Φ. As "some state models `release ∨ refund`" it is the SAT problem above.

A compile-time solver is outside the ledger trusted base and inside the authoring trusted base for every property the circuit does not re-establish. A wrong `unsat` admits a stuck escrow forever; the per-witness branch check will not notice. That is the cost named in the Aeon design (`design.md:50`) and the reason the U0 proposal keeps SMT discharge out of U0 (`UNIFIED-PROPOSAL.md:33,256`). A compile-time-only use does not escape that cost. `unknown` and `timeout` have to fail closed (`docs/MORIARTY-PRODUCT-CONTRACT.md:68`). The recovery-viability obligation is an acceptance-predicate obligation (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:70`). A compiler pass does not discharge it.

### The Episode

An Episode cannot be one proof: a stage that waits on evidence does not have its witness yet, and ZKIR cannot pause. It should be one proof per stage, linked by ledger-authenticated commitments: predecessor head, intent digest, episode id, cumulative budget, obligation roll-forward. That is ledger induction. The architecture allows it for Midnight-resident lineage and says it is not recursive verification (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:79`). U2 may ship exactly that profile (`ROADMAP.md:34`).

A join that verifies two parent proofs inside the circuit is ZR09, owned by U4 (`docs/MORIARTY-BACKEND-REQUIREMENTS.md:23,38`), under a planning assumption of March 2027, with per-transaction recursion explicitly not assumed (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:87`). §2 and §13 (`DESIGN.md:35-38,306-307`) put joins in U3 and never say which join. The ledger-linked reading keeps U4 in U4. The in-circuit reading drags U4 into U3. Compensation has to be a new stage whose `on_refund` effects are checked on their own domain. Folding inverse deltas into one cross-domain sum is the global rollback the per-domain law (`DESIGN.md:221`) exists to refuse.

### Bounded execution

A compiled tree evaluates in `O(nodes)` straight-line instructions and fits a stage once the cap is a number. Exhaustiveness is not a stage; running SAT inside one would violate the bound. The §10 intent, specialized, with comparisons and a short Poseidon digest, sits next to the measured `decide` circuit (225 instructions, 2,507 rows). That is an analogy, not a measurement of MIL.

The worst case is the cap. One literal `checkedMul` is already 57.8% of the loan `accrue` proxy (`openspec/changes/aeon-refinement-integration/design.md:78-83`), and that proxy overweights lookups. A runtime `u128` `mulDiv` is a larger gadget. A cap of hundreds of nonlinear nodes does not fit a stage anyone has proved. Comparison nodes and nonlinear nodes need separate caps. No such number appears anywhere in MIL/1. `docs/FOOTGUNS.md:132-136` already requires explicit sizes.

### Which §11 properties hold

1. **One Φ.** Right freeze. False of the examples: `failed`, `never`, `funded`, `exercised`, `equity`, `settle`, `return` are not in the grammar (`DESIGN.md:126-142` versus `:255` and `CATEGORY-MAP.md:48,108`).
2. **Declared holes.** Holds for holes whose types exist. `Route` and `Program` are not defined (`DESIGN.md:194-196`).
3. **Monotone completion.** Does not hold. See the sign counterexample.
4. **Counterexample rejection.** A failing evaluation has a computable object: one false leaf and the witnessed values of its atoms, in one walk. A minimal unsatisfiable core is the wrong object and, for Φ₁, an iterated SMT loop. The useful authoring witness for a bad escrow is a model of `release ∧ refund`, or of `after(deadline) ∧ ¬release ∧ ¬refund`.
5. **Six outcomes.** The right vocabulary, inherited from the product contract. No checker is wired to them.
6. **Per-clause cost.** Computable as worst-case operator cost at the profile width, independent of runtime values, dependent on width and on constant-folding after completion. An author-declared number can say `1` on a `mulDiv`. The digest dwarfs any clause and is not a clause cost. The compiler computes the table.
7. **Footprints.** Holds for ground cells.
8. **Syntactic trust.** Holds at compile time. Insufficient at proof time unless `anchored` is tied to a ledger read.
9. **Canonical bytes.** Achievable, and not yet an encoding. `and`/`or` commutativity, hex case, and absent-versus-default change the digest. U0 cannot hash-bind what is unspecified.
10. **Residue.** Safe for the relation when it is an opaque digested string that Φ does not parse. A model can delete an expressible clause, mention it in residue, and get `satisfied`. The field records the gap the way *[df]* `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:729-733` uses residue. It also conceals a clause the grammar could have carried. I disagree that it is the most important machine-authorship property (`DESIGN.md:289`). The iterable failure object is what stops a model looping.

## Defects

1. **The showcase fails §6.** `delivered(B, ≥ 20) ∧ after(validity.end)` is a model of `release ∧ refund`, and a state before the deadline with nothing delivered models the negation of `release ∨ refund ∨ after(deadline)` (`DESIGN.md:251-256` against `:187`). `failed` does not parse.
2. **Those two formulas are the wrong theorems.** Validity of the disjunction forbids the waiting state. Unsatisfiability of the conjunction forbids the late-success/refund race U3 has to demonstrate (`ROADMAP.md:39`). Locked collateral, `refund never` (`CATEGORY-MAP.md:48-49`), is a normal product and is ill-formed under this rule.
3. **No negation.** The grammar has none (`DESIGN.md:133-142`). The derived state `not terminal` (`:185`) and `not exercised` (`CATEGORY-MAP.md:108`) are not in the language. Bounded atomic `not` is decidable and is what exclusive refunds are made of.
4. **`mulDiv` is mistyped for its own examples.** The declared type is `Qty × Share × Share → Qty` (`DESIGN.md:78`). Lending applies it to a value, an LTV, and `1` (`CATEGORY-MAP.md:57`). Terms have `+`, `−`, `mulDiv`, `min`, `max` only (`DESIGN.md:130`). `outstanding · (1 + bonus)` (`CATEGORY-MAP.md:59`) is not a term. The cross-multiplied prop form does not produce a seize amount.
5. **The wider intermediate wraps the field.** `r` is the 255-bit BLS12-381 scalar (`wiki/zkir/zkir-instruction-set.md:92`). A product of two `u128`s is 256 bits and is not a native `mul`. `less_than` cannot take 256 bits; the chip stops at 253 (`wiki/contradictions.md:116`). Lowering `a·d ≤ b·c` as `mul` then `less_than` is silent modular coercion, which the numeric profile forbids (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:52`).
6. **`unbounded_within(Scope)`** (`DESIGN.md:157`) contradicts finite episode budgets (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:83`) and "bounded everything" (`DESIGN.md:22`).
7. **Two conservation laws want the same freeze.** MIL's law is `Σ balance deltas = supply delta` (`DESIGN.md:221`). U0's E1 sets both sides to zero (`UNIFIED-PROPOSAL.md:90`). Freezing both contradicts every issuance program. S0 can specialize the general law by fixing the supply delta at zero.
8. **Category-map "expressible" is often prose.** Governance's policy commitment (`CATEGORY-MAP.md:158`) is one sentence, not a Φ clause or a stage field. *[df]* `lean/DefiKernel/Typed/Transition.lean:19-28` shows what a specified footprint looks like: reads, writes, and supply deltas on the transition. MIL's independence check is in that family only after cells are ground.

## Missing

**In the MIL/1 freeze.** Version header with numeric caps: Φ depth, Φ nodes, nonlinear nodes, `min`/`max` nodes, `k_of_n` width, effect lines, footprint cells, episode length, fan-in. `Qty` width. The limb rule for products that do not fit in `r`. Poseidon domain tag, residue included, unknown tags rejected. Operator variance table. Acceptance-clause list (everything the circuit evaluates). Episode link fields: predecessor, cumulative budget, episode id. Atomic `not`. A product that yields a quantity. Delete `unbounded_within`.

**Library, after that freeze.** Fee tiers, TWAP, ACTUS schedules, vote counting, insurance pools. Governance process. Matching strategies.

**Later milestones.** U1: measure specialized-versus-universal digest cost; certify checked add and one wide comparison. U2: one specialized two-asset escrow, ledger-checked signature, hostile mutations of digest, amount, clock, and asset. U3: ledger-linked joins and the late race, still without recursive verification. U4: recursive certificates and imported-evidence verification. Post-U2, optional, named in the trusted base: a complete Φ₁ solver whose `unknown` rejects. N-party clearing. A bounded intra-stage trace if flash loans ever return.

## Disagreements

I agree with one predicate language, per-domain conservation, base-per-quote, debt distinct from supply, residue as an opaque field, and flash loans sitting outside an atomic stage. ZKIR could unroll a bounded internal trace; the exclusion is the stage statement, and it is the right product cut.

I disagree that §6 closes recovery viability. The acceptance predicate still has to establish a closure path.

I disagree that disjoint-footprint joins are a candidate for n-party clearing (`CATEGORY-MAP.md:39`). Every order writes the pool. Intersecting writes are interference. *[df]* refuses a missing write (`Transition.lean:59`); it does not treat shared writes as parallel. Clearing needs a multi-signer stage with one shared write set. That open item is real, and the suggested carrier cannot work.

I disagree that competing-slash order can stay open while encumbrances are core (`CATEGORY-MAP.md:206,249`). Two `enforce` effects write one cell, so footprint disjointness rejects them. A total order on seizes of one id is a core rule, short of a slashing library.

I disagree with "no global rollback by construction" (`DESIGN.md:189`). The anchoring rule blocks one lie, an anchored foreign fact. An episode-level sum across domains is another, and only the compiled per-domain equation stops it.

I disagree that "the U0 item is the whole of it" (`DESIGN.md:310`). Below.

## Top three changes

1. **Rewrite `DESIGN.md:187` and delete the §12 fallback (`:296-297`).** Per witness, the circuit enforces a signed priority or an explicit `race`, with one branch bit. The authoring check is `after(deadline) ⇒ release ∨ refund`, decided on Φ₀ by difference logic, `unknown` rejecting, solver absent from U0. The circuit evaluates every acceptance clause. Poseidon digest in, ledger signature over that public input. A clause omitted from evaluation is absent from the relation, and no negative control repairs it.

2. **Put a width rule next to `DESIGN.md:78-80`.** `Qty` is `u128`. A product that does not fit in `r` is a two-limb gadget, never `mul` plus `less_than`. Generalize `mulDiv` to quantities, or add a second operator and stop using the share-typed one on LTVs. Cap nonlinear nodes per stage (four is a plausible U3 budget, estimated from the accrue proxy, not measured). Add atomic `not`. Delete `unbounded_within` (`:157`). Add the variance table §7 needs, with `Signed` factors excluded from the syntactic walk.

3. **Split `DESIGN.md:303`.** U0 freezes the versioned grammar, caps, Poseidon domain, locus of each field, footprint sort, authority enum including `issue` and `enforce`, residue as opaque digested bytes, and conservation as `Σ deltas = supply delta`, with S0's supply delta fixed at 0 so E1 (`UNIFIED-PROPOSAL.md:90`) is a specialization. U0 does not include the unsat checker, polarity-as-theorem, universal interpreter, or Episode proofs. Deferring the grammar lets U2 sign the twelve-field profile (`deliverables/u0-study-2026-09-28/ARCHITECTURE-COVERAGE-REPORT.md:117`) and leaves escrow as a second profile with no pinned correspondence. Deferring the encoding or the limb rule freezes a digest and a certificate that later arithmetic cannot share.
