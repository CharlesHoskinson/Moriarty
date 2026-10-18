# Independent UIREQ030 normal oracle

`EXPECTED-NORMAL.json` contains complete literal input components/full arrays and
checked results for the original3 pairs in both32/64 layouts plus declared normal
supplements. `INPUT-SCHEMA-FREEZE.json` precedes population. `DERIVATION.md` explains
exact bigint equations and `FIELD-COVERAGE.json` includes every node/padding limb.

Proposed fixture destination:
`crates/mori-core/tests/fixtures/word-arithmetic/EXPECTED-NORMAL.json`.
Proposed direct Rust consumer destination:
`crates/mori-core/src/sherlock/word_arithmetic_independent_normal_tests.rs`.
Root may apply `REGISTRATION-SNIPPET.rs.txt` inside the proposed word_arithmetic
module. The consumer calls only frozen public names and compares literal fields;
it does not parse the data fixture, import a reference producer or regenerate
expected values from actual output. Its12 normal tests remain uncompiled/unrun.

Eight ordinary Python data checks passed. Their exact tool result is retained;
this is mathematical data consistency, not Rust, circuit, SDK, proof or target
acceptance. Source author received metadata readiness/pins only. Root must assess
source and confirm expected freeze precedes actual subject selection/execution.
No security/rejection/overflow outcome cases or full UIREQ030 closure are claimed.
