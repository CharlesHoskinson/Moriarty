# Compiler and native-inventory finite models

Source baseline: `0699194f95524033699288678d8405df078c4f40`.
Quint: `0.32.0`. Run from the repository root:

```bash
bash models/sherlock/quint/push-q3/check.sh
```

The runner typechecks all twenty-eight Quint files, runs 92 deterministic Quint
tests and eight committed Python tooling unit tests, and samples 10,000 traces
per model with `--max-steps 30`. Eleven models use `modelInv`; the three pinned
defective source projections use `sourceInv`. A repaired classifier variant
additionally uses `--step classifyRepaired --invariant repairedInv`. All fifteen
positive runs passed; every requested witness was reached. Its separate
`preservesDynamicTargets`, `stableAcrossFuel`, `belongsToActiveStep` and `sourceMatchesCodec` runs deliberately find the
source counterexamples described below, which the runner checks explicitly. Raw results are in
`verification.log` (79 tests before the strengthened selector/classifier checks),
`selector-sweep.log` (all nine current dispatcher tests) and
`classification-repin.log` (all eight current classifier tests and fresh
10,000-trace source/repaired runs), and `program-pattern-rows.log` (ten pattern
row tests plus its 10,000-trace run). The selector sweep checks all
6,000 positive-fuel selector combinations plus exhaustion and reaches every
one of the 39 rule IDs. This is exhaustive within that selector quotient;
the sampled runs are not exhaustive proofs of complete machines.
Negative searches also explicitly set `--max-samples 10000`: Quint otherwise
defaults to one sample when a seed is supplied. Expanding the classifier's
input set exposed that implicit budget by selecting a valid multiplication
first. `python3 models/sherlock/quint/push-q3/check_counterexamples.py` runs the
four actual negative commands from the runner and verifies that each finds
its intended invariant violation; see `counterexample-search-check.log`.
That focused integration check failed before the budget fix and passed after it.
Terminating worklist and scheduler traces intentionally stop before 30 steps.
`exportMulBindings` uses `--backend=typescript`: the default Rust evaluator is
limited to i64 integers and cannot represent the actual u128 maximum. Its
finite operands are 0, 1, 2, 3 and 2^128 - 1, without truncating multiplication.

