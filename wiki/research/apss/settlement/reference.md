---
title: "APSS settlement: reference"
diataxis: reference
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: reference
tags: [moriarty, apss, research]
---

# Settlement reference and annotated primary literature

Captured September 19, 2026. Twelve reference entries are organized by claim, including a linked Kachina paper supplement to the official Kachina documentation. `manifest.json` retains raw artifact hashes, timestamps, redirects and the failed initial PCD fetch. This is a bounded reading, not a systematic review or proof audit. `reading-coverage.json` lists inspected sections and PDF pages. Full capture or rendering is not full reading.

The principal implementation constraint is **SET-07's guaranteed/fallible phase distinction**: fallible failure may retain guaranteed effects and fees. No other entry overrides that deployed-interface obligation.

## SET-01 — CAKE framework

**Source:** Chiplunkar and Gosselin, [Introducing the CAKE framework](https://frontier.tech/the-cake-framework), February 15, 2024. Primary design essay. Read the layer definitions, settlement, oracle/bridging discussion and trade-off taxonomy.

**Contribution:** separates application, permission, solving and settlement; distinguishes faithful information transfer from user-tolerated value-transfer costs. Acknowledges asynchronous sub-transaction failure.

**Limits:** six designs and a fees/speed/guarantees trilemma are the authors' organizing proposal. Dated market counts and timing examples are not current measurements. No formal intent language, soundness theorem or universal settlement implementation is supplied.

**Moriarty relationship:** use the layer boundaries, retaining signed asset/authority constraints and conditional completion. Do not import a managed routing service as the language's deployment authority.

## SET-02 — Atomic Cross-Chain Swaps

**Source:** Maurice Herlihy, [paper](https://arxiv.org/abs/1801.09515), 2018; captured PDF has 10 pages. Read introduction, model §2.2 and outcome definitions; visually read PDF pages 2–3, including Figures 1–3.

**Contribution:** models swaps as a graph with hash/time-lock coordination and defines acceptable outcomes for conforming parties. The model provides a concrete way to discuss safety against deviation separately from all parties completing the exchange.

**Assumptions:** known delay sufficient for publication/confirmation, reliable contract semantics, explicit recorded transfers and the construction's graph conditions. Asset lockup is an acknowledged cost. No theorem here establishes safety for an arbitrary halt model.

**Moriarty relationship:** require explicit timing/environment assumptions and represent escrow exposure; do not infer generic cross-chain atomicity from bounded local evaluation. Formal proofs and complexity claims were not independently rederived.

## SET-03 — Cross-chain Deals and Adversarial Commerce

**Source:** Herlihy, Liskov and Shrira, [PVLDB paper](https://www.vldb.org/pvldb/vol13/p100-herlihy.pdf), 2019, 14 PDF pages. Read safety/liveness definitions, system model and protocol overview; visually read PDF page 4 (printed p.103).

**Contribution:** identifies acceptable payoffs, no indefinite escrow and successful all-party completion as distinct properties. Gives synchronous decentralized and semi-synchronous shared-ledger approaches.

**Assumptions:** ledger and contract behavior, communication assumptions, cryptographic identities, participant conformity and acceptable-payoff preferences. The property of eventual release still needs the protocol's model.

**Moriarty relationship:** express partial fills, retained obligations and remedy rights as first-class outcomes. A successful local rollback is not a remedy for an already finalized remote transfer. Selected definitions were inspected; complete protocol proofs and performance were not reproduced.

## SET-04 — Proof-Carrying Data and Hearsay Arguments from Signature Cards

**Source:** Chiesa and Tromer, [ICS 2010 proceedings PDF](https://conference.iiis.tsinghua.edu.cn/ICS2010/content/paper/Paper_25.pdf), 22 pages. Original author-hosted URL failed with connection resets; conference copy succeeded. Read abstract, §1.3 and §§4.1–4.2; visually read PDF pages 1, 4 and 14 (printed 310, 313, 323).

**Contribution:** formalizes compliance of a computation graph and proofs attached to messages. Compliance is a specified predicate over computation, not a generic word for trustworthy finance.

**Assumptions:** this historical construction uses an assisted prover with signed-input-and-randomness functionality and associated trust assumptions. It is not evidence that modern recursion always requires those devices.

**Moriarty relationship:** explicitly bind history predicates, roots and dependencies. A proof of compliant supplied inputs does not authenticate arbitrary off-chain events or perform unique ledger consumption. Construction security was not independently audited.

## SET-05 — Nova: Recursive Zero-Knowledge Arguments from Folding Schemes

**Source:** Kothapalli, Setty and Tzialla, [ePrint 2021/370](https://eprint.iacr.org/2021/370.pdf), paper associated with CRYPTO 2022; captured revision is hash-bound and has 46 pages. Read abstract and introductory folding/IVC discussion; visually read p.4, including Theorem 2's stated scope.

**Contribution:** folding reduces accumulated relation checking, supporting IVC; succinct zero-knowledge compression is a further construction. Costs remain related to the step function and commitment scheme.

**Limits:** source claims are relative to its cryptographic model. No current library revision, curve choice, circuit constraint count, finalizer, Solidity/Compact verifier or Moriarty performance was tested. The captured URL can change; use the stored hash.

**Moriarty relationship:** retain a separate final verification boundary and complete financial step relation. Neither a small recursive step nor a folded accumulator supplies native Midnight correspondence by itself.

## SET-06 — IBC channel and packet semantics, ICS-004

**Source:** [pinned specification](https://github.com/cosmos/ibc/blob/6eb8792e987220d7afcc8c926426f3af5695cb7b/spec/core/ics-004-channel-and-packet-semantics/README.md), commit `6eb8792e987220d7afcc8c926426f3af5695cb7b`. Read packet fields, acknowledgements, timeout overview and timeout handling. This is the selected ICS-004 path, not a blanket statement about every IBC version.

**Contribution:** commitments, sequences and authenticated receive/acknowledgement/timeout processing distinguish safe transport from application outcomes.

**Assumptions:** the trusted client and counterparty consensus, correct commitment verification, reachable destination evidence and application callbacks. A missing reply is not proof of non-receipt. A generic acknowledgement can encode application success or error.

**Moriarty relationship:** require authenticated evidence before refunding or reusing authority after uncertain execution. Pin the message schema and application success predicate. No IBC implementation or bridge was executed.

## SET-07 — Midnight transaction semantics

**Source:** [official page](https://docs.midnight.network/concepts/how-midnight-works/semantics), update marker September 18, 2026. Entire substantive page read. Locators: “Transaction fallibility,” first three paragraphs; “Well-formedness”; “Phase execution.”

**Contribution:** distinguishes well-formedness, guaranteed and fallible phases. A guaranteed-phase failure prevents inclusion; a fallible failure can preserve guaranteed effects and fees. Phase execution checks declared effects against execution and applies nullifier/state checks.

**Moriarty relationship:** compile and prove a phase-aware effect contract. Success, partial success and non-inclusion must remain distinct; language-local atomic rejection is not general ledger rollback. Bound fees and other preserved effects in signed intent.

**Limits:** documentation acquisition is not execution against the pinned Preview backend. Each compiler/ledger version and transaction shape still needs source checks, tests and actual finalized readback.

## SET-08 — Compact language reference

**Source:** [official reference](https://docs.midnight.network/compact/reference/compact-reference), redirected from `/compact/reference/lang-ref`. Read overview and witness declarations; inspected targeted boundedness/disclosure passages. The whole long reference was captured but not read exhaustively.

**Contribution:** distinguishes public ledger state, zero-knowledge circuits and local code; describes compile-time bounds and lack of recursion. The witness section warns that callers can provide their own implementations and witness results are untrusted.

**Moriarty relationship:** a compiler must constrain witness-dependent effects in the circuit, with explicit input semantics. Trusted TypeScript behavior is insufficient. Existing Compact bounds are not proof of a feasible Moriarty circuit or a bound on total distributed work.

**Limits:** no compiler version was run and no complete feature conformance review performed. Current documentation may differ from Moriarty's admitted toolchain; pin before implementation.

## SET-09 — Midnight consensus

**Source:** [official consensus page](https://docs.midnight.network/concepts/network-architecture/consensus), update marker September 18, 2026. Read the architecture description and AURA/GRANDPA sections.

**Contribution:** separates block production from finality, documenting AURA and GRANDPA, with validator selection distinct from application proofs.

**Moriarty relationship:** proof validity, block inclusion and finality require different receipt fields. Permissioned validators in a network model do not imply an administrator must approve language programs. The SDK must pin the actual deployed finality policy.

**Limits:** broad explanatory security language is not a newly verified theorem. No validator set, implementation or finality certificate was inspected. The linked GRANDPA PDF was identified but not read in this bounded bank; do not infer its exact thresholds or liveness theorem from this page.

## SET-10 — Flash Boys 2.0

**Source:** Daian et al., [arXiv paper](https://arxiv.org/abs/1904.05234), captured PDF visibly marked v1, April 10, 2019, 23 pages. Read abstract and §VII; visually read pp.1 and 14.

**Contribution:** documents ordering competition, priority gas auctions and historical consensus incentives around extractable value.

**Moriarty relationship:** successful verification does not mean best execution, fair ordering, inclusion or minimal fee. Signed cost limits and disclosure boundaries remain relevant even with correctness proofs.

**Limits:** measurements concern historical Ethereum conditions and the paper's sample/model. No Midnight MEV rate, current attack profitability or universal economic theorem is claimed. Charts beyond the visually inspected §VII page were not independently checked.

## SET-11 — ERC-7683: Cross Chain Intents, captured draft

**Source:** [official draft](https://eips.ethereum.org/EIPS/eip-7683), creation date April 11, 2024; captured September 19, 2026. Read abstract, motivation, order/dependency definitions, resolver guarantees/assumptions, previous-draft rationale and security considerations.

**Contribution:** current text standardizes solver-facing resolver output; explicitly leaves authorization and settlement implementations flexible. Resolver assurances retain implicit chain liveness/censorship assumptions and named conditions.

**Moriarty relationship:** a common representation can describe bounded execution and payment claims without becoming the acceptance authority. Preserve the full exposure window and actual settlement verification.

**Limits:** the older origin/destination-settler draft is discussed as prior design, not the current normative interface. Draft status matters. A solver's resolver whitelist is local risk policy, not permission to deploy Moriarty programs. Full ABI details and all instruction variants were not audited.

## SET-12 — Kachina documentation and linked foundational paper

**Sources:** [official Kachina overview](https://docs.midnight.network/concepts/kachina); Kerber, Kiayias and Kohlweiss, [Kachina—Foundations of Private Smart Contracts](https://eprint.iacr.org/2020/543.pdf), captured Revision 4, 88 pages. Read the full substantive overview, paper abstract and Appendices F/H selected passages; visually read paper pp.68 and 72.

**Contribution:** separates public/private state and transcript-based execution. Appendix F adds liveness assumptions to a base model without them. Appendix H sketches multi-interaction workflows and marks full treatment as further work.

**Moriarty relationship:** private computation can be correct while a future/pending operation still needs parties to act. Never infer the current ledger's phase behavior from the foundational model; SET-07 remains the specific interface reference.

**Limits:** UC security proofs, dependency model and 88-page paper were not reviewed end-to-end. Neither overview nor theory establishes current source-to-Preview correspondence or mandatory PCD implementation.

## Artifact and coverage conventions

PDFs were downloaded with Scrapling, text-extracted with `pdftotext -layout`, and rendered by PixelRAG's `pixelshot` at 110 DPI. Six PDFs produced 203 page images and renderer-generated `chunks.json` files. The `pixelrag chunk` stage was invoked; it verified existing page chunks and skipped regeneration. Eleven specific page images were read visually. No embedding index, similarity ranking or full-document visual audit is claimed. Failed author-site capture and the initial zero-shard chunk invocation remain recorded as limitations in the coverage manifest.

Diátaxis structure follows the [official framework](https://www.diataxis.fr/): this reference identifies sources and limitations, [explanation](explanation.md) develops concepts, [how-to](how-to.md) guides an audit, and [tutorial](tutorial.md) gives a conceptual learning exercise.


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
