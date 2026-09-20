---
id: apss.applications.app06
title: "Private applications with separate resource predicates"
status: draft
source_id: APP06
reviewed_at: 2026-09-19
type: reference
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---

# Private applications with separate resource predicates

Source: [Zexe: Enabling Decentralized Private Computation](https://www.cs.umd.edu/~imiers/pdf/zexe.pdf). Publication/version: 2019-02-21 (captured manuscript). Retrieved 2026-09-19T16:51:29.799300+00:00.

SHA-256: `387a9c0ebc4dcb458619da5340384b9172f54cd81acba1d2423d24f9d8e7f1e4`. Capture: [`captures/APP06.pdf`](../../../../attachments/apss/applications/APP06.pdf). Independence key: `zexe`.

Evidence locator: PDF pp. 29–31; sections 6.1–6.2. PDF pages visually read: 29, 30, 31.

**Source claims.** Zexe describes custom assets using record predicates for minting/conservation and private decentralized exchanges using access predicates. It distinguishes trade confidentiality from trade anonymity, discusses intent-based versus order-based exchanges, and notes a trade-off with market price discovery. Its privacy discussion assumes anonymous communication channels for user interaction.

**Moriarty inference.** Private DeFi applications need explicit disclosure goals and witness distribution rules in addition to validity proofs. The language can support applications with different ownership or issuance policies without adopting a universal identity operator.

**Limitation.** The captured manuscript is dated February 21, 2019. Its construction/performance is not a measured Midnight backend capability. A private proof does not automatically hide mempool timing, network identity, user-to-solver communication or application choice. Application policy authority is distinct from permission to deploy an application.


Evidence archive: [captured source manifest and reading records](../evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
