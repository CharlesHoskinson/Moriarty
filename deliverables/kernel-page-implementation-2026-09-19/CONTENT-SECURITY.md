# Content security review — candidate 02

Verdict: **CHANGES REQUIRED** for one substantive threshold-assurance error and one misleading shared-failure summary. The rest of the reviewed security boundary is substantially consistent with the controlling design. One privacy clarification is recommended.

Reviewer: independent delegated GPT-6 Astra review, 2026-09-19. Classification: repository observation and reviewer inference, not a cryptographic implementation audit.

## Candidate and scope

Checkout: `/home/charl/Moriarty-pages-20260919`. All 17 entries in `/home/charl/research/moriarty-kernel-implementation-2026-09-19/candidate-02.json` matched their SHA-256 digests when inspected. No candidate files were edited.

Read the full `site/kernel.html`, `site/src/kernel/evidence.ts`, `site/src/kernel/EvidenceInspector.tsx`, and scenario evidence, release-condition, observation, recovery and history messages in `site/src/data/kernel-scenarios.mjs`. Compared the consolidated design, product contract, backend requirements and approved interactive-page design, plus scoped K, loan, swap and DeFiFormal result records. Startup loaded `moriarty-dev:develop` and inspected guarded status, including `status --json`; no pending transactions. Status reports unresolved implementation admission/accounting inputs, which do not prevent this read-only content review. No network execution, external source acquisition, native proofs or tests were performed. There is no project-root Graphify graph to query; source files supplied the evidence.

## Required corrections

### CS-01 — P2: A signing threshold is not a threshold of honest participants

**Exact claim:** `site/src/kernel/evidence.ts:31`: “That the threshold of honest participants took part, and that each checked the policy first, is an inference under the named protocol and corruption assumptions”. Line 33 assumes only that fewer than the threshold are corrupt. The static row at `site/kernel.html:324` instead says “enough honest participants”, so the enhanced page additionally strengthens the static claim.

**Why it is wrong:** Under the stated toy 3-of-5 model, two corrupt participants and one honest participant can meet the signing threshold. Fewer than three corrupt participants does not imply that three honest participants signed. This is a direct counting counterexample under the page’s own model, not a claim about a particular uninspected MPC implementation. A secure protocol under its specified adversarial bound can require honest participation; the bound and honest policy enforcement must actually support the inference. Generic MPC also does not itself specify a signing threshold, security model or individual approval evidence.

**Controlling source:** `docs/superpowers/specs/2026-09-19-federated-kernel-interactive-page-design.md:92` requires only distribution of signing power under a corruption model, while `docs/MORIARTY-CONSOLIDATED-DESIGN.md:33,97,99` keeps corruption assumptions separate from proof/oracle truth. None supports a threshold of honest approvers.

**Correction:** Replace with: “Under the named threshold-signature protocol and its corruption bound, producing the signature requires honest participation. Any inference that policy was checked also assumes those honest participants enforced the policy before signing. The signature itself does not identify the participants or record their individual approvals.” If retaining a numerical explanation, state that with t required signing shares and at most f corrupt shares, at least t−f honest shares are needed under the protocol assumptions; do not imply t honest signers. Align static and interactive wording. This corrects assurance wording, not the toy threshold configuration.

### CS-02 — P2: Shared operators do not make distinct mechanisms literally “count once”

**Exact claim:** `site/kernel.html:310–312`: “Placing them together adds protection when they bind the same statement. Where one operator controls several of them, those checks share a failure and count once.”

**Why it needs correction:** Common statement binding is necessary, but not sufficient, for a composition to add protection: the relevant verifier must require the checks, and their actual assumptions and failure modes matter. Operator overlap introduces correlated dependencies; it does not collapse ZK soundness, a threshold corruption bound and hardware integrity into one identical assurance. The later static text (`site/kernel.html:329–334`) and inspector (`EvidenceInspector.tsx`, common-binding panel) are appropriately more careful and explicitly retain distinct assumptions. The always-visible lead contradicts that distinction and the intended refusal to give a security score.

**Controlling source:** Approved page design, “Trust, privacy and history views”, explicitly says “Show shared operators as correlated dependencies, not multiplying independent protection.” Consolidated design responsibility table keeps ZK soundness, corruption threshold and hardware attestation separate. The page’s destination discussion itself correctly demonstrates why mere presence of proof checks among honest signers does not enforce them at a signature-only destination.

**Correction:** “Combining mechanisms can add protection when the required checks bind the same statement and are enforced at the relevant acceptance point. Shared operators create correlated failure risks; the distinct cryptographic and hardware assumptions still need separate analysis.” Remove “count once”; keep the existing careful inspector qualification.

## Recommended clarification

### CS-03 — P3: Qualify the privacy statement and name its separate assumption

**Exact claim:** `site/src/kernel/evidence.ts:20`: “The encoded relation holds for the bound statement, without revealing the witness.” The assumptions list names soundness, pinned circuit/key and intended semantics, but does not separately name zero-knowledge or public-statement disclosure.

**Assessment:** This is normal shorthand for a ZK proof, not an asserted deployed end-to-end privacy guarantee. Nevertheless, this is an inspector specifically promising exact assurance and assumptions. Soundness is distinct from zero-knowledge, and public inputs/outputs can reveal facts about a private witness. Other page text properly says proof possession does not supply the successor’s witness, but does not describe the public disclosure boundary.

