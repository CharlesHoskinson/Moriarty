# Sherlock recursion restart navigation graph

This bounded graph links the published DoctorWatson factory export (`066c360b7735bddf6e8950a118c15d76b2953c78`), the separate Source44 source candidate, and the current recursion-loop receipt. It is navigation and provenance context; it does not authorize or claim a build, proof, wallet send, deployment, or benchmark.

Graphify structural extraction covers 24 files (21 published Python modules and 3 Source44 Rust candidate files), producing 1017 nodes and 2870 edges after adding receipt-backed loop and provenance nodes. Every published export path is hash-checked against its source manifest. See `coverage.json` for exact pins and exclusions.

The current loop receipt describes Source44 as a candidate and not yet integrated. The parent proof is not produced; Build9 is independently prepable; MP2 keyed artifacts are missing; terminal D is not produced; original wallet submission remains uncertain; stagenet deployment has not occurred; the synthetic benchmark is not measured. The new Source44 task has not executed, and its empty task-history file does not represent historical native execution history.

ProjectFenrir AgentBus commit `01c7202b9963881fe3b3e2771e031bc75be19223` exists locally as merged PR #84. Merge state does not establish live activation. The loop treats any adapter as optional advisory input and retains no event-driven adoption or launch edge here.

Edges between code entities are Graphify AST-extracted relationships. Stage dependencies and provenance edges are marked `EXTRACTED` from the cited receipt fields. The graph adds no semantic call edges across Source44 and factory files. It excludes raw proof/key material, wallet identifiers, private SDK paths, target execution results, and the larger historical graph.
