---
id: moriarty.wiki.index
title: Moriarty research index
type: overview
status: active
created: 2026-09-02
updated: 2026-09-07
tags:
  - moriarty
  - navigation
updated_at: 2026-09-07T18:01:49Z
---

# Moriarty language wiki index

[[wiki/overview|Vault overview]] · [[wiki/workflow|Research workflow]] · [[wiki/canvases/moriarty|Language map]] · [[wiki/meta/provenance|Provenance mapping]]

Read this page before searching externally. Merge new evidence into the existing
topic pages and preserve their source and claim identifiers.

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
- [Journal CLM-0187](research-journal.md)
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
- [Candidate A continuation evidence](research-journal.md) —
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
- [ZKIR K definition documentation](https://github.com/CharlesHoskinson/Moriarty/blob/archive/pre-cleanup-2026-09-07/experiments/zkir-k/docs/01-overview.md) — fifteen chapters, from getting started to the instruction reference, the verdict model, the tooling, the oracles, the known divergences and the design limits; each chapter checked cross-vendor as developer and formal methods reader
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

## Development and historical research

Use the [complete roadmap](../ROADMAP.md), [MC01-MC08 program](../openspec/MORIARTY-COMPLETION-PROGRAM.md) and [report reconciliation](../openspec/REPORT-RECONCILIATION-2026-09-07.md) for current execution and acceptance requirements.

Earlier Candidate A, S01/S02, A4/A5, K and SDK feasibility work is [archived](../docs/ARCHIVE.md). Its original results and unfinished obligations remain historical; they do not authorize an old execution queue. The retained research pages explain the source findings and their limitations.

Current reviewer rules are in [AGENTS.md](../AGENTS.md). [Review guidance](../docs/COUNCIL_REVIEWS.md) links to the superseded Council requirements for historical interpretation.
