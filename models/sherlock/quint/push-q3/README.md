# Compiler and native-inventory finite models

Source baseline: `0699194f95524033699288678d8405df078c4f40`.
Quint: `0.32.0`. Run from the repository root:

```bash
bash models/sherlock/quint/push-q3/check.sh
```

The runner typechecks all twenty-two Quint files, runs 63 deterministic Quint
tests and eight committed Python tooling unit tests, and samples 10,000 traces
per model with `--max-steps 30`. Eight models use `modelInv`; the three pinned
defective source projections use `sourceInv`. A repaired classifier variant
additionally uses `--step classifyRepaired --invariant repairedInv`. All twelve
positive runs passed; every requested witness was reached. Its separate
`preservesDynamicTargets`, `stableAcrossFuel` and `belongsToActiveStep` runs deliberately find the
source counterexamples described below, which the runner checks explicitly. Raw results are in
`verification.log`. These are sampled finite checks, not exhaustive proofs.
Terminating worklist and scheduler traces intentionally stop before 30 steps.

| Model | Requirement and source | Checked scope |
| --- | --- | --- |
| `nativeInventory.qnt` | U023; `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs`, `report`, `Family::owner` | Four named table rows plus an unknown rule; event multiplicity versus unique-rule coverage, disjoint gap/unlisted partitions, source-only coverage versus unwired native acceptance |
| `witnessStorage.qnt` | U024 supporting behavior; `crates/mori-core/src/sherlock/helper_machine/witness_plan/strategy.rs`, `evaluate_with_storage` | Five-node scheduler harness; exact ordered cache arguments, reads always executed, baseline order retained, release after final consumer, output live through publication, semantic work untouched |
| `treeInterning.qnt` | U024 selected candidate; `69aeb882d:crates/mori-core/src/sherlock/optimization_intern.rs`, `intern_tree` | Bottom-up hash-consing with full leaf payload and ordered canonical child keys; seven-node fixture becomes four canonical nodes with structural sum 18; reversed children remain distinct; shape-only merging is detected |
| `planExports.qnt` | U031; `openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md:225` and `interfaces.md`, Separate compilation/export plans | Requirements model: separate constraint/witness dispositions and omissions, native closure versus structured witness support, field/layout/lookup/interaction/custom-gate gates, ordered repeated typed inputs, source/component membership, explicit format qualification |
| `residualWorklist.qnt` | U033; same spec at line 239 and `interfaces.md`, Bounded static residualization | Requirements model: finite memoized rule locations, recursive backedge without unrolling, both dynamic successors, size-limit generic fallback, all three masks preserve the dynamic input record |
| `residualBlockBudget.qnt` | U033; `c41f2da11:qa/holdout/contracts/U033.json` | Updated contract model counts one shared generic block inside the positive size cap, rejects zero as tooling refusal, and retains per-rule charges even when charged and free rules share one block |
| `residualFlow.qnt` | U033; `abe5ee146:qa/holdout/contracts/U033.json` | Records require a known producing block and correct rule membership; transitions follow same-block, explicit successor or generic-exit flow; invalid records are refused |
| `s5ResidualProjection.qnt` | U033; `cfbab29c5:models/sherlock/qa/residualize.py` | FIFO worklist and exact five-rule graph; source structural invariants pass while the dynamic-target contract fails for the `arith` family |
| `s5ResidualRepaired.qnt` | U033; `a82446698:models/sherlock/qa/residualize.py` | Repaired source retains all dispatch targets and explicitly delegates out-of-family calls; finite worklist, size bound and reachable-path coverage agree |
| `rustResidualIdentity.qnt` | U033; `2d1b2b373:crates/mori-core/src/sherlock/compile_artifact/draft_plans.rs:143` | Source-derived WIP counterexample: the same literal program selects static rules `{84}` at fuel 0 versus `{80,102}` at fuel 1, changing per-program IR content |
| `rustExportClassification.qnt` | U031; `b705bfacb:crates/mori-core/src/sherlock/export_plans.rs`, `classify` | Source-derived counterexample: a rule-195 scope return above a pending multiplication frame is attributed to multiplication instead of restoration |