| Model | Requirement and source | Checked scope |
| --- | --- | --- |
| `nativeInventory.qnt` | U023; `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs`, `report`, `Family::owner` | Four named table rows plus an unknown rule; event multiplicity versus unique-rule coverage, disjoint gap/unlisted partitions, source-only coverage versus unwired native acceptance |
| `witnessStorage.qnt` | U024 supporting behavior; `crates/mori-core/src/sherlock/helper_machine/witness_plan/strategy.rs`, `evaluate_with_storage` | Five-node scheduler harness; exact ordered cache arguments, reads always executed, baseline order retained, release after final consumer, output live through publication, semantic work untouched |
| `treeInterning.qnt` | U024 selected candidate; `69aeb882d:crates/mori-core/src/sherlock/optimization_intern.rs`, `intern_tree` | Bottom-up hash-consing with full leaf payload and ordered canonical child keys; seven-node fixture becomes four canonical nodes with structural sum 18; reversed children remain distinct; shape-only merging is detected |
| `planExports.qnt` | U031; `openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md:225` and `interfaces.md`, Separate compilation/export plans | Requirements model: separate constraint/witness dispositions and omissions, native closure versus structured witness support, field/layout/lookup/interaction/custom-gate gates, ordered repeated typed inputs, source/component membership, explicit format qualification |
| `residualWorklist.qnt` | U033; same spec at line 239 and `interfaces.md`, Bounded static residualization | Requirements model: finite memoized rule locations, recursive backedge without unrolling, both dynamic successors, size-limit generic fallback, all three masks preserve the dynamic input record |
| `residualBlockBudget.qnt` | U033; `c41f2da11:qa/holdout/contracts/U033.json` | Updated contract model counts one shared generic block inside the positive size cap, rejects zero as tooling refusal, and retains per-rule charges even when charged and free rules share one block |
| `residualFlow.qnt` | U033; `abe5ee146:qa/holdout/contracts/U033.json` | Records require a known producing block and correct rule membership; transitions follow same-block, explicit successor or generic-exit flow; invalid records are refused |
| `kResidualDispatch.qnt` | U033; `8ffed57fc:crates/mori-core/src/sherlock/compile_artifact/draft_plans.rs`, `dispatch_group`, and `reference.rs` | Finite pre-state selector quotient over all 39 rule IDs; seven disjoint groups, terminal priority, positive-cap generic fallback, and rule 177 entering arbitrary next arguments directly |
| `s5ResidualProjection.qnt` | U033; `cfbab29c5:models/sherlock/qa/residualize.py` | FIFO worklist and exact five-rule graph; source structural invariants pass while the dynamic-target contract fails for the `arith` family |
| `s5ResidualRepaired.qnt` | U033; `a82446698:models/sherlock/qa/residualize.py` | Repaired source retains all dispatch targets and explicitly delegates out-of-family calls; finite worklist, size bound and reachable-path coverage agree |
| `rustResidualIdentity.qnt` | U033; `2d1b2b373:crates/mori-core/src/sherlock/compile_artifact/draft_plans.rs:143` | Source-derived WIP counterexample: the same literal program selects static rules `{84}` at fuel 0 versus `{80,102}` at fuel 1, changing per-program IR content |
| `rustExportClassification.qnt` | U031; `b705bfacb:crates/mori-core/src/sherlock/export_plans.rs`, `classify` | Source-derived counterexample: a rule-195 scope return above a pending multiplication frame is attributed to multiplication instead of restoration |
| `exportMulBindings.qnt` | U031; `32fcbce3e:crates/mori-core/src/sherlock/export_plans.rs`, rule-153 `classify` arm | Actual u128-bound multiplication projection; ordered captured-left and returned-right bindings, sourced success or error result, rejection of swapped operands, wrong paths/results and the historical missing-result placeholder |
| `programPatternRows.qnt` | U023 compiler/native codec; `388b92d9b:crates/mori-backend/src/sherlock_machine/c6_program_context_canonical.rs`, `step`/`var_step`, and `crates/mori-core/src/sherlock/helper_machine/c6_phys_program.rs` | Pattern-row segment: previous-tag width selection disagrees with literal owner encodings; current-tag-at-sub0 variant preserves all row boundaries in the finite projection |

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

Reported source finding: `sherlock_native_inventory.rs` expected 9/9 covered
rules for factorial/tree fold despite expanded C6 coverage in the table and the
owner's reported 37/41. Sent to sherlock-c4 and sherlock-fable; the corrected
source expectations are recorded below.

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
At that pin, the test diff had a scope regression but not operand-value
assertions, and checked multiplication yielding `PrimitiveResult(Err(10))`
still exported an ordinary UInt128 result placeholder. The scope regression
also selected from factorial, which contains no let/match construct that creates
the needed scope frame. These follow-up findings were reported to the owner.
Commit `32fcbce3e` repairs the error arm with a sourced `CheckedU128Result`,
error code and `uiPrimError` value; adds a MAX-times-two overflow test; selects
scope restoration from a dedicated match-under-multiply program; and asserts
actual multiplication operands `uiU(3)`, `uiU(2)` and result `uiU(6)`. This worker
confirmed those source changes. The owner reports five Rust tests passing;
this worker did not rerun them.
The multiplication-binding model assumes a valid rule-153 reference record
with UInt operands and represents KDisplay values by constructors. It checks
bindings against the supplied record and independent arithmetic conditions;
it does not execute Rust, serialize JSON or verify non-multiplication exports.

