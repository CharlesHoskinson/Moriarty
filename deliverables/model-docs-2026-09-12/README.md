# Model documentation and WSL access

Collected by a GPT-5.6 Terra agent using Scrapling. The frozen corpus contains 20 official documents, including the full Claude Opus 5 system card and Grok 4.6 Model Card. Raw captures and extracted text hashes are in [MANIFEST.json](MANIFEST.json); scope and gaps are in [SOURCE-COVERAGE.md](SOURCE-COVERAGE.md).

## Verified intended CLI routes

| Requested model | CLI | Exact tested selection | Live completion |
| --- | --- | --- | --- |
| Gemini 3.8 Flash | AGY | `gemini-3.8-flash-high` | PASS |
| Claude Opus 5 | Claude Code | `claude-opus-5` | PASS |
| GPT 6 (official name: GPT-6 Astra) | Codex | `gpt-6-astra` | PASS |

Each route returned `MODEL_ACCESS_OK` with a successful terminal result in WSL2. [Verification receipt](cli/verified-access.json) links the commands and captured results. This checks one live completion per route, not tool permissions, quality, sustained availability, or every reasoning setting. Claude reports the canonical Opus 5 identity; AGY and Codex completion records do not separately attest a returned backend model ID.

Use `agy models` to select Google model/effort slugs. The intended routes are AGY for Google, Claude CLI for Anthropic, and Codex CLI for OpenAI. Exploratory direct-Gemini and cross-provider AGY failures are retained separately and do not invalidate those successful routes.

## Sources and reading

[Reviewed source notes](READINGS.md) cover model limits, reasoning controls, prompting, tool contracts, and source limitations. The OpenAI collection uses its model card and model guide; a distinct GPT-6 system card was not located in the bounded official documentation search. This is a bounded relevant documentation corpus, not every page on each provider website.

[Grok 4.6 reading notes](grok-reading-notes.md) cover its exact documented ID, reasoning and caching controls, safety-card limits, and the distinction between product labels and runtime identity. No Grok CLI probe was collected in this task.

## Knowledge graph

The graph is generated from [corpus](corpus), which wraps each deduplicated full text with provenance. Raw HTML and PDF are excluded from the scan to avoid duplicate entities. Graph extraction and reading notes preserve source references and mark inference separately from vendor statements.

Open the [interactive graph](graphify-out/graph.html), [JSON graph](graphify-out/graph.json), or [graph report](graphify-out/GRAPH_REPORT.md). The graph represents all 20 captured documents: 246 nodes, 291 relationships, 11 group relationships, and 23 named communities. [Build receipt](graphify-out/build-receipt.json) and [source validation](graphify-out/source-validation.json) record complete hash, endpoint, and 20-file semantic-cache validation. [Diagnostics](graphify-out/graph-diagnostics.txt) found no collapsed relationships. The four original extraction chunks remain in `graphify-out/` for audit.

The graph is selective and undirected; original relationship direction remains in the extraction chunks. It has seven disconnected components and one isolated node. No cross-provider relationships were added during merging. Vendor claims are source evidence, not reproduced results; some source inconsistencies remain explicitly ambiguous. Extraction usage and monetary cost are unavailable and recorded as `null`.

The required [native benchmark](graphify-out/benchmark-native.txt) estimates 145.6× fewer tokens for its two matched generic queries; [six documentation queries](graphify-out/benchmark.txt) estimate 72.1×. These figures measure estimated graph-response size against reading the full corpus. They do not measure answer correctness, retrieval recall, or latency. The native CLI also reports that the installed skill is version 0.9.48 while the package is 0.9.53.

Rebuild the validated exports with the recorded environment:

```bash
/home/charl/.local/share/uv/tools/graphifyy/bin/python3 /home/charl/Moriarty/deliverables/model-docs-2026-09-12/build_graph.py
```

The builder verifies the frozen sources before and after export, checks the curated community membership, saves the semantic cache and incremental manifest for covered files, and runs both benchmarks.

## Foreman footguns and Grok CLI observation

All 24 entries of [Foreman’s trap register](/home/charl/foreman/AGENT_TRAPS.md) were read. The [reading receipt](foreman-footguns-reading.json) pins the full file. Apply its content checks, negative controls, same-invocation recovery, and terminal-reason diagnosis. Historical CLI recipes do not override current model routing or permission scope.

A subsequent Grok design run completed with a valid proposal bound to the frozen session input. The request used `grok-4.6` at high effort; returned telemetry names `grok-4.6-build`. The [CLI receipt](cli/grok-design-access.json) records both. The captured official sources do not establish their alias mapping. The initial two-turn attempt failed; the same-session continuation completed after reading the offloaded input.
