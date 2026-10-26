# Compiler and native-inventory finite models

Source baseline: `0699194f95524033699288678d8405df078c4f40`.
Quint: `0.32.0`. Run from the repository root:

```bash
bash models/sherlock/quint/push-q3/check.sh
```

The runner typechecks all sixteen Quint files, runs 46 deterministic Quint
tests and eight committed Python tooling unit tests, and samples 10,000 traces
per model with `--max-steps 30`. Seven models use `modelInv`; the pinned defective
s5 source projection uses `sourceInv`. All eight
positive runs passed; every requested witness was reached. Its separate
`preservesDynamicTargets` run deliberately finds the source counterexample
described below, which the runner checks explicitly. Raw results are in
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
| `s5ResidualProjection.qnt` | U033; `cfbab29c5:models/sherlock/qa/residualize.py` | FIFO worklist and exact five-rule graph; source structural invariants pass while the dynamic-target contract fails for the `arith` family |
| `s5ResidualRepaired.qnt` | U033; `a82446698:models/sherlock/qa/residualize.py` | Repaired source retains all dispatch targets and explicitly delegates out-of-family calls; finite worklist, size bound and reachable-path coverage agree |

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
