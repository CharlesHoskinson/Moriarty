# Wiki Log

## [2026-10-26] ingest | s7 recursion and aggregation
- Disposition: New
- Raw: clusters/s7-recursion/wave-1/FINDINGS.md

## [2026-10-26] ingest | s2 witness synthesis
- Disposition: New
- Raw: clusters/s2-witness/wave-1/FINDINGS.md

## [2026-10-26] ingest | s3 commitments and MSM
- Disposition: New
- Raw: clusters/s3-commit-msm/wave-1/FINDINGS.md

## [2026-10-26] consensus | C0-C4 opened as candidates
- C0 (phase profile first) is supported independently by all three wave-1 clusters; cross-review pending.

## [2026-10-26] ingest | s4 lookups, s5 quotient and NTT, s9 orchestration
- Disposition: New
- Raw: clusters/s4-lookups/wave-1/FINDINGS.md; clusters/s5-quotient-ntt/wave-1/FINDINGS.md; clusters/s9-orchestration/wave-1/FINDINGS.md

## [2026-10-26] consensus | C5-C7 opened; C0 now backed by six of six clusters

## [2026-10-26] ingest | s1 keys, s6 opening, s8 decider and settlement
- Disposition: New
- Raw: clusters/s1-keys/wave-1/FINDINGS.md; clusters/s6-opening/wave-1/FINDINGS.md; clusters/s8-decider-settle/wave-1/FINDINGS.md
- Privacy: ten verbatim captures of private projectfenrir files (recl9 report, logs, keygen stats, sources; stand-in decider report) stay local via info/exclude; the pre-commit hook now rejects any such capture

## [2026-10-26] consensus | C8-C10 opened; C0 backed by nine of nine; C10 meets the two-source test
- C10 (persistent prover with resident prepared keys) is s1 rank 1 and s8 rank 1 with consistent arithmetic (1.07x and 1.08x); it goes to cross-review with C0
- s8 rank 5 adds a domain-doubling caveat to C1 (fusion at doubled k is 25.3% slower)

## [2026-10-26] consensus | cross-review round 1 (Muse Wonder Full, Gemini Argon)
- C0 agreed as a prerequisite measurement, not a speedup; no speedup row agreed
- C1, C2, C5 and C10 weakened (single sources, shared Loki priors, unlike denominators); C10's decoded-key cache may already exist in ledger main
- The cross-stage reading was withdrawn: the assumed stage shares add up to 216-241% of one proof
