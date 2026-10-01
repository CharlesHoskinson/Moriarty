# Midnight Rust experiment delivery

Status: reviewed local experiment, published through merged PR12. The implementation/result candidate is `7e7f3861293ebdb40cdd9a1ab0e53c4e5624966f`; manifest SHA256 `de7d9583183d160ee5556a0a4d65d7085dd4263bcf380662aa8f1be9e42766aa` binds 74 files.

## Completed work

- [x] Pull full pinned official Midnight ledger/ZK sources and use required Rust dependencies.
- [x] Run 12 Rust integration tests, 2,506 leaf mutation controls and 25 Core-rejected signed-candidate cases across both signature schemes.
- [x] Exercise official Merkle state, actual final-pairing proof verification and local privileged ledger rejection/atomicity tests.
- [x] Preserve 79 passing existing beta regression tests and all production consumer hashes.
- [x] Repair the independently found numeric-key canonicalization defect, retaining its failing regression.
- [x] Obtain full fresh Astra medium approval for the revised exact candidate, with independent fixture reproduction and seven focused signature/proof tests.
- [x] Obtain exact Grok4.6 high full review: requested `grok-4.6`, session model `grok-4.6`, returned per-turn/usage identity `grok-4.6-build`, effort high, terminal `end_turn`, process exit0. Verdict approve-local-experiment; no blocking implementation defects.
- [x] File source collection SRC-0114, source inventory/ledger and wiki navigation through inspected transaction `ingest-midnight-rust-20260930`.
- [x] Publish the reviewed candidate and both audit receipts through [PR12](https://github.com/CharlesHoskinson/Moriarty/pull/12). Merge commit `db23fe0203fc2ae282f39ad1d1b10d2cd550e7d1`, observed merged at `2026-10-01T02:30:07Z`.

Final audit and delivery files are outside the immutable candidate manifest. They record review/publication status; they do not change runtime bytes or acceptance scope. The initial Astra changes-required verdict and interrupted first Grok run remain preserved in audits/REPAIR-1.md.

[Measured results and remaining product gates](RESULT.md). Production `/3` codec, authenticated key/head provenance, full native financial correspondence/PCD, atomic financial ledger caller and Preview settlement remain open. The beta remains PreparedUnqualified; this work supplies tested primitives and fixtures.

Current read-only vault lint reports 20 navigation issues (11 dead links, seven duplicate basenames, two stale index entries), zero provenance errors. The earlier frozen lint had 21 issues before DELIVERY.md existed. No clean-vault claim is made.

[Fresh Astra receipt](audits/astra-audit-2.json) · [Fresh Grok receipt](audits/grok-audit-2.json). Both review the full exact candidate, actual result receipts and scope limits; neither approves production financial acceptance.

[Actual merge receipt](evidence/pr-merged.json). This delivery-only completion update follows the reviewed implementation merge and keeps all 74 manifest-bound files unchanged. The preserved PLAN.md is the plan at implementation freeze; this file records its completed review/publication steps.
