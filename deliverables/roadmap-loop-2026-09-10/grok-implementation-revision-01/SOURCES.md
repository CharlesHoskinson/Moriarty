# Grok 4.6 sources and compatibility findings

Status: research/design evidence, not product acceptance. Designer: GPT-6 Astra at high effort. Retrieval date: **2026-09-11 UTC** (September 10 in America/Denver; exact acquisition times are in the raw receipts). Public research was explicitly requested. Existing wiki index/hot references were checked for Grok material; `docs/GROK-TIMEOUTS.md` supplies relevant local observations. No canonical wiki claims were merged.

The official [documentation index](https://docs.x.ai/llms.txt) and [complete Markdown corpus](https://docs.x.ai/llms-full.txt) were acquired with Scrapling 0.4.15. The corpus contains **181 sections**. Astra high subsequently read every captured body: all 181 sections in 190 full-text chunks, including unrelated audio, image/video generation, consumer integrations and business administration. [ALL-DOCS-READING.md](ALL-DOCS-READING.md) explains coverage and additional findings; [inventory-of-reading.json](inventory-of-reading.json) records every section, hash, read chunk and disposition. Eight gRPC sections contain only headings, and some rendered fields are absent; those gaps are explicit, not treated as reviewed protocol definitions. This covers the finite captured corpus, not unpublished documentation, every recursively linked external source or every installed local guide. No separate exact-4.6 coding-prompt recipe was found in the index or targeted official-site search; the worker prompt is our design inference from the model and harness documentation.

Immutable original response bodies, timestamps, requested/canonical URLs, HTTP status and SHA-256 hashes are under [raw/receipts.json](raw/receipts.json). [The initial section inventory](raw/documentation-inventory.json) preserves every archived page and its original targeted inspection scope; the separate reading inventory records the later complete captured-body review. The selected body hashes identify extracts from that single official corpus; they are not falsely represented as independent page requests. `docs.x.ai/robots.txt` returned 200 and allowed search/AI input, excluding training; `media.x.ai/robots.txt` returned 404. Public sources were fetched without authentication, cookies, bypass, or new telemetry. No novel site pattern or cookie needed saving.

## Exact model evidence

| Source | Inspection and finding |
| --- | --- |
| [Official safety index](https://x.ai/safety) | Followed its exact Grok 4.6 PDF link; establishes publisher provenance. |
| [Grok 4.6 model card](https://media.x.ai/v1/website/card-4p6-4cd2dc57.pdf) | Entire 42-page card text inspected. Cover: August 12, 2026; revision: August 17. Sections 2 and 6/12 distinguish evaluated capability from reliability; coding scores depend on harness/effort. SWE-Marathon high is 31.9%; internal hallucination rate is 1.7%. Those are benchmark observations, not Moriarty success probabilities. The card excludes unattended high-stakes financial decisions without appropriate oversight/validation. It does not prohibit bounded software development or establish financial correctness. |
| [Grok 4.6 overview](https://docs.x.ai/developers/grok-4-6) | Exact model `grok-4.6`: 500,000-token context, text/image input, text output; low/medium/high/xhigh, high default. Overview says no text output limit. This describes model capacity, not an unlimited request or spend budget. |
| [Introducing Grok 4.6](https://x.ai/news/grok-4-6) | August 12 launch; provider reports a focus on longer agentic coding work. Marketing/evaluation claims remain source claims, not replicated Moriarty evidence. |
| [Models](https://docs.x.ai/developers/models) | At retrieval, API price below 200k prompt tokens: $2 input/$0.50 cached/$6 output per million; at or above 200k: $4/$1/$12. Model aliases may move. Record requested and returned identity; no dated exact-4.6 snapshot is invented. Team/OAuth billing is not established by this table. |

## API facts that affect the design

- **Reasoning:** exact 4.6 supports low/medium/high/xhigh and cannot disable reasoning. `presencePenalty`, `frequencyPenalty` and `stop` are not supported with reasoning models. A generic schema listing `none` does not grant it for 4.6. High is retained; this design does not escalate to xhigh or guess a magic temperature. [Reasoning](https://docs.x.ai/developers/model-capabilities/text/reasoning)
- **Context and output:** Responses `max_output_tokens` defaults to 128,000 and includes reasoning; a larger request value is possible. `context_management` is parsed but not executed, while `background` and `truncation` are unsupported. Therefore the overview's model output claim does not remove the request cap. This is an API finding, not a CLI flag or the CLI's actual effective cap. [Responses reference](https://docs.x.ai/developers/rest-api-reference/inference/responses)
- **Compaction:** the explicit `/v1/responses/compact` endpoint accepts a conversation that still fits; retain its opaque output unchanged. It does not rescue an already oversized input. This is distinct from Build's own session compaction and does not justify another API loop. [Context compaction](https://docs.x.ai/developers/advanced-api-usage/context-compaction)
- **Stable context:** unchanged prefixes help caching; Responses `prompt_cache_key` and Chat `x-grok-conv-id` improve routing. Cache hits are not guaranteed. Keep stable worker instructions followed by a short current task; use supported CLI session continuation rather than inventing flags for these API fields. [Caching](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/best-practices), [cache misses](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/multi-turn)
- **Tool use:** API client functions execute on the caller, built-in tools on xAI; parallel calls are enabled by default. All requested client results must be matched before continuing. This is not permission for simultaneous writes or proof campaigns. Tool schemas support a limited JSON Schema subset; validate substantive output and resource authority independently. [Function calling](https://docs.x.ai/developers/tools/function-calling), [structured outputs](https://docs.x.ai/developers/model-capabilities/text/structured-outputs)
- **Latency and accounting:** streaming provides observations during long agentic calls. API rate limiting is per model/team RPS and TPM; cached/reasoning tokens still count. Preserve 429/error metadata and use bounded backoff within remaining admission, not blind full worker replays. API cost ticks are per request, with 10^10 ticks/USD; this does not establish complete CLI/OAuth accounting. [Streaming](https://docs.x.ai/developers/tools/streaming-and-sync), [rate limits](https://docs.x.ai/developers/rate-limits), [errors](https://docs.x.ai/developers/debugging), [cost tracking](https://docs.x.ai/developers/cost-tracking)
- **Storage:** stateful Responses are retained 30 days by default; ZDR changes available persistent server storage features. The [WebSocket guide](https://docs.x.ai/developers/advanced-api-usage/websocket-mode) documents a separate most-recent-response in-memory continuation exception even with ZDR or `store=false`; it is not a CLI switch. Do not infer CLI privacy settings or upload credentials/private witnesses from an API option. [Text generation](https://docs.x.ai/developers/model-capabilities/text/generate-text), [data/privacy](https://docs.x.ai/developers/faq/security)

## Local CLI observations and documentation boundaries

Observed `grok --version`: **grok 1.0.13 (5e9a58528b76)**. Command resolves to `/home/charl/.grok/bin/grok-1.0.13`. [Captured help](raw/grok-help.txt) supports `--cwd`, `--model`, `--reasoning-effort`, `--prompt-file`, `--max-turns`, `--no-plan`, `--no-subagents`, `--always-approve`, `--allow`/`--deny`, `--sandbox`, JSON/native streaming and Messages-compatible streaming. These help results prove parsing availability, not successful model execution or isolation. Root separately owns runtime canaries and adopted launch permissions.

| Boundary | Consequence |
| --- | --- |
| Headless `--worktree` does **not** create a worktree, per installed help. | Create/verify the existing isolated checkout first, then pass `--cwd`; never assume worktree isolation from that flag. |
| Local `--session-id` creates a new UUID and rejects reuse; web headless guide says create/resume. | Use installed help: resume only the exact captured ID with `--resume`; never `--restore-code` on dirty work. |
| Local `--max-turns` bounds main-agent model rounds; tool calls/subagents can multiply work. Local docs say no CLI dollar-budget feature. | Retain host campaign time/memory/CPU/bytes/attempt/spend controls; a turn cap is not a comprehensive budget. |
| Native `json` is terminal-only; `streaming-json` exposes tool/usage/end/error events. | Prefer native streaming when the existing supervisor can parse it; use actual activity and terminal `end`/`end_turn`, not output quietness, as observations. |
| Messages-compatible output can zero-fill unknown usage/cost. Native JSON records incompleteness and omits unknown complete cost. | Keep unknowns and actual requested/returned model names; do not infer free use or add cached buckets twice. Compaction/side-model usage can be excluded from CLI aggregates. |
| Streams/JSON can include `thought`/thinking fields. | Retain tool/results, terminal metadata and final concise rationale while excluding private reasoning; do not copy private session transcripts into repository evidence. |
| Plan mode and permission mode are distinct; always-approve does not approve a plan. | Use the existing approved noninteractive edit invocation with `--no-plan`. Preserve deny rules/admin locks/sandbox/campaign gates; do not bypass a safety denial. |
| Official hook docs state timeout/crash/malformed output is fail-open. | Hook installation cannot substitute for guarded campaign admission or prove interception. |
| Build `/loop` is a periodic scheduler, expires after seven days. | Keep the active native host goal; do not add a second scheduler or recreate the goal. |
| API cache/compaction/background parameters are not matching CLI switches in inspected help. | No invented flags, global config changes or new SDK orchestration. |

The installed guides in [local-docs-manifest.json](raw/local-docs-manifest.json) are version-adjacent shipped documentation, not observed runtime behavior. Relevant details were checked against actual help and repository receipts. No installed config, credential file or unrelated session transcript was copied.

## Actual parent-owned availability evidence

The root's [first Grok canary](grok-native-json-availability.json) used positional input and failed with exit 1/device error. The [corrected canary](grok-native-json-availability-02.json) added `--single`, returned exit 0, READY and `end_turn` in 3.799 seconds, with actual usage key `grok-4.6-build` for requested `grok-4.6`. This is empirical request/return mapping, not independently proven checkpoint equivalence. The native JSON contained private reasoning; the public receipt omits it. Installed help explains the failure boundary: positional input opens the interactive route, while `--single`/`--prompt-file` select headless execution. It is not evidence of failed authentication or a broken native JSON format.

The [Opus canary](opus-availability-02.json) returned actual `claude-opus-5`, provider `firstParty`, for requested `opus`; the revised audits pin that returned model. Availability-only canaries establish no source-edit capability or product approval. The root subsequently retained a successful [source-tool smoke](grok-source-smoke.json) and [independent check](grok-source-smoke-root-check.json): actual file edit/new report/approved offline verifier, exit 0 and end_turn, four turns in 28.637 seconds, returned `grok-4.6-build`. The verifier remained unchanged and root reran it successfully. This proves the scoped invocation worked on that fixture, not global filesystem isolation or product acceptance.

## Dispositions and remaining uncertainty

The card describes pretraining through January 2026 plus later supplemental generated data; current model docs label knowledge cutoff February 1. Preserve these distinct descriptions; neither licenses assuming current repository/API facts from memory. The documentation advertises no fixed model output limit while the Responses reference specifies an adjustable 128k default; use the request-specific bound. The API cost guide speaks broadly of exact costs but installed CLI docs explicitly admit unknown/partial OAuth costs; preserve observed CLI metadata over an inferred bill. The older web session-ID wording loses to actual 1.0.13 help. Generic multi-agent prompting belongs to `grok-4.20-multi-agent`, not Grok 4.6 coding; its 4/16-agent controls are not adopted.

The exact CLI model catalog, effective context/compaction settings, account-specific quotas and successful tool execution are runtime facts for the root's bounded canaries, not claims from this document. No exact frozen provider checkpoint, valid financial proof, product result or future completion guarantee follows from model documentation.

## Applicable official document inventory

Every row links directly to the corresponding official source; full captured body and hash live in the corpus/section inventory. All share retrieval date 2026-09-11 UTC. The table below is a selected applicable-source list. All captured bodies, including these reference rows and every omitted section, were subsequently read in full; see the complete reading inventory for individual dispositions and source gaps.

| Document | Inspection scope |
| --- | --- |
| [Headless & Scripting](https://docs.x.ai/build/cli/headless-scripting) | applicable guide inspected |
| [CLI Reference](https://docs.x.ai/build/cli/reference) | applicable guide inspected |
| [Enterprise Deployments](https://docs.x.ai/build/enterprise) | full captured reference body read; relevant distinctions retained |
| [Background Tasks](https://docs.x.ai/build/features/background-tasks) | applicable guide inspected |
| [Hooks](https://docs.x.ai/build/features/hooks) | applicable guide inspected |
| [MCP Servers](https://docs.x.ai/build/features/mcp-servers) | applicable guide inspected |
| [Permissions](https://docs.x.ai/build/features/permissions) | applicable guide inspected |
| [Plan Mode](https://docs.x.ai/build/features/plan-mode) | applicable guide inspected |
| [AGENTS.md](https://docs.x.ai/build/features/project-rules) | applicable guide inspected |
| [Sandbox](https://docs.x.ai/build/features/sandbox) | applicable guide inspected |
| [Sessions](https://docs.x.ai/build/features/sessions) | applicable guide inspected |
| [Skills, Plugins & Marketplaces](https://docs.x.ai/build/features/skills-plugins-marketplaces) | applicable guide inspected |
| [Subagents](https://docs.x.ai/build/features/subagents) | applicable guide inspected |
| [Worktrees](https://docs.x.ai/build/features/worktrees) | applicable guide inspected |
| [Modes and Commands](https://docs.x.ai/build/modes-and-commands) | applicable guide inspected |
| [Grok Build](https://docs.x.ai/build/overview) | applicable guide inspected |
| [Settings](https://docs.x.ai/build/settings) | applicable guide inspected |
| [Reference](https://docs.x.ai/build/settings/reference) | full captured reference body read; relevant distinctions retained |
| [Asynchronous Requests](https://docs.x.ai/developers/advanced-api-usage/async) | applicable guide inspected |
| [Context Compaction](https://docs.x.ai/developers/advanced-api-usage/context-compaction) | applicable guide inspected |
| [Deferred Chat Completions](https://docs.x.ai/developers/advanced-api-usage/deferred-chat-completions) | applicable guide inspected |
| [Prompt Caching](https://docs.x.ai/developers/advanced-api-usage/prompt-caching) | applicable guide inspected |
| [Best Practices & FAQ](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/best-practices) | applicable guide inspected |
| [How It Works](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/how-it-works) | applicable guide inspected |
| [Maximizing Cache Hits](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/maximizing-cache-hits) | applicable guide inspected |
| [What Breaks Caching](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/multi-turn) | applicable guide inspected |
| [Usage & Pricing](https://docs.x.ai/developers/advanced-api-usage/prompt-caching/usage-and-pricing) | applicable guide inspected |
| [WebSocket Mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode) | applicable guide inspected |
| [Microsoft Foundry](https://docs.x.ai/developers/community/microsoft-foundry) | applicable guide inspected |
| [Cost Tracking](https://docs.x.ai/developers/cost-tracking) | applicable guide inspected |
| [Debugging Errors](https://docs.x.ai/developers/debugging) | applicable guide inspected |
| [Security](https://docs.x.ai/developers/faq/security) | applicable guide inspected |
| [Grok 4.6](https://docs.x.ai/developers/grok-4-6) | applicable guide inspected |
| [Comparison with Chat Completions API](https://docs.x.ai/developers/model-capabilities/text/comparison) | applicable guide inspected |
| [Generate Text](https://docs.x.ai/developers/model-capabilities/text/generate-text) | applicable guide inspected |
| [Multi Agent](https://docs.x.ai/developers/model-capabilities/text/multi-agent) | applicable guide inspected |
| [Reasoning](https://docs.x.ai/developers/model-capabilities/text/reasoning) | applicable guide inspected |
| [Streaming](https://docs.x.ai/developers/model-capabilities/text/streaming) | applicable guide inspected |
| [Structured Outputs](https://docs.x.ai/developers/model-capabilities/text/structured-outputs) | applicable guide inspected |
| [Models](https://docs.x.ai/developers/models) | applicable guide inspected |
| [Pricing](https://docs.x.ai/developers/pricing) | applicable guide inspected |
| [Rate Limits](https://docs.x.ai/developers/rate-limits) | applicable guide inspected |
| [Chat Completions](https://docs.x.ai/developers/rest-api-reference/inference/chat-completions) | full captured reference body read; relevant distinctions retained |
| [Models](https://docs.x.ai/developers/rest-api-reference/inference/models) | full captured reference body read; relevant distinctions retained |
| [Responses](https://docs.x.ai/developers/rest-api-reference/inference/responses) | full captured reference body read; relevant distinctions retained |
| [Advanced Usage](https://docs.x.ai/developers/tools/advanced-usage) | applicable guide inspected |
| [Code Execution Tool](https://docs.x.ai/developers/tools/code-execution) | applicable guide inspected |
| [Function Calling](https://docs.x.ai/developers/tools/function-calling) | applicable guide inspected |
| [Overview](https://docs.x.ai/developers/tools/overview) | applicable guide inspected |
| [Remote MCP Tools](https://docs.x.ai/developers/tools/remote-mcp) | applicable guide inspected |
| [Streaming & Synchronous Requests](https://docs.x.ai/developers/tools/streaming-and-sync) | applicable guide inspected |
| [Tool Usage Details](https://docs.x.ai/developers/tools/tool-usage-details) | applicable guide inspected |
| [Web Search](https://docs.x.ai/developers/tools/web-search) | applicable guide inspected |

The root reported a substantive Opus attempt that exited zero but returned only an `ExitPlanMode` fragment. That result is not approval. [REVIEW-PROMPTS.md](REVIEW-PROMPTS.md) requires an actual final verdict and uses `dontAsk` for the self-contained no-tools route; the earlier plan-mode READY canary remains truthful availability-only history. The root’s [corrected substantive Opus receipt](/home/charl/Moriarty/.worktrees/sp03-expression-k-macro05/deliverables/sp03-expression-k-correction-2026-09-10/review-05-opus-correction-01/opus-review.json) subsequently finished in 189.118 seconds, exit zero, with an actual PASS_SCOPED verdict and main model `claude-opus-5` (auxiliary usage retained). Its scope is the identified K source candidate and separately stated resource proposal, not approval of this loop design.
