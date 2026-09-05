# Moriarty execution roadmap and Candidate A handoff

Recorded 2026-09-05 UTC. This is the durable roadmap requested after the overnight
session. It is not a release verdict or a replacement for the controlling XML.
Status vocabulary: implemented, recorded-tested, independently reviewed,
specified-only, and open are separate claims. Experimental implementation is S3;
the adopted B design is S2. No production compiler/SDK completion is implied.

## Canonical locations and recovery

- Main repository: `/home/charl/Moriarty`, branch `main`.
- Implementation worktree: `/home/charl/Moriarty/.worktrees/s01-audit-start`,
  branch `s02-model-comparison`; do not infer its branch from the directory name.
- Implementation handoff anchor: `d14cfea98a1c9213e5ef5f12c1a088f4e966083d`.
- Main wiki anchor before this handoff: `d247bca7c65a3eca1b27755bc3eff7dc2ba78511`.
- Canonical session database: `/home/charl/Moriarty/.foreman/session.db`.
- [Snapshot and preserved uncommitted handoffs](../evidence/session-snapshots/2026-09-05-candidate-a-handoff/README.md).
- [Candidate A completion XML](../deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml).
- [Research journal](../wiki/research-journal.md), especially CLM-0148–CLM-0159;
  [wiki log](../wiki/log.md); [Council contract](COUNCIL_REVIEWS.md).
- [Controlling XML v1.3](../deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml):
  S01–S15, D01–D22, G01–G24. Earlier twelve-package or eighteen-output plans do
  not replace this scope.

The snapshot inventory pins the prior source/spec/plan/evidence locators. Paths
marked `s02-model-comparison` are relative to the implementation worktree, not
necessarily present on main. Recover a tracked artifact with `git show
d14cfea98a1c9213e5ef5f12c1a088f4e966083d:PATH`. Do not overwrite newer files with
snapshot bytes. No Git remote was configured at handoff; no GitHub publication
or main-branch integration of the experimental implementation is claimed.

## What the overnight work accomplished

These are repository observations backed by the named commits and retained
receipts. Counts describe recorded runs, not fresh measurements of future code.
Source facts derive from SRC-0031 (XML), SRC-0033 (comparison boundary) and
SRC-0035 (adopted common design), as indexed in the research journal.

| Work product | Actual scope and status | Commit/evidence anchor |
| --- | --- | --- |
| S01 package | Earlier local theorem/specification package retained; not mechanized proof; Council backfill open | `96782de`; `openspec/changes/s01-intent-theorem-freeze/` |
| Common observation and policy model | Full Core observations, branch policy predicates and complete plan binding | `45883ba`, `320dc53` |
| Persistent signing | Before/after-resolution profiles, key-local freshness checks and exact registration | `928ec40`, correction `f2941d4` |
| Shared execution | Per-operation proposals, bound verification, atomic ledger/authority commit | `194af05`, `46fe589`; evidence `0d5926b` |
| Classified rejection | Original attempts/evidence retained; semantic, stale and consumption failures do not move money | `38cf13d`; evidence `4f3bb75` |
| Generic installment lifecycle | Both race orders, stale loser, second fill, separately signed recovery ten/five; still abstract candidate fixtures | `d1c475f`; evidence `f4c7bc5` |
| A finite agreement interpreter | Close/Pay/If/When, ordered inputs, complete transactions and rollback | `068b7cd`, `d3f5dd6`, `ea35cad` |
| A projection and real workloads | Actual swap deposits/settlement/refunds and installment fill/recovery traces | `d4a714c`, `3c648ec`, `c002a84` |
| A finite-record correspondence | Recorded 53 cases from 14 ITFs; 84 checker tests; nine retained semantic mutation controls | source `2f53817`; evidence `a9deaec`; aggregate `0266df2`; audit `d7384d2` |
| A authority adapter | Recomputed exact requests/results/effects; checked cancellation identity, not a Core result or authority grant | source `e84f737`; evidence `46946aa`; audit `7a7bf5d` |
| A authority boundary | Full-plan signing/verify/commit guards, coupling and reachable rejection; one actual funding transaction under both profiles | `d14cfea`; author report preserved in snapshot |
| B native graph design | Independent design review corrected Time0, stranded/exhausted escrow outcomes and deadline precedence; no B evaluator yet | `fe011a1` |
| Durable research record | Journal through CLM-0159 with explicit scope and failed-check limits | main `d247bca` |

