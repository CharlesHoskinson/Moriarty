# Sherlock

Sherlock is the proposed universal interpreter for Moriarty. The user named it
on 2026-10-15. The [Candidate B design](../../openspec/changes/sherlock-universal-interpreter-design/design.md)
integrates the research into a final-design candidate awaiting fresh independent
review. This directory's existing executable model sources and recorded
observations remain Candidate A scope; they do not implement or validate B.
No production compiler backend or released Sherlock profile is supplied here.

The proposed reference machine evaluates different supported programs using
unchanged transition rules. It makes calls, saved environments, return frames,
recursive immutable values and execution budgets explicit. First-order named
self-recursion and mutual recursion are the initial fragment; higher-order
closures remain an explicit extension. Candidate B specifies declaration-driven
nominal datatypes and ordered match plans as required implementation work;
the current K/Quint models retain their narrower concrete data forms.

| Artifact | Actual scope | Remaining work |
|---|---|---|
| [K definition](k/README.md) | Generic pure expression machine with named function tables, checked UInt128, lists, trees and explicit frames | Rule execution, metered helper refinement, financial operations, suspension and compiler/circuit correspondence |
| [Quint companion](quint/README.md) | Finite factorial/list/tree algorithms with explicit frames, work accounting and a local waiting/resume scenario | Arbitrary program loading, lexical environments, K projection and financial correspondence |

Each model's README and source manifest define its exact membership and limits.
The K fixtures have independently written expected values. Quint has a separate
test module and independently written arithmetic expectations. Source files and
expectations alone establish no observed execution result.

[Observed checks](observations.json) record K frontend success, eight passing
Quint tests and 32 sampled seed-1 traces with no invariant violation. K produced
syntax Kore and compiled text; no runtime `definition.kore` or rule execution
was observed. All seven Quint witnesses occurred. These results concern the
exact recorded model sources and do not establish cross-model refinement.

K's structural helper rewrites are outside its semantic fuel schedule. Candidate B
names the unchanged schedule `sherlock-k-a/1`, the companion's distinct schedule
`sherlock-quint-a/1`, and a proposed new `sherlock-helper-microstep/1` with explicit
lookup/bind/validation/equality controls. Same-fuel correspondence between those
profiles is not established. K's stack-limit breach remains Rejected(error4),
and terminal publication remains uncharged in its A schedule. A bounded
machine step does not yet imply bounded helper work or circuit cost. The Quint
companion uses scalars 0–15, eight fixed immutable node slots, at most eight
frames and six concrete initial scenarios. It is an algorithm abstraction,
not an implementation of the generic K interpreter.

The financial extension must preserve existing Core/2 successful-prefix await
behavior, ordered effects and residual duties. The pure K fragment does not
model it. Quint's checkpoint is a local machine suspension, not a financial
commit. Exact future universal Commit/Yield/Await instructions remain open.

Sherlock's reference semantics can be realized by per-program circuits, one
universal circuit, or a fixed relation family. Those are separate backend
alternatives. No cost winner is established by this package. Ledger-9 recursion
does not require a VerifyProof instruction; adapting Sherlock to the existing
custom-decider route remains separate implementation and qualification work.

Clean is a required design stage between the reference models and native
integration. Define Sherlock's semantic state and representation relation in
Lean, construct a bounded transition circuit in Clean, and state the
correspondence obligations before connecting the circuit to Midnight recursion.
A Lean inductive semantic datatype and a bounded circuit encoding serve
different purposes. Fuel bounds attempted runs without requiring every user
function to pass Lean's structural termination checker. Existing Clean APIs,
witness-generation support and backend gaps require a pinned reuse inventory.
The existing private Fenrir integration has merged a generic pure Lean reference
foundation with eight normal and exact-remaining-zero publication checks and
scoped independent source approval. That child implementation does not establish
a Clean step circuit, K/Lean refinement, soundness/completeness, Midnight exporter,
native proof or financial acceptance. Actual Clean implementation stays in that
existing private fork; private source, paths and pins are excluded here.

The [implementation plan](../../docs/SHERLOCK-IMPLEMENTATION-PLAN.md) sequences
the rule/cost inventory, immutable indexed `ProgramImage`, deterministic complete
`StepRecord` consumer, reference reconciliation, bounded `MachineImage`, generic
ADT/frame/arithmetic components, separate `ConstraintPlan` and `WitnessPlan`,
export capability inventory, resource reporting, ordered `ReferenceSlice`
batching and bounded static residualization. The [concept matrix](../../docs/research/sherlock/CONCEPT-ADOPTION.md)
maps all ten primary-source studies to independent rebuilds, components,
comparison evidence and priorities. No donor code/dependency or foreign proof
backend is selected; no representation or realization has a measured cost win.

The shared [interface contract](../../openspec/changes/sherlock-universal-interpreter-design/interfaces.md)
separates bounded circuit `StepInput`/`StepOutput` from host-decoded reference
records. New code/data/frame/environment/serialization/helper limits and admission
rules require explicit supported/rejected/exhausted/unsupported dispositions.
Closed-body/unique-function admission, full generic model/compiler coverage and
cross-layer correspondence remain implementation tasks. Independent normal
oracles precede execution; all security analysis, requirements, hostile tests,
theorems and judgments remain Opus 5.5-owned.

All U0–U7, M1–M17 and MC/SP/G acceptance exits remain unchanged. Model frontend
checks, concrete executions, sampled invariants, correspondence proofs, native
proof verification and target ledger observations are separate evidence.
The first useful passing slice supplies scoped evidence while every retained
financial, composition, conformance and release requirement stays in force.
