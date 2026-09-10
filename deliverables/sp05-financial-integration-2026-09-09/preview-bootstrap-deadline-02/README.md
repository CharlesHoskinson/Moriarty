# Final persistence deadline correction

Original bootstrap candidate6dd025b7f2527485f9b05e75ad151b3fa110a3c9a2a73c0f31cc277897a284db and all4 exact source bytes/check logs were verified and archived before correction. See original-candidate/ and preservation-map.json. Original reviews/manifests remain unchanged; the original Grok prompt continues to refer to original bytes.

GPT6 reproduced a successful PERSISTED result after the final directory fsync advanced beyond the absolute deadline. Source change is one deadlineCheck immediately after that fsync. The connected regression advances the clock inside the actual final fsync callback, asserts exact PREVIEW_LAUNCH_DEADLINE, and confirms the complete new3-file set plus original backups remain intact. No rollback or pending-marker recreation occurs for already coherent saved files.

RED15pass/1expectedfail (missing rejection); GREEN74pass/0fail/0skip using npm --prefix experiments/moriarty-midnight-financial run test:preview. Raw logs and reviewer reproducer retained here. git diff --check passed. Only bootstrap source and its test changed; other slices/package/fullswap test remain byte-identical. No services, original private state, proofs, transactions or network operations.

Corrected candidate is source-candidate-02.json (also /tmp/moriarty-preview-bootstrap-source-02.json), pending independent review. No Preview acceptance claimed.
