# Initial review finding and repair

Initial frozen commit 95ff2cb7c9632fde458bd52be53442c96947e6d2, manifest SHA256 17f412906eae41cc19d3d0a00d912768ff8578fe0a6dc2be67a29a762d5107e3. Fresh Astra review required changes for numeric-index object key ordering in Node canonicalization. [Initial audit](astra-audit-1.json) and [external reproduction](../evidence/integer-key-reproduction.json) preserve that finding.

The initial Grok4.6 high review was intentionally interrupted after the defect was confirmed, before any approval could apply: process exit130, no structured audit result/model identity returned. [Interrupted receipt](grok-audit-1-interrupted.json) is not approval. The revised frozen candidate requires both fresh full reviews; no initial source approval is reused.

A nested numeric-index golden vector was added first and the actual Rust cross-language test failed (exit101), retained in [regression red run](../evidence/integer-regression-red.txt). The exporter now recursively emits object members directly in UTF8 key order, without reconstructing a JavaScript object. Numeric-index keys at the root, nested object and array-element object are included in the regression. Number values and non-scalar Unicode refuse the exporter subset. All five financial frame bytes/economics remain unchanged; new vectors expand generic codec coverage.
