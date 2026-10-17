# Sherlock

Sherlock is the proposed universal interpreter for Moriarty. The user named it
on 2026-10-15. The [Candidate B design](../../openspec/changes/sherlock-universal-interpreter-design/design.md)
integrates the earlier research. The [current adopted revision03](../../docs/sherlock/design.md)
contains the subsequent design and review reconciliation. This directory's existing executable model sources and recorded
observations remain Candidate A scope; they do not implement or validate B.
The separate [B HOST payload source](b-core-host-source-20261016/README.md),
merged in [PR #120](https://github.com/CharlesHoskinson/Moriarty/pull/120), compiled
ten files with 96 Enabled row-effect payloads to metadata only, with six warnings.
At that PR120 source cut, nothing was linked, loaded, run or tested; selector,
driver, guards and admission were then unimplemented. Later scoped work passed
[four owned-observation tests](../../docs/results/sherlock-b-complete-observation-20261017/README.md)
and [nine complete-comparison tests](../../docs/results/sherlock-b-trace-comparison-20261017/README.md).
The latter do not execute owner steps. The separate
[44-test finite model result](../../docs/results/sherlock-quint-design-20261017/README.md)
is model evidence. These observations retain their exact source cuts; full B,
generic, financial, native and target acceptance remain open. The qualified R4
HOST Stage1 basis is distinct from the unadopted proposed v3a amendment.
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
| [K definition](k/README.md) | Generic pure expression machine with named function tables, checked UInt128, lists, trees and explicit frames | Engine coverage beyond one N01/fuel1 common-projection match, metered helper refinement, financial operations, suspension and compiler/circuit correspondence |
| [Quint companion](quint/README.md) | Finite factorial/list/tree algorithms with explicit frames, work accounting and a local waiting/resume scenario | Arbitrary program loading, lexical environments, K projection and financial correspondence |

Each model's README and source manifest define its exact membership and limits.
The K fixtures have independently written expected values. Quint has a separate
test module and independently written arithmetic expectations. Source files and
expectations alone establish no observed execution result.

The historical [observed checks](observations.json) record K frontend success,
eight passing Quint tests and 32 sampled seed-1 traces with no invariant violation;
all seven Quint witnesses occurred. Those source-specific captures remain unchanged.
A later local K compile exited 0 in 4.293 seconds and produced an unvalidated
`definition.kore`; [PR #119](https://github.com/CharlesHoskinson/Moriarty/pull/119) now publishes its receipt.
That record remains compile-only history. A later single N01/fuel1 raw engine run
completed 12 stages with 11 zero-exit children. Raw logs and states show startup,
a `uiU(7)` literal return leaving fuel 0, `uiDone(uiU(7))` publication and a following
zero-rule probe with byte-identical terminal Kore. PR #119 also publishes
the engine receipt and its [literal compile-to-engine file binding](../../docs/audits/2026-10-16/sherlock-k-onecase-source05/FINAL-SOURCE05-RESULT-PUBLICATION-RECORD.json),
which is not producer authenticity or validation of the generated definition. The
case's post-startup common projection matched an independently frozen two-record
expectation with no first difference. The other 215 fixed inputs remain unobserved;
no full 216-run K,
general K correctness or refinement result follows. Neither those historical
captures nor this compilation establishes cross-model refinement.

The [current correctness-loop observations](../../docs/SHERLOCK-CORRECTNESS-LOOP.md)
include [PR #109's 35 codec tests and separate 12-run/49-record typed round trip](../../docs/audits/2026-10-16/sherlock-layout-codec-delivery/AUTHOR-REPORT.md)
and [PR #110's full eight-program/216-run/5,355-record typed preservation](../../docs/audits/2026-10-16/sherlock-layout-codec-full-normal-delivery/AUTHOR-REPORT.md).
The independent Machine-to-corpus matches are separate from internal codec evidence.
[PR #111](https://github.com/CharlesHoskinson/Moriarty/pull/111) adds nine
arithmetic/fuel host tests and private Core test registrations; [PR #113's canonical
Core capture](../../docs/audits/2026-10-16/sherlock-core-cfg-test/README.md) compiles
86 inputs and passes all 189 library tests, including those nine and the eleven
pre-existing draft-source tests. This is direct-rustc cfg(test) evidence, without
Cargo/workspace, SDK, integration-test or doctest qualification; cached extern
library/proc-macro and cached lexer source authenticity remain unestablished. The counts are
separate captured scopes, not a combined total.
[PR #112](https://github.com/CharlesHoskinson/Moriarty/pull/112) adds
`--k-observations` to the existing QA consumer for supplied K projection documents;
a supplied-data match does not establish K engine execution or a semantic match.
[PR #118](https://github.com/CharlesHoskinson/Moriarty/pull/118) adds a
[post-startup Rust host comparison](../../docs/audits/2026-10-16/sherlock-full-original-fixtures/README.md)
of fixed typed-AST transcriptions of the eight original fixtures: 569 ordinary
prefixes (27,997 records) and eight separate fuel1000/stack64 runs (561 records)
match independently frozen literals. It is not a K-source parser, compiler or K
startup execution. [PR #117](https://github.com/CharlesHoskinson/Moriarty/pull/117)
records [two local Core/2 financial programs](../../docs/audits/2026-10-16/sherlock-financial-core2-delivery/README.md)
matching frozen expectations, including 79 direct original-JSON checks; it does not
implement Sherlock financial lowering or cover the full financial family.

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