The latest boundary author report records 40 boundary tests, 15 adapter tests,
441 Python tests, four typechecks, and 100 sampled funding traces without its
safety predicate failing. Each of five actions was witnessed in all traces;
the committed profiles split 50/50. These are author-run finite results.
At the snapshot, root source reviews preceded a two-branch action-effect syntax
correction. Subsequent A0 intake independently ran forty boundary tests and one
hundred funding traces on the corrected source and obtained nonauthor review.
Evidence is committed at `0190cb9`/`95899b3`; the historical author Python receipt
lacks a complete contemporaneous Python dependency/source closure. Local A0 is
accepted with that limitation disclosed, not as Council or whole-A acceptance.

Model/checker authorship overlaps. The frozen Python oracle is a separate
implementation, but shared authorship must remain disclosed. Adapter historical
RED provenance is incomplete and documented; do not reconstruct missing source
bytes and call them original evidence.

## Time and unsuccessful work

Git history records the Moriarty-only reset at 01:19 Denver and implementation
commits from about 01:58 through 08:30 on September 5. This is a commit timeline,
not a process-utilization measurement. There is no reliable complete breakdown
of productive versus wasted hours.

Avoidable orchestration/documentation overhead, rejected B drafts, fixture/name
and action-effect mistakes, and slow failed validations caused rework. Two
direct Apalache runs exhausted 4 GiB and 8 GiB heaps in InlinePass before checking
states. Those failures establish neither a checked invariant nor a model
counterexample, and do not justify rejecting an architecture. Receipts are at
`evidence/s02-model-comparison/candidate-a-apalache-offline/`, commit `93fce82`.
Do not resume blind memory escalation or reopen Foreman development.

## Immediate Candidate A roadmap

Every work package uses EARS requirements and OpenSpec scenarios.
The [package index](../openspec/WORK-PACKAGES-EARS.md) maps all packages.
Eight Candidate A changes refine S02.
The program change covers S01–S15 and G01–G24.
Existing adopted S01/S02 packages remain unchanged.
A0 local intake and A1 reviewed plans are accepted; A2–A7 remain open.
A2 installment fixtures (`35959ae`) and A3 swap fixtures (`3f440d2`) are
implemented and locally reviewed, with original compiling RED/GREEN preserved.
Swap ordinary authority routes (`c3c89fa`) now pass five tests, including all
twelve ordinary scenarios under both profiles. Installment authority routes
(`955f56b`) pass seven tests, including both race orders, two fills, fresh
cancellation and nonce1 recovery/refusal cases. Both adversarial/action-harness
tasks follow; shared regressions and full A2/A3 acceptance remain open.

The XML handoff expands these dependencies into executable acceptance contracts.
No phase below is checked off merely because its plan exists.

| ID | Remaining deliverable | Depends on | Completion evidence |
| --- | --- | --- | --- |
| A0 | Local intake accepted; Council open | Existing `d14cfea` | Evidence `0190cb9`/`95899b3`: independent 40-test and 100-trace results, native source review, sixteen-file closure and 149-member archive audit |
| A1 | Local plans adopted; behavioral acceptance remains A2/A3 | A0; adopted authority design | Commit `82d2c0b`: nonauthor review, two final static typechecks, eight exact assembled modules and twelve pinned frozen imports |
| A2 | Actual A installment authority lifecycle | A1 | Both race orders, second fill, fresh cancellation attempt, newly signed recovery ten/five and deadline cases; deterministic tests and per-action witnesses |
| A3 | Actual A swap authority lifecycle | A1; shared adapter/boundary | Separate depositor signatures, disposition signatures, real funds, settlement/refunds/timeouts, staleness, explicit no-effect rejection |
| A4 | Integrated exports and independent comparison | A2 + A3 | Complete mandatory inventory, source-pinned schema, full Python result/effect comparison, separately checked authority history and killed mutants |
| A5 | Bounded model-checking evidence | Stable A2/A3; factoring may be investigated earlier | Reviewed semantics-preserving factoring, explicit model bounds/properties, terminal checker receipts and exact limitations |
| A6 | Candidate A acceptance dossier and Council | A0–A5 | Per-obligation validator, independent source/evidence review, exact requested provider verdicts and resolved material dissent |
| A7 | Commit and integrate only admitted work | A6 and repository review policy | Explicit commits, reviewed merge, clean source pins; push only to verified configured remote |

### Historical draft-plan corrections, resolved by A1

The original archived `2026-09-05-moriarty-s02-candidate-a-authority-installment.md`
was unadopted. Commit `82d2c0b` replaces it on the experimental branch with
reviewed installment and swap plans. The original snapshot retains the following
draft errors for provenance; they are corrected in the adopted plans:

