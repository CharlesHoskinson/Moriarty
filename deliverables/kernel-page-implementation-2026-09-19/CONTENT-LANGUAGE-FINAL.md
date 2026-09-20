# Final independent content review: language boundaries

Candidate: `candidate-03.json`, checked 2026-09-19 in `/home/charl/Moriarty-pages-20260919`.
Reviewer: delegated GPT-6 agent, continuing its independent language-content review. No other review was read. No candidate files were edited.

## Verdict

**APPROVE within language-content scope. No material residual issue found.**

Rechecked the complete language-related content of the article, explorer, evidence inspector/fixtures and scenario messages against the approved page design and the normative sources identified in `CONTENT-LANGUAGE.md`. The permissionless product boundary, optional federation, APSS concerns, four separate acceptance judgments, native target and open recursion/private handoff status, external enforcement limits, and DeFiFormal/no-Lean meaning remain consistent with the repository design. Current scoped results, target requirements and illustrative outcomes remain distinguishable.

All five content files match their final candidate-03 SHA-256 manifest entries:

- `site/kernel.html`
- `site/src/kernel/KernelExplorer.tsx`
- `site/src/kernel/EvidenceInspector.tsx`
- `site/src/kernel/evidence.ts`
- `site/src/data/kernel-scenarios.mjs`

## Closure and final changes

**CL-01 closed.** `site/src/kernel/evidence.ts:32` and the static threshold description in `site/kernel.html` now distinguish the signing threshold from honest participation. The t required shares / at most f corrupt qualification gives the appropriate t − f lower bound under the stated protocol assumptions. The text retains the necessary limits: verification authenticates the signature bytes and group key, while policy-checking and honest participation depend on assumptions; it does not identify individual informed approval. This no longer infers t honest participants from fewer than t corrupt.

**CL-02 addressed.** The continuation disclosure now says each stage and predecessor fan-in remain bounded while growing finite history continues within signed episode and lifetime budgets. It does not promise unlimited computation or storage. This matches consolidated design:66,70,83, language alignment:45,49 and backend ZR09/ZR12.

The revised ZK wording separates soundness from zero knowledge and qualifies witness privacy relative to the public statement and intended disclosure. It explicitly excludes an end-to-end confidentiality or witness-handoff guarantee. This matches product contract:49, consolidated design:35,81–85 and backend ZR14.

The evidence section now makes common binding necessary but insufficient: relevant checks must actually be enforced, and shared operators create correlated dependencies while distinct proof/threshold/hardware assumptions remain separately analyzed. This preserves the consolidated design:33,42–46,97–99 boundary.

The custody wording consistently calls the displayed value the last confirmed/accounted escrow balance and states that an unresolved external attempt leaves current location unknown. The duty discharge label now requires every fill to be accepted against all signed predicates. These explanations preserve the existing separation between observation, acceptance, reservation and continuing duties; they introduce no language or deployment capability claim.

Direct links now accompany the finite K and scoped Preview loan/swap result statements. Their local target result files exist and support the scoped wording. These links improve traceability without turning their finite/historical results into full-language proof acceptance.

## Limits

Approval concerns factual language and design correspondence for these exact candidate-03 content bytes. It does not certify reducer behavior, browser rendering, native proofs, protocol security, live upstream releases or present network state. Repository status was refreshed and still reports unresolved campaign bindings and no financial evidence in that runtime store; historical result documents remain separately scoped evidence. No network transactions or proof campaigns were run, and no candidate edits were made. This review does not independently approve publication or replace other required reviews.
