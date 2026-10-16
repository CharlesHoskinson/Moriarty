# Sherlock correctness loop

Status: required development workflow from the October 15 user instruction.
The ordinary complete-observation consumers now accept host, image, Quint and
pure Lean diagnostic artifacts. Actual execution, projection correspondence and
the remaining cross-layer circuit/native harness require separate evidence.
This operationalizes UIREQ-021–026 and the existing implementation plan;
it does not adopt Candidate B, freeze its schema or close a roadmap gate.

## Iterate by transition slice

Develop K semantics, the finite Quint abstraction, Lean/Clean and handwritten
circuits together for each admitted transition slice. The implementation plan's
phases describe dependencies; they are not permission to postpone model and
circuit comparison until the end. Start with the existing scalar/frame/call
slice, then extend the same loop to helpers, data/matching and financial
continuations as their profiles and interfaces are reviewed.

1. Name the changed source rules, admitted inputs, profile, bounds and existing
   requirement/component rows. Preserve the old profile when semantics change.
2. Have a non-author freeze expected complete states and work prefixes before
   running the implementation. Reuse the committed independent normal oracles
   where their domain applies; add independently derived cases for new coverage.
3. Update the affected K definition, Quint instance/projection and Lean semantic
   definitions. Record any layer that does not represent the slice as a gap.
   Do not silently reduce the intended language to the existing finite model.
4. Implement the bounded Clean component and the handwritten circuit component
   for the same declared step interface. Bind their actual witness and constraint
   implementations separately. Record shared gadgets or generated code: agreement
   between shared implementations supplies less independent evidence.
5. Run only separately inspected and admitted checks. Compare complete projected
   traces against the frozen expectations and between the available realizations.
   Report the first divergent field, rule and prefix, including each raw state.
6. Repair the responsible implementation, model or projection from its contract;
   retain the failing receipt. A changed expectation needs an independently
   reviewed semantic correction, not a regenerated implementation output.
   Add the resolved case to the regression corpus and recheck affected layers.
7. Before accepting the slice or an optimization, refresh applicable reviews and
   the matrix below for the exact candidate. A missing required layer leaves
   cross-layer acceptance open even when independent source work can continue.

## Distinct jobs for each layer

| Layer | Required comparison artifact | What a result establishes |
|---|---|---|
| Independent corpus | Input program/data, expected complete prefixes, provenance and profile | A fixed expectation independent of the implementation under test |
| K | Actual rule trace and complete machine cells, or explicit execution gap | Concrete semantic execution only when the rules actually ran; frontend emission is syntax evidence |
| Quint | Concrete instance, field/action projection, tests, sampled traces and declared bounds/seed | Finite abstraction observations; sampling is not exhaustive checking or refinement |
| Lean reference | Recursive semantic values and fuel-run trace on the admitted profile | Reference observations; separately scoped theorem evidence remains separate |
| Clean | Bounded pre/post images, materialized states, component locators, separate witness/constraint dispositions | Component evidence only for checks actually performed; a witness computation alone does not validate constraints |
| Handwritten circuit | Same bounded interface and projections, actual component/constraint locators and independent check receipts | Comparison with both reference expectations and Clean on the declared common domain |
| Native/ledger | Exact exporter, relation/statement and qualified target receipts | Separate native/target evidence; it cannot be inferred from the other rows |

Use existing private Fenrir Clean integration. Private source, pins, theorem
artifacts and paths remain there; public records identify scope and the shareable
binding only. Public upstream Clean API membership is not the private circuit
membership. No new backend, public fork or developer approval dependency is added.

## Compare complete observations and declare the clock

Compare code/function identity and ordering, control phase, typed values,
environments, saved caller environments, ordered frames, depth, selected arm,
raw outcomes, consumed/remaining work and terminal publication. Include bounded
helper cursor/resume state, logical allocation and physical occupancy where
represented. Financial slices additionally use the existing complete ordered
effects, fees, gross debit, liabilities and residual-duty projections.
Diagnostic formatting is not a canonical state codec.

