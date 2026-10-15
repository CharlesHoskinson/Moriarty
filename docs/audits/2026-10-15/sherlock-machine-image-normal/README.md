# Flat host snapshot representation — normal observations, 2026-10-15

An unadopted private `cfg(test)` proposal encodes complete typed Snapshots as
flat tables, IDs and ordered slices, then materializes them structurally. It
retains full code/function/parameter order, held expressions/functions, every
control variant, nine frames and saved environments, values, counters and outcome.
No recursive Expr/Value/Tree/Frame/Control payload is stored inside encoded rows.
Detached held syntax is copied structurally: old Snapshot exposes no original
source-occurrence identity for equal expressions, so none is invented.

Caller limits count table rows, ordered-ID cells and copied UTF-8 name bytes.
Refusal is tooling-only and leaves the source snapshot unchanged. These limits
do not bound temporary traversal storage, allocation overhead, reconstruction,
recursive Clone/Drop/equality, serialized bytes, native cells or circuit cost.
The decoder is private and assumes normal finite acyclic coherent typed rows,
including encoder outputs. It supplies no arbitrary-image admission policy.

Five source-first tests include independently written snapshots and expected
encoder rows/usage, a separate hand-written decoder row fixture, and complete
pending factorial caller frames. Structural fixtures cover all ten control
variants, nine frame variants and outcomes; this is not a reachability claim.
Inverse agreement alone would be internal consistency. The common normal
consumer separately checks encode/decode materialized initial states and every
StepRecord against the unchanged independent K literals for all eight programs
and 216 fuel prefixes, including complete code, environments, saved frames,
rule/charge, counters, publication and run/step final state.

Behavioral RED compiled a separate always-refuse encoder, then failed the first
N08 materialization assertion; it does not supply individual RED for every case.
Genuine GREEN passed all 72 host tests. The separately captured public checker
verified both frozen JSON transcriptions and also passed 72 tests. It now binds
all 21 compiled Rust files, itself, the seven frozen input files and rustc.
Actual source hashes, compiler hash and raw stdout/stderr are retained. Gzip
captures are lossless. Commands use an existing pinned Rust 1.99.0 compiler:

```text
python3 -I -B models/sherlock/reference/check-reference.py --rustc ABSOLUTE_EXISTING_RUSTC --timeout-seconds 30
```

Direct stages had 30-second limits; the checker had a 90-second outer limit.
All ran with 4 GB memory, swap disabled, task limit 256, network disabled,
read-only source and only new scratch writable. Receipts record actual exits,
elapsed times and output hashes. No Cargo, SDK, K, Lean or native action was
performed by these checks.

This is source preparation toward B-T04, not adoption of a B schema or completion
of B-T04/A4. Public exports and source `/1` and `/2` are unchanged. Native tags,
field widths, wire/canonical codecs, supported-image admission, helper metering,
financial continuation, compiler/Clean/circuit correspondence, actual proofs
and Midnight stagenet acceptance remain open. No security judgment, full
roadmap milestone or retained U/M/MC/SP/G/ZR/MNR exit closes here.