**Controlling source:** `docs/MORIARTY-PRODUCT-CONTRACT.md:49` separates confidentiality, noninterference and history correctness; consolidated design privacy row names public-transcript, network, observer and hardware leakage; backend ZR14 requires a declared observer and leakage model.

**Suggested correction:** “The encoded relation holds for the bound public statement. Under the proof system’s zero-knowledge assumptions, the proof discloses no additional witness information beyond that statement.” Add that public effects, metadata and authorized disclosures remain visible as specified, and that this is not an end-to-end confidentiality or witness-handoff guarantee. No extra privacy implementation should be implied.

## Claims checked and supported within scope

- **TEE measurement versus computation:** The static row and inspector authenticate measured code/configuration and report data, then explicitly condition input/output claims on measured code binding the I/O and on the verifier’s trust policy. Hardware/root integrity, freshness and rollback assumptions are named. Missing program proofs are not replaced. This satisfies the design’s required qualification; it is a conceptual vendor-root profile, not a claim that every vendor uses an identical quote format.
- **Foreign threshold compromise:** The toy 3-of-5 is labeled illustrative. A signature-only destination can bypass honest signers’ policies after threshold compromise; Midnight cannot undo the foreign effect. The hypothetical checking destination names exactly a relation and consumption check. This matches consolidated design line 97 and avoids claiming a generic proof-enforcing foreign chain. “Any transfer” is understood within the explicitly illustrative account’s ordinary asset/ledger validity rules, not an override of foreign consensus validity.
- **Bindings and identities:** Intention, program, domain, stage/request, epoch/membership and complete effects follow the approved page’s teaching scope. Source/Core/ZKIR/key identities are correctly distinguished and linked by correspondence rather than claimed equal. This compact list is not a complete implementation of the much larger canonical stage statement.
- **External truth and finality:** The responsibility map distinguishes inclusion, execution, finality and delivery. Document evidence meets only its checked predicate; it is not physical delivery or legal truth. A deadline does not establish nonexecution. The fixture’s authenticated-failure release is explicitly policy-specific. Success accounting is separated from acceptance if a predicate is absent. Replay/consumption explanations are fixture/native-boundary claims, not a promise to reverse an external transfer.
- **Recovery and duties:** Unknown outcomes retain reservations; elapsed time and a recovery signature do not permit refunds. Recovery needs authority, evidence, assets, resources and exclusive consumption. Prefix effects persist, compensation is a new action, and accepted duties are not erased by failure. This matches the consolidated and product contracts.
- **Private witness and history:** The continuation disclosure names accepted predecessors, duties, consumed identifiers and witness availability. Shared ancestry is not equated with duplicate spending. Native recursion/private handoff remain requirements work; ZKIRv3 remains the target; a future interface is not advertised as released. Backend ZR09/10/14 and consolidated history requirements support these distinctions. The page does not claim full private-state completeness or a history theorem.
- **Present versus planned evidence:** The page explicitly describes an unsigned, unsubmitted JavaScript illustration and lists the kernel, common ZK/MPC/TEE statement, external adapter, OWS/x402, native recursive history, private handoff and full correspondence as development work. The K `RESULT.md` records finite comparisons, not a theorem. The September 17 loan result records a fixed test-asset payment clearing due amounts while 4,500,000,000 principal remained outstanding, explicitly uncertified for mandatory PCD/I2. The swap result is scoped actual financial execution with raw acceptance flags still false. The website’s limited loan/swap claim is supported without converting those records into general product acceptance. DeFiFormal is accurately a semantic reference with no automatic theorem transfer or Lean dependency.
- **OWS/x402:** The page follows the consolidated design’s scoped wallet authority and payment/availability/delivery distinction, with planned integrations explicitly unproven. The fixed-request retry trace is an illustrative target policy, not evidence that a live x402 integration already enforces it.

## Source map and limits

Primary repository authorities: `docs/MORIARTY-CONSOLIDATED-DESIGN.md`; `docs/MORIARTY-PRODUCT-CONTRACT.md`; `docs/MORIARTY-BACKEND-REQUIREMENTS.md`; `docs/superpowers/specs/2026-09-19-federated-kernel-interactive-page-design.md`; `ROADMAP.md`.

Scoped observations: `deliverables/preview-loan-2026-09-17/recovery-run01/RESULT.md`; `deliverables/sp05-financial-integration-2026-09-09/preview-swap-exit-01/RESULT.md`; `deliverables/k-lifecycle-execution-2026-09-17/RESULT.md`; `deliverables/defiformal-study-2026-09-19/RESULT.md`.

The website’s source list links to the roadmap, which in turn links these K/loan/swap records. Direct links beside each current-result bullet would improve traceability but their omission is not a factual contradiction. This review accepts the retained records as scoped repository evidence; it did not independently replay chain receipts, validate current remote main URLs or establish present deployment status. No generic external protocol/security claim was newly certified. The two required corrections are derived from the page’s own assumptions and controlling design. Candidate approval requires rechecking changed wording against a fresh manifest; this report approves neither native security nor live publication.
