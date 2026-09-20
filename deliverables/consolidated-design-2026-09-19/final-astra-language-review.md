# Final independent language review — frozen candidate v1

Verdict: **approved — DESIGN CONSOLIDATION ONLY**.

I read the complete `review-candidate-v1.txt` (665 lines), including all eleven files, and the complete `candidate-v1-manifest.json`. I checked the embedded file contents against the manifest: all eleven names, byte counts and SHA-256 digests match after removing the packet's separator newlines. No candidate file was edited. No proof, build, deployment or implementation acceptance is claimed.

## Findings requiring substantive design changes

None. The candidate honors the user's product constraints and the newer full-recursion planning direction. The unresolved implementation obligations are explicitly open and do not prevent approval of this specified-only design consolidation.

## Constraint and preservation checks

- **Permissionless language:** ROADMAP vision, consolidated design Vision/Responsibility boundary, and UNI-001 preserve independent public authorship, compilation, proving and deployment. Application owner consent is distinct from project admission. The optional kernel is not required for direct Midnight use.
- **Native target and no Lean dependency:** the public pipeline ends at native Midnight ZKIRv3, with explicit Compact correspondence if used. UNI-003 and ZR01/06/11 preserve soundness/completeness and actual ledger integration. The qualified-successor wording does not invent a version number or replace native proving. DeFiFormal remains a semantic reference, not a deployed runtime or transferable native theorem.
- **Small core and certified basis:** typed bounded semantics, exact quantities, explicit effects and assertions, separate authority and persistent duties are coherent. Jets preserve values, refusal, effects, preconditions and declared costs; caller framing and invalid-witness exclusion remain required. PR17 is correctly limited by its older surface, WShape and concrete constraint premises.
- **Authenticated intention:** UNI-004/005 and the canonical stage statement preserve gross caps, fees, net outcomes, liability consent, allowed solver choices, disclosures and recovery. Alternate valid routes remain admissible. The candidate does not equate identity across representations with equal hashes.
- **Partiality and liveness:** the design distinguishes uncommitted candidates, accepted phase effects and committed workflow stages. Authorized guaranteed effects may create duties; mere request receipt cannot impose recipient duties. Stage termination does not imply lifecycle completion. Initiate/complete/reconcile/recover/disclose/amend authorities have explicit scopes without forced perpetual authority. UNI-006–008 preserve relevant failure and recovery behaviors.
- **History and MC03:** ROADMAP U4, UNI-009/017, ZR02/04–10/16 and the history section retain genuine two-step native recursive financial evidence. Ledger induction is an explicitly different mechanism and cannot close MC03, full MC05 or full-agenda release. The old categorical rejection of DAG history is expressly superseded.
- **Private composition and MC06:** U4, UNI-009/010, ZR09/14/16 and traceability retain private successor, split, branch histories, bounded multi-parent join, separate-party handoff, resource uniqueness, authenticated completeness, disclosure and witness availability. The five retained composition operators remain U4 scope. Full MC06 has not been replaced with generic aggregation or a ledger-head argument.
- **Future backend:** the sixteen ZR proposals distinguish mandatory functional/correctness needs from measured performance. They cover relation/VK/genesis binding, arbitrary satisfying witnesses, verification guards, deferred accumulator/decider completion, exact effect/currentness integration, resource bounds and independent retained verification. Dedicated opcodes, VK strategy and accumulator construction remain implementation choices. Approximately March 2027 is consistently a dated user planning assumption, not verified release support.
- **Kernel and external assumptions:** UNI-011–013 and backend integration requirements distinguish ZK, threshold authorization, TEE, external finality and actual delivery. Bare-threshold compromise is acknowledged. OWS/x402 neither grant unrestricted authority nor prove service delivery. Concurrent budgets and logical retry identity survive failover.
- **Roadmap preservation:** U0–U7 is the single sequence; P/C/K are aliases and MC/SP/G obligations and resource histories remain incorporated. MC02 integration does not silently become certified. MC07 conformance counts and scope remain explicit; U7 requires remaining release predicates. The bounded Pel artifact is explicitly symbolic and unready until concrete interfaces, checks and limits are bound.

## Minor editorial corrections and optional clarification

1. `openspec/changes/consolidated-language-kernel/requirements.md`, opening paragraph: change **UNI-001–016** to **UNI-001–017**. The specification includes UNI-017 for ZR01–ZR16. This is a nonblocking stale range; the normative clause and backend linkage are present.
2. Optional precision: `specs/consolidated-language-kernel/spec.md`, UNI-014 hostile witness, narrow “rounded reciprocal” to “an incorrectly rounded reciprocal, or reuse of an already-rounded reciprocal without the required error/directed-rounding correspondence.” Correctly specified directed rounding can be legitimate; the surrounding positive scenario and design already make the intended distinction clear. No change to the financial requirement is needed.

## Open implementation questions, not conditions on this approval

The chosen native release tuple, concrete signature/recursion costs, adversarial-witness theorem, certified jet basis, rich-source/ledger pipeline, state rollover/recovery realization, private handoff mechanism and exact external adapter enforcement remain to be established in their named U milestones. The candidate does not overstate these as achieved. Before execution, bind the U1 feasibility sub-results carefully so an unavailable recursive interface does not block unrelated U2 slice work; the current roadmap already permits scoped basis dependencies and independent preparation.

This approval establishes that the proposal is coherent and preserves scope. It is not a formal theorem, native capability verification, implementation audit, campaign authorization or release approval. The immutable v1 manifest remains the reviewed candidate identity; subsequent edits should retain this review with their explicit dispositions.
