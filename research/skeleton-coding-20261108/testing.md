# Skeleton Coding and Sherlock VM assurance: local versus end-to-end tests

Source: arXiv 2610.38214v1 (Golatkar et al.). Research input, not evidence.

## 1. Mapping

The paper's chain (§2.2) is `z_i = h_i(x_i, z_{i-1})`. Theorem 2.2 says that when only `z_n` is observed, exact identification needs `2^n` end-to-end tests. When each `z_i` is observable, `4n` local tests are enough. Theorem 2.4 splits the cost into the local tests `Σ N_i(G_i)` plus the compatibility tests `J_S`. `J_S = 0` if and only if the conjunction of the local contracts implies the goal.

| Paper | Sherlock |
|---|---|
| `z_i` (state between steps) | `sherlock-c6-phys/1` pre/post carriers under `StepInv` (24 clauses, `crates/mori-core/src/sherlock/helper_machine/shared.rs:250`; host checker at `shared/owner_runtime.rs:5240`) |
| `h_i` (one component) | One owner-step relation per rule: `c6_<rule>_step.rs` plus `c6_<rule>_consumer.rs` |
| Hidden intermediate inside the dispatch | `Facts` (`c6_dispatch_step.rs:533-557`): `running`, `ct`, `depth_zero`, `fuel_zero`, the 27-way `site` one-hot, `htag`, the helper lanes `h`, `task`, `validate_marker`, `equal_differ` |
| A second component inside the dispatch | `path(facts, rule)` (`c6_dispatch_step.rs:1229-1298`): a pure Boolean function over `Facts` |
| Global composition | The one-hot leaf, `sherlock-leaf-stmt/1`, `rules_d` and `steps` (SRC-003, requirements.md:25), node and decider (O4-O6, assurance.md:14-16) |

Each dispatch circuit is `Facts ∘ path_r` (`c6_dispatch_step.rs:1314-1317`). A circuit asserts only `selected`, so `Facts` is never observed.

**Where current tests observe only end-to-end outputs:**

1. **Dispatch exclusivity** (`c6_dispatch_tests.rs:97-137`). For each pre-state of fact(3) and fold, the test runs the whole circuit of every other rule. Each run rebuilds the full `Facts`: the PROG parser walk, three checked value reads and a 128-bit decomposition of every image and helper lane (`c6_dispatch_step.rs:608-801`). The test runs all of about 100 rules only when `i % 16 == 0` (`:109`) and otherwise runs about 5 neighbours (`:111-124`). With about 100 cached circuits it needs about 47 GB and over 90 minutes per program (`:142`). U023 reached its wall cap after 16,000 s with exit 4 and wrote no receipt (`qa/sherlock/runs.jsonl:394`). The checker also turns every error into a boolean (`:85-87`, assurance.md:70).
2. **Guard coverage comes only from reachable states.** A wrong `Facts` atom or guard is seen only on a state from P1-P3 that both reaches the atom and lets the error change `selected`. `equal_differ` (`:745-801`) is reached only on fact(3) Equal states. This is the singleton-indicator case of Theorem 2.2 (§E.2); SAS-013 (requirements.md:58) names the gap.
3. **The mutation harness** (`crates/mori-backend/src/qa_mutation.rs:835`) shuffles all advice cells uniformly and observes only whether `MockProver::verify` rejects. The recent receipts mutated 3 of 403,843 cells at k=16 and 1 of 2,259,225 at k=18 (tasks.md:32; budgets at `tests/soundness_mutation.rs:811-830`).

**Where the repo already tests locally.** Each per-rule step test compares the complete post-state with the host denotation, for example `c6_return_left_tests.rs:287`. SAS-005 (requirements.md:50) requires complete ordered observations, not final values. Keep them.

**An important structural fact.** Step and dispatch circuits declare every lane as `PublicInput` and have no private inputs (`c6_step_harness.rs:150-200`; `c6_dispatch_consumer.rs:155-180`). At the IR level each fragment is therefore a deterministic predicate over its statement. Two layers remain for under-constraint. The first is the IR predicate itself: is the relation too large? The second is ZKIR-to-PLONK compilation together with the stdlib gadgets, which all fragments share.

## 2. Proposals

**P1. Split the dispatch into a `Facts` contract and a selector contract (Theorem 2.4).**
- *Local test A, the facts contract:* build one circuit that exposes `Facts` as checked outputs, or emits `facts == F_host(pre)` against a new host projection `F_host`. Run it once per step of P1-P3. Add directed single-lane perturbations for each atom.
- *Local test B, the selector contract:* `path_r` is pure Boolean logic over a finite abstract domain. That domain is `ct` × site (27) × `htag` (0..9) × the guard atoms. Check both properties exhaustively per (site, `htag`) group, or in the planned Quint selector model (assurance.md:42):
  - (i) at most one `path_r` holds;
  - (ii) the `r` that holds equals `owner_select` (`owner_runtime.rs:1722`) evaluated on the same abstract facts.
