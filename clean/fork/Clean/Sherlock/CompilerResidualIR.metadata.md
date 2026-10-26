# Actual HostOnly residual IR contracts (U033)

This module follows `compile_artifact/draft_plans.rs` at `8ffed57fc`, as retained
by the U033 passing-check commit `abc64975d`. It is separate from the historical
Python examples and abstract preservation/provenance obligations in
CompilerResidualization. The JSON pins the actual committed Rust bytes and the
Lean source/query bytes. No private Clean source or additional library is used.

The actual partition has seven groups and thirty-nine K-A rules, each covered
once. It is fixed independently of the program, fuel and recursion depth.
Universal/family/per-program modes change static bindings, not this partition.
A matching family folds layout/family; other groups fold only layout. Per-program
block annotations fold code/layout/rule-occurrence tokens, while the top-level
binding declaration additionally names declarations and operand slices. The
actual program-image token hashes raw source bytes. This contract establishes
neither a canonical ProgramImage identity nor SHA injectivity.

Zero size limit is a configuration refusal before execution. A positive limit
below seven keeps the first `limit - 1` groups and sends every other rule to
one generic block. At seven or above, all groups remain and there is no generic
block. The emitted count is `min(limit, 7)` and is at most the supplied limit.
Source RULES order is ascending; filtering uncovered rules models Rust's sorted
generic vector. Exact prefix/map/sort correspondence still requires proof.
The source supplies a dense conservative successor graph, with no successors
for publish. The call block has a self edge, `unrolled=false` and a dynamic
frame check. This is no minimum-specialization or size/cost improvement claim.

The dispatcher models actual pre-step control, top frame, environment membership
for variables, empty-stack and zero-fuel decisions. Publication retains priority.
Ten named dynamic inputs include control/data, environment/frames, fuel/work,
helper cursor, capacity checks and outcome. Converting a complete Rust Snapshot
to these tags is an explicit obligation. The source chooses generic dispatch
when no rule is assigned to the chosen group. It then calls the reference
machine's step and refuses a produced rule assigned to another block. The
single-boundary contract retains the supplied produced record exactly, including
whole before/after states, rule and charge, and does not step a terminal state.
It does not prove the supplied step is Rust's implementation.

Full-loop termination, saturated fuel+1 event caps, no-progress failure, state
continuity, prefix work, width-one serialization and source-stop correspondence
remain open. In particular, the source's `occ:<K-rule-line>` tokens identify
rules, not distinct source AST locations. The abstract RecordProduced and
ResidualExecution obligations must still be instantiated with the actual Rust
lookup and executor. Matching reference streams and adding IDs cannot substitute
for this production relation. No A/B fuel equivalence or B/C6/financial result
follows, and no circuit/native/stock-verifier/ledger result is claimed.

No new `sorry` or custom axiom is declared. Twenty-three statements have exact
sibling `#print axioms` queries. Sherlock-fable's capped `lean-build-15.log`
checked the current module and queries, then completed the full 21-job package
with exit 0. Eight queries have empty axiom sets, twelve use only `propext`,
three use `propext` and `Quot.sound`. The JSON pins the complete observed log,
exact sets and checked current/committed sources. The agent read the full log;
it did not execute the build. Build 14's two reserved-token hypothesis names
were corrected before that successful retry, without changing statements.

Static source comparison passed for all seven groups/thirty-nine rules/ten
dynamic inputs and thirty-two positive size cases; declaration/query coverage
was also checked. That audit is not Rust execution. Two own capped attempts
exited 3 because `heavy.lock` was occupied, so no local compiler started. The
saved sandbox restriction also prevents `memcap` from reaching the user scope
bus. No uncapped build or delegation of a denied operation was attempted.
Owner/evaluator U033 PASS messages are advisory here; this agent has not
independently read and pinned the associated full holdout trace or run it.
