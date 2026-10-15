# Optimization 09: shared inventories, independent semantics and oracles

Status: nonsecurity software/proof-workflow design, specified-only. **P0:** freeze a source-to-source rule/tag inventory; **P1:** share mechanical encoding metadata; **P2:** reduce repeated semantic/encoding bookkeeping through narrow module interfaces. No model, Lean, build, proof, benchmark or native command ran. Compilation/elaboration time, proof-maintenance effort and native costs are unmeasured. Security theorem statements, proof judgments and hostile tests remain Opus-owned.

Baseline: Candidate A freeze SHA256 `57d830f46fee57c56ed8b854d21f5dd7a695d2977d061b1ef6905eaecd7de30e`; K source `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`. UIREQ-024/026 apply. Frozen inputs pin Quint main `fff2752e…`; that exact original is retained in diagnostics. Current Quint main is `6c152f5c…` after two return-type annotations. sources.json records both; the repaired file is not silently substituted into the immutable baseline.

**Source facts.** K defines concrete controls/frames at lines38–62 and charged transitions at101–226; recursive helpers are separate at228 onward. Exact fixture fuel totals remain unfrozen. Quint's frozen Program enum contains only Factorial/ListSum/TreeSum, with algorithm-specific frames, scalar15 and eight nodes/frames; its README/model coverage explicitly denies a general input-code interpreter. Quint's official frozen language reference states no recursive operators [Q1]. Public Clean provides fixed-size `ProvableType`/`ProvableStruct` (`Provable.lean:16,319`), `FormalCircuitBase.main` (`Formal.lean:36–40`), a separate `FormalCircuit` specification field (`199–202`) and `InductiveTable.step` (`Table/Inductive.lean:58–63`). These interface declarations are reuse facts, not assessed theorem results.

**Recommendation/inference: one inventory, several deliberately separate meanings.** Proposed source mapping interfaces:

```text
RuleRow = {id, kSourceSpan, sourceDigest, controlClass,
           frameAction, envAction, workDelta, terminalClass}
mapKToLean(ruleId) -> semanticStepCaseId
mapLeanToWire(caseId,profile) -> encodingComponentId
mapWireToClean(componentId) -> componentInterfaceId
projectQuint(actionId,scenario) -> named K-step sequence + costAdapter
```

Manually reconcile every K machine rule, including zero-charge startup/publication, rejected push, frame replacement, typed return and helper boundaries. Preserve exact A costs/caps; a newly metered-helper schedule needs a distinct identifier. Do not infer semantics from matching constructor names. The rule inventory records correspondence obligations and omissions; it does not establish them.

Use a proposed `EncodingSchema` for enum tags, ordered record fields, widths, capacities and layout version. A pinned generator may emit matching enum declarations, record accessors, serializers and rule-coverage tables. Its receipt must record generator source/version, input schema/model digests, invocation and output digests. No generator exists or ran in this lane. Generate mechanical metadata only: independently authored K guards, Lean transition behavior and Clean component meaning remain reviewable. Generating the same semantic guard and its oracle from one template would merely duplicate its mistake.

**Module/lemma maintenance boundaries.** Keep recursive `Syntax/State` separate from `SemanticStep/Run`, with fuel-indexed run composition around the fixed step relation. Keep `BoundedEncoding` separate from semantic types; a finite wire arena is not the recursive Lean value type. Centralize semantic bookkeeping about environment extension/restoration, ordered frames and work/depth arithmetic, and encoding bookkeeping about fixed field order/size. Keep circuit components and their witness programs behind explicit typed interfaces; public Clean's bundled circuit interface motivates a component boundary rather than ad hoc opaque fragments. Actual circuit/theorem obligations are separately assigned, not filled by generated proof placeholders or host flags. Required development belongs in existing private Fenrir; no private source or theorem is imported into this proposal.

**Independent normal evidence.** Keep all eight hand-frozen K result literals: factorial3=6, sum3=6, list123=6, tree45=9, bool/let=7, caller-local=12, mutual-even4=true, constructed-list=[1,2]. Before future execution, independently enumerate selected intermediate controls, frame payloads, caller maps and work deltas. A future pinned reference run may derive an observed trace for comparison; that trace never replaces the independently written expected observations or establishes a proof. Add a different program table reusing function IDs, nested-shadowing result11, and pending tree-right traversal as discriminators against duplicated examples.

**Quint projection gap.** Its terminal tick consumes work while K publication is free; Waiting has no current K counterpart, and scalar15 cannot stand for UInt128. The adapter must name the restricted factorial/list/tree subset, structural-value/arena mapping, multi-K-step projection and exact cost adjustment. Generate a coverage matrix, not a new generic-interpreter claim from shared final values. General calls, lexical scope, mutual recursion and finance remain outside that projection.

**Measurable preservation/efficiency target.** Every mapped boundary retains complete declared observations/work/caps; metadata-only regeneration changes no semantic source. Later compare changed-source fanout, duplicated definitions and independently observed elaboration/check time for the same concrete edit, while retaining the independent normal oracle. No improvement is measured now. Counterargument: inventories and generators add another maintenance layer and can synchronize errors across languages. Start with a small manually reviewed mapping, share only stable structural metadata, and expand when actual repeated edits justify it.