- *Compatibility:* if A is a complete contract (the circuit's `Facts` equal `F_host(pre)` for every pre-state under `StepInv`), then `J_S = 0` for exclusivity. The abstract check covers every realizable pre-state because it quantifies over all abstract facts. The partition by `Facts` is congruent by construction. That is the checked argument SAS-013 needs to remove the heuristic label.
- *Expected saving:* the number of checks per program falls from about (steps × 5 + steps/16 × 100) full-Facts checks to one `Facts` check per step. Exclusivity becomes an exhaustive millisecond-scale evaluation. Peak memory falls from about 47 GB to the size of one circuit. (Estimate.)
- *Gained:* exclusivity holds on every abstract state, not on 1/16 of the visited states.
- *Lost:* nothing, if the facts contract is complete. If A is only tested, `J_S` is still nonzero, and the receipt must say "tested facts contract".

**P2. Emit the selector once in the leaf.** Build one shared `Facts` and all `path_r` as a `sel[]` vector, constrained by `Σ sel = enable` and by `rules_d` (SRC-003). This is what the universal leaf needs anyway (O4). Under P1(B), the dispatch test then becomes one check per state: `sel` equals the one-hot vector of the fired rule.
- *Lost:* nothing; it moves work into the M4 leaf.

**P3. Target mutation at named interfaces.** Run the cell campaigns at three levels:
- **Gadget level, once, exhaustively at small k:** `decompose`, `ConstrainBits`, comparison, `CondSelect`, the checked value read. Every cell is in scope (Proposition 3.1: `Σ 1/α_i` instead of `Π`).
- **IR level:** raw-statement mutation of each named lane read by a guard or written by a rule. These are the `Facts` sources, the post lanes in each rule's frame, and `rules_d`/`steps`. Each candidate needs a `ProofPreimage::check` of milliseconds to seconds, not a `MockProver` run. Label them "IR preprocessing" until a keyed run repeats them (SAS-004).
- **Circuit level:** map region labels (`Allow.region_contains`, `qa_mutation.rs:102-114`) to IR identifiers. Sample cells stratified by region so that every named interface region gets mutants, then fill the remainder at random.
- *Expected saving:* for the same wall time, coverage shifts from about 1e-6 of cells to every named interface.
- *Lost:* the uniform sample's neutrality. Keep a small random stratum. Under Proposition 3.2, `β_i` is the share of under-constraint that the harness cannot express, such as multi-cell coordinated forgeries. Keep swaps and coordinated mutations, and keep a weakened-constraint control per region (SAS-004).

**P4. Decompose `c6_dispatch_tests.rs` along task 3.3.** Use three tests: T1 for fired-rule acceptance plus the facts contract on every step; a seconds-long T0 for exhaustive selector exclusivity; T2 for directed boundary cases per partition class with one budget key per class group. Drop `% 16`; return typed failure classes (SAS-003).

**P5. Make the composition contracts explicit.** Turn the retained dispatch premises (`c6_dispatch_step.rs:34-38`) into named post-contracts of the paired step fragment, so that the step fragment refuses non-canonical IMAGE/HELPER. Add one SAS-010 compatibility test per premise. A per-rule contract must state preservation of all 24 `StepInv` clauses, not only the rule's own postcondition. `StepInv` is the `z_i` contract that lets the per-step contracts compose across the chain.

## 3. Where the paper does not apply

- **Adversarial soundness.** The paper conditions a prior over programs that an agent samples. It reaches "posterior correctness one" because tests eliminate wrong hypotheses (§2, §E.3). An under-constrained circuit is not a wrong program in that prior. It is a relation that admits extra witnesses that an adversary chooses. Tests run on honest inputs check completeness, meaning that the graph of the step relation is included. They do not check that the relation is functional. O2 is universally quantified (assurance.md:12,19). Theorem 2.4 with `J_S = 0` needs complete Hoare contracts, and for circuits only a theorem or an exhaustive checked argument provides one (SAS-009). The proposals cut test cost and widen finite coverage; they do not close O2.
- **Completeness versus soundness.** P1(A) on honest states is a completeness test of `Facts`. It becomes soundness evidence only together with raw-statement mutations (P3), or with a proof that each atom is a function of its lanes, which the all-public statement makes tractable.
- **Domain size.** The `4n` bound needs a 4-entry truth table for each `h_i`. A rule's domain is about `2^(128·lanes)`. Local exhaustiveness exists only after an abstraction (`Facts`), and that abstraction must itself be proved or tested complete.
- **Cross-rule interactions.** A rule's path contains the negations of earlier rules in its group and the P0..P2 precedence (`c6_dispatch_step.rs:25-28,1229-1298`). Its contract is therefore not local to the rule. `BBookkeepingTaskRelease` heads both the Validate and the Equal groups and negates nothing (`:1248-1253`). A per-rule local test misses priority bugs, stale-result versus plan selection (`:1287-1296`), and helper-phase sequences that span steps. SAS-013 requires each priority boundary to be tested explicitly. P1(B) covers this only because it checks the whole selector, not one rule at a time.
- **Imperfect judges.** `ProofPreimage::check` does not exercise compilation (assurance.md:28), so its `β_i` against constraint-level faults is 1. Under Proposition 3.2, accepting a fragment with it says nothing about the compiled relation. Proposition 3.1 helps the factory loop, not the security claim.
