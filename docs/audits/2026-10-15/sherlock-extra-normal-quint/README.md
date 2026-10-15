# Sherlock: additional K-A Quint trace comparisons

The unchanged generic K-A model passed 126 new deterministic TypeScript tests:
two complete normal traces and 124 published fuel prefixes for N07 ordered
three-argument evaluation and N08 named recursive factorial two. Typechecking
passed separately. Both commands used seed 1, one sample per test where
applicable, 60-second limits, 4 GiB memory, disabled network and read-only inputs.

The three new source files are bound in RESULT.json. The independent literal
reader reconstructed all 250 source comparisons before model execution. It
checks complete control, environments, ordered source/code, saved caller frames,
counters, raw outcomes and each rule/charge schedule against the frozen JSON.
No evaluator produces expected snapshots or work. Serializer and reader remain
available under models/sherlock/oracles/.

Original stdout/stderr are losslessly compressed here; recorded hashes refer to
decompressed bytes. The full literal readback is compressed separately. Raw
wrapper argv remain locally retained, with their meaningful Quint options in
the public result record. Source preparation/audits predate actual execution;
the result record supersedes their UNRUN status for these exact test bytes.

The earlier six-program 98-test evidence retains its own exact-source scope and
was not rerun. Together the two cohorts cover the eight normal programs; this
is not one combined 224-test invocation. K execution, Lean correspondence, full
grammar/security coverage, B helpers, circuits, native proofs and Midnight
stagenet acceptance remain open. No parent roadmap exit closes.
