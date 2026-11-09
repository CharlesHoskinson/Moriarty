BUNDLE: base=49bb0200b848b72e45eb377c1ed63936874cc427 head=49bb0200b848b72e45eb377c1ed63936874cc427 diff=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 manifest=f7519e46987713d8a1025d5644b1646e14ce98aacaf8deea678a25c1f08c47ae

VERDICTS

| Item | Verdict | One-line reason |
|---|---|---|
| 1 Soundness of composition | OBJECT | The conditional segment theorem is sound, but L-D does not justify the required genesis construction, and the shape abstraction loses slot information. |
| 2 Generality | OBJECT | The uniform abstract relation supports arbitrary finite shapes; the planned chain and tree circuits require different keys, and F20 does not exhaust every bracketing. |
| 3 S-slot leaf semantics | OBJECT | The intended slot behavior is clear, but Lean assumes its main semantic obligation, and the one-step `rules_d` requirements conflict. |
| 4 Gate criteria | OBJECT | G-A has a dependency cycle; G-A′ requires clarification of unavailable measurements; U/F selection contradicts the prescribed S choice. |

FINDINGS

- id: 1.1
- item: 1
- severity: OBJECT
- location: `lean/Aggregation.lean:1153–1179`; `lean/AggregationAxioms.lean:23–25`; `research/tree.md:68–79`; `research/builds.md:63–69`.
- problem: L-D models accumulation as a binary fold with `zero_add`, and assumes `scale false a = zero` as an equality of accumulator values. The implementation retains bases when scaling, and its folding challenge hashes all input encodings. Dropping a neutral input therefore need not preserve the accumulator or challenge. Furthermore, `genesis_scale_by_bit` leaves `carried` untouched: it proves neither realization (a) nor the required realization (b) for `prev_acc`.
- proposed change: **Lean statement and design:** replace the claimed implementation correspondence with a transcript-aware statement. For (a), state the constrained neutral carry encoding explicitly. For (b), cover both predecessor-proof and carried accumulators, including normalization and the off-circuit mirror. Establish ignored-carry irrelevance and invalid-leaf refusal under the declared accumulation-soundness premise; do not require equality after deleting fold inputs.
- evidence: `AccAlgebra.accumulate` uses `alg.add` at lines 1165–1167; the theorem removes only `proofAcc` at lines 1172–1179. The actual encoding-sensitive collapse requirement appears in `code/ivc.rs:353–361`.

- id: 1.2
- item: 1
- severity: OBJECT
- location: `model/aggregation.qnt:156–162,179–185,379–386`; `model/aggregation_test.qnt:270–277`; `research/tree.md:147–159,177`.
- problem: The manifest abstraction is a flattened certificate list without slot bits or tree structure. By inspection, moving a unary child from L to R, with no local leaf, preserves `accOf`, the segment, coverage, and depth. The model therefore cannot distinguish this shape change. Its shape mutant merely truncates the list. Also, manifest checking is placed inside `canDecide`, although the design assigns manifest replay to `verify_chain`.
- proposed change: **Quint model and F8 interface:** represent the manifest as structured nodes containing slot bits and child references. Represent the actual ordered proof/carry fold inputs, including disabled-slot treatment. Separate decider acceptance from bundle replay verification, and add a mutant that changes L/R placement while preserving the flattened certificate sequence.
- evidence: `accOf` concatenates L, leaf, R and self without slot markers at lines 157–162; `canDecide` checks only equality with that list at line 182. F8’s actual fold distinguishes left-proof, left-carry, right-proof, right-carry and leaf inputs.

- id: 1.3
- item: 1
- severity: note
- location: `code/ivc.rs:311–321,327–377`; `design/requirements.md:26–27`; `qa/ivc/REPORT.md:243–245`.
- problem: The current code still lacks the required height and step range constraints and folds an unauthenticated `prev_acc` at genesis. These are known **code gaps**, already forbidden by SRC-004/005, rather than missing normative requirements. The SHKIVC05 collapse fixes predecessor-proof encoding parity; it does not complete carry hardening. The independent model’s proposed hardening also has a recorded failing test.
- proposed change: **Code and refinement obligation:** complete task 2.3’s existing checks: bounded `prev_h`, `steps`, and resulting `h`, with `1 ≤ steps ≤ S`; select and document genesis realization (a) or (b). Connect the height-derived genesis bit to the model’s absent-predecessor case using positive, bounded certified heights. Resolve the independent hardening-model failure before citing that proposal as validated.
- evidence: Only nonzero `steps` and field addition are constrained at lines 315–317; `prev_acc` enters the fold unscaled at line 377. The report explicitly limits cancellation to its algebraic model and records 9 passing tests and one failing hardening test. This does not establish a practical cryptographic attack.

