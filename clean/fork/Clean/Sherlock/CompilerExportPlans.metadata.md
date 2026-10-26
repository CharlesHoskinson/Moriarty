# Real host export-plan contracts (U031)

This `Init`-only module follows the selected-record Rust API in
`sherlock/export_plans.rs` at `3bb404f15`, byte-identical in the current S9
union `93f61a41c`. The current JSON also pins that union's CLI/module wiring.
It is separate from the historical
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

The current operand-aware classifier labels equality `UInt128Eq` only when both
actual operands are UInt values. All other pairs, including Bool/Bool and
UInt/Bool, receive `ValueEq`. The shape-only classifier remains available;
`classifyWithOperands` refines its equality label using supplied actual operand
sorts. These projection premises still require Rust correspondence. Pending
frames below the top do not affect either classifier. The checked-failure
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
current dependency queries. Sherlock-fable's capped `lean-build-14.log` checked
the preceding twenty-five statements; the whole build failed on the sibling
residual module's reserved-token binders. After that correction, `lean-build-15.log`
replayed the same export sources/queries and completed the full 21-job package
with exit 0. Eleven queries have empty axiom sets, fourteen contain only `propext`.
The JSON pins the complete successful log and those checked sources;
earlier receipts remain historical. The Lake glob needs no shared build-file
edit. This agent read the complete log and source pins. Two own capped attempts
could not start because `heavy.lock` was occupied (exit 3); its saved sandbox
restriction also prevents `memcap` from using the user scope bus. No uncapped
build or delegation of a denied operation was attempted. Rust tests, artifact/CLI
integration and the U031 holdout for this newer exporter have no independently
inspected passing receipt here.

At `3fabcca1c`, S5 wires the CLI to the core exporter and removes its duplicate
descriptor API. Its core copy still predates the rule/top-frame operand binding,
checked-error and `ValueEq` repairs. The committed primary-scenario PASS trace
and run record for that older copy are pinned separately in the JSON; they do
not validate this module's newer `3bb404f15` contracts. S9's `93f61a41c` retains
the fixed exporter. This branch difference was reported to sherlock-fable and
Quint-3. CLI correctness, concrete source/binary binding and the newer U031
holdout remain separate evidence obligations.

Evaluator `7f0942c3d` adds two non-arithmetic negative controls. The module now
names LeftF explicitly and states that rule 149 through LeftF(Mul) classifies
as ArenaRead, while rule 195 through ScopeF above a pending RightF(Mul)
classifies as ScopeRestore. The holdout forbids CheckedU128 operations there;
these exact primary labels follow the current source rather than a stronger
requirement added to the holdout. **The two new statements are pending kernel
checking.** Current source/query pins contain twenty-seven statements; the
previous twenty-five-statement build does not certify the changed Lean bytes.
