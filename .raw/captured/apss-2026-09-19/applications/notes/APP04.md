---
id: apss.applications.app04
title: "Findel: composition can preserve the wrong bargain"
status: draft
source_id: APP04
reviewed_at: 2026-09-19
---

# Findel: composition can preserve the wrong bargain

Source: [Certifying Findel Derivatives for Blockchain](https://arxiv.org/pdf/2005.13602). Publication/version: arXiv:2005.13602v1, 2020-05-27. Retrieved 2026-09-19T16:51:11.751859+00:00.

SHA-256: `7c45007d2b024d6de4a5bc58d1aa1cc9e3f1cc2fca1e0e5797a63cbd2fa7ba92`. Capture: [`captures/APP04.pdf`](../captures/APP04.pdf). Independence key: `findel-certification`.

Evidence locator: PDF pp. 7–8; example 2.3, limitations and motivating option example. PDF pages visually read: 7, 8.

**Source claims.** Arusoaie's paper gives a formal Coq treatment of Findel and exposes a contract in which a participant can obtain an incentive while controlling whether the other branch proceeds. The inspected pages also describe the examined Findel model's two-party limitation, lack of loops and absence of balance constraints preventing debt.

**Moriarty inference.** A well-typed compositional contract can fail the developer's intended economic bargain. Application specifications must name who controls choices, who owes each residual obligation, and which progress or funding assumptions make the bargain feasible. Add a negative test in which an actor takes a reward but avoids its intended reciprocal duty.

**Limitation.** These findings concern the examined Findel version, not every financial DSL. The selected pages do not establish the complete soundness of the certification system or every paper case study. A bounded repetition limit is an engineering design choice, not itself a safety defect; the danger is failing to model the intended lifecycle.
