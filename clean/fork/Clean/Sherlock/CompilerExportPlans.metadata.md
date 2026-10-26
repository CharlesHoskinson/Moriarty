# Real host export-plan contracts (U031)

This `Init`-only module follows the selected-record Rust API in
`sherlock/export_plans.rs` at `32fcbce3e`. It is separate from the historical
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

The generic Eq operator is still labeled `UInt128Eq`, despite the reference and
draft checker admitting arbitrary/unequal operand sorts. That source limitation
was reported and is not silently repaired by the model. The checked-failure
placeholder from `97412cb2c` is retained as `legacyResultBinding`; its theorem
states the historical missing value/source. At `32fcbce3e`, failure is instead
typed `CheckedU128Result`, with a bound source, displayed value and error code.
The current model states retention of all those fields. The supplied error
printer still requires correspondence to actual KDisplay; a typed-only result
remains explicitly unbound when no primitive result is present. These data laws
prove neither the arithmetic operation nor complete failure/witness semantics.

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
current dependency queries. Sherlock-fable's capped `lean-build-12.log` checked
the preceding nineteen-statement module and its queries: nine empty axiom sets,
ten containing only `propext`, then a successful complete 19-job build and exit 0.
That receipt is pinned historically. **The checked-error follow-up has not yet
been compiled:** its current source/query pins contain twenty-one statements.
The Lake glob needs no shared build-file edit. This agent read the build log and
source pins; its own capped execution remains blocked by the user scope bus, and
it attempted no uncapped build. Rust tests, artifact/CLI integration and the
U031 holdout have no independently inspected passing receipt here.
