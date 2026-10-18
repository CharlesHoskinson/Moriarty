# Editorial update 011 — retained native audit provenance

Status: Root/Sol factual design-log entry; no website implementation or circuit-source change. Latest actual circuit-source entry remains 005.

The corrected retained R3 audit passed and both independent result reviewers accepted its scoped build-provenance result. Root subsequently verified durable preservation of 3,554 files (3,498 target files, 55 packet files and one external marker), totaling 2,628,344,281 bytes, then completed all six cleanup operations. The three tmpfs mounts are gone, cgroups are absent and the final receipt records `cleanupOwed:false`. Archived files remain read-only/nonexecutable; native outputs remain unaccepted and the SDK target ELF was not executed. Proofs, native target semantics, stagenet and full Sherlock completion remain open. No requirement status changes; the current published count remains 15/35.

For the existing assembly-history concept, the [R3 archive](../../docs/results/sherlock-native06-association-audit-20261018/README.md) supplies distinct provenance nodes: reviewed source, selected corrected outer operation, observed retained audit, two independent scoped result opinions, verified persistent preservation and actual cleanup. Connect those records by their exact identities and preserve the first wrapper failure. Do not draw an acceptance edge from audit provenance to SDK target execution, proof or stagenet. The 3,554-file copy map is metadata; the public archive does not ship the compiled payload.

The finished site still follows the full Sherlock acceptance condition. This entry adds no circuit iteration, cost/layout winner or public deployment.