Source regression at `3fabcca1c`: s5's new CLI selects the core exporter, but
that module is byte-identical to defective `b705bfacb`, without the later
`6a6774a0c` / `32fcbce3e` / `3bb404f15` repairs. Its committed U031 trace says
PASS despite the scope/pending-multiplication misclassification and missing
value/error bindings. `audit_s5.py` checks that exact module identity and the
new route. The existing model violates `belongsToActiveStep` for rule 195 with
`[Scope, RightMul]` and rule 149 with `[LeftMul]`; see
`s5-export-regression-counterexample.log` and the directed classifier tests.
This is a source-derived regression finding, reported to s5, s3, the evaluator
and fable. The correctly repaired consolidated exporter remains at
`93f61a41c`; the older s5 PASS is not evidence for those repairs.
The evaluator accepted the finding and strengthened U031 at `7f0942c3d`:
rules 149 and 195 must not report any `CheckedU128*` operation. The model now
samples both cases; old-source witnesses are reached and the repaired source
classifies the scheduling/restoration steps without multiplication. No real
Rust run was performed by this worker. The evaluator independently reports
the repaired s9 binary passing the strengthened holdout. Its own union
`461e76d8c` has the exact repaired `3bb404f15` exporter, although the union's
committed U031 run still used its older local holdout; the consolidation must
record the new-E doctorwatson check separately.

`audit_u031_observations.py EVALUATOR_SHARE_DIR` checks the two SHA-pinned
evaluator JSONs at rules 195/149 with unmodified `7f0942c3d` plan validators.
The observations report `ArenaRead, ScopeRestore` at position 24 and
`ArenaRead, ArenaRead` at position 22. The audit checks the source/value
bindings, separate canonical manifest/plan hashes, operation/node hash recipes,
namespaced component IDs, component/node membership and dependency order.
All ten mutations (five per observation) are rejected for their intended
reasons, including a rehashed false multiplication that reaches the new
forbidden-operation check. Results are in `u031-observation-audit.json`.
These probes requested no emissions, so this audit does not check emitted
file bytes; the evaluator's complete holdout PASS is a separate external report.
The raw inputs remain external and no Rust/native/target execution is claimed.
The consolidated ledger is now confirmed too: `369281b01` records run
`20261026T035253Z-U031` exit 0 at source head `82cbf4f4e`. Both commits carry
the exact `7f0942c3d` holdout tree and exact `3bb404f15` exporter. That closes
the earlier current-E evidence gap; no binary execution by this worker is implied.

At `abe5ee146`, the evaluator added producing-block IDs and successor discipline
to residual records after the post-hoc replay finding. `residualFlow` covers
that metadata contract, including the distinction that exiting a generic block
is allowed while entering one still needs an edge. It does not establish causal
execution of IR blocks; actual reference-prefix equality and the Rust producer
must still be audited. In particular, metadata-valid repeated records are not
asserted to be valid machine traces by this model.

The read-only U033 audit requested by sherlock-fable, subsequently pinned to
committed rewrite `8ffed57fc`, confirmed that it removes trace-derived rule
membership and the second
reference replay: it selects a group from the pre-state, steps once, checks
the rule assignment and records the selected block. Its dense successor sets
are conservative overapproximations permitted by `abe5ee146`; exact/minimal
control-flow edges are not a stated requirement. Scalar-check can go to publish
and match can go to match/frame/publish. An earlier suggestion to restrict call
to call/frame/publish was wrong: rule 177 evaluates an arbitrary next argument
directly, allowing scalar, scalar-check, branch and match as well. The correction
was sent to sherlock-s5 and sherlock-fable before any production narrowing.
`kResidualDispatch` includes directed branch, match and missing-variable
regressions for that transition. It samples independent abstract snapshots,
not whole residual runs; expression values, environments and stack contents
beyond the inspected tags/predicates are abstracted. No Rust runtime result
is claimed by this source correspondence check.
At `abc64975d`, the owner's committed run record binds `uireq-033` exit 0 to
the exact `8ffed57fc` source, with 4.344 seconds wall time and 72,998,912 bytes
peak RSS. `audit_s5.py` verifies that binding, unchanged residual source and
source-commit ancestry. Its committed trace copy at `3fabcca1c` has SHA-256
`a226a8942fde60983d79ad00eb0ee24b6df4fa650a1916b4d8f1d2cbfa387b1f`
and says `PASS uireq-033`. The audit also verifies all seven residual functions
are unchanged at that newer commit, which removes the unused legacy exporter.
The original ignored trace file was not read. The evaluator independently reported the same
holdout passing on that source. These are external runtime reports, not Rust
executions by this worker, and show no residual size or performance improvement.

The evaluator subsequently shared raw factorial(3) residual output and a fresh
reference-only companion from the same saved binary. The separate
`audit_u033_observation.py RESIDUAL.json REFERENCE.json PROBE.py` pins both
file hashes, extracts unmodified validators from evaluator commit `abe5ee146`,
and compares the observations without running the binary. It confirms the IR's
canonical hash and program-source hash, seven blocks, 39 boundary rows, 114
equal StepRecords, all complete boundary snapshots, reference work 113,
unchanged host summary and `Done uiU(6)` at fuel 4000/stack 128. Five mutations
(unknown/wrong producing block, missing exercised successor, changed reference
record and wrong IR hash) are rejected for the intended reasons. Results and
both SHA-256 pins are in `u033-observation-audit.json`. This is a concrete observation audit, not a fresh
Rust run, complete refinement proof or native/target acceptance test. The two
large input JSON files remain in the evaluator's external shared directory.

The U023 PROG parser finding at `388b92d9b` is separate from the inventory
projection. At pattern-row sub-position 0, the emitter stores the incoming
tag in `nxt.hold`, but selects the row width from `org.hold`. Pattern tags
1 (PNil) and 5 (Wildcard) have width 1. A stale 1/5 therefore terminates a
wider row after its tag; a new 1/5 after a non-unit tag fails to terminate.
The stale tag can also come from the expression table: Expr Var=1 and If=5
precede the pattern table with the same shared hold register. The native
owner confirmed this source finding and committed the current-tag fix at
`d570c75cc` in both the program-context and duplicated validate-field parser.
Both source patches were audited: `pat_start` is exactly PAT_ROW/sub0,
`pat_hold` selects the current lane there and `org.hold` elsewhere, and the
width table/`var_step` are unchanged. This matches the repaired model.

`programPatternRows` compares literal owner-codec row encodings with both
the old latch rule and a proposed current-tag-at-sub0 variant. It explores
one/two rows, all seven pattern tags, prior tags 0 through 8, and zero-valued
usize payloads. Ten tests and 10,000 traces pass the repaired boundary
invariant; the old source violates the codec boundary after one lane in
`program-pattern-counterexample.log`. One test exhausts all 504 initial cases
(56 one/two-row sequences times nine prior tags), checking every lane prefix.
The row-boundary source argument is local: at sub0 the current tag selects
its owner-codec width; later positions use that same latched tag, and
`var_step` finishes exactly at width minus one before decrementing the row
count and resetting sub0. It therefore composes row by row under the parser
state/valid-tag assumptions. This is a source argument, not a formal Rust or
circuit refinement proof. The finite model is a segment projection, excluding
the rest of PROG, gates other than 1, other range/UTF-8 checks, full field
arithmetic and native execution. The owner reports that the previously failing
tree-fold step0 now accepts; full fragment reruns remain external evidence.

The U024 CLI replay at `381f9e902` was source-audited against the tree model.
Core interning is unchanged, preservation is checked before reporting, and
technical cuts resume from interned snapshots. The CLI census excludes inline
roots (six child allocations become three), whereas the model's seven-to-four
count includes its root. The owner's committed run log records U024 passing;
this worker did not rerun Rust or read its ignored receipt trace. The stale
U023 inventory-count expectations were corrected at `d9908d374` to 37/41.
