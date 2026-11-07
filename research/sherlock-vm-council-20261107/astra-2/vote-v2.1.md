# VOTE astra-2 ACCEPT — DRAFT v2.1

I ACCEPT the design in `openspec/changes/sherlock-vm-consensus-20261107/`, reviewed as the saved 13-file snapshot `astra-2/review-v2.1/` with hashes in `astra-2/review-v2.1.sha256`. This supersedes my v1 and v1.1 OBJECT votes. I have no remaining soundness or security objection to the specified design and qualification plan.

The objections are resolved:

1. The dispatch receipt refuses known missing dependencies, and negative evidence must reach its claimed boundary. IR preprocessing, circuit checks, native proofs, target observations and universal claims have separate evidence rules. Independent oracles and raw hostile-witness paths remain required. References: SAS-001..014; assurance §§1–4.
2. Task 4.9 authenticates canonical initial carriers against the committed program/profile and `init_d`; task 4.6 checks invariant preservation and shared-carrier premises. SRC-005 consistently specifies neutral-carry or justified ignore semantics, with a separate security argument required for an unscaled retained carry. Counter bounds, actual leaf statement facts, terminal pairing, forged outcome labels and nonmember family keys have explicit gates. References: SVM-002; SRC-001..006, SRC-013; tasks 2.3, 4.4–4.9, 5.2.
3. Receipts can be committed before `state-commit`, freshness follows declared dependencies, and Preview tooling/evaluator additions are explicit. Optional B work cannot replace mandatory Sherlock fee/security evidence; adopting S>1 or another guarded feature activates its required checks. Recovery records the old on-chain key state and active-state disposition. References: SAS-002; SWF-005, SWF-010..012; SMN-005/007; workflow E7, E12–E14.

The v2.1 document check found 56 unique requirement IDs, matching capability specifications and traceability, and five cells in each assurance O1–O7 row. All snapshot hashes verified. These checks establish document consistency only.

Implementation obligations remain: choose and justify the genesis construction at the pinned library version; execute the hostile-witness, complete-chain and exact-target gates; measure resource costs. The public-source/live-runtime reproduction gap remains stated. Retained financial obligations remain open outside this pure-profile qualification. This council review ran no proof, exploit, build or deployment and established no practical cancellation attack.
