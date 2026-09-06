# Moriarty language wiki index

Read this page before searching externally. The wiki is initially sparse and
will grow by merging evidence into durable topic pages.

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
  evidence state, and next falsification test.

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
  naming, testing, distilled from the full kframework.org crawl (SRC-0026) and
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

- [Compact DSL feasibility and SDK specification](../deliverables/moriarty-compact-dsl-feasibility-and-sdk-specification.md) — S3 evidence boundary, footguns, 65 SDK components, and 28 wire contracts.
- [Work-package Council advisory](../deliverables/moriarty-work-package-council-advisory-2026-09-03.md) — frozen Grok, Sol, and exact Fable 5.1 review plus deterministic remediations.
- [OpenSpec work packages](../openspec/WORK-PACKAGES.md) — twelve evidence-gated sprint instruction sets.
- [Semantics, intent, compiler, proof, and SDK prompt](../deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml) — focused twelve-sprint assignment with an exact intent-refinement theorem and standards review.

## Workstream pages

The consolidated decision report maps these pages to all 18 controlling
deliverables. Narrower pages for migration, governance, APIs, and user research
will be added as those workstreams produce independent evidence.