`planExports` and `residualWorklist` do **not** claim an implemented Rust exporter
or residualizer. Neither existed in this baseline. Their correspondence remains
to be checked against an actual Rust implementation. The fixed masks currently
share one abstract dynamic graph; immutable image decoding/folding is outside
this model. `midnightSupported` witnesses a hypothetical capability combination,
not any existing backend or successful target execution.

The storage harness projects Sequence contents to ordered leaf lists and uses
an abstract publication node. It does not assert that this five-node graph is a
complete admitted Rust snapshot plan. It omits plan validation, types, locations,
read failures, cache scope changes and full decoded records. The observed
scratch-slot peaks are **3** for LastUse and **5** for RetainAll, with three slots
reclaimed; both the publication input and result remain live. These are model
counts, not measured bytes or runtime improvement.
The complete boundary's work/duties/frame fields are opaque preserved inputs.

The tree model projects the selected `sherlock-a-intern-equal-tree-nodes/1`
candidate on seven expanded postorder occurrences. It checks the interned
representation against independent structural prefix encodings and sums;
runtime expression traversal and complete reference/candidate machine traces
are outside this finite model. Fuel/frame/control/outcome metadata is opaque
and preserved. Its four-node result is a canonical-node count, not allocated
bytes, retained interner-table memory or a measured speedup. The source's full
trace preservation predicate and tests were read, but not run here.

The inventory model pins the inspected source-only/unwired baseline; it must be
updated when receipts or recursion wiring change. It does not validate receipt
provenance, actual event streams, the full rule table, dispatch constraints or
source-to-circuit refinement. Empty inventory coverage is vacuously complete;
that does not qualify native acceptance.

Three negative tests deliberately remove a dynamic successor, confuse fallback
with exhaustion, or subtract semantic work on a cache hit. Each checks that the
invariant rejects the mutation. They are model sensitivity checks, not findings
of these bugs in Rust.

No SDK load, proof generation, stock-verifier test, ledger interaction or
stagenet qualification was run. No U predicate is closed by these models.
Update the source correspondence and rerun the script whenever the named Rust
functions or normative interfaces change.

Reported source finding: `sherlock_native_inventory.rs` still expects 9/9 covered
rules for factorial/tree fold despite expanded C6 coverage in the table and the
owner's reported 37/41. Sent to sherlock-c4 and sherlock-fable; Rust reproduction
is pending their inventory run.

Two more findings were reproduced from committed Python tooling source at
`cfbab29c591e5f4aa634952596aee601fffcde1f` and sent to sherlock-s5,
sherlock-c4 and sherlock-fable:

- U033: `specialize('family', family='arith', limit=64)` removes `R_call` from
  `R_fetch`'s dynamic-control dispatch, without a family guard or generic
  fallback. The source projection violates `preservesDynamicTargets` after one
  worklist visit. See `s5-residual-counterexample.log`.
- U031: `prepare` accepts only a backend ID, using a hard-coded demonstration
  STEP. Its five witness nodes omit `typedInputs` and `typedOutputs`;
  `typedOperations` is a list of operation names. This is an honest tooling
  model, with execution/acceptance `none`, rather than typed plans bound to an
  actual Rust step.

`audit_s5.py` loads those two exact Git objects and reproduces the observations
without opening another worktree or copying its files. Its output is in
`s5-source-observations.json`. The positive requirements model does not adopt
the dropped-successor behavior. The source projection is deliberately pinned
to the defective version so later repairs do not erase the counterexample.

At `a82446698`, the owner repaired the lost `R_call` edge with an explicit
generic target/fallback and added typed input/output fields. The repair was
executed from its Git object, all eight of its Python unit tests passed, and
`s5ResidualRepaired` checks the repaired finite graph. U031's remaining scope
gap persists: these are demonstration plans, with one generic input/output per
node and no actual Rust-step parameter or per-operation operand arities.

The actual repaired Python source is also compared with hand-authored BFS
block prefixes and generic frontiers for all four masks at size limits
0 through 5: 24 cases passed. These expectations are explicit lists/sets,
independent of the source worklist implementation.

The evaluator's actual-Rust contract at `c41f2da11` requires a positive size
limit, includes generic blocks in that count, and requires executed per-record
reference streams. `residualBlockBudget` models that updated size contract with
a reserved generic block. Earlier prototype models intentionally retain their
zero-budget behavior and their own specialized-block counts; they do not claim
the later Rust wire contract. No real residual execution or Rust export-plan
result has been observed by this worker yet. U023's later qualification record
at `64822e4c7` likewise remains outside the source-only inventory projection.

