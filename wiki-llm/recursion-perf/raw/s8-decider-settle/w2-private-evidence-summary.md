# Archived recl9 and stand-in evidence: wave 2 summary
> Source: private projectfenrir recursion/stagenet-strawman/recl9/REPORT.md; recursion/stagenet-strawman/recl9/keys/{node4,decider}/keygen.stats.json; recursion/stagenet-strawman/decider/REPORT.md
> Collected: 2026-10-26
> Published: Unknown

**SOURCE:** Current checkout git hash `843c672dbe6c4fd2fafa5848f413c12db201e364`. This identifies the inspected checkout, not necessarily the historical measurement commit. This file summarizes private material; no private source text is reproduced. No new performance run was made.

## node4
**SOURCE:** `recursion/stagenet-strawman/recl9/keys/node4/node.vk`: 2,189 bytes; SHA-256 `ed68cf2d2971e01b290578d8528a0c6df4cc240edb7c07cdf18ad9bc8a30f2b0`. Version-2 architecture header occupies 18 bytes, outer k/public-count another 5 bytes; inner VK header starts at byte 23 with version 3 and k20. Its u32 fixed-commitment count is 27; remaining 48-byte commitments number 18. This is an offline artifact inspection, not a proof benchmark.
**SOURCE:** Recorded cost metadata: advice rows 958,095; table rows 1,032,465; advice columns 15; pre-keygen fixed columns 45; permutations 18; unusable rows 7; usable k20 rows 1,048,569. Metadata SHA-256 `c144d48af0676c8d69df53df849a1f96a3206c06486535f9c84c1d0c0b0ac4b1`.

## decider
**SOURCE:** `recursion/stagenet-strawman/recl9/keys/decider/decider.vk`: 2,189 bytes; SHA-256 `f37acaadbef22b6b004f63e895e60526ba148eb9a3bb845e1005f26d15dd5f5e`. Version-2 architecture header occupies 18 bytes, outer k/public-count another 5 bytes; inner VK header starts at byte 23 with version 3 and k20. Its u32 fixed-commitment count is 27; remaining 48-byte commitments number 18. This is an offline artifact inspection, not a proof benchmark.
**SOURCE:** Recorded cost metadata: advice rows 841,682; table rows 1,032,209; advice columns 15; pre-keygen fixed columns 45; permutations 18; unusable rows 7; usable k20 rows 1,048,569. Metadata SHA-256 `5cdf212d917f9fe6a5aa26b9cd587afe1f392dafb15635d936c8156e4af5618f`.

**SOURCE:** Recorded node PK setup 104.5 s; prove 867.3 s; peak RSS reported 14.99 GB. Recorded D PK setup 46.9 s; prove 586.4 s; total 634.8 s; proof 6,432 bytes; stock ledger verify 0.021 s. D stats use `peak_rss_kb=14965544`, so preserve units instead of interpreting the report’s decimal GB as a precise GiB measurement.

**SOURCE:** Official k20 SRS file hash recorded by both key metadata files: `1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1`. This is an archived provenance assertion, not an independent new ceremony audit.

**SOURCE:** The different stand-in report records total D advice rows 832,015, table rows 1,032,209, pairing/subgroup rows 316,833 (attributed to a gadget measurement), PK-setup-stage RSS 12.8 GB and whole-process peak 14.9 GB. Its wording describes the PK as about 12.8 GB resident, but the underlying observation is process RSS; it does not isolate live PK allocation. Stand-in prove 1,183 s and PK setup 148 s were under shared load.

**ESTIMATE:** Stand-in remainder is 832,015 - 316,833 = 515,182 rows. It combines verifier transcript, fold, 63-base resolve/collapse and binding; no independent per-subcomponent profile is available. Never transfer that split to recl9 or Sherlock as measured.

**OPEN QUESTION:** Exact quotient extension factor and retained live bytes for each deployed Sherlock key still require object introspection. The archived recl9 counts support conditional sizing, not a measured deployed-key RSS.

**SOURCE:** The recl9 report records four leaf proving durations 34.8, 35.3, 44.2 and 49.4 s. Together with N 867.3 s and D 586.4 s these are component timings, not a new matched end-to-end run.
