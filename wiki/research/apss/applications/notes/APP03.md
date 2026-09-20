---
id: apss.applications.app03
title: "Marlowe semantic guarantees and refunds"
status: draft
source_id: APP03
reviewed_at: 2026-09-19
type: reference
created: 2026-09-19
updated: 2026-09-19
tags: [moriarty, apss, research]
---

# Marlowe semantic guarantees and refunds

Source: [Marlowe: Implementing and Analysing Financial Contracts on Blockchain](https://link.springer.com/content/pdf/10.1007/978-3-030-54455-3_35.pdf). Publication/version: 2020-08-07. Retrieved 2026-09-19T16:51:10.579681+00:00.

SHA-256: `0905f607a2b58ec97b3c1d98eada2845bd1438f0f6bf884c995593022ea9e0e7`. Capture: [`captures/APP03.pdf`](../../../../attachments/apss/applications/APP03.pdf). Independence key: `marlowe`.

Evidence locator: PDF pp. 2, 10–11; sections 2, 5.1–5.3. PDF pages visually read: 2, 10, 11.

**Source claims.** Marlowe builds financial contracts from a small number of constructs, accounts and continuations. The authors describe Isabelle proofs for semantic properties, including money preservation and a timeout after which an empty transaction closes a contract and returns account funds. The proof translation differs from Haskell in identifier and map representation.

**Moriarty inference.** Separate language-wide conservation/boundedness properties from a particular developer's desired payment. Expose termination and recoverability with their hypotheses. A refund transition being enabled is not a guarantee that a network participant will submit it.

**Limitation.** This 2020 paper is evidence of its described Marlowe design and proof scope, not a current Cardano deployment audit or a Moriarty theorem. Returning escrow cannot erase separate outstanding debt. Mechanized semantics and a compiled ledger implementation need an explicit correspondence argument; close-to-source translation alone is insufficient.


Evidence archive: [captured source manifest and reading records](../evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
