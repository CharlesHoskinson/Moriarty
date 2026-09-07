# Moriarty language wiki index

Read this page before searching externally. The wiki is initially sparse and
will grow by merging evidence into durable topic pages.

## Current direction — 2026-09-06 reset

**Complete product roadmap:** [ROADMAP.md](../ROADMAP.md) consolidates current capabilities, all remaining work and acceptance criteria. The linked wiki pages preserve the research notes and source dispositions.

**Current report reconciliation:** [combined graph and review](../deliverables/moriarty-report-plan-review-2026-09-07/README.md) and [remaining-plan amendment](../openspec/REPORT-RECONCILIATION-2026-09-07.md). Moriarty is Midnight-centric. Early financial/intent and complete native/ledger checks constrain successor designs; MC01-MC08 acceptance remains open. See CLM-0918 through CLM-0921.

**Latest execution plan:** [MC01–MC08 completion program](../openspec/MORIARTY-COMPLETION-PROGRAM.md)
covers the seven requested language, financial, proof, acceptance, composition,
conformance, and correspondence gaps. The plans are S2, specified-only.
See [review and loop receipts](../evidence/moriarty-completion-program-2026-09-07/README.md)
for actual audit and runtime status. Old A4/A5 work remains superseded.

**Latest review:** [Midnight network choice and faucet diagnosis](../evidence/midnight-network-review-2026-09-07/README.md).
Local Docker is the daily development default; Preview is preferred for early
public integration and Preprod remains valid for final validation. Faucet URL
drift and wallet sync are separate from network availability; public settlement
now has [finalized Preview deployment and call](../evidence/midnight-preview-2026-09-07/README.md).
DUST replay recovered the hidden reservation; a fresh call settled and exact
readback passed. Preview is the sole public execution target this sprint. See
CLM-0205 and SRC-0061–0062; prior failures remain in CLM-0204.

**Latest implementation output:** [Midnight Docker settlement evidence](../evidence/moriarty-midnight-network-2026-09-07/README.md),
[published documentation capture](../evidence/midnight-docs-2026-09-07/README.md), and
[R3 native result](../evidence/moriarty-native-ivc-r3-2026-09-07/README.md).
Local NIGHT/DUST/deploy/call transactions settled; Preprod attempts are historical
and no longer required for this sprint. Native recursive setup exhausted rows at k17;
no recursive proof was produced. CLM-0200–0202 records this sprint.

Previous output: [R2b outcome-intent workspace and evidence](../evidence/moriarty-r2b-outcomes-2026-09-06/README.md).
At `/intents`, sign bounded outcome authority before choosing a pool route, or
settle loan dues through the same checker. Independent gross debit/net goal
checks, local expiry and atomic nonce consumption are implemented. Required
real proofs remain unavailable; example changes/reload reset local nonce history.
[Three held-out cases](../evidence/moriarty-r2b-heldouts-2026-09-06/README.md)
remain needs-extension. The [intents amendment](../docs/research/2026-09-06-intents-report-integration.md)
continues to control the scope. R2 exact-plan mode remains at `/language`.

The [PCD report integration](../deliverables/pcd-report-integration-2026-09-06/README.md)
and [ACTUS/DeFi target study](../deliverables/moriarty-design-sprint-2026-09-06/README.md)
remain controlling foundations. The [native R3 test](../experiments/moriarty-native-ivc-r3/README.md)
is implemented and blocked at recursive setup. No general DSL, contract theorem,
complete conformance or real PCD integration is claimed complete.

The [user reset](../raw/assignments/moriarty-target-first-reset-2026-09-06.md)
supersedes the old A4/A5 execution loop. Its results and unfinished obligations
remain historical evidence. The current work is ACTUS/DeFi-led language design:

- [Postmortem](../docs/postmortems/2026-09-06-moriarty-verification-detour.md)
  and [mandatory footguns](../docs/FOOTGUNS.md).
- [PCD research](../docs/research/2026-09-06-pcd-bounded-dsl.md): proof-carrying
  transactions, bounded correctness and backend questions.