1. It incorrectly calls for preparing/signing the original parent nonce again
   after a fill. `canPrepareSigning` requires AuthorityUnused. Instead, the
   original parent policy must already cover the residual cancellation view;
   prepare a fresh **attempt** with current facts and reverify it. Only recovery
   introduces a newly signed nonce1 policy.
2. Two fills produce parent/consumption revision2, not revision1. Cancellation
   before any fill is revision1; cancellation after one fill is revision2.
   Recovery must preserve the cancelled parent and nonce0 record exactly.
3. Its witness command incorrectly joins names with commas; use separate CLI
   arguments. Its fixed step budget must be derived from the actual action path,
   not used to cut off recovery. It lacks concrete typed helper bodies and
   independent expected payloads and therefore needs a real implementation plan.
4. Its phrase “proves sampled finite paths” must become “observed finite traces.”

Check these against `authorization.qnt`, `execution.qnt`, `consumption.qnt` and
the existing generic installment harness. Do not change common rules to make
the erroneous draft pass.

## Whole-program roadmap after Candidate A

| Sprint | Deliverable | Status at this snapshot |
| --- | --- | --- |
| S01 | Terminology and intent theorem freeze | Local package completed earlier; Council backfill open |
| S02 | Core and intent semantic alternatives | Active: A incomplete; B adopted design; C/D not completed; no selection |
| S03 | Semantic motions | Open: evidence-backed constructor/type/effect/authorization/visibility decisions |
| S04 | Mechanized reference semantics | Open: normative environment, executable reference and theorem spine |
| S05 | Surface and certifying elaboration | Open: shared typed surface/Core and certificates |
| S06 | Compact compiler and translation validation | Open: general shared backend and differential/mutation evidence |
| S07 | Intent verifier and CAKE boundaries | Open: pre-signing rejection across lifecycle boundaries |
| S08 | ERC compatibility profiles | Open: evidence-backed adapters or explicit rejection |
| S09 | Proof-of-intent vertical slice | Open: real proof, plan verification, signing and settlement |
| S10 | Complete developer interface | Open: safety spine and supported API/data contracts |
| S11 | ACTUS source lock and conformance harness | Acquired prior material is input, not a completed sprint |
| S12 | ACTUS typed packages and independent semantics | Open: all 18 types and all 277 fixtures, two independent semantics |
| S13 | ACTUS shared compilation and backend correspondence | Open: every fixture through shared surface/Core/backend; no bypass |
| S14 | Adversarial conformance and audit | Open: full attack, mutation, privacy and rollback exercises |
| S15 | Terminal decision | Open: language, library plus certifier, verifier-only or justified stop |

S02 still needs genuinely different B intent-Core, C two-calculus and D library-only
implementations, comparable workloads, model checks and evidence-backed selection.
The B foundation plan is preserved as draft-only; it must not distract a
Candidate-A-only session. Finishing A cannot select A by default.

All 24 XML release gates remain unclosed by this handoff. The source XML contains
their exact predicates. ACTUS scope remains 32 taxonomy rows, 18 executable
types, 276 contract fixtures plus one analysis-date fixture: no exclusions,
quarantines, allowlist tricks, fixture-specific compiler bypass or unsupported
certification claim. S01 tests and S02 sampled runs cannot satisfy those gates.

## Operating rules for the next session

- Work on Moriarty. Foreman repairs and tasks1463–1466 remain retired; only use
  released tools. Record unavailable Council transport without repairing it.
- Requested Council members remain `gpt-6-astra`, `grok-4.6`,
  `claude-fable-5-1`; native reviewers or model self-reports are not substitutes.
- Use Quint, not TLC. Sample while implementing; model-check as separately
  required by XML S02. Keep finite evidence distinct from universal proof.
- Preserve frozen Core/swap, scope digest, raw XML and common contracts. A
  demonstrated specification conflict needs an explicit disposition, not a
  silent semantic amendment or hidden filter.
- Give each worker exact ownership. Parallelize only independent deliverables.
  Keep one review package per meaningful unit and retain dissent; do not reopen
  completed design votes or archive unchanged material repeatedly.
- Track command/PID, source closure, start and terminal result for expensive
  jobs. Poll the same job; elapsed wait alone is not failure. Inspect a minimal
  action/type slice before repeating an expensive full-suite compile.
- Record progress in the existing wiki and typed session database. Numerical
  results need commands and scopes. A snapshot is a recovery point, not liveness.
- The full product goal remains open. This request saves a handoff; it does not
  authorize representing a new loop, dispatch or sprint as already running.