`python3 models/sherlock/quint/push-q3/audit_qualification.py` reproduces a
separate U023 checker finding at `64822e4c7` and `0eb431b92`: both exact checker
versions accept reference/recorded result 6 with indexed state result 999, as
long as the supplied state hash matches. The checker compares a recorded
scalar with the reference and never decodes it from the indexed state.
Repository/indexer/reference oracles are in-memory unit fixtures; no real
network, wallet, transaction, proof or qualification was exercised. Results
are in `qualification-counterexample.json`; findings were sent to the
evaluator, sherlock-c4 and sherlock-fable. Holdout source was never edited.

The evaluator accepted that finding and repaired the checker at `489bbdaaf`.
The same audit script now uses a pinned binary 128-bit result field and key
body, preserving both historical counterexamples and checking eleven repaired
fixtures: two correct little/big-endian states pass and nine wrong-result,
record, key, initial-value, unchanged-field, layout-hash, effects and range
cases fail for their intended reasons. This confirms the targeted binding
repair with unit oracles. It does not prove source-to-key correspondence or
that the declared result window is compiler-derived; those remain explicit
qualification evidence obligations. No actual stagenet qualification is claimed.

Rust WIP `2d1b2b373` was source-audited, not built or run here. Its per-program
specializer derives static rule membership from `run_records`, so identical
code can change IR with fuel. `rustResidualIdentity` records that source-derived
counterexample; content identity is represented structurally, without a SHA
collision argument. The WIP's residual stream is separately produced by a
second reference run after post-hoc block labeling, and its multiplication
plan omits the left operand held in `Frame::Right`. All three findings were
sent to the owners and sherlock-fable; no final implementation verdict is
claimed for this explicitly WIP version.

The later Rust export module at `b705bfacb` also scans every pending frame for
a binary operation. For `Return(3)` with `[Scope, Right(Mul,2)]`, the active
K-A rule is 195 (scope restoration), while its manifest reports multiplication.
`rustExportClassification` preserves this one-step counterexample; FrameRestore
in that model denotes the restoration category, not a restriction against a
more specific ScopeRestore operation name. The owner reports a working-tree
repair selecting by rule/control/top frame and binding actual operand paths;
the targeted source repair was then confirmed at `6a6774a0c`. Its classifier
variant passes three directed tests and a 10,000-sample run using the custom
step named above, while the historical default step retains the bad source.
The concrete Rust runtime tests remain queued; the pinned test diff had a
scope regression but not the reported operand-value assertions. A separate
source finding remains: checked multiplication yielding `PrimitiveResult(Err(10))`
is currently exported with an ordinary UInt128 result placeholder. That needs
an explicit checked-result/error type or located Unsupported disposition.

At `abe5ee146`, the evaluator added producing-block IDs and successor discipline
to residual records after the post-hoc replay finding. `residualFlow` covers
that metadata contract, including the distinction that exiting a generic block
is allowed while entering one still needs an edge. It does not establish causal
execution of IR blocks; actual reference-prefix equality and the Rust producer
must still be audited. In particular, metadata-valid repeated records are not
asserted to be valid machine traces by this model.

The read-only U033 working-tree audit requested by sherlock-fable confirmed
that the rewrite removes trace-derived rule membership and the second
reference replay: it selects a group from the pre-state, steps once, checks
the rule assignment and records the selected block. Its dense successor sets
are conservative overapproximations permitted by `abe5ee146`; exact/minimal
control-flow edges are not a stated requirement. Tighter edges are possible:
scalar-check to publish; call to call/frame/publish; match to match/frame/publish.
No committed rewrite or runtime result is claimed by this working-tree audit.

The U024 CLI replay at `381f9e902` was source-audited against the tree model.
Core interning is unchanged, preservation is checked before reporting, and
technical cuts resume from interned snapshots. The CLI census excludes inline
roots (six child allocations become three), whereas the model's seven-to-four
count includes its root. The owner's committed run log records U024 passing;
this worker did not rerun Rust or read its ignored receipt trace. The stale
U023 inventory-count expectations were corrected at `d9908d374` to 37/41.
