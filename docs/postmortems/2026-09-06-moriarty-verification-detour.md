# Moriarty verification detour — postmortem

Date: 2026-09-06. Owner: the lead coding/research agent.
Status: evidence-based process assessment; no new native verification run.

## What went wrong

I let verification of a candidate execution model become the main objective
before showing that the model could express the user's financial workloads or
support a usable developer interface. ACTUS and the DeFi Kernel had already
been supplied as implementation targets. I failed to keep them in control of
the work order. The resulting experiments contain useful evidence, but the
effort did not produce the unified DSL design the user needed.

This was a planning and orchestration failure. It was not caused by the user
failing to provide a purpose. My subsequent explanation suggesting that we
needed to discover a purpose was wrong. Describing the repository as having no
Compact implementation was also too broad: E00 already generated a narrow
atomic-swap specialization. Neither correction makes the general DSL complete.

## Evidence and sequence

| Evidence | What it establishes | Consequence |
| --- | --- | --- |
| [DeFi assignment](../../deliverables/moriarty-defi-kernel-deep-research-prompt-2026-09-03.xml), purpose and G03/G17 | The requested product was a bounded semantic/authoring layer for the DeFi Kernel, with Compact generation, SDK and non-toy workload demonstrations. | The kernel was a design input, not optional future background. |
| [Controlling v1.3 assignment](../../deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml), ACTUS requirements around lines 530–590 and W10 | It requires 18 executable ACTUS types as typed packages, 277 fixtures, two independent semantics, every ordered result field, one shared compilation path and developer operations. | The product objective and acceptance targets already existed. |
| [Roadmap](../MORIARTY_ROADMAP.md), historical “Whole-program roadmap after Candidate A” | Surface, general compiler, developer interface and ACTUS were sequenced after candidate verification. | Target-driven expressivity was deferred while effort accumulated on one candidate. |
| [Journal](../../wiki/research-journal.md), CLM-0148 through CLM-0186 | Work concentrated on authority/lifecycle models, wrappers, source reviews, native export failures, input preservation and factoring. A4/A5 acceptance remained open. | Local evidence grew without establishing the end-to-end language. |
| [E00 scope](https://github.com/CharlesHoskinson/Moriarty/blob/archive/pre-cleanup-2026-09-07/evidence/semantic-scope/moriarty-core-0.0.0-e00.2.json) and [toolchain results](https://github.com/CharlesHoskinson/Moriarty/blob/archive/pre-cleanup-2026-09-07/experiments/moriarty-core-swap/toolchain-results.json) | A finite interpreter and generated canonical swap Compact specialization existed; proof compilation was mock. | Reuse this narrow baseline honestly; do not call it a general compiler or real proof pipeline. |
| [Reset directive](../../raw/assignments/moriarty-target-first-reset-2026-09-06.md) | The user replaced the execution loop with postmortem, target study, unified-semantics planning, developer mock and PCD requirements. | Older continuation instructions no longer authorize another A4/A5 run. |

These are repository observations. The causal assessment below is my inference
from that sequence, rather than a claim that each individual experiment was
technically invalid.

## Root causes and contributing factors

1. **I optimized a subordinate milestone.** “Finish A4/A5” became a self-contained
   completion objective. I did not repeatedly check whether its next experiment
   would resolve a question needed by ACTUS, DeFi or the developer workflow.
   The language's intended workload did not govern candidate selection.
2. **I tried to establish too much about a provisional model too early.**
   Freezing and checking the candidate's mechanics preceded representative
   financial semantics. Correctness of a chosen relation cannot establish that
   it is the right relation for amortization, liquidation or composed contracts.
3. **I failed to limit the cost of uncertainty.** Native export and memory
   problems generated investigations, preservation work and follow-up designs.
   The completion loop lacked an aggregate decision budget and a product-level
   stop condition. More careful receipts did not resolve the missing design.
4. **Process became a competing deliverable.** Scoped reviews and authentic
   receipts were useful, but their volume and repeated intake obscured how
   little new developer capability had been demonstrated. Reviewers checked
   their assigned predicates; I was responsible for selecting worthwhile ones.
5. **I communicated internal progress as product progress.** Phase labels and
   test totals forced the user to ask whether a DSL existed. A clear answer
   should have led with the narrow implementation, unfinished language design,
   absent general proof pipeline and missing target demonstrations.
6. **Recovery preserved obligations better than intent.** Checkpoints retained
   detailed execution queues. I failed to reconcile those queues with the
   original targets and the user's increasingly explicit direction change.

## Cost and impact

The retained [goal-runner observation](../../evidence/moriarty-reset-2026-09-06/goal-observation.json)
reports **3,862,661 tokens** and **11,013 seconds** for the old goal, whose state
was `usageLimited`. These are tool-reported counters. They are not a billing
invoice, CPU-hour measurement or complete project-wide cost ledger. There is
no evidence here for an exact dollar amount or for treating every counted token
as wasted work.

The material waste was work performed before its relevance to the intended DSL
was established, plus the user's repeated effort to recover the purpose and
understand progress. A4/A5 remained incomplete; full ACTUS coverage, unified
semantics, a general compiler and a usable developer workflow were not delivered
by that campaign. The cost of reusable work cannot be cleanly separated from
avoidable work using the available counters.

## What remains useful

| Retain | Reuse condition |
| --- | --- |
| Pinned ACTUS sources, reference vectors and DeFi taxonomy/source audit | Use them immediately as semantic requirements and coverage evidence; source acquisition is not implementation coverage. |
| E00 interpreter, swap Compact generation and translation-validation artifacts | Re-evaluate against the new shared semantics. Preserve the mock-proof limitation. |
| Candidate A authority, stale-state, rejection and lifecycle models | Treat as candidate definitions and regression examples. Map each to a target behavior before adopting it. |
| A4 wrappers, factoring comparisons, raw failures and bounded local checks | Preserve historical predicates and bounds. Do not rerun or finish them merely to recover sunk cost. |
| Runtime failure investigations and source freezes | Retain for future tool diagnosis if a selected design needs the same path. They are not the new product roadmap. |

Experimental anchor: `s02-model-comparison` at
`fbea1cee5de497a3e58fe3081bb49b4059edd97b`, in `.worktrees/s01-audit-start`.
CLM-0186 remains the historical account of the successful A5 copied-view
typecheck and failed A4 export preservation. It does not authorize continuation.

## Corrective actions

- **Done in this reset:** document the failure; add mandatory
  [footguns](../FOOTGUNS.md) to repository instructions; mark the old roadmap
  historical; study primary PCD sources with Scrapling and independent research
  agents; begin a concrete ACTUS/DeFi requirements study.
- **Next design cycle:** finish the target matrix, compare unified semantics,
  specify the financial compliance relation and finite bounds, and produce a
  developer mock proposal grounded in real fixtures and DeFi behaviors.
- **Before substantial implementation:** review those concrete outputs, then
  select a bounded backend feasibility experiment and a property-driven
  implementation slice. Do not restart a broad formal campaign first.

Success is a developer being able to explain the contract, its behavior, its
proved properties and the acceptance meaning of its transaction proof. More
phase checkmarks alone will not satisfy the reset.
