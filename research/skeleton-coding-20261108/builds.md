# Skeleton coding for the remaining Sherlock builds

Source: Golatkar et al., "Skeleton Coding", arXiv 2610.38214v1, read in full (pp. 1-28). Companion: `review.md` in this folder (verdict: pilot). Recommendation: **adopt skeletons narrowly, as the brief for 2.3, 4.2 and the decider; use them for the GPU crate only if G-A opens the track. Never treat a skeleton contract as evidence.**

## 1. What transfers

| Paper result | Transfers? | Reason |
|---|---|---|
| Thm 2.2 (local vs end-to-end, 2^n vs 4n) | **Yes, more strongly** | An underconstrained cell is a point-function bug (E.2): honest traces never reach it; a malicious prover picks it. SAS-003 (`requirements.md:48`) boundary refusals are the paper's local tests. |
| Prop 2.3 (requirements invisible to output tests) | **Yes** | Soundness and row count are invisible to honest output: node tests pass whether or not `prev_h < 2^64` is constrained. |
| Thm 2.1 (structure fixes test cost) | Partly | 2.3 is brownfield (`ivc.rs` exists); the skeleton fixes only the change's decomposition. It matters most for M4. |
| Thm 2.4, J_S = 0 under Hoare contracts | **No** | Circuit contracts compose only over the *same assigned cells*; an unlinked copy breaks it silently (design §2.2). Lean `segment_sound` (`Aggregation.lean:979`) discharges J_S for the model, nothing for Rust. |
| Prop 3.1 (formal checks, cost Σ 1/α_i) | Only for model-level contracts and exact mechanical checks | The Lean package "does not ... establish Rust, constraint or native refinement" (`clean/fork/README.md:4-5`). |
| Prop 3.2 (imperfect checks) | **The regime for all circuit code** | MockProver/mutation tests have e_i ≈ 0 but β_i > 0: they refuse only the hostile witnesses someone wrote. β_i is what to drive down. |
| Empirical numbers (48.4%, 15-26x) | **No** | Python, pass rate (completeness only), reference-derived specs (`review.md` §2). |
| Fig. 2, §4.1 "model fatigue" | **Yes, as a warning** | Task 2.3 is one ~450-word paragraph (`tasks.md:40`); the skeleton must replace prose, not add to it. |
| Fig. 8 (persistent skeleton) | Yes | Prose drifts now: `design.md:83-84` and `tasks.md:40` cite `ivc.rs:308-310`, `:343`, `:364`; the code is at `:313-314`, `:350`, `:368`. |

GPU kernels are the exception. A wrong MSM or NTT yields a proof the verifier rejects: a completeness failure, not a soundness one. Correctness is bit-identity with BLST/`best_fft` (SRC-018, `requirements.md:40`), an oracle exists and differential tests nearly cover the edge classes: the paper's ProgramBench setting.

## 2. Worked skeleton: task 2.3 (`crates/mori-recursion/src/ivc.rs`)

Paper style (App. A vii). Checks: **L** = Lean theorem (Prop 3.1, model level), **Q** = Quint invariant (Prop 3.1, bounded model level), **M** = MockProver/raw-witness negative (Prop 3.2, β > 0), **X** = exact mechanical check (Prop 3.1), **J** = informal judge (Prop 3.2, e > 0).

