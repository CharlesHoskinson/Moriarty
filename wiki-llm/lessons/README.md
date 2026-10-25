---
title: "Lessons learned: Loki and the Sherlock factory"
project: lessons
date: 2026-10-24
tags: [lessons, factory, loki, sherlock, agents, prover]
---

# Lessons learned

These are the lessons from running the DoctorWatson factory, the Loki prover work and the Sherlock drivers. Each lesson
says what happened, why it matters and what to do next time. Add new lessons to the page for their topic, newest first,
with the date and the evidence (commit, trace, log path or measurement).

| Page | Covers |
|---|---|
| [factory-and-agents.md](factory-and-agents.md) | Running many agents: sandboxes, buses, commits, filters, retries, budgets, host resources |
| [prover-performance.md](prover-performance.md) | What makes the Midnight prover slow or fast; measurements that overturned expectations |
| [correctness.md](correctness.md) | What the Lean spec, Quint models, harness and fuzzing caught, and stock-prover pitfalls |
| [sherlock.md](sherlock.md) | The Sherlock recursion chain, keys and deployment |

Related: [Loki design](../loki/LOKI-DESIGN.md), [Loki 10x verdict](../loki/10x/LOKI-10X.md),
[speed-up scouts](../loki/speedups/), [AgentBus decision](../doctorwatson-agentbus/DECISION.md).