Do not compare raw step numbers across `sherlock-k-a/1`,
`sherlock-quint-a/1` and proposed `sherlock-helper-microstep/1`.
Declare a rule/action map and observable boundaries, account for all intervening
steps, and retain each profile's raw charge/outcome. The historical scalar15
`universalMachine.qnt` uses `sherlock-quint-a/1` with charged terminal return.
The new generic [K-A Quint counterpart](../models/sherlock/quint/K-PROFILE-README.md)
uses `sherlock-k-a/1`: exact charged K rules and free return/error publication
before exhaustion. A's unmetered helpers are not B microsteps.
Preserve existing frame-cap spellings and precedence under their
owned policies. Missing mappings remain gaps, not skipped passing comparisons.
For batching, sharing, residualization and caches, compare every declared internal
prefix and earliest stop against the uncached baseline, not just the last value.

Begin with the existing K fixtures, committed normal program/prefix corpus and
generic finite K-A Quint counterpart, restricted to their explicit common
domain. It interprets immutable `ExprId -> Expr` syntax and source-ordered
function tables, retains nine frame forms and lexical restoration, and evaluates
named-call arguments in source order. Its exact UInt128 bounds do not
establish exhaustive numeric or target-field coverage. The historical scalar15
algorithm instance and its eight normal tests/32 sampled seed-1 traces retain
their separate scope. Preserve mutual recursion, caller restoration, argument
order, pending tree traversal and occurrence accounting as coverage rows.
General ADTs, B helpers and financial continuation need their own model extensions and cases.
Existing security-owner resource tests and B001–B009/A4/B002 cases retain their
original scope; this document neither authors substitute hostile tests nor claims
that normal trace agreement satisfies them.

## One evidence matrix per changed slice

Record the following fields in the existing change/review evidence, rather than
building a new scheduler or treating this document as an implemented file format:

- Requirement/rule/component IDs, case IDs, source file hashes, profile, bounds,
  projection version and the complete declared comparison domain.
- Per-layer status: not implemented, not run (with reason), observed pass, observed
  mismatch, or unsupported. Identify omitted coverage and the next concrete fix.
- Exact tool versions/commands, source and import membership, seed/run/step limits,
  exit/terminal status, raw output location and digest for each actual execution.
- First divergence and expected/actual complete projections; keep failed runs and
  fixes linked. Bind private evidence privately without publishing its paths.
- Actual review identities, scope and dissent; separately identify any theorem,
  constraint, exporter, native or target evidence and its remaining assumptions.

A source, profile, bounds, projection, compiler or gadget change invalidates the
affected comparisons. Re-run that affected closure before reusing its result.
An unchanged historical receipt stays valid only for its recorded candidate.
Report unavailable stages explicitly rather than counting them as successful.

## Required stagenet end-to-end evidence

The [October 15 stagenet instruction](../raw/assignments/moriarty-stagenet-acceptance-2026-10-15.md)
requires an actual Midnight stagenet run after the relevant semantic/component checks.
Compile supported source through the real compiler/native path, produce and
independently verify its proof, deploy and call on Midnight stagenet, then read back the
complete resulting state and ordered effects against the independent expectations.
Use two distinct supported programs as required by the retained U2 contract;
full financial/continuation and release coverage remains in scope.

Pin the actual Midnight stagenet network identity, endpoints, toolchain/ZKIR/ledger/verifier
versions, relation and key/artifact identities, submission references and readback
position. Check compatibility rather than treating a target name as a version.
Retain fee, debit, liability and residual-duty observations where applicable,
plus the existing security-owner negative and failure requirements. Keep this
row not run until its actual receipts exist. Preview and Fenrir devnet evidence
retain their own scopes. Select execution through the existing admission and
resource limits; this document supplies no new configuration or execution grant.

## Current gaps and execution boundary

