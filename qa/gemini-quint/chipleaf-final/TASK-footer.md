## Output files

- `qa/gemini-quint/chipleaf-final/@NAME@.qnt` and `qa/gemini-quint/chipleaf-final/@NAME@_test.qnt`.
- `qa/gemini-quint/chipleaf-final/REPORT-@NAME@.md`, written LAST, with:
  1. a verdict per question of this lane: SOUND / SOUND WITH CONDITIONS / NOT SOUND, each condition stated as a constraint on the code, with the file and line that meets it (or does not);
  2. every finding numbered, with its witness (Quint definition name), the source lines it refers to, and the constraint that closes it;
  3. a table mapping each attack question to its mutant module, the expected `quint run --invariant invRefine` exit code, and whether the honest witness refines under the mutant;
  4. if you find a real gap in the Rust code: a precise description of a failing MockProver test that would show it (which witness, which cells, which gate or argument should fail but does not). Do not change the Rust code;
  5. an explicit list of every place where your model abstracts or differs from the Rust (sizes, field, hash, rows);
  6. the exact `quint` commands (typecheck, `quint test --main ...`, `quint run --main ... --invariant ... --max-samples ... --max-steps ... --seed ...`) with the expected exit code of each, and honestly every command you ran and what it printed.
