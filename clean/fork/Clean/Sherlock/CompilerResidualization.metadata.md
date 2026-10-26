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

At `577f0470b`, native inventory reporting becomes receipt-aware. A rule's
declared membership in `RECEIPTED` selects `compiled-and-run`; absent membership
leaves it source-only. Dispatch selection appears only with a dispatch receipt.
The current list is empty. `FragmentEvidence` models those declarations and
keeps nonproving `ProofPreimage::check` receipts distinct from keys, proofs or
verification. It authenticates no receipt or run; that remains a concrete
source/evidence obligation, separate from the fixed report flags above.

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

U033's `abe5ee146` contract additionally requires each residual execution record
to name its producing block, satisfy rule membership (or generic dispatch),
follow an allowed block edge and preserve state continuity. `ResidualBlock`,
`ResidualRecord` and `ResidualExecution` state these obligations with a supplied
actual executor. `RecordProduced` requires an execution equation, not merely
an annotation on a reference stream. The witness theorem exposes that equation;
a negative control proves that an executor returning no records cannot support
a nonempty certified stream. These requirements complement semantic preservation
and do not prove it. Actual immutable lookup, Rust block execution, rule/charge
maps, block-ID encoding and correspondence remain unproved. The Python sample
models do not instantiate this real-IR execution contract.

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
At `dbe33d95d`, its E1 entry is bound to run
`20261026T001248Z-U024-evidence`. The agent read the pinned trace, checked its
digest and found all six named Rust tests passing. That observed C4 execution
covers the stated A-profile fixture, including negative controls; it is not an
agent-run test or evidence for the subsequent real-simulate replay requirement.

Actual draft-route replay landed at `06ec0f855`; the evaluator-schema record
and R1 receipt follow at `a069ccbff`. `ReplayLogical` names events, work, outcome,
remaining fuel, maximum depth and peak structural child occurrences. The replay
contract refuses an unknown candidate or different complete observations;
success preserves every supplied deterministic logical metric. `applied` means
the interner table is nonempty, not that a physical improvement was proved.
Reference/candidate physical child counts are independent measurements over the
initial state, each resumed `before` state and the final state. They are not
byte size, total allocations or peak resident memory. `attachOptimization`
preserves the separately supplied B owner result; linking this wrapper and the
metric/checker definitions to Rust remains a correspondence obligation. No
same-fuel equivalence between A and B profiles is inferred.

The agent inspected R1's pinned six-test passing trace and C4's passing
`20261026T004810Z-U024` holdout trace at source head `a069ccbff`. Its factory
record reports 1.746 seconds and 72,126,464 bytes maximum RSS for the entire
holdout command; these are neither optimization speedup nor memory reduction
measurements. The agent executed neither command and establishes no binary/source
reproducibility result. The old Python optimization-record checker was removed;
the evaluator-owned replay is the current acceptance check, with the record
still declaring no native acceptance or adoption.

No new `sorry` or custom axiom is declared. The sibling `Axioms.lean` lists all
theorem dependency queries. Sherlock-fable independently ran the capped Lake
package build with Lean 4.33.1. The recorded `lean-build-9.log` shows the actual
U024 replay and U023 receipt follow-ups with all thirty dependency queries
succeeding, then a successful 17-job package build and exit 0. Twenty queries
have empty axiom sets; ten contain only `propext`. The JSON records those sets and
pins the successful observed log by digest. The agent read the complete log
and checked the current owned source pins; it did not run the build itself.
Its sandbox still cannot connect `memcap` to the user scope bus, and it attempted
no uncapped build. No native, stock-verifier or ledger evidence is claimed.

Actual U031/U033 Rust implementations now exist on the integration branches.
Their follow-up contracts live in `CompilerExportPlans` (selected-record host
exporter in S9 union `93f61a41c`, fixed at `3bb404f15`) and `CompilerResidualIR`
(seven-group K-A residualizer at `3fabcca1c`, unchanged from `8ffed57fc`/`abc64975d`).
The earlier Python examples remain historical.
The new modules supply neither Rust refinement nor native acceptance. The U024
replay retains its separate source and observed holdout evidence described above.

**Current status: LOCKED_VERIFIED.** The coordinator's README-conformant
DoctorWatson run `20261026T035951Z-sherlock-lean-build` passed the full 21-job
package with preflight, `memcap 48G` and a reported real outer `heavy.lock`.
Ledger commit `8853c8f8c` records a private inner `DW_LOCK` proxy; the JSON keeps
that distinct from the real outer lock confirmed by the coordinator. All thirty
queries and current/run-HEAD source bytes match: twenty empty sets and ten
`propext`-only sets. The agent inspected the complete trace, wrapper and ledger
line; it did not run the build. Whole incremental build measurements are
6.229 seconds and 890,322,944 bytes peak RSS, not optimization performance.
Builds 1–18 remain historical capped/unlocked kernel observations.

The S10 R1 receipt at `42df98819` now names a committed `trace_copy`. The agent
read the complete copy, matched its digest to `trace_sha256` and found every
one of the six declared preservation/negative-control tests passing. These are
the existing `06ec0f855` fixture results carried portably, not new tests run by
this agent. The JSON pins the receipt/copy bytes and named cases. This supplies
neither all-program preservation nor a B/C6/financial, native or performance result.

