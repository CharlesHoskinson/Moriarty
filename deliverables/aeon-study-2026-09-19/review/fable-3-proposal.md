**Independent advisory: Aeon concepts for Moriarty**

**Position.** I agree with the initial author's narrow-native-layer preference but reject its ordering and its headline. Synthesis is the least valuable Aeon idea for Moriarty right now. The valuable ideas are explicit evidence classification, static discharge of financial obligations over all admitted inputs, and source-local counterexamples. None of these touch the open Preview, PCD or correspondence gates. All Aeon work must stay off the SP05, SP09 and task 5 critical path and must consume no Preview attempts.

**What already exists versus what is missing.** The roadmap shows Moriarty already has financial pre reads, postconditions with atomic rejection, static action bounds, a runtime work ledger, a complete-lifecycle corpus with rejection and retry cases, and native K agreement on finite cases. In Aeon terms, Moriarty already enforces refinements at runtime and tests them on finite corpora. What it lacks is a compile-time theorem that a postcondition holds for every admitted snapshot, a machine-readable trust frontier, and diagnostics that name the failing state. The initial research says this too, but its roadmap treats holes and synthesis as the first experiment. That inverts the value order.

## Feature classification

| Feature | Class | Present today | Missing capability |
| --- | --- | --- | --- |
| Refinements and VCs | NOW | Runtime checks, finite tests | Static discharge over all admitted inputs |
| Trust reports | NOW | Prose assumption lists in RESULT files | Machine-readable frontier per action and artifact |
| Counterexamples and IDE | NEXT | Typed rejection codes | Concrete failing snapshot with source span |
| Linear capabilities and typestate | NEXT | Kernel and ledger uniqueness checks | Compile-time duplicate-consumption diagnostic |
| Totality and work bounds | REJECT as import | Finite bounds, static work, ledger, reserve | Nothing. Aeon's optional metrics are weaker |
| Qualifier inference | DEFER | None | Only after exact fragment is documented |
| Typed holes and restricted synthesis | DEFER | None | Needs A1 and A2 as its validator |
| Optimization fitness | DEFER | None | Only as a ranking over admissible candidates |
| General ADTs and polymorphism | REJECT | Fixed schemas | Conflicts with bounded profiles and RP01 |
| FFI | REJECT | None by design | Would create unverifiable trust roots in kernel |
| GP and LLM proposals | DEFER | None | Only behind an independent validator |
| Backend adoption | REJECT | Compact lowering | Aeon runtime is Python, not Midnight |

**Rationale for the two NOW items.** A static verification condition checker for the pure financial expression fragment can be built as a read-only consumer of the existing compiler in the successor source tree. It needs no new evaluator. The 48-constructor runtime already fixes widths, scale, rounding and overflow, so the encoding target is concrete. The trust frontier is even cheaper. Each RESULT file already lists assumptions in prose. Emitting them as a structured artifact costs little and directly serves SP12 gate evidence.

**Rationale for the four REJECT items.** Totality import would weaken an existing guarantee. Aeon returns a trivially true constraint when no metric is given. Moriarty's mandatory finite bounds are stronger and should not be described in Aeon vocabulary. ADTs, polymorphism and FFI expand the language contract that RP01 has not frozen. Backend adoption contradicts the September 10 non-negotiable Midnight target.

**Rationale for DEFER on synthesis.** The reproduced synthesis result is a single constant. A validator that calls type checking on the substituted program is sound only if that type checker is the same static discharge Moriarty does not yet have. Building holes before A1 would create exactly the second-authority evaluator the initial author warns against. Defer until A1 and A2 are accepted and a financial task corpus shows authoring failures that synthesis would fix.

## Ordered roadmap and evidence separation

Four evidence kinds must never be conflated in any Aeon-derived artifact:

- **Compile-time theorem.** A solver-discharged VC over the documented fragment, with named domain, encoding version and solver outcome. Applies to pure expressions only.
- **Runtime enforcement.** Kernel pre and post checks, atomic rejection, work ledger. Already authoritative and unchanged by this work.
- **Finite tests.** The lifecycle corpus, K agreement, adversarial cases. Evidence of conformance on sampled inputs.
- **Ledger evidence.** Finalized Preview transactions with independent readback. The only evidence that closes MC02, SP05 and SP09.

A static VC result is never recorded as ledger evidence. A finite corpus pass is never recorded as a theorem.

**Increment A0: evidence taxonomy and trust frontier.** Scope: a schema and CLI flag that emits, per action and per RESULT artifact, the transitive set of assumptions: snapshot trust, oracle, kernel version, compiler build, solver fragment, proof correspondence status, ledger correspondence status. Prerequisite: none. Acceptance: for the three September 12 audited PRs, the emitted frontier matches the prose assumption lists in their audits, and one intentionally stripped assumption is detected as a diff. Non-goal: no new assumption is discharged.

**Increment A1: static discharge of pure financial obligations.** Scope: encode the eight financial expressions and postconditions of one repayment action into an exact integer theory with explicit width, scale and rounding axioms. Discharge over all admitted snapshots, not one. Report four distinct outcomes: valid, refuted with model, unknown, unsupported. Prerequisite: A0, documented decidable fragment. Acceptance: the lifecycle corpus's four positive stages verify, the nine rejection cases produce refutation models, and two injected encoding mutations produce disagreement with the evaluator. Non-goal: effects, custody, authority, transfers.

