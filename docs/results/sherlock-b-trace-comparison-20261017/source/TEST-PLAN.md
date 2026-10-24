# Positive comparison cases frozen before implementation/execution

All expectations below are defined from supplied data, independently of any owner transition output. Test source is authored before the comparator source. None has executed.

1. Identical initial values and present empty record slices compare equal; equal cloned nonempty records compare equal, including deliberately discontinuous supplied snapshots.
2. Different initial wins over record and length differences, with original initial references returned.
3. In one record, before wins over event/after; event wins over after; after is identified when earlier fields match. All returned references point to original supplied fields.
4. Record zero after wins over record one before. Equal earlier records advance the reported index.
5. Equal common prefix followed by extra records reports both lengths/common count in either direction, including zero versus one. A common-record difference precedes length.
6. Direct typed contrasts in each admission group, layout options/flags, program constants/order, image origin/counters/options/raw references/arena ordering must report the correct snapshot location. This exercises full concrete snapshots without constructing private SegBase or Admitted internals.
7. Every public StepEventC field is changed separately from one explicit public event literal. Each reports the same raw event location, preserving IDs/options/locators/deltas and tags. No actual owner event or transition is inferred.

Prospective checks: Root reviews the complete new source, obtains fresh applicable classification, registers the leaf, compiles only the ordinary SDK-free production/core external-test closure using already completed ordinary dependencies, lists exact test names, and executes only these tests serially under bounded time. No broad Cargo/devtest/SDK closure or engine-step test is proposed. The exact admitted/capture closure is not authorized by historical R44 classification.