- [New design cycle](../docs/superpowers/plans/2026-09-06-actus-defi-pcd-replanning.md):
  target study, unified-semantics proposal and developer mock.
- [Journal CLM-0187](research-journal.md#clm-0187-user-reset-after-verification-detour)
  records the scope change. Older continuation instructions below do not
  authorize automatic native work.

## Foundations

- [LLM Wiki research workflow](sources/llm-wiki-pattern.md) — provenance and
  domain adaptation of the persistent-wiki method.
- [Open questions](open-questions.md) — research gaps and verification routes.
- [Contradictions](contradictions.md) — source drift and unresolved conflicts.
- [Pinned Marlowe V1 baseline](marlowe-baseline.md) — exact V1 algebra,
  transition path, validator evidence, and Cardano boundary.
- [Marlowe repository and documentation graph](marlowe-repository-graph.md) —
  36 pinned repositories, 100 live documentation pages, graph coverage, and
  extraction limitations.
- [DeFiFormal taxonomy audit](defiformal-taxonomy.md) — reproduced 72-protocol
  corpus, 60-construction suite, M2+M3 family/facet decision, historical M4+
  crosswalk, and Moriarty formal-behavior boundary.
- [Formal assurance matrix](formal-assurance.md) — theorem qualifications and
  the Moriarty proof boundary.
- [Midnight repository inventory](midnight-repositories.md) — 74 official
  repositories, active Compact source relocation, and backend pins.
- [Reproduced benchmarks](benchmarks.md) — Marlowe scaling, typed values, the
  compiled Moriarty escrow, and the E00 atomic-swap stop test.
- [Moriarty architecture](moriarty-architecture.md) — Core, surface, Compact,
  ZKIR, Runtime, and trust boundaries.
- [Security boundaries](security.md) — threat and audit partition.
- [Port decision](decision.md) — scorecard and go/no-go recommendation.
- [Research program](research-program.md) — agentic-council synthesis, XML
  prompts, evidence-gated sprints, intent correctness, CAKE and ERC review,
  backend stop test, and incremental decision graph.
- [Research journal](research-journal.md) — iteration decisions, semantic scope,
  evidence state, branch-only S02 foundation progress, Council restart status,
  historical Foreman repair attempts, the Moriarty-only workflow reset,
  four-alternative S02 coverage, delegated design, the branch observation-carrier
  implementation, atomic settlement, rejected attempts, and executed installment
  races with signed recovery, Candidate A's full transaction/projection interpreter,
  actual swap and installment traces, the completed finite-record Python comparison,
  the completed authority observation adapter with explicit historical RED
  limitations, both offline Apalache heap failures, and the candidate-specific
  signing/verification/commit boundary under implementation, and the reviewed
  native Candidate B design with explicit stranded-escrow and clock boundaries.
- [Candidate A continuation evidence](research-journal.md#clm-0186--checkpoint-after-the-a5-view-typecheck) —
  admitted A4 source gate and failed-export preservation, successful A5 view
  typecheck, and saved follow-up drafts; exports and verification remain open.

## K Framework and ZKIR semantics

Started 2026-09-03 on branch `zkir-k-semantics`. Goal: an executable K
Framework semantics of ZKIR v3. Pins: K v7.1.337 (`4a46d123`), arc-zkir
(`fd1c24e1`), `midnight-ledger` `ledger-9` `92e8bdd3` for the specified
surface, standalone `midnight-zkir` `2ffe2d17`.

- [ZKIR semantics in K: plan](zkir-k-semantics-plan.md) — the synthesis:
  configuration, sorts, rule shape, pin, field representation, parsing route,
  test oracle, and the milestones with their results.
- [The ZKIR semantics in K, as built](zkir/zkir-k-definition.md) — module
- [ZKIR K definition documentation](../experiments/zkir-k/docs/01-overview.md) — fifteen chapters, from getting started to the instruction reference, the verdict model, the tooling, the oracles, the known divergences and the design limits; each chapter checked cross-vendor as developer and formal methods reader
  layout, what a run produces, the three layers of checking against the Rust
  crate, the divergences found, and the Agda and k-rust oracles.
- [K Framework overview](k-framework/k-framework-overview.md) — what K is,
  repository layout, install paths, toolchain commands, backends.
- [K tutorial section 1](k-framework/k-tutorial-basic.md) — lessons 1.1 to
  1.22: syntax, attributes, configurations, rules, strictness, builtins,
  casts, debugging, backends, symbolic execution, proofs.
- [K tutorial section 2](k-framework/k-tutorial-intermediate.md) — lessons
  2.1 to 2.17 (most are stubs that defer to the user manual): macros, fresh
  constants, KLabels, overloading, #Or, function context, MInt, KORE, REPL.
- [K best practices for the ZKIR definition](k-framework/k-best-practices.md) —
  functions and totality, rule discipline, collections, builtins, backends,
  naming, testing, distilled from the full kframework.org crawl (SRC-0039) and
  applied to the ZKIR modules.
- [K user manual digest](k-framework/k-user-manual.md) — production, cell and
  rule attributes, rewriting semantics, module system, kompile/krun/kprove
  options, claims.
- [K builtin domains](k-framework/k-builtins.md) — Int, Bool, String, Map,
  List, Set, Bytes, MInt, Float, and what fits a prime-field circuit IR.
- [K backends, tools, and pyk](k-framework/k-backends-and-tools.md) — LLVM
  and Haskell backends, KORE, kore-rpc, pyk pipeline and KCFG proofs.
- [K documentation graph](k-framework/k-documentation-graph.md) — 315-node
  graphify graph of the K docs built through agy; hubs, communities, health.
- [ZKIR instruction set](zkir/zkir-instruction-set.md) — program structure,
  all 34 spec instructions with witness and constraint semantics, JSON
  format, worked precompile example, the 2ffe2d1 extensions.
- [ZKIR types and values](zkir/zkir-type-system.md) — BLS12-381 scalar
  field, the thirteen-type surface, conversions, typed outputs, invariants.
- [ZKIR VM operational semantics](zkir/zkir-vm-semantics.md) — state,
  transition rules per instruction, preprocessing, proving modes, errors,
  spec versus crate.
- [Compact to ZKIR pipeline](zkir/compact-to-zkir-pipeline.md) — how
  compactc lowers circuits, v2 default and v3 flag, artifacts, ledger
  consumption.
- [ZKIR formal specification and Agda mechanization](zkir/zkir-formal-spec-agda.md)
  — arc-zkir pins, formal model, trust base, theorems, module map, timeline.
- [ZKIR v3 in-circuit versus off-circuit divergences](zkir/zkir-v3-divergence-review.md)
  — the 13 review findings and their consequences for a K definition.
- [Midnight's K tooling](zkir/midnight-k-tooling.md) — k-rust and
  k-framework-ts: scope, supported K subset, pins, Lean export, maturity.

## Active specification artifacts

- [Candidate A roadmap](../docs/MORIARTY_ROADMAP.md) — A0–A3 locally accepted;
  A4 checker/producer units, the amended17+97 preliminary gate and final-source
  parser admitted; case014 export failed with heap exhaustion and no ITF.
  The compiler diagnostic timed out after entering compile. The no-flatten
  tiny control completed; its original failed type predicate is preserved and
  a reviewed correction admits the retained tiny data. The full contrast
  compiled but exceeded its JSON intake limit. The tiny-evidence package is
  independently audited and committed, and a bounded retained-output plan is
  adopted. Seven A4 observer controls are independently admitted; both native
  pairs and their evidence are independently admitted. The one full diagnostic
  failed with heap exhaustion after entering `runSimulator`. The bounded
  retained-output inspection failed an import-order predicate and resource
  report parsing; both failures are preserved. A separately reviewed correction
  passed the retained-output structural inspection and has independent/root
  admission. A4 literal wrappers are being implemented. H1, full A4–A7 and
  Council stay open.

- [Execution roadmap](../docs/MORIARTY_ROADMAP.md) — saved overnight results,
  Candidate A completion dependencies, and the still-open S01–S15 program.
- [EARS and OpenSpec package index](../openspec/WORK-PACKAGES-EARS.md) — A0–A7
  completion changes, all sprint contracts, and the complete release-gate mapping.
- [Candidate A completion XML](../deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml) —
  focused handoff with prior-source references and corrected lifecycle requirements.

- [Moriarty restart](../docs/MORIARTY_RESTART.md) — Foreman development closed by user direction; resume S02 from preserved foundations, with type-sketch approval and acceptance reviews distinct from implementation.
- [Prompt v1.3 execution audit](../docs/superpowers/reviews/2026-09-04-moriarty-v1.3-execution-audit.md) — signing, parser, effect-completeness, and validator findings with execution dispositions.
- [S01 intent-theorem package](../openspec/changes/s01-intent-theorem-freeze/README.md) — completed, independently reviewed specification package whose ten local predicates passed at S3; mechanization, full effect completeness, runtime verification, backend and ledger correspondence, ACTUS, pilots, and all 24 prompt release gates remain open at this boundary.
- [S01 evidence manifest](../evidence/s01-intent-theorem-freeze/evidence-manifest.json) — reproducible local package-gate evidence for the candidate-unmechanized theorem and exact-transfer falsifier; it grants no signing authority and establishes no proof, backend, ledger, ACTUS, or pilot result.
- [Council review requirements](../docs/COUNCIL_REVIEWS.md) — requested reviewer identities, binding requirements, and the still-open S01 backfill and S02 review queue.
- [Council runtime binding intake](../docs/superpowers/reviews/2026-09-05-council-runtime-binding-intake.md) — inspected missing carrier, requested-versus-observed model-route limits, and the pending runtime implementation boundary.
- [S02 delegated common-design decision](../docs/superpowers/specs/2026-09-05-moriarty-s02-common-design-decision.md) — three completed expert proposals, majority choices and preserved dissent, superseding the earlier carrier sketch; implementation and Council gates remain open.
- [Foreman PID-namespace research assignment](../deliverables/foreman-grok-4-6-pidns-deep-research-prompt-2026-09-05.xml) — read-only Fable handoff for the observed launcher degradation and complete Grok 4.6 model-card review; research results remain pending.
- [Compact DSL feasibility and SDK specification](../deliverables/moriarty-compact-dsl-feasibility-and-sdk-specification.md) — S3 evidence boundary, footguns, 65 SDK components, and 28 wire contracts.
- [Work-package Council advisory](../deliverables/moriarty-work-package-council-advisory-2026-09-03.md) — frozen Grok, Sol, and exact Fable 5.1 review plus deterministic remediations.
- [Earlier OpenSpec work packages](../openspec/WORK-PACKAGES.md) — the preceding twelve-package program, not the controlling XML v1.3 S01–S15 sequence.
- [Semantics, intent, compiler, proof, SDK, and ACTUS prompt](../deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml) — version 1.3 assignment with an exact intent-refinement theorem, standards review, and a no-exclusion 277-vector ACTUS completion gate.
- [ACTUS public-source acquisition manifest](../evidence/actus-public-source-acquisition-2026-09-03.json) — 270 fetched public URLs, one explicit robots failure, 220 documentation host rewrites, 14 repository pins, 32 taxonomy rows, and all 277 reference fixtures.
- [ACTUS public code survey](../evidence/actus-public-code-survey-2026-09-03.json) — executable contract surface, Haskell harness gaps, private Java boundary, license constraints, and prompt consequences.
- [Intent standards research source](../deliverables/moriarty-intent-standards-2026-09-03/report-source.md) — complete CAKE and NEAR documentation crawl plus current Ethereum/OIF lifecycle, trust, and SDK findings.
- [Intent prompt council advisory](../deliverables/moriarty-intent-prompt-council-advisory-2026-09-03.md) — blind Grok, Sol, and Fable review, preserved dissent, and the single correction pass applied to prompt version 1.2.

## Workstream pages

The earlier consolidated decision report maps these pages to its 18 deliverables.
The controlling XML v1.3 now requires D01–D22 and S01–S15; the old report and
twelve-package manifest do not establish completion of that larger program.
Narrower pages for migration, governance, APIs, and user research will be added
as those workstreams produce independent evidence.
