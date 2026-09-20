---
title: "Design-review evidence and limits"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, language-design, review]
---

# Design-review evidence and limits

Repository observations and local experiment results are distinct from inferred design recommendations. The captured source hashes bind the dirty working-tree files; a Git HEAD alone does not describe this candidate.

- [Root review](../../../deliverables/whole-language-review-2026-09-19/REVIEW.md)
- [Independent Astra review](../../../deliverables/whole-language-review-2026-09-19/astra-review.md)
- [Reviewer status and Grok timeout](../../../deliverables/whole-language-review-2026-09-19/REVIEW-STATUS.json)
- [Root source hashes](../../../deliverables/whole-language-review-2026-09-19/root-source-hashes.json)
- [Astra source hashes](../../../deliverables/whole-language-review-2026-09-19/astra-source-hashes.json)
- [Reserve experiment inputs and outputs](../../../deliverables/whole-language-review-2026-09-19/reserve-repro-output.json)
- [Root experiment reproduction](../../../deliverables/whole-language-review-2026-09-19/reserve-root-reproduction.json)
- [Focused regression output](../../../deliverables/whole-language-review-2026-09-19/local-regression.log)
- [Interactive source/design graph](../../../deliverables/whole-language-review-2026-09-19/graphify-out/graph.html)
- [Graph observations and limits](../../../deliverables/whole-language-review-2026-09-19/graphify-out/ARCHITECTURE_OBSERVATIONS.md)

Seventy focused existing local tests passed with no failures/skips. The reserve result was independently reproduced at root. No full language suite, new native proof, target experiment or public transaction ran. The graph covers106 selected files; its structural edges are not correspondence proofs. Existing raw source studies and previously reviewed artifacts remain unchanged.

[Explanation](explanation.md) · [Index](index.md).

