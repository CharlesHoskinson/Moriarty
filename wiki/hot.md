---
id: moriarty.vault.hot
title: Current research context
type: overview
status: active
created: 2026-09-07
updated: 2026-09-11
tags:
  - moriarty
  - research
sources:
  - SRC-0111
  - SRC-0112
  - SRC-0113
  - SRC-0109
  - SRC-0110
  - SRC-0108
  - SRC-0106
  - SRC-0107
  - SRC-0104
  - SRC-0100
  - SRC-0101
  - SRC-0102
  - SRC-0103
  - SRC-0097
  - SRC-0077
  - SRC-0078
updated_at: 2026-09-11T17:24:22Z
---

# Current research context

The [complete roadmap](../ROADMAP.md) and [report reconciliation](../openspec/REPORT-RECONCILIATION-2026-09-07.md) control current work. MC01-MC08 acceptance remains open.

[[wiki/index|Research index]] · [[wiki/moriarty-architecture|Architecture]] · [[wiki/open-questions|Open questions]] · [[wiki/workflow|Workflow]]

**PCD, 2026-09-11.** The [[wiki/decisions/pcd-midnight-native-architecture|Midnight-native PCD decision]] (SRC-0111–0113, CLM-0946–0960) recommends ledger-anchored certified state with bounded native certificates.

- **Why.** Contract proofs cannot be recursively verified; head read-then-write plus immutable keys give history by ledger induction; recursion (`ledger-10`, pull request 738) is reserved for off-ledger segments, attestations and imports.
- **Status.** The [PCD roadmap](../openspec/PCD-ROADMAP-2026-09-11.md) is adopted into OpenSpec planning by the [PCD integration amendment](../openspec/PCD-INTEGRATION-2026-09-11.md) as specified-only work. Six design defaults and the certificate k bound await the user.
- **Next.** Run E1 (read-then-write linearity on Preview) and E2 (step-relation fit) first. Preview PCD acceptance remains open.

**Language.** The [language/action dossier](../deliverables/defi-language-design-2026-09-07/README.md) recommends bounded `.mori` financial blocks and K. Use the partial-payment/residual-duty case in the next admitted language/K slice. The [four DeFi papers](../deliverables/defi-taxonomy-papers-2026-09-08/README.md), [vault supplement](../deliverables/erc4626-vault-report-2026-09-08/README.md) and [report8](../deliverables/defi-report8-comparison-2026-09-09/README.md) are S2 design input and close no sprint.

**K.** [[wiki/k-framework/k-best-practices|Bounded K results]]: the [numeric extension](../deliverables/numeric-k-2026-09-09/README.md) checks rounding, overflow-before-division, dust and ProRata against 64 cases. Successor semantics, proofs and Preview acceptance remain open.

**Syntax and assets.** The [TypeScript-style language report](../deliverables/language-design-2026-09-09/REPORT.md) (CLM-0940–0942) keeps ordinary `.mori` first. The [security-token report](../deliverables/security-token-transformations-2026-09-09/README.md) (SRC-0110) recommends explicit assets, claims, encumbrances and operation-specific authority across SP01–SP12.

**Midnight.** [[wiki/midnight-readiness-lessons|Midnight lessons]]: the local loan trace was reviewed. The [failed local swap](../deliverables/sp05-financial-integration-2026-09-09/local-swap-01/reviewed-result.json) stopped at initialize comparison, with a native-reserve versus indexed-projection conflict still open. Preserve private state and charged reservations, with no automatic retry. Local swap, Preview, failed-transaction nonmutation and PCD gates remain open.
