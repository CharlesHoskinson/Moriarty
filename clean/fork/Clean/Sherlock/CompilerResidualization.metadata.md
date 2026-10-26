# Compiler and residualization contracts (U023/U024/U031/U033)

This module states mathematical interfaces for the Sherlock finish push. It uses
`Init` only. Its source base is `0699194f9`, inspected on both implementation
branches. Source hashes, theorem names and verification disposition are in the
matching JSON. These are newly authored contract models, not extracted Rust.

The five family-to-owner mappings and fixed inventory/table separation follow
`compile_artifact/native_inventory.rs`. That source explicitly reports C6
recursion unwired, native StepRecords absent and no native verification. The
inventory definitions retain those dispositions; different program labels and
exercise counts cannot change their supplied common table.

`Preserves` compares a supplied complete observation function on every input.
`Materialization` separates decoded semantic state from host scheduling counts,
as the four witness schedules in `draft_witness.rs` do. Its theorem requires an
actual decoded-equality premise. It does not establish that premise for Rust.
Complete observations must retain values, outcomes, order, boundaries, duties,
frame/resource checks and semantic work; arbitrary instantiation with just a
final value does not discharge U024.

`SeparatePlans` makes the constraint and witness artifacts distinct parameters.
Capabilities remain separate per feature. The concrete counterexample proves
that constraint export support alone does not imply witness-program support.
It implements no exporter and assigns no capabilities to the real backend.

At `cfbab29c5`, U031/U033 add Python report-only models, with no execution or
acceptance. `toolingSupports` tracks primitive capabilities of their three
listed backends, while aggregate whole-plan export stays unsupported because
the fixed operation manifest includes a native closure, lookup and interaction.
Those capability flags are not real Midnight export evidence. `ModelRule` and
`modelTargets` mirror the synthetic five-rule graph, including both dynamic
conditional successors and the recursive back edge. The arithmetic-family
whitelist originally dropped the call successor of fetch. The negative witness
is retained explicitly as `legacy_model_arith_prunes_call`. At `a82446698`,
all targets remain and the arithmetic family routes the call through a named
generic fallback. `model_arith_keeps_call_via_generic` states that repair.
The same revision adds typed witness input/output annotations; Lean mirrors
the exact generic `in`/`out` domain labels, which do not supply actual step
operand binding. This model is not a specializer for the actual Sherlock
reference definition. Its `arith`/`recursive` family labels
are separate from the five native inventory families, so specialization keys
now carry family identity strings instead of the native family enum.

The three residual masks have immutable specialization keys. Profile/layout
tokens are context identities even for the empty code-binding mask; runtime
state is never read by key construction. Worklist and size refusal execute the
generic operation on the same runtime state, without creating an application
exhaustion. The preservation theorem is conditional on the proposed residual
function matching the generic function at every complete observation. Actual
residual IR, finite specializer, runtime recursion, intermediate stop maps,
canonical identity encoding and Rust correspondence remain obligations. The
module does not claim any of these missing implementations exist.

U024 subsequently landed on factory-u-c4 at `69aeb882d`, with a candidate-only
tree interner and complete `sherlock-k-a/1` observation comparison. The new
`InternTree`/`InternNodeKey` definitions preserve full UInt128 leaf values and
ordered canonical child IDs. Equal stable keys preserve decoded content. Given
the interner's leaf/branch equations, structural induction proves tree-content
preservation. The actual HashMap invariant, live Arc identity stability and
iterative work-stack equations still require Rust correspondence evidence.
`InternObservation` names the exact compared initial state, ordered records
(rule line, charge, complete before/after snapshots) and final state. Its
equality theorem mirrors acceptance, not first-divergence diagnostic priority.
Effect/duty preservation is vacuous for this A-profile candidate; no claim
transfers to financial or helper profiles. Physical sharing counts in Rust
tests are not measured bytes, prover memory or performance evidence here.
The preservation record added at `87d1a2e2c` retains those six observation
planes, distinct evidence/gap tables, no acceptance and candidate-only adoption.

No new `sorry` or custom axiom is declared. The sibling `Axioms.lean` lists all
theorem dependency queries. Sherlock-fable independently ran the capped Lake
package build with Lean 4.33.1. The recorded `lean-build-4.log` shows the updated
U024/U031/U033 module and all twenty dependency queries succeeding, then a
successful 17-job package build and exit 0. Fifteen queries have empty axiom
sets; five contain only `propext`. The JSON records those exact current sets and
pins the successful observed log by digest. The agent read the complete log
and checked current source pins; it did not run the build itself.
Its sandbox still cannot connect `memcap` to the user scope bus, and it attempted
no uncapped build. No native, stock-verifier or ledger evidence is claimed.

The evaluator subsequently required actual `mori simulate` integration for
U031/U033 and U024 replay over D23/D35/tree45 with unchanged complete observations
and allocation counters. These models do not supply that integration; its Rust
landing and corresponding source-level contracts remain to be reconciled.
