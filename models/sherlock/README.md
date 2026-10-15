# Sherlock

Sherlock is the proposed universal interpreter for Moriarty. The user named it
on 2026-10-15. This directory contains design models, not a production compiler
backend or a released language profile.

The proposed reference machine evaluates different supported programs using
unchanged transition rules. It makes calls, saved environments, return frames,
recursive immutable values and execution budgets explicit. First-order named
self-recursion and mutual recursion are the initial fragment; higher-order
closures and general nominal datatypes remain extensions.

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

K's structural helper rewrites are outside its semantic fuel schedule. A bounded
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
No Sherlock circuit, correspondence theorem or Midnight exporter is supplied
by this draft.

All U0–U7, M1–M17 and MC/SP/G acceptance exits remain unchanged. Model frontend
checks, concrete executions, sampled invariants, correspondence proofs, native
proof verification and target ledger observations are separate evidence.
