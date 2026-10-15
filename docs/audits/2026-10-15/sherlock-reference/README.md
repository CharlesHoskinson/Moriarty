# Sherlock host reference delivery evidence

This directory records the typed `sherlock-k-a/1` host reference child delivery.
See [capability and limits](../../../SHERLOCK-REFERENCE-STATUS.md). No full U/M,
MC/SP/G/ZR/MNR/OSR, ACTUS, DeFi or composition exit is closed.

- `RED-SOURCE-REQUEST.txt` retains the exact initial four-file source closure.
  `RED-RESULT.json` records a successful compile followed by the intended missing
  transition panic (exit101), before the implementation was written.
- `HOST-COMPARISON-RESULT.json` retains the actual successful direct-rustc
  comparison, including source/compiler/generated/executable hashes and command
  output: eight test functions, six normal programs and 92 literal fuel prefixes.
  This is complete-state finite host evidence, not a formal or circuit result.
- `OPUS-RED-*` and `OPUS-COMPARISON-*` retain source-inspected command
  classifications with actual returned model identity and terminal records.
  They classify execution scope only; they are not correctness or adoption votes.
- `OPUS-PHASE0-AUTHOR*` records the separate Opus-authored B proposal copied
  verbatim into `models/sherlock/reference/security-phase0.md`. Authoring is not
  independent approval. Its hostile contracts remain unexecuted.

The comparison consumer mechanically transcribes the frozen JSON expectations.
The oracle author did not retain the original temporary literal serializer;
the JSON, derivation notes and digests are retained. No lost generator was
reconstructed or represented as original provenance.

Full-candidate independent nonsecurity and security reviews are recorded
separately from the command classifications. Native authority remains zero.
