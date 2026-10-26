# Real host export-plan contracts (U031)

This `Init`-only module follows the selected-record Rust API in
`sherlock/export_plans.rs` at `97412cb2c`. It is separate from the historical
Python example in CompilerResidualization. Source/query hashes and exact Rust
source pins are in the matching JSON. The module models data-shape and
classification laws, not an extracted Rust implementation.

Six typed request names round-trip. Canonical request order removes repetition.
Record positions are one-based: zero is refused and one selects the head of the
supplied exact K-A stream. The primary operation depends on the executed rule,
returned control and top frame; a pending frame below the top does not select
an operation. Checked multiplication at rule 153 and ReturnF restoration at
185 have named statements. Concrete stream selection, source/state binding and
complete operational correspondence remain unproved.

Two source limitations are mirrored explicitly, not silently repaired. The
current generic Eq operator is labeled `UInt128Eq`, despite the reference and
draft checker admitting arbitrary/unequal operand sorts. Checked primitive
failure produces an ordinary UInt128 type placeholder without a bound value,
source path or error variant. `error_result_has_no_bound_value` states that
omission; it does not validate the placeholder as a computation result. Both
findings were reported to the source owner/coordinator.

Operation bindings and witness-node bindings are distinct typed records. Separate
constraint/witness plan types share one manifest association. The source's
component descriptors, copied typed values and declared structured-witness
capability do not prove reconstruction of the complete state or execution of a
constraint system/witness program. Captured frame operands and restored
environments require particular attention. Dependency coverage, unique IDs,
topological order and slot/lifetime correctness remain obligations.

An identified body's own ID is excluded from the supplied canonical bytes.
An artifact hashes the complete serialized plan, including its plan ID. These
are different hashing roles. No JSON encoder, SHA implementation, global hash
injectivity, ID inequality or actual file correspondence is proved here.
Only constraint-plan.json and witness-plan.json have planned emitted paths;
the other four requests are Unsupported. Filesystem/serialization failures are
separate, and already-written artifacts need not roll back after a later error.

The actual backend identifies itself as `sherlock-host-export/1`, with
`host-u128/1`, and declares field layout unsupported. Constraint JSON declares
no witness computation; neither format is a Midnight/ZKIR exporter. Attaching
the tooling disposition leaves the supplied application object unchanged; the
actual CLI binding still needs proof. No cross-profile fuel equality, circuit,
proof, stock-verifier or ledger result follows.

There are no new `sorry` or custom axioms. The sibling Axioms module lists all
nineteen dependency queries. **This new module has not been compiled.** Its Lake
glob needs no shared build-file edit. Capped execution in this agent's sandbox
remains blocked by the user scope bus; no uncapped build was attempted. Rust
tests, artifact/CLI integration and the U031 holdout were still pending when the
source was inspected; no passing runtime evidence is claimed.
