# Optimization 07: deterministic witness and storage pipeline

Status: nonsecurity, specified-only. **P1:** explicit trace/witness-plan schemas and independent observations; **P2:** program-index and immutable-object caches; **P3:** checkpoint I/O after its semantics exists. No models, builds, proofs, SDK/native calls or benchmarks ran. All host, I/O, circuit and native costs remain unmeasured.

Baseline: Candidate A freeze SHA256 `57d830f46fee57c56ed8b854d21f5dd7a695d2977d061b1ef6905eaecd7de30e`; reference K SHA256 `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`. UIREQ-024's complete observation/work predicate and UIREQ-026's required Clean stage remain unchanged. This is host representation/computation work, not a new logical cost profile.

**Public source facts.** Clean's public core at `bba820f8bdd07d6ea70c43051f4fdf96b2f5186f` provides `Circuit.witnessIR` and `witnessVectorProgram` (`Basic.lean:165–175`). Its WitnessIR distinguishes structured `.ir steps out` from nonserializable `.native` closures (`WitnessIR.lean:19–38`). `Operations.witgenJson?` serializes flattened operations with structured witness programs, version1 and localLength; it returns an error for native closure programs (`WitnessExport.lean:136–175`). `generateNextRow` and `witnessesWithData` provide table-row witness plumbing (`Table/WitnessGeneration.lean:41–100`); table JSON includes circuit/assignment metadata (`Table/Json.lean:45–68`). These are existing interfaces, not Sherlock exporters or observed results.

**Recommendation/inference: stage the host pipeline explicitly.** Proposed APIs:

```text
prepare(program,profile) -> ProgramIndex + LayoutPlan
stepReference(state,program) -> nextState + frozenSourceSiteId
materialize(state,objectStore) -> completeLogicalState
appendTrace(pre,site,post) -> TraceRecord
lowerWitnessPlan(cleanOperations,LayoutPlan) -> WitnessPlan
emitWitnessRow(WitnessPlan,stepInputs) -> candidateFieldRow
persistChunk(records,objectDictionary) -> ChunkDescriptor
```

TraceRecord includes step index, source-site identity, complete pre/post logical states and consumed/remaining work. Assign source-site IDs in an external frozen inventory; current K rules lack a completed named-rule/work oracle. Serialize environments in a fixed key order, frames/constructor fields in semantic order, and integer quantities in an explicit canonical form. Those serialization rules are new host design, not existing K features. Every emitted snapshot must reconstruct code, control, environment, ordered frames, depth/limit, fuel/initialFuel and terminal output. References are storage indirection, not permission to omit a field.

ProgramIndex memoizes first-occurrence function lookup and prepared slot/layout metadata. Cache entries are scoped by the exact input program and representation version; the same integer function ID in two programs is not one cache key. Preserve argument evaluation before original lookup/binding outcomes. Cache hit/miss changes physical diagnostics only: the same A transition, work delta and exhaustion boundary remain visible.

Arena/frame caching stores immutable serialized node/frame contents and versioned environment snapshots. A possible key is `(programVersion,encodingVersion,arenaVersion,objectIndex)`, with exact underlying bytes retained. Do not assume a particular hash or cryptographic identity. Deduplicate physical blocks or serialize deltas only if materialization restores the entire logical state, including saved caller locals and pending non-tail returns. Sharing must not alter structural equality or occurrence-based traversal accounting. A bounded LRU may evict data and recompute it; eviction cannot change source fuel, result or declared frame cap. Any new storage refusal/capacity profile must be separately named rather than relabeled an A outcome.

**Clean interface and missing compiler work.** WitnessPlan must state field/limb layout, operation order, offsets, IR version and concrete inputs to each witness computation. Structured IR can express shared computations/compact loops, but its U64 arithmetic wraps modulo2^64 (`WitnessIR.lean:31–33`); it is not checked UInt128. Sherlock's encoding, UInt128 witness operations, semantic-state adapter and Clean step constraints are new work. Generated candidate rows are raw data; no host `verified`/`proofValid` flag or expected result substitutes for those missing components. Clean JSON export is not a Midnight exporter. Actual required integration belongs in existing private Fenrir; no private source/path/pin is included here.

**Segmentation gap.** Chunking an offline trace at a logical boundary is a storage choice. Candidate A pure K has no durable wait/resume/checkpoint semantics. A proposed checkpoint envelope must retain all state/object/code/layout/work data needed for exact reconstruction; no checkpoint I/O or resume implementation is claimed. Trace chunks do not establish financial commit boundaries or native recursive-proof segments. Financial effects, duties and suspension observations remain absent and require separately specified extensions.

**Normal comparisons and predicate.** Retain all eight frozen fixtures and their independent result literals. Add two programs reusing functionId7 but returning11 and22 respectively; repeated cache-cold/cache-warm runs of each; nested-shadowing result11; and tree45=9 chunked while its right traversal is pending. Freeze complete intermediate snapshots/work expectations before later execution. The measurable target is identical materialized logical traces/terminal envelopes and exact work/caps, with separately lower reported witness-generation time, bytes copied, peak resident objects or I/O for a stated workload. No numeric saving is claimed.

**Counterargument.** Metadata, materialization and serialization can outweigh saved traversal; cached full states may increase memory. Start with deterministic uncached output, retain it as the comparison artifact, then evaluate caches independently. Host gains alone establish no circuit-row, native-proof-time or target-cost improvement.
