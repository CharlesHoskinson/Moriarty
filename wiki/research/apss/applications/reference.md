---
diataxis: reference
id: apss.applications.reference
title: Applications literature reference
status: draft
documentation_type: reference
type: reference
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---

# Applications literature reference

Bounded first pass: twelve substantive sources across ten independence groups. Anoma's two sources and CoW's two sources are related, not independent corroborations. Dates below distinguish publication from capture; historical papers do not establish current deployment properties. All full captures, SHA-256 values, final URLs and source independence keys are in manifest.json (`.raw/captured/apss-2026-09-19/applications/manifest.json`). The source notes separate claims, inference and limitations.

| ID | Source and kind | Application relevance | Detailed annotation |
|---|---|---|---|
| APP01 | [CAKE framework](https://frontier.tech/the-cake-framework) — Architecture essay | Application versus authority/solver/settlement boundaries | [Source note](notes/APP01.md) |
| APP02 | [Composing contracts: an adventure in financial engineering](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/07/contracts-icfp.pdf) — Primary research paper | Composition beyond a fixed financial-product catalog | [Source note](notes/APP02.md) |
| APP03 | [Marlowe: Implementing and Analysing Financial Contracts on Blockchain](https://link.springer.com/content/pdf/10.1007/978-3-030-54455-3_35.pdf) — Primary research paper | Language-wide semantic invariants versus application properties | [Source note](notes/APP03.md) |
| APP04 | [Certifying Findel Derivatives for Blockchain](https://arxiv.org/pdf/2005.13602) — Primary research paper | Counterexample: compositional syntax does not assure a fair bargain | [Source note](notes/APP04.md) |
| APP05 | [Rich Specifications for Ethereum Smart Contract Verification](https://arxiv.org/pdf/2104.10274) — Primary research paper | Resource effects and modular collaboration specifications | [Source note](notes/APP05.md) |
| APP06 | [Zexe: Enabling Decentralized Private Computation](https://www.cs.umd.edu/~imiers/pdf/zexe.pdf) — Primary research paper | Private assets and exchanges; confidentiality versus anonymity | [Source note](notes/APP06.md) |
| APP07 | [Anoma resource logic specification](https://specs.anoma.net/main/arch/system/state/resource_machine/data_structures/proof/logic.html) — Official technical specification | Shared versus application-specific resource predicates | [Source note](notes/APP07.md) |
| APP08 | [Intents from the resource model perspective](https://anoma.net/blog/intents-rm) — Primary author explanation | Exact desired resources versus predicates defining acceptable outcomes | [Source note](notes/APP08.md) |
| APP09 | [ComposableCoW README](https://raw.githubusercontent.com/cowprotocol/composable-cow/bc4d5f66fe044fdd8fbc78c9cf6ae99158f06a4c/README.md) — Official implementation documentation | Conditional orders, optional discovery, cumulative funding risk | [Source note](notes/APP09.md) |
| APP10 | [CoW AMM technical specification](https://raw.githubusercontent.com/cowprotocol/cow-amm/bb4b1ee43eeffd463bb8a981e343844a5a515e2a/docs/amm.md) — Official implementation documentation | Hard pool validity versus advisory order generation | [Source note](notes/APP10.md) |
| APP11 | [ERC-7540 Asynchronous ERC-4626 Tokenized Vaults](https://eips.ethereum.org/EIPS/eip-7540) — Normative interface standard | Asynchronous request/claim semantics | [Source note](notes/APP11.md) |
| APP12 | [ERC-7683 Cross Chain Intents](https://eips.ethereum.org/EIPS/eip-7683) — Draft interface standard | Solver-facing interoperability versus settlement verification | [Source note](notes/APP12.md) |

## Bibliographic records and inspected evidence

**APP01. CAKE framework** — Ankit Chiplunkar and Stephane Gosselin / Frontier Research. 2024-02-15. Retrieved 2026-09-19T16:51:06.370343+00:00. HTML/Markdown sections inspected; no PDF page claim. Capture SHA-256 `bccc16d661cb2f519645e215f5a3fb8644969c02ac451bd0db556fcf862c09ff`.

**APP02. Composing contracts: an adventure in financial engineering** — Simon Peyton Jones, Jean-Marc Eber and Julian Seward. 2000-08-23 (manuscript); ICFP September 2000. Retrieved 2026-09-19T16:51:07.943034+00:00. PDF pages visually inspected: 1, 2. Capture SHA-256 `dc92534929a60841d58858b44a73ad6b53d788935142353300f46b4e6cf0829b`.

**APP03. Marlowe: Implementing and Analysing Financial Contracts on Blockchain** — Pablo Lamela Seijas, Alexander Nemish, David Smith and Simon Thompson. 2020-08-07. Retrieved 2026-09-19T16:51:10.579681+00:00. PDF pages visually inspected: 2, 10, 11. Capture SHA-256 `0905f607a2b58ec97b3c1d98eada2845bd1438f0f6bf884c995593022ea9e0e7`.

**APP04. Certifying Findel Derivatives for Blockchain** — Andrei Arusoaie. arXiv:2005.13602v1, 2020-05-27. Retrieved 2026-09-19T16:51:11.751859+00:00. PDF pages visually inspected: 7, 8. Capture SHA-256 `7c45007d2b024d6de4a5bc58d1aa1cc9e3f1cc2fca1e0e5797a63cbd2fa7ba92`.

**APP05. Rich Specifications for Ethereum Smart Contract Verification** — Christian Bräm, Marco Eilers, Peter Müller, Robin Sierra and Alexander J. Summers. arXiv:2104.10274v2, 2021-09-09. Retrieved 2026-09-19T16:51:12.919569+00:00. PDF pages visually inspected: 2, 22. Capture SHA-256 `9a961855938ecd54383ac3c8fd98069d1b05d8ebf07ed1e88d145044111fe46d`.

**APP06. Zexe: Enabling Decentralized Private Computation** — Sean Bowe, Alessandro Chiesa, Matthew Green, Ian Miers, Pratyush Mishra and Howard Wu. 2019-02-21 (captured manuscript). Retrieved 2026-09-19T16:51:29.799300+00:00. PDF pages visually inspected: 29, 30, 31. Capture SHA-256 `387a9c0ebc4dcb458619da5340384b9172f54cd81acba1d2423d24f9d8e7f1e4`.

**APP07. Anoma resource logic specification** — Anoma specification contributors. Captured page reports Anoma Specification v0.1.2-bcd7b10c72; mutable main URL. Retrieved 2026-09-19T16:51:30.974339+00:00. HTML/Markdown sections inspected; no PDF page claim. Capture SHA-256 `c310a90a06b116b1f7b8d06673825df4e7838e2e7408cd46dab146bd6046fa28`.

**APP08. Intents from the resource model perspective** — Yulia Khalniyazova / Anoma. 2024-08-21. Retrieved 2026-09-19T16:51:32.457140+00:00. HTML/Markdown sections inspected; no PDF page claim. Capture SHA-256 `fb9cad5d7c51c75de9ad4e23dc43dede5048726d164f7e8d7f4f105cb3072ca6`.

**APP09. ComposableCoW README** — CoW Protocol repository contributors. 2026-09-12T15:07:54Z. Retrieved 2026-09-19T16:51:33.650142+00:00. HTML/Markdown sections inspected; no PDF page claim. Repository commit `bc4d5f66fe044fdd8fbc78c9cf6ae99158f06a4c`; pinned bytes match the initial capture. Capture SHA-256 `5915b88256e19b3a0b3071b2f5591e37b6e8eed3e3104f8c4b3e725587ff8966`.

**APP10. CoW AMM technical specification** — CoW Protocol repository contributors. 2024-05-14T08:27:31Z. Retrieved 2026-09-19T16:51:34.855067+00:00. HTML/Markdown sections inspected; no PDF page claim. Repository commit `bb4b1ee43eeffd463bb8a981e343844a5a515e2a`; pinned bytes match the initial capture. Capture SHA-256 `f13252e5578bdf62d72398f8443fde2e7515b3b42390c01186540acb56d649b7`.

**APP11. ERC-7540 Asynchronous ERC-4626 Tokenized Vaults** — Jeroen Offerijns, Alina Sinelnikova, Vikram Arun, Joey Santoro, Farhaan Ali and João Martins. Final, captured 2026-09-19. Retrieved 2026-09-19T16:51:54.365510+00:00. HTML/Markdown sections inspected; no PDF page claim. Capture SHA-256 `1191348dda76df6e603350e0b6979c4d09d4a0abab4450cc6a1957f724dc631b`.

**APP12. ERC-7683 Cross Chain Intents** — Francisco Giordano, Mark Toda, Matt Rice, Nick Pai, Alexander Lindgren, Mark Gretzke and Chris Cashwell (captured draft). Draft resolver-based specification, captured 2026-09-19. Retrieved 2026-09-19T16:51:55.471147+00:00. HTML/Markdown sections inspected; no PDF page claim. Capture SHA-256 `a4d5db4af9d56dceb95128dfd269240edbd40eb3f378b21ea28ce530910aa385`.

## Coverage boundaries

The five PDFs total 157 rendered pages; twelve selected pages were actually visually read. PixelRAG `pixelshot` rendered all pages at 100 DPI. Rendering is not reading. visual-reading.json (`.raw/captured/apss-2026-09-19/applications/visual-reading.json`) names exact one-based PDF pages and zero-based tile files. Additional text extraction supported metadata and section navigation; it is not claimed as full-page visual inspection.

No benchmarks, proof artifacts, full code audit, contract deployments, live trading or network feasibility experiments were reproduced. CAKE and Anoma explanatory claims are not promoted into theorems. CoW documentation was pinned to the exact latest commit affecting each captured file and independently compared to the initial raw bytes. The current ERC-7683 draft is not interchangeable with older versions.

## Evidence classes

| Class | What it establishes here | What it does not establish |
|---|---|---|
| Captured source statement | What named authors/specifications say in these bytes | Truth of all economic or security claims |
| Selected visual reading | Actual content on listed paper pages | Reading of unlisted pages |
| Design inference | A motivated Moriarty design option or proposed requirement | Implemented feature or accepted proof |
| Conceptual exercise | Arithmetic consequences of stated assumptions | Execution, formal proof or ledger settlement |

The documentation split follows [Diátaxis](https://www.diataxis.fr/): [explanation](explanation.md) develops the design reasoning; [how-to](how-to.md) supports a concrete design audit; [tutorial](tutorial.md) teaches a bounded conceptual exercise. This reference is for lookup, not a deployment procedure.


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