```rust
/// Relation constant (SRC-014; tasks.md:40): G-A provisional S, 16 unless G-A says otherwise.
pub const S_MAX: u64 = 16;
const MAGIC: &[u8; 8] = b"SHKIVC05"; // bumped: S_MAX and genesis semantics change the relation

impl SherlockIvcRelation {
    /// Requires: nr_cols in 1..=4, self_k in 10..=24.
    /// Ensures:  cost_model(&new(4, 19, leaf_vk)?, Some(19)).k == 19, or the shortfall
    ///           is recorded for G-A and k=20 kept.                          [X: A16; T1 check]
    pub fn new(nr_cols: u8, self_k: u32, leaf_vk: MidnightVK) -> io::Result<Self>;
}

/// Ensures: every satisfying assignment has x < 2^64.
///   [L: lower_than_fixed_correct :1194 (gadget at the bound); M: x = 2^64, p-1 refused]
fn assign_height(std: &ZkStdLib, l: &mut impl Layouter<F>, v: Value<u64>)
    -> Result<AssignedNative<F>, Error>;

/// Ensures: 1 <= steps <= S_MAX (assert_not_equal_to_fixed 0; assert_lower_than_fixed S_MAX+1).
///   [L: leaf_only :441, leaf_out_of_range :447, L-E non-power-of-two bound (Axioms :55);
///    Q: leafSlotOk :130; M: steps = 0, S_MAX+1, p-1 refused at the boundary (SRC-004)]
fn constrain_steps(std: &ZkStdLib, l: &mut impl Layouter<F>, steps: &AssignedNative<F>)
    -> Result<(), Error>;

/// Requires: prev_h < 2^64 (assign_height); 1 <= steps <= S_MAX (constrain_steps).
/// Ensures:  h = prev_h + steps over ℕ, and h < 2^64 is checked directly, not inferred.
///   [L: field_eq_exact :1144, height_no_wrap :1125, weaker_pair_insufficient :1132;
///    Q: heightsOk :319 (HMAX = 2^62, :35); M: prev_h = 2^64-1, steps = 1 refused]
fn next_height(std: &ZkStdLib, l: &mut impl Layouter<F>,
    prev_h: &AssignedNative<F>, steps: &AssignedNative<F>) -> Result<AssignedNative<F>, Error>;

/// Ensures: is_genesis == (prev_h == 0); is_genesis => prev_d == init_d.
/// Refinement premise: "carried slot absent" iff prev_h == 0. This holds only because every
/// certified node has h_lo < h_hi (steps >= 1).
///   [Q: genesisAtNode :139, genesisOk :358; L: non_genesis_refused :1063;
///    J: the premise itself (tasks.md:38 notes the model uses an explicit bit)]
fn genesis_link(std: &ZkStdLib, l: &mut impl Layouter<F>, prev_h: &AssignedNative<F>,
    init_d: &AssignedNative<F>, prev_d: &AssignedNative<F>) -> Result<AssignedBit<F>, Error>;

/// SRC-005 realization, recorded before keygen. (b) shown:
/// Ensures: is_genesis => both proof_acc and prev_acc scale to the neutral element, so the
///          pairing value of the fold equals that of leaf_acc alone.
///   [L: needs NEW lemma genesis_scale_carried; genesis_scale_by_bit :1172 drops only proofAcc,
///    i.e. it models today's code; M: two genesis proofs differing only in a well-formed
///    ignored carry both pass the decider; an invalid leaf at genesis is refused through the
///    final pairing (A13, predicates.yaml:161)]
fn genesis_carry(l: &mut impl Layouter<F>, scalar: &ScalarChip, not_genesis: &AssignedBit<F>,
    proof_acc: &mut AssignedAccumulator<S>, prev_acc: &mut AssignedAccumulator<S>)
    -> Result<(), Error>;

/// Requires: lanes == [binding, prog_d, prev_d, cur_d, rule, prev_h, steps] (SRC-001, frozen;
///           the k=19 cut may NOT drop `binding`). Ensures: leaf pairing deferred under the leaf key.
///   [L: Premises.leaf_sound :176 is a PREMISE; M: 6-lane shape refused]
fn verify_leaf(/* as ivc.rs:356-365 */);

/// Ensures: public = [self_repr, prog_d, init_d, cur_d, h, acc]; acc = collapse(accumulate(
///          [proof_acc, prev_acc, leaf_acc])); the statement is stmt(node(carried, leaf, off)).
///   [L: chain_sound :1034 (model); Q: foldLeaf :202, shapeOk :381;
///    J: Rust fold order vs Quint accOf order, unchecked; M: 3-step fold passes (A16, completeness only)]
fn circuit(&self, std: &ZkStdLib, l: &mut impl Layouter<F>,
    _i: Value<Vec<F>>, w: Value<IvcWitness>) -> Result<(), Error>;

/// Ensures: off-circuit next_acc(w) == the circuit's public acc.  [M: ivc_node_folds_three_leaves]
pub fn next_acc(&self, node_vk: Option<&MidnightVK>, w: &IvcWitness) -> Result<Accumulator<S>, String>;
```

