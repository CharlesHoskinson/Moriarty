---
id: apss.applications.app05
title: "Resource specifications and modular collaboration"
status: draft
source_id: APP05
reviewed_at: 2026-09-19
type: reference
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---

# Resource specifications and modular collaboration

Source: [Rich Specifications for Ethereum Smart Contract Verification](https://arxiv.org/pdf/2104.10274). Publication/version: arXiv:2104.10274v2, 2021-09-09. Retrieved 2026-09-19T16:51:12.919569+00:00.

SHA-256: `9a961855938ecd54383ac3c8fd98069d1b05d8ebf07ed1e88d145044111fe46d`. Capture: [`captures/APP05.pdf`](../../../../attachments/apss/applications/APP05.pdf). Independence key: `2vyper`.

Evidence locator: PDF pp. 2 and 22; contributions and implementation/evaluation. PDF pages visually read: 2, 22.

**Source claims.** Bräm and colleagues propose resource-oriented specifications and modular reasoning about collaborating contracts, including unverified external code and re-entrancy. Their 2Vyper implementation translates Vyper/specifications through Viper to SMT verification. The paper states that selected liveness aspects are expressed as safety properties.

**Moriarty inference.** Prove application-owned resource transfers and obligations at composition boundaries, not merely arithmetic outputs. An interface should describe effects and assumptions so a consumer need not assume the collaborator is benign. Hidden external effects cannot disappear from the proof footprint.

**Limitation.** Ethereum/Vyper re-entrancy semantics do not directly describe bounded Moriarty/Compact execution. A successful verification result establishes the supplied specification, not unstated user intent. The selected evaluation page identifies implementation and method; no benchmark reproduction or current tool qualification was performed here.


Evidence archive: [captured source manifest and reading records](../evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
