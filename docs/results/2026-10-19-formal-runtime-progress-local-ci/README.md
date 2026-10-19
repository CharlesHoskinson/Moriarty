# Local check qualification, October 19

The local requirement-evidence check exited 1 before publication. Its source-corpus transcription matched, but historical requirement-evidence bindings were stale. Eight clauses remain currently reportable as closed; sixteen is the historical declared count. UIREQ002, 006, 017, 022, 025, 027, 030 and 035 require reconciliation.

Root compared all 31 affected source paths with base `ccd12aac0fb7969ab382acbc659c56dae3b87bfc`: their bytes are unchanged and disjoint from the 143 paths in this publication. This is a pre-existing failure, not a green local-CI result. The publication does not refresh evidence hashes or close any EARS.

The accompanying gzip is an exact inverse of the full diagnostic JSON. The independent scoped results remain six K operations, four public-runtime operations and two new Await checks; see their separate records and limits. Clean B17 was independently kernel-checked and merged to projectfenrir PR24.
