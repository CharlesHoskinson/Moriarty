# DoctorWatson factory source graph

This metadata-only refresh retains the prior Graphify extraction: 19 DoctorWatson factory Python files plus 2 separately labeled optional AgentBus candidate files. The graph contains 792 nodes and 2,065 edges, including 2,011 `EXTRACTED` and 54 `INFERRED`; it has zero failed Python sources, dangling endpoints, or duplicate IDs. The Python AST was not rerun because all source hashes still match.

The existing 17 context pins and their graph-node hashes were refreshed. Literal cross-file evidence lines were rechecked against current text. The public `LOCAL-RESULTS.json` and workflow wiki are recorded as hash-only external locators, without graph nodes.

## Current validation metadata

Cleanup source remains pinned to the frozen candidate. Its author tests (31), independent reviewer tests (5), and final factory suite (264) passed. This graph refresh does not independently validate those test results. The AgentBus adapter has 18 author and 212 independent tests passed, and its shared tool merged to ProjectFenrir PR84 at commit `01c7202b9963881fe3b3e2771e031bc75be19223`; live activation remains false. It remains a separate optional coordination candidate and does not establish live dispatch or factory adoption.

## Limits

Python module edges remain the original Graphify AST and cross-file resolution output. Shell, Markdown, Quint and OpenSpec files remain hash-only context. No source/API body changed, and no semantic graph claims were added. This metadata refresh ran no tests, builds, Quint checks, cleanup/apply, Git mutation, provider call, SDK operation, native proof, wallet access, or target action. Private SDK/cache and runtime data remain excluded.