Writing it surfaced three facts the prose hides: the L-D lemma models only the realization the design calls insufficient; one k=19 candidate ("binding folded into rules_d", `tasks.md:40`) breaks the SRC-001 freeze (`requirements.md:23`); the genesis premise is unproved.

**M4 (4.2).** Components: `dispatch(pre) -> sel` (SVM-003/004, `requirements.md:11-12`), one `fragment_r(pre, sel_r) -> cand_r` per rule with a declared write set, `select` (design §2.2(4)), `slots(S) -> (rules_d, steps)` with Quint `leafOk` (`aggregation.qnt:84`) as contract. Fragments are Prop 3.1/3.2's independent components: resample each until its own `soundness_mutation` run passes (`crates/mori-backend/tests/soundness_mutation.rs:1-13`). **Decider:** one SRC-006 contract, L `decider_pinning` (`:1070`) plus M public-value mutation.

## 3. Connecting to Lean and Quint

At the model level the Lean and Quint definitions *are* the contracts: `NodeCert.stmt S (node c (some f) off)` (`Aggregation.lean:94`, `chainOf :125`) is the node postcondition and `leafSlot` (`:87`) is `constrain_steps`. Cite, don't restate (fatigue risk). Rust-to-model refinement stays unchecked. Three bridges, by value:
1. **Name binding (T0).** Each `[L:]`/`[Q:]` tag resolves to a `#print axioms` entry in `AggregationAxioms.lean` or a `pure def` in `aggregation.qnt`. Catches name drift, not semantic drift.
2. **Model-based negatives.** Each Quint adversary action (`forge_link`, `reuse_leaf`, `replay`, `shape_mismatch`) becomes a node raw-witness test (`quint-execute-spec`), lowering β_i.
3. **Gadget lemmas.** L-E, `field_eq_exact` and a new `genesis_scale_carried` state the exact gadget arithmetic, closest to the code.

Full refinement (Clean-style circuit extraction) is out of reach for midnight-circuits code and should not be claimed (SAS-009, `requirements.md:54`).

## 4. Workflow change for `tasks.md`

- **Who.** The lead writes each skeleton (a **[lead]** item; `tasks.md:5`) after F20/F21 and before 2.14. The skeleton becomes a 2.14 council input, which makes the review concrete.
- **Where.** In `openspec/changes/sherlock-vm-consensus-20261107/skeletons/<task>.rs`, not compiled, with symbol anchors (`ivc.rs::next_height`), never line numbers.
- **Brief.** The task row shrinks to: goal, the skeleton path, the predicate IDs and the gate. The loop agent gets the skeleton plus the cited Lean/Quint excerpts only.
- **T0.** New row F24 (`check-skeleton.py`, F14 pattern; `workflow.md:48`): each skeleton function exists with the same signature; each tag resolves; each **M** tag names an existing test; each **J** tag appears in the SAS-001 record as "unchecked refinement".
- **T1/T2.** The M tests run where their weight allows. Mutation runs at T1 (`workflow.md:49`). Node negatives at k=19 run under `heavy` (`RUST_TEST_THREADS <= 4`, SRC-016).
- **Ticking.** Predicates tick boxes; a skeleton grants nothing.

**Effort (ESTIMATE), 4-6 lead days total:** 2.3 skeleton 3-4 h; `genesis_scale_carried` 0.5-1 day; 4.2 skeleton 1-2 days; decider 2 h; GPU (from `gpu.md` §3) 0.5 day; F24 checker 1 day.

**Risks.**
1. **Over-detail** (Fig. 2): one Ensures line per function; cite theorems.
2. **Drift** (already shown): F24, re-run at every refactor.
3. **False assurance**: an "L" tag on Rust reads as a proof; keep J/M labels and SAS-009 wording.
4. **Wrong structure frozen** (the k=19 cut may need another decomposition): skeleton edits are lead commits, reviewed in 2.14.
5. **No measurement**: n = 1 per component gives no evidence skeletons helped. Adopt on cost and reviewability; run `review.md` §3's pilot only if a comparison is wanted.
