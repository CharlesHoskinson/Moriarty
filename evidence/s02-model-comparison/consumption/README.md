# S02 parent-consumption evidence

This directory preserves contemporaneous evidence for the bounded parent
consumption bookkeeping task. It is sampled simulation and deterministic test
evidence only. It is not candidate, authorization, recovery, gate, or model-
checking evidence.

`red/` contains the exact sixteen-test snapshot and raw missing-module failure.
`pure-signatures/` contains the compile-only signature scaffold and typecheck.
`pure/` contains raw pure-helper REPL attempts, the corrected one-expression-
per-line input, and the successful complete-state output. The first REPL
attempt returned process status zero despite import errors. The directory-
corrected multiline attempt loaded the module but stayed at continuation
prompts. Both transcripts are retained; neither is counted as a helper check.

The three `stage-*` directories preserve the exact harness snapshot, cwd,
argv, exit codes, raw streams, and hashes for first fill, second fill, and
cancellation. `final/` preserves the final typechecks, exactly sixteen tests,
the 10,000-sample safety/witness run, both intentional negated-witness
violations, JSON ITF inspection, focused Python check, S01 validator, and
diff check.

The two top-level ITF files are the requested complete terminal paths. Their
`status: violation` values refer only to intentional negated witnesses used to
produce reaching traces. The traces demonstrate cancellation reachability;
they do not report a safety failure.