- id: 1.4
- item: 1
- severity: concern
- location: `lean/Aggregation.lean:176–215,932–944,979–980,1042–1075,1091–1119`; `design/design.md:31,115–117`; `research/tree.md:163–166`.
- problem: The central conditional argument is convincing: linkage composes exact runs, positive leaf lengths give ordered coverage, and decider plus contract pinning selects the committed program and initial state. However, the design does not clearly name all applicability conditions. Result independence across different chunkings needs determinism and terminal halting. `Carrier` also needs an explicit interpretation compatible with internal `StepInv`/`decodeStep` boundaries. Finally, F8’s explanation incorrectly suggests that a nonempty transition must change its state digest; positive height, rather than digest inequality, prevents duplicate-position reuse.
- proposed change: **Design and premise documentation:** name determinism and terminal halting for L-C/result uniqueness; define the carrier domain used at internal boundaries; correct the coverage explanation to rely on positive, nonwrapping heights. Preserve the distinction between certificate extraction, leaf soundness, structural composition, and contract replay enforcement.
- evidence: `leaf_sound` is a hypothesis at lines 204–208; `Decided.cert` assumes a certified tree at line 1043. Determinism and terminality are explicit hypotheses at lines 1092–1093. Replay is separately enforced by Quint’s accepted-statement set in `model/aggregation.qnt:186–188,366–370`.

- id: 2.1
- item: 2
- severity: OBJECT
- location: `design/design.md:14,22,74–79,115`; `research/tree.md:144–147,178,300–320`; `code/ivc.rs:53,183–188`.
- problem: One uniform F8 circuit could serve both chain and tree schedules. The implementation plan instead retains the linear SHKIVC05 relation and optionally introduces SHKTRE01, with a different public statement and a re-keyed decider. Thus the bundle supports one key set per selected node relation, not the brief’s literal requirement that one key set serve both aggregation shapes. Calling the chain a degenerate tree in the mathematical model does not make those circuit keys interchangeable.
- proposed change: **Design and key policy:** require the uniform relation for both serial-chain and tree schedules, or explicitly amend the requirement to permit choosing a node relation before freezing keys. State that changing keyed S also changes the key closure; independence from S concerns the final execution statement, not identical VKs or accumulators.
- evidence: F8 replaces `[vk, prog, init, cur, h, acc]` with interval endpoints at lines 144–147 and explicitly requires a new node relation and decider key at line 178. The S-dependent cascade is acknowledged in `research/prover.md:210–215`.

- id: 2.2
- item: 2
- severity: concern
- location: `model/aggregation.qnt:18–35,63–79`; `model/aggregation_test.qnt:118–187,204–211,364–412`; `model/check-f20.sh:12–37`; `lean/Aggregation.lean:682–703`.
- problem: F20 supplies bounded execution evidence: lengths 1–9, four S values, sampled bracketings, and bounded invariant sweeps. It does not exhaust every bracketing as requested. Its trace-index digests also make state boundaries reveal height, which is stronger than ordinary state-digest injectivity. Several mutants inject malformed states directly; for example, the range mutant also contradicts the slot count. Lean’s arbitrary-shape results establish equality and soundness conditional on successful certification, not existence of a certificate for every permitted execution and shape.
- proposed change: **Formal gate and model:** label the existing coverage precisely. Enumerate the promised bounded bracketings, add isolated constraint-removal mutants, and include state/height-independent examples where appropriate. Add a Lean construction theorem from a valid finite execution to certificates for permitted chunkings and bracketings if universal representability is to be claimed.
- evidence: Random bracketing is defined at test lines 150–187; samples and sweep bounds are explicit in the runner. Lean’s shape/S statements take accepted certificates as hypotheses. Heights are bounded in Quint by 2^62; exact field arithmetic is separately addressed by Lean L5.

- id: 3.1
- item: 3
- severity: OBJECT
- location: `lean/Aggregation.lean:35–42,85–89,204–208`; `model/aggregation.qnt:73–94,194–197`; `research/prover.md:359–361`; `design/tasks-excerpt.md:23`.
- problem: The intended S-slot semantics are suitable, but F21 does not prove the requested slot-composition theorem. `Leaf` contains only endpoints and a count; exact execution is supplied wholesale by `leaf_sound`. There are no physical slots, enable bits, disabled identities, publication events, or enabled-count derivation. Quint constructs a correct prefix by definition, and `canLeaf` prevents stepping past its synthetic halt; it does not independently establish the general slot-to-`Leaf` bridge.
- proposed change: **Lean statement and Quint model:** add a slot-level relation with boolean enables, lane linkage, disabled identity, enabled-count derivation, and terminal publication followed only by disabled slots. Prove its projection supplies `leaf_sound`’s execution component for arbitrary S ≥ 1. Add independent malformed-slot cases and the one-enabled-slot case at capacity S > 1.
- evidence: The missing induction is explicitly requested as L-B in `research/prover.md:359–361`. Task 4.2 requires agreement with the formal slot semantics, while Lean currently assumes the resulting run at lines 204–208.

- id: 3.2
- item: 3
- severity: OBJECT
- location: `design/requirements.md:23`; `design/design.md:42,48`; `research/prover.md:210–212`.
- problem: SRC-001 requires `rules_d == rule` whenever `steps == 1`. The design instead specifies a Poseidon digest of S selected rule IDs, using the raw rule lane when S=1. For a capacity-eight leaf with one enabled slot, those prescriptions do not define the same value without an additional special case. Disabled-slot rule-ID encoding is also unspecified.
- proposed change: **Design and leaf statement contract:** define one exact `rules_d` encoding, including disabled slots and count binding. Either restrict raw-rule compatibility to keyed S=1 or define a `steps == 1` special case consistently. Add a capacity-S, one-enabled-slot conformance case before keygen.
- evidence: SRC-001 says “when `steps == 1`”; design line 42 says “for S=1.” Short leaves make these conditions different.