The typed A host component, independent normal test consumer and generic finite
K-A Quint projection exist. The [recorded model evidence](audits/2026-10-15/sherlock-k-profile-quint/README.md)
binds typecheck exit 0 and 98 passing deterministic normal tests: six complete
traces and 92 published fuel prefixes, using Quint 0.32.0's TypeScript engine,
seed 1 and one sample per test. These are captured local results, not a fresh
run for this document. The [exact-source reviews](audits/2026-10-15/sherlock-k-profile-quint/reviews/README.md)
record completed Muse and Opus reviews and retain their limitations.
A [separate 126-test addition](audits/2026-10-15/sherlock-extra-normal-quint/README.md)
checks N07/N08 complete traces and 124 prefixes against the unchanged model.
The old 98 checks were not rerun as part of that addition. The [flat image and
ID interpreter](audits/2026-10-15/sherlock-program-image-normal/README.md) have
captured 46-test host evidence on all eight normal programs and 216 prefixes.
These modules are private test-only preparation, with no bounded-image or
circuit acceptance claim.
The [normal grouping baseline](audits/2026-10-15/sherlock-reference-slices/README.md)
retains every internal record for groups 1/2/4/8. Its separate 59-test host
capture covers all 216 prefixes at each size against the unchanged literals;
it is not an optimized circuit or a completed B slice/identity implementation.
The [unadopted flat snapshot proposal](audits/2026-10-15/sherlock-machine-image-normal/README.md)
adds caller-limited host row tables and complete structural materialization. Its
captured 72-test closure includes the same eight independent normal programs
and 216 prefixes; it is private test-only preparation, not bounded native
representation, canonical state admission or a B-T04 completion.
The [finite call-helper proposal](audits/2026-10-15/sherlock-call-helper-normal/README.md)
adds three literal host units and a retained real-context adapter. Its integrated
79-test capture checks 12 call-completion boundaries against unchanged frozen
snapshots, plus a separate complete List/Tree case. H diagnostics stay separate
from A fuel/work; full B budgets, source-occurrence propagation and schema remain
open. This is normal projection evidence toward B-T03, not phase acceptance.
K runtime execution, full K/Quint refinement, canonical state serialization,
B schema/helper work, field/constraint correspondence and full grammar coverage
remain open. The pure Lean foundation has its separately recorded narrow scope.
The typed A [first-difference comparator](../crates/mori-core/src/sherlock/compare.rs)
now supplies the shared observation comparison primitive, with its own
[normal RED/GREEN evidence](audits/2026-10-15/sherlock-trace-comparison/README.md).
A Sherlock bounded Clean step circuit, paired handwritten-step comparison adapter
and complete cross-layer runner are not yet delivered; those implementation
tasks remain unchecked.

Use the current [AGENTS.md routing](../AGENTS.md#security-ownership-restored--october-15)
for authors and independent reviews, with Opus 5.5 high restored for new security
work; historical Argon results keep their actual identities. Security requirements,
hostile cases, theorem statements and judgments stay with the designated security owner.
This workflow grants no execution authority: K executors, model checks, Clean/
SDK relation checks and native actions retain their existing command-specific
limits. Frontend checks, source inspection and host tests must be reported at
their actual evidence class. Neither this loop nor a passing finite corpus is a
soundness, completeness, correspondence or release proof.

The [source-origin normal comparison](audits/2026-10-15/sherlock-source-origin-normal/README.md)
adds actual image-machine occurrence/frame-owner metadata and real call-helper
capture. Three new literal source-path/cursor cases pass alongside the unchanged
eight programs/216 prefixes (82 checks total). This does not adopt B, bind a
program digest or establish circuit/native correspondence.

The [typed host checkpoint comparison](audits/2026-10-15/sherlock-host-checkpoint-normal/README.md)
checks actual image/helper cut/resume against selected complete frozen boundaries,
including pending returns/cursors and retained origins (86 host checks total).
Internal event equality is supplemental; no canonical serialization, B/native or
financial continuation result is inferred.

The [private draft source capture](audits/2026-10-15/sherlock-draft-source/README.md)
adds the lexer (cached rlib; source correspondence unproven)/parser/checker-to-image path. Eleven new host tests pass,
including factorial(4) = 24 and tree folding = 7 through both actual interpreters.
The 86 older tests compiled but were filtered out and were not rerun. These
literal AST/value comparisons supply no new complete-prefix, K/Quint/Lean,
constraint or target correspondence. The draft envelope remains unregistered
and private test-only preparation. All missing layers above retain their gaps.

## Scoped offline input/readback observation

The [offline helper](../models/sherlock/STAGENET-OFFLINE-README.md) inventories existing Core/1 inputs and compares all supplied
JSON fields, retaining ordered arrays and missing/present distinctions. Its
captured R2 result records 16 passing normal assertions for the bound source.
This is preparatory tooling for complete readback reporting, with no native
execution or independent semantic oracle. It does not implement the missing
Sherlock/Clean/native adapter or public Moriarty commands and closes no
UIREQ/B/U/M/MC/SP/G exit or Midnight stagenet qualification.
