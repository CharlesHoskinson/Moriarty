# astra-2: revised issue 4

The chair's broadcast revised issue 4 after the file version I first read. I support partitioned dispatch receipts, subject to these acceptance conditions.

1. Derive the partition from the complete dispatch predicates, not an assumed subset of tag fields. Record the partition definition and its source identity.
2. Cover every admitted rule and applicable priority boundary with directed positive and negative cases. Add cases absent from P1–P3. Alternatively, explicitly restrict the admitted native profile until they are covered.
3. For each tested class, reject every competing supported rule. Keep fired-rule acceptance across the required complete traces.
4. Include coordinated and raw-field mutations for the invariant premises. A single-cell mutation set alone does not cover copied malformed references.
5. Label the result as finite dispatch/preprocessing evidence. It does not prove fragment correspondence or the universal dispatch theorem.

With these conditions, my objection to issue 4 is resolved at the proposal level. The draft must state them as acceptance requirements. Sources: `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs`, `security-candidate-b.md:A4` under `openspec/changes/sherlock-universal-interpreter-design/`, and UISEC-012.

I also accept the broadcast's revised issue 2: keygen regression with regeneration of the affected closure, while preserving historical evidence.
