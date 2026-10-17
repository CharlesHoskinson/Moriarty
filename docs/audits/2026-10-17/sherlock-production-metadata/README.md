# Production Sherlock metadata closure — 2026-10-17 UTC

One admitted local SDK-free metadata check compiled the canonical public non-test
Sherlock A+B module tree at [commit `06f7ff03`](https://github.com/CharlesHoskinson/Moriarty/tree/06f7ff03a245e0ab6fae271f09cd75bf813f0330).
The version child and the one library metadata/dep-info compile both exited 0.
The compiler read exactly 24 source files after lexical path normalization:
23 existing production files and the 67-byte canonical module wrapper. All 18
`cfg(test)` modules were excluded. Compiler stderr contains six warnings.

The [source manifest](source-membership.json) pins each existing public source by
Git commit, Git blob and SHA-256, with exact links; those bodies are not duplicated
here. [wrapper/lib.rs](wrapper/lib.rs) is the exact generated root. The
[receipt selection](receipt.selected.json), [result selection](result.selected.json),
[normalized dep-info](dep-info.txt), [read set](dep-info-read-set.json),
[full normalized child logs](logs/compile-diagnostic.stderr) and
[curated actual-review summary](review-summary.json) record the narrow result.

[evidence-provenance.json](evidence-provenance.json) declares every normalization
and retains original raw record hashes and byte counts. Log and dep-info text
remove only the absolute attempt-directory prefix; the two compiler
`families/../fragments` spellings remain. Receipt/result files are selected
projections, not raw copies. Private controls, raw reports, provider captures
and compiled artifacts are excluded. These are local trusted records and digest
bindings, not third-party authenticity attestations. The aggregate tool wall time
is not used; it differs from the sum of the recorded child durations.

This result establishes module/name/type closure for this exact snapshot only.
It establishes no whole-`mori-core` Cargo build, test compilation or execution,
program runtime, semantics, A/B correspondence, SDK/native, circuit/proof, ledger,
stagenet, full-B or full-Sherlock acceptance. Later source changes neither affect
nor inherit it. DedicatedSorts, finance Absent, HostOnly, genesis and one uncut
segment remain the qualified B scope. Native authority is zero and C4 stays open.
The earlier incident60460 remains not admitted and on hold, with no retrogrant,
retry or output/artifact reuse.
