# Finite call-helper proposal: normal host observations

The private `cfg(test)` helper advances function lookup and parameter binding
through explicit cursors and fixed-size events. It preserves the source table's
first matching function, stages pairs in source order, binds the tail first,
scans fresh environment cells for duplicate names, and retains caller/resume
context. Lookup and binding have separate legacy completion events. The proposed
`sherlock-helper-microstep/1` clock counts helper work separately from those
completions; this slice does not freeze or implement that profile's full budget.

Three source-first normal unit tests have literal event/state expectations:
three arguments with first-match lookup, zero arguments, and ordinary duplicate
parameters producing BadBinding. Their event/helper/visit counts are 24/22/14,
8/6/1 and 16/14/7. The standalone pure-std Rust compilation and all three tests
passed. This source has no circuit or native dependencies.

The adapter retains the complete real Snapshot, program and indexed ProgramImage,
argument values, caller environment, top-first frames, origin and result slots.
It resolves helper handles into those objects and materializes complete A
StepRecords only at lookup/binding completions. Host preparation, interning,
cloning and observation remain unmetered; they are not bounded circuit work.
Helper events retain A fuel/work; H diagnostics use the separate proposed clock.

Four adapter tests use the existing full-snapshot consumer. Six manually
transcribed Invokes from N05, N07 and N08 produce 12 completion boundaries checked
against unchanged independent frozen K literals, including recursive caller
environments and both pending multiplications. Every internal helper prefix
projects to its current A boundary. A separate literal List/Tree case checks two
more complete boundaries and preserves the saved scope and structural values.
No old or candidate evaluator generates those expectations.

The integrated public checker passed all 79 host tests, verified both frozen
JSON transcriptions, and retained the existing eight programs and 216 fuel
prefixes. It binds all 25 compiled Rust files, itself, seven frozen input files
and the pinned Rust 1.99.0 compiler. Captures include exact argv, exits, elapsed
times, hashes and lossless stdout/stderr. The checker used the existing ordinary
4 GB / swap-zero / 256-task envelope, a 90-second outer timeout and 30-second
inner limits, network disabled, read-only source and only fresh scratch writable.
No Cargo, K, Lean, SDK, proving or native operation ran in these host checks.

This is an unadopted helper proposal and a normal A-boundary projection toward
B-T03, not B execution, refinement or phase acceptance. Snapshot still lacks a
causal source-occurrence field: the adapter requires an explicit real Call ExprId
and does not prove its dynamic provenance. Image/source association assumes the
unchanged builder-produced image; no source digest binding or arbitrary-image
admission is supplied. Existing four-sort typed values are the common domain;
general ADTs are absent. Missing function/body/error continuation, full B fuel,
capacity/allocation precedence, schema, helper families, circuit constraints,
security cases, proofs, export, financial behavior and Midnight stagenet evidence
remain open. Public runtime exports and source `/1` and `/2` are unchanged. No
parent U/M/MC/SP/G/ZR/MNR exit closes here.
