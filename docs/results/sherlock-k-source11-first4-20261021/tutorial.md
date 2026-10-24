# Inspect the recorded first4 result

Start with `ROOT-TEN-PURE-TESTS-RESULT.json`, then open `actual-pure-tests/stderr.bin`. It names all ten tests once and ends with the recorded Ran 10 / OK envelope. The empty stdout and exact command/result files are alongside it.

Next read `ACTUAL-PHASE-SUMMARY.json`. For each public fixture the five phases are render, kast-json, depth1-reduce, full-print and whole-decode. Follow one case under `actual-windows/`; read each command and process result before its stdout/stderr. These are historical captured streams, not commands to execute.

Finally read `ROOT-INDEPENDENT-WHOLE-COMPARISON.json`. Root records whole typed-AST/byte equality, complete source preservation and final identity. The independent expected bodies are intentionally absent from this publication; the preparer did not inspect them. The raw full-print streams are actual observations, while Root's JSON supplies the attributed comparison result.

This exercise reviews existing evidence. It does not rerun K, install the absolute-pinned receiver or establish production correspondence.
