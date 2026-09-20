# Final content security review — candidate 03

Verdict: **PASS_SCOPED**. No unresolved blocking factual findings in the reviewed security/evidence copy. Candidate 03 closes CS-01 and CS-02 and implements the CS-03 privacy clarification. This is approval of the educational content within the stated repository-grounded scope, not an audit of a deployed kernel or native proof implementation.

Reviewer: independent delegated GPT-6 Astra review, 2026-09-19. Checkout: `/home/charl/Moriarty-pages-20260919`. The original candidate-02 findings remain preserved in `CONTENT-SECURITY.md`; this report does not rewrite their history.

## Exact candidate

All 17 file digests in `candidate-03.json` matched the checkout on this review. Manifest SHA-256: `def0be053d711274e9ace2f7f6f5ea3f504deb06145edf61a5cfd3b3f1c35afd`.

Content files inspected:

| File | SHA-256 |
|---|---|
| `site/kernel.html` | `c33ab0367bb9a12938af374e4677ec5c5c63a4f223b27b5ab01843fba18e99d1` |
| `site/src/kernel/evidence.ts` | `f08e662bfe397da37b63148e560585e8f9b212191e51a77440c5502ca3750549` |
| `site/src/kernel/EvidenceInspector.tsx` | `3368fe0d6e5325578af01e3ed3f5dac1a2128bda6d120afe315ba574600665db` |
| `site/src/data/kernel-scenarios.mjs` | `c2a989b14d09c2d7e2e08f36337fd3d9653e2d3da2c02da6988fbe7ad4fcebf8` |
| `site/src/kernel/KernelExplorer.tsx` | `ede763dcf43cc9cd17da764b3795b3e6531ae42fdc89d69a35a720f9f1619e56` |

No candidate files were changed. Guarded `status --json` was refreshed and showed no pending transactions. The existing implementation admission/accounting gaps remain outside this read-only content review.

## Finding closure

**CS-01, threshold assurance — closed.** Both the complete static table and enhanced inspector now explicitly distinguish t required signing shares from honest participation. With at most f corrupt shares, they state the lower bound t−f under the protocol assumptions, not t honest signers. Honest policy checking remains an assumption and signatures are not represented as individual informed-approval records. The corruption model, key-generation/refresh and membership-epoch assumptions remain visible. The toy 3-of-5 example is still illustrative, not a selected production configuration. This aligns with the approved page design’s threshold-distribution claim and the consolidated design’s separate corruption assumptions.

**CS-02, common binding and correlated failure — closed.** The lead now says combining mechanisms *can* add protection when the required checks bind the same statement and are enforced at the relevant acceptance point. It expressly says binding is necessary, not sufficient. Shared operators are correlated dependencies; proof, threshold and hardware assumptions still require separate analysis. This agrees with the detailed static paragraph and inspector. The unsupported “count once” formulation is gone.

**CS-03, ZK privacy — closed.** Static and enhanced copy now names the bound public statement and a separate zero-knowledge property. Public effects, metadata and authorized outputs remain visible as specified. The inspector expressly rejects an end-to-end confidentiality or witness-handoff guarantee. The original relation/soundness, pinned circuit/key and semantic-correspondence caveats remain. This matches the product contract’s confidentiality distinction and backend ZR14’s observer/disclosure boundary.

## Full-context recheck

Re-read the full HTML article, full mechanism/binding/destination fixtures and inspector component, plus the relevant scenario and explorer messages. This was not limited to the replacement sentences.

- **TEE:** Reports identify measured code/configuration and bound report data. An input/output execution claim still depends on measured code binding those values and on a trusted verification policy. Hardware/root integrity, freshness and rollback remain explicit assumptions; a report does not replace a missing program proof. No generic attestation-as-computation-proof claim was introduced.
- **Foreign enforcement:** Signature-only compromise still bypasses honest signers’ off-destination policy checks. Midnight cannot undo the foreign effect. The hypothetical destination lists precisely a signature, named relation and consumption rule rather than claiming universal Moriarty enforcement. Ordinary destination transaction validity is implicit in the illustrative account model; the text is not a claim that a signature overrides foreign consensus rules.
- **Bindings:** Intention, distinct source/Core/ZKIR/key identities and their correspondence, domain, stage/request, epoch and complete effects remain consistent with the approved teaching scope. The inspector is not represented as a full canonical-stage implementation.
- **Finality and unknown outcomes:** Inclusion, execution, finality and delivery remain distinct. A document predicate is not physical delivery or legal truth. Timeout does not establish nonexecution or justify release. Unknown now also explicitly distinguishes the last confirmed escrow account from the unknown current location of a submitted amount; scenario and UI wording agree. Authenticated external success is accounted separately from agreement acceptance if a condition is missing.
- **Recovery and histories:** Recovery retains prefix effects and needs signed authority, evidence, controlled assets, resources and consumption rules. An observed success cannot be erased by a later failure report. Shared ancestry remains distinct from consuming one identifier twice. The added history text correctly bounds each stage and predecessor fan-in, permits growing finite authenticated histories within signed episode/lifetime budgets, and does not promise unlimited execution or storage. Native recursion and private handoff remain open requirements, and proof possession does not deliver a successor’s witness.
- **Present versus planned:** The kernel, adapters, common ZK/MPC/TEE statement, OWS/x402 integration and complete source-to-ledger correspondence remain development work. The page does not claim that JavaScript checks produce native proofs. ZKIRv3 remains the target and future interfaces are not advertised as released. Current K/loan/swap statements now link directly to the scoped result records checked in the original review. Their finite comparison and fixed Preview-result scope has not been promoted to mandatory PCD, full language correspondence, private history or release acceptance. DeFiFormal remains a semantic reference without automatic theorem transfer or a Lean dependency.

## Authority and limits

Comparison basis: `docs/MORIARTY-CONSOLIDATED-DESIGN.md` (responsibility table, canonical statement, conditional settlement, history, evidence requirements); `docs/MORIARTY-PRODUCT-CONTRACT.md` (proof/phase/privacy boundary); `docs/MORIARTY-BACKEND-REQUIREMENTS.md` (ZR03, ZR09, ZR10, ZR14, ZR16 and external evidence); and `docs/superpowers/specs/2026-09-19-federated-kernel-interactive-page-design.md` (responsibility/evidence contract and trust/privacy/history views).

Scoped current-result evidence remains the K lifecycle, September 17 Preview loan, Preview swap and DeFiFormal `RESULT.md` records enumerated in `CONTENT-SECURITY.md`. Their original acceptance limitations remain applicable.

No native proofs, chain receipts or external protocol implementations were independently re-executed. No live deployment/link validation, accessibility review, general code audit or new browser/test result is claimed here. No new external research was needed. This verdict applies to the exact candidate-03 bytes above and closes only this reviewer’s factual security/evidence scope.
