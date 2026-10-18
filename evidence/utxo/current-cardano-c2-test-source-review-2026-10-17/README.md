# C2 r2 test-source review: first candidate rejected

The independent Opus 5.5 review rejected this 57,480-byte test candidate before execution. The full review is in `review/REVIEW.json`; the frozen source and all 14 reviewed inputs are in `review-context/`. No test, implementation, proof or ledger operation was executed by these jobs.

The reviewer requires two corrections. T15 must take its namespace and mutable-state baseline from a fresh r2 module before its first call, and every call within T15 must use that module. The module docstring must describe the combined file and its computed NON-ORACLE hashlib expectations accurately. The reviewer accepted the existing file-absence handler for the proposed ROOT RED scope and recorded optional tightening separately.

The author and reviewer both completed with exit 0 and whole schema-valid outputs. Their actual assistant envelopes report `claude-opus-5-5`: 34 author messages and 22 reviewer messages. These are separate contexts from the same provider. The reviewer read the whole candidate and all 14 inputs; the trusted wrapper found no changed input pins. The source was parsed for structure only: 18 methods, T00 through T17. Neither the author prediction nor the review prediction is an observed test result.

`ROOT-DISPOSITION.json` retains the rejection. Source and ordinary selftest readiness are false. A correction was subsequently requested in a separate source revision; those later bytes and their pending audit are outside this packet. All historical comments and author-time status strings remain unchanged. `COPY-MANIFEST.json` identifies every exact public copy. Provider captures, stderr, credentials and private material are excluded.

The frozen proof and setup JSON are public test fixtures. Their presence here establishes no cryptographic verification, setup trust or authenticated ledger fact. Native authority remains zero. Builder reliance, operator readiness and the complete Cardano Preprod prototype remain open, together with all 20 UTXO families, financial obligations, recursion, private handoff and multi-parent composition.