- id: 4.1
- item: 4
- severity: OBJECT
- location: `design/tasks-excerpt.md:11–15`; `design/design.md:111`; `research/order.md:123,142–146,177`.
- problem: G-A requires A16, but G-A blocks node keygen 2.4. A16 is measured by 2.6 after keygen and uses the G-A thread choice. Task 2.3 also consumes G-A’s provisional S while supplying the fixed-node row result used by G-A. The stated ordering is cyclic.
- proposed change: **Gate:** separate a preliminary decision using C0 and cost-model results from post-keygen measurement confirmation. Define when the provisional S_MAX is selected, when final S replaces it, and whether failed confirmation triggers a local key cascade. Make the task dependencies match that acyclic sequence.
- evidence: Task 2.15 requires “A16 or its recorded shortfall” and blocks 2.4; task 2.6 produces A16 at the G-A thread count. Recording a shortfall must not silently substitute for an unavailable measurement.

- id: 4.2
- item: 4
- severity: OBJECT
- location: `design/tasks-excerpt.md:21–25`; `design/design.md:50`; `research/order.md:79`.
- problem: G-A′ precedes leaf implementation and keygen, yet its criteria include leaf RSS and projected time at the chosen S. Actual leaf time and RSS are obtained at 4.4. The gate does not identify which inputs are forecasts, how they are bounded, or how a failed first measurement changes an allegedly final S.
- proposed change: **Gate:** distinguish preimplementation row estimates and resource forecasts from measured A20 acceptance. Define forecast provenance and uncertainty, then specify the reduction/re-key path if the selected S exceeds the measured cap. Do not require a measured receipt before the relation capable of producing it exists.
- evidence: Task 4.10 precedes 4.2 and requires leaf RSS ≤16 GB; task 4.4 records actual RSS and proving time at the chosen S.

- id: 4.3
- item: 4
- severity: OBJECT
- location: `design/requirements.md:35–36`; `design/design.md:50,94`; `design/tasks-excerpt.md:22,25`.
- problem: SRC-013 mandates F when k_U >17, and F mandates S=1. SRC-014 and G-A′ prescribe S=4 at k_U=18. Other wording makes F merely an option. These rules cannot select one consistent relation and S at that boundary.
- proposed change: **Design, requirements and gate:** publish one decision function for U/F and S before 4.2. Decide whether U at k_U=18, S=4 is admissible; update every conflicting clause and account for measured batching overhead rather than treating approximate row growth as an exact guarantee.
- evidence: The conflicting branches occur directly in requirement rows 35–36; task 4.4 otherwise postpones U/F selection until keygen.

- id: 4.4
- item: 4
- severity: concern
- location: `research/tree.md:480–487`; `research/order.md:81`; `design/tasks-excerpt.md:12`; `design/design.md:109–111`.
- problem: Low solo CPU utilization is called equivalent to low concurrent slowdown, and some gate text permits `f3 ≤1.4 or U ≤6`. Low CPU use does not bound memory-bandwidth contention or aggregate RSS. The documents also alternate between conjunctive admission criteria and alternative reasons to consider the tree.
- proposed change: **Gate:** require measured f3 and aggregate memory for adoption; use U only to motivate the concurrency experiment. Separate reasons to investigate from mandatory admission conditions. Preserve the recorded F22 kill unless a revised relation passes a new probe.
- evidence: Tree criterion 2 calls U “Equivalent,” while the task uses “or.” The existing F22 failure is recorded in `design/tasks-excerpt.md:7`.

- id: 4.5
- item: 4
- severity: note
- location: `research/builds.md:74–90,94–110`; `pilot/pilot-smoke.md:9–20,111–132`; `pilot/pilot-amendment-1.md:90–98,120–123`; `qa/c6/REPORT.md:309–324`.
- problem: Skeletons make obligations reviewable but do not discharge them. The smoke has low mutation coverage and defective grading; its amendment requires regrading. The C6 report also records defects found beyond local gadget checks and says they were subsequently fixed. None supplies universal leaf soundness or Rust-to-model refinement.
- proposed change: **Gate and skeleton briefs:** preserve the L/Q/M/J evidence labels and seven-lane binding contract; require the amended regrade before pilot inference. Use the C6 findings to seed shared-carrier and dispatch-compatibility checks, with refreshed model/source fingerprints. Keep skeleton and reviewer outputs as inputs, not formal-gate evidence.
- evidence: `research/builds.md:96–101` explicitly leaves refinement unchecked. The amendment requires six smoke regrades and excludes advisory lane verdicts from pilot outcomes.

ABSTENTIONS (if any): None. This is a read-only assessment of the supplied statements, source and recorded results; no build, proof, model run or test was executed.