**Increment A2: source-local counterexamples.** Scope: when A1 refutes, map the model to a concrete snapshot plus source span, and confirm the evaluator rejects that snapshot with its existing typed code. Prerequisite: A1. Acceptance: every refutation in the corpus round-trips to an evaluator rejection with the same code. A counterexample that the evaluator accepts is a blocking defect. Non-goal: IDE protocol work.

**Increment A3: linear capability diagnostics.** Scope: a static pass flagging duplicate consumption of a transfer or authorization handle within one action body. Prerequisite: A1. Acceptance: the existing duplicate-period test and one new double-spend source case are flagged statically, and the kernel still rejects them at runtime with unchanged codes. Non-goal: replacing kernel or ledger uniqueness checks.

**Deferred D1: typed holes.** Enter only after A1 through A3 and after a reviewed corpus of at least ten authoring tasks where a human failed or was slow. Validator must be A1, not a new checker. No hole survives into an admitted program.

## EARS requirements

- **AEO-01.** The check command shall emit a machine-readable trust frontier listing every assumption on which an action's stated guarantees depend.
- **AEO-02.** When a pure financial obligation is checked statically, the checker shall report exactly one of valid, refuted, unknown or unsupported and shall never map unknown or timeout to refuted or valid.
- **AEO-03.** When the static checker refutes an obligation, the system shall produce a concrete admitted snapshot and source span, and the evaluator shall reject that snapshot with a typed code.
- **AEO-04.** The static checker shall encode exact UInt widths, scale, rounding and overflow semantics identical to the executable runtime, and any divergence detected by mutation control shall block acceptance.
- **AEO-05.** While an Aeon-derived artifact is present, the system shall not record a static result, finite corpus result or synthesized candidate as ledger, PCD or Preview evidence.
- **AEO-06.** When a transfer or authorization capability is consumed more than once in an action body, the static pass shall report it, and kernel rejection shall remain in force unchanged.
- **AEO-07.** If a synthesized or externally proposed term is ever accepted, the system shall have validated it with the same static checker and evaluator as human-authored source, and shall reject any program containing an unresolved hole.
- **AEO-08.** The system shall retain every existing MC and SP gate identity and shall not close, rename or weaken any gate on the basis of Aeon-derived work.

## Foreman Pel delivery controls

Run each increment as one Pel task with the standard implement, verify, review flow. Bind the full verification command under the candidate-full gate to the existing language test suite plus the increment's mutation control. Use an isolated worktree at the admitted base. The review policy must be independent review, with the reviewer instructed to check AEO-02, AEO-04 and AEO-05 explicitly. The Pel task must have no publication step and no network transport to Preview. Exit codes two and three are not deliveries. A0 through A3 need no RP03 campaign admission because they submit no transactions. D1 needs a reviewed corpus amendment before a task is written.

## Risks, counter-position and endorsement

**Three highest risks.**

1. **Encoding drift.** The solver theory and the runtime disagree on rounding or overflow, and a valid verdict masks a real rejection. Mitigation is the mutation control in AEO-04 and a rule that the evaluator is authoritative on disagreement.
2. **Evidence laundering.** A green static check or a synthesized constant gets cited in a RESULT file as progress on SP05 or SP09. Mitigation is AEO-05 and A0's structured frontier, which makes the evidence kind explicit.
3. **Attention diversion.** Aeon work consumes implementer and reviewer capacity that the open Preview gates need. Mitigation is a fixed cap of four small increments and no synthesis until a measured authoring failure exists.

**Strongest counter-position.** A reviewer could argue that all four increments should be rejected because the critical path is Preview settlement and PCD, and no Aeon idea moves those gates. Static checking of a pure fragment is a nice-to-have when the kernel already rejects the same inputs at runtime. This position is defensible. My answer is that A0 is not Aeon-specific and directly serves G01 through G24 evidence hygiene, and A1 is the only route to a compile-time theorem, which SP11 formal conformance eventually requires. But I would accept dropping A2 and A3 if capacity is the binding constraint.

**What would change my opinion.** If the audited corpus already contains a case where runtime rejection came too late to prevent a wasted Preview attempt, A1 and A2 move from NOW to urgent. If the exact integer fragment cannot be encoded decidably within a documented timeout, A1 moves to DEFER and A0 stands alone. If an authoring corpus shows repeated human failures on pure arithmetic, D1 moves to NEXT.

**Conditions for endorsing a shared plan.**

- The plan names A0 and A1 as the only NOW items and keeps them off the SP05, task 5 and RP03 paths.
- The plan adopts the four-way evidence separation and AEO-05 verbatim.
- The plan states in one sentence that no Aeon feature has been implemented and no gate has moved.
- Every increment has a falsifiable acceptance with a mutation control.
- The plan preserves all MC, SP, PCD and Preview gate identities and the two consumed public attempts.
- Synthesis, FFI, ADTs and backend adoption are recorded as DEFER or REJECT with the rationale above, and any reopening requires a reviewed amendment.

Under those conditions I would endorse a consensus plan as research advice. This is not a release gate decision and not a Council receipt.