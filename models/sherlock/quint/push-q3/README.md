# Compiler and native-inventory finite models

Source baseline: `0699194f95524033699288678d8405df078c4f40`.
Quint: `0.32.0`. Run from the repository root:

```bash
bash models/sherlock/quint/push-q3/check.sh
```

The runner typechecks all eight Quint files, runs 26 deterministic tests, and
samples 10,000 traces per model with `--invariant modelInv --max-steps 30`.
All four runs passed; every requested witness was reached. Raw results are in
`verification.log`. These are sampled finite checks, not exhaustive proofs.
Terminating worklist and scheduler traces intentionally stop before 30 steps.

| Model | Requirement and source | Checked scope |
| --- | --- | --- |
| `nativeInventory.qnt` | U023; `crates/mori-core/src/sherlock/compile_artifact/native_inventory.rs`, `report`, `Family::owner` | Four named table rows plus an unknown rule; event multiplicity versus unique-rule coverage, disjoint gap/unlisted partitions, source-only coverage versus unwired native acceptance |
| `witnessStorage.qnt` | U024 supporting behavior; `crates/mori-core/src/sherlock/helper_machine/witness_plan/strategy.rs`, `evaluate_with_storage` | Five-node scheduler harness; exact ordered cache arguments, reads always executed, baseline order retained, release after final consumer, output live through publication, semantic work untouched |
| `planExports.qnt` | U031; `openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md:225` and `interfaces.md`, Separate compilation/export plans | Requirements model: separate constraint/witness dispositions, native closure versus structured witness support, field/layout/lookup/interaction/custom-gate gates, ordered repeated inputs, explicit format qualification |
| `residualWorklist.qnt` | U033; same spec at line 239 and `interfaces.md`, Bounded static residualization | Requirements model: finite memoized rule locations, recursive backedge without unrolling, both dynamic successors, size-limit generic fallback, all three masks preserve the dynamic input record |

`planExports` and `residualWorklist` do **not** claim an implemented Rust exporter
or residualizer. Neither existed in this baseline. Their correspondence remains
to be checked against sherlock-s5's implementation. The fixed masks currently
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
