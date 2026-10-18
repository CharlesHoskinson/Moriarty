# Positive Boolean source-check specimens

These two programs check ordinary documented syntax in Core/1 and Core/2. They combine `<`, `or`, prefix `not`, `and`, `>`, Ensures, fresh repeated names/literals and a second action. Core/2 adds an Await event binder whose guard is retained in the arena before runtime. No scenario or action execution is used.

Expected complete Programs were manually enumerated from source in lexical statement/action order and child-first expression order, with author parser/DTO exposure declared. Core/2 uses the adopted finite M01 premise; Core/1 uses implementation-exposed representation and finite Program precedents, not a general normative proof. Both have 21 nodes: roots2/8/12/16/17 in choose and20 in inspect_limit. The false state initializer allocates no arena node; the guard binder allocates none. The expression table retains repeated false/name occurrences and the second action. No output was used to generate these values.

Expected check JSON includes complete actions/roots, exact source SHA256, labels and empty diagnostics. Its ASCII pretty form follows the existing checker/DTO and JSON exporter, with lexicographic object keys, two-space indentation and final LF. Sources include final LF. Human success uses the existing fixed Core status lines, with empty stdout. Expected results are frozen source-derived corroboration, not independent financial/work goldens.

A source-check pass asserts static admission only. No claim is made that all requires/ensures hold for arbitrary runtime inputs, that the await advances, or that a compiler/backend admits the same source. No undocumented Ne spelling is selected. All original 27 constructor obligations and complete language/Program/Target/Artifact, finance, execution, proof and ledger acceptance gates remain open.

The literal fixture test deliberately compares complete frozen JSON instead of generating snapshots, to keep pre-execution expectations immutable and show the exact source in assertion messages. It uses real public checker binary calls and public parse/stage::load APIs. The CLI test does not implement a second parser/evaluator.
