# Expanded-scope closure review — candidate v2

**Verdict: approved — COMPLETE DESIGN CONSOLIDATION ONLY.**

I read all 897 lines of `review-candidate-v2.txt`, including the eleven candidate files and all three appended Mina study reports, and the complete `candidate-v2-manifest.json`. All eleven embedded candidate files match the manifest's names, byte counts and SHA-256 digests. This review covers the newly authorized Mina-derived requirements as well as the consolidated architecture. The original v1 review remains unchanged.

**Required corrections: none.**

The v1 editorial findings are corrected: the requirement index includes UNI-017, and UNI-014 now rejects an already-rounded reciprocal only where the required exactness/rounding relation is missing. No newly introduced substantive conflict was found.

MNR01–MNR08 are useful, evidence-grounded refinements of ZR01–ZR16. MNR01/02/03/06/07 faithfully retain the inspected Pickles/Kimchi lessons: transcript order and representation, deferred-obligation lifecycle, legitimate branch and padding masks, heterogeneous shape and key policy, parameter identity, and batching soundness. MNR04/05 and the cache/key portions of MNR03/06 follow the appended developer report's distinctions between parsed proof data, verification, auxiliary output, current ledger state, dynamic-key integrity/authorization, testing mode and artifact provenance. MNR08 appropriately requires component-level assurance coverage; it does not present the historical audit as certification of current Mina or Midnight. I assessed the appended audit-scope summary, not the audit PDF independently in this closure pass.

These clauses preserve implementation freedom. They do not require Mina's Pasta curve cycle, IPA, Step/Wrap architecture, API names, fixed two-parent limit or a foreign proof backend. Their negative cases are proposed acceptance tests, not alleged deployed exploits. Historical TODOs, cache behavior and omitted-verification example code retain appropriate limitations. Sound order-insensitive batching remains permitted, and tests explicitly do not replace probabilistic soundness arguments.

The complete design continues to preserve:

- Permissionless public authorship, compilation, proving and deployment; owner authorization remains distinct from maintainer workflow.
- Native Midnight Halo2-derived PLONK/KZG and pinned ZKIRv3, with no Lean dependency or automatic DeFiFormal/Mina theorem transfer.
- One bounded typed stage relation, exact arithmetic and rounding, complete effects, gross debit versus net outcomes, separate liabilities/authority, explicit consent and certified primitive correspondence for adversarial witnesses.
- Conditional settlement, continuations, partial effects and residual duties, with stage termination separate from workflow liveness. Recovery authority has its own signed lifetime and revocation semantics.
- Full MC03 real native recursive financial steps and MC06 private successor/split/branch/join scope. Ledger induction, aggregation or signatures cannot close them. Kernel-free private handoff is explicit; optional kernel-assisted profiles retain separate assumptions.
- Bounded fan-in and per-stage work without silently imposing one fixed global history depth; successor episodes preserve existing signed cumulative budgets.
- The approximately March 2027 horizon as a dated user planning assumption, never verified release support.
- Optional federated orchestration with distinct ZK/MPC/TEE/finality assumptions, constrained OWS/x402 behavior and permissionless direct Midnight use.
- One U0–U7 sequence, explicit inherited MC/SP/G and MPLR traceability, accepted semantic inputs and extension requalification, retained conformance denominators and no resource-history reset.

All implementation, native feasibility, theorem, private-handoff, target correspondence and ledger acceptance exits remain open where the evidence is absent. That is appropriate for this proposal and does not require new implementation work before design approval. The reviewed Pel file remains an explicitly unready internal template, not product acceptance infrastructure.

No candidate edits, builds, proof runs or deployments were performed. This verdict is not implementation acceptance, a cryptographic security audit, campaign authorization or release approval.

## Bounded precision-correction addendum

**Approved for design consolidation.** I compared the exact final candidate bytes of the three amended files against their frozen v2 packet contents. No additional required correction was found.

MNR01 now scopes cross-field/limb conversion to non-native arithmetic; MNR03 uses backend-specific shape parameters instead of implicitly requiring Mina's domains/chunks/rounds arrangement. These changes preserve the required semantics without importing Mina implementation choices. MNR08 accurately separates extracted-text coverage from the narrower visual page inspection. Explicit inheritance of ZR implementation responsibility, and the added U3/U5 phase owners, clarify delivery responsibility without altering the public acceptance boundary or MC03/MC06 scope.

I independently checked the final register and specification: all **17 UNI clauses**, **35 MPLR mappings**, **16 ZR declarations/mappings** and **8 MNR declarations/mappings** are present, with no missing or unexpected numeric IDs. All 17 UNI IDs appear in the crosswalk, including compact grouped references. This checks presence and the reviewed mappings, not implementation completion or proof coverage.

Reviewed final SHA-256 values:

- `docs/MORIARTY-BACKEND-REQUIREMENTS.md`: `a04866c52a2546996666a51c373b4d60e8855e1213592cd8f916ade52ca7eee1`
- `openspec/changes/consolidated-language-kernel/requirements.md`: `3bb04dbd1a1677b38e423e097c90a6f2e032676045c87cc30a47db005141dbb0`
- `openspec/changes/consolidated-language-kernel/traceability.md`: `93e9d4110e2c3f9260d023dfd78362ce786672abb71639d49e0cefce6bd17b74`

README and GitHub Pages changes are outside this addendum. No candidate files were modified, and no builds or proofs were run.
