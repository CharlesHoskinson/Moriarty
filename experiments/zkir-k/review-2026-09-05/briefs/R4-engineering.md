# Review R4: K engineering quality, extension surface, tooling and documentation

Files: all of experiments/zkir-k/semantics/*.k with emphasis on zkir-syntax.k,
zkir-ext.k, zkir-check.k, zkir-test.k; tools/zkir_kast.py, check_corpus.py,
extract_test_inputs.py, gen_handmade.py; the wiki pages named in COMMON.md;
K user manual at /home/charl/Moriarty/repos/runtimeverification/k/docs/user_manual.md.

Questions to answer:
- K idioms and pitfalls: function totality claims ([function, total]) that are
  not total, overlapping function rules without owise or priorities
  (nondeterministic results), unused-variable and non-exhaustive-match warnings
  that hide bugs, list/set/map patterns that the LLVM backend evaluates
  differently from the Haskell backend, string-based dispatch ("add", "mul")
  where sorts would be safer, cells that should be part of the program term,
  `symbol` naming collisions with instruction constructors.
- Will the definition behave the same under the Haskell backend (symbolic
  execution, kprove)? What blocks writing claims about it (hash functions as
  uninterpreted symbols, non-linear field arithmetic)?
- ZKIR-EXT: is layering the 2ffe2d1 surface on the 92e8bdd3 definition sound
  (strict decoding via a priority rule, Bool negation, Bytes<32> aliasing
  bytes32, load_constant), and what in 2ffe2d1 changed that the extension does
  not model?
- The preprocessor: every serde behaviour of ir.rs it mirrors and any it does not.
- Static checks (ZKIR-WF): are they exactly the producer obligations of the
  spec (docs/zkir-v3-spec.md section 8) or a different set? List differences.
- Tests: what a green run does and does not establish; flakiness risks
  (random seeds, oracle binaries built from patched worktrees).
- Documentation: does wiki/zkir/zkir-k-definition.md claim anything the
  receipts do not support?
