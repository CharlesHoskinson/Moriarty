# Assurance findings (Formal-Lean-B, assurance fork, 2026-11-15)

A spec status of OPEN or GAP is not a finding. Rows here are contradictions, stale numbers or invariants violated by recorded values that the spec does not already acknowledge.

| ID | Requirement | Kind | Evidence | Proposed verdict |
|---|---|---|---|---|
| F-B-AS-1 | SQG-013 (and NAG-005, LPC-002 in Aggregation) | stale cited number | The MERGE-WORK width has three recorded values: 5,284 MEASURED (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:96`, the `merge_work` key of T10m); "5,128 columns (MEASURED)" in the code comment at openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/frozen.rs:164` to `:165`; 4,988 MEASURED and 5,121 DERIVED in the NAG-005 status and `bus/coverage-B.md`. All three are at most 5,400, so the bound holds on every recorded value (`sqg013_merge_work`). The code comment is stale against the T10m measurement. | GUARD HOLDS (the 5,400 bound holds; reconcile the comment and the NAG-005 status text, owner lane T8c) |
