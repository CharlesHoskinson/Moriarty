# Google / AGY source reading notes

All six assigned captures were read in full, including code examples and page footers. Capture time is `2026-09-12T15:51:54+00:00` throughout. Findings describe the supplied documentation snapshot; no API request, benchmark, installed-model enumeration or AGY execution was performed.

| Source and upstream URL | Complete-read coverage | Scope |
|---|---:|---|
| [agy-headless-model-selection.md](corpus/agy-headless-model-selection.md), [AGY headless](https://www.agy.dev/docs/cli/headless/) | Lines 1–1936 | CLI protocol, permissions, model selection and usage |
| [google-gemini-38-guide.md](corpus/google-gemini-38-guide.md), [Latest model guide](https://ai.google.dev/gemini-api/docs/latest-model) | Lines 1–753 | Exact Gemini 3.8 Flash features and migration |
| [google-gemini-38-model-card.md](corpus/google-gemini-38-model-card.md), [Model specifications](https://ai.google.dev/gemini-api/docs/models/gemini-3.8-flash) | Lines 1–102 | Exact Gemini 3.8 Flash capability/limit table |
| [google-gemini-function-calling.md](corpus/google-gemini-function-calling.md), [Function calling](https://ai.google.dev/gemini-api/docs/function-calling) | Lines 1–8601 | Generic API mechanics, mostly Interactions examples |
| [google-gemini-models.md](corpus/google-gemini-models.md), [Model catalog](https://ai.google.dev/gemini-api/docs/models) | Lines 1–340 | Catalog and version naming |
| [google-gemini-thinking.md](corpus/google-gemini-thinking.md), [Thinking](https://ai.google.dev/gemini-api/docs/thinking) | Lines 1–1700 | Generic reasoning/state mechanics plus model-specific effort table |

Total coverage: **13,432 lines**. Where tool output was truncated, the missing ranges were subsequently reread. This is complete reading of the supplied extracted text, not a claim that every linked page, original HTML element or linked formal model-card report was acquired.

## Practical findings

1. **Keep three identities separate.** The API model is `gemini-3.8-flash`, documented as stable/GA. AGY lists `gemini-3.8-flash-high` and `gemini-3.8-flash-medium`; use `agy models` to discover selectable installed slugs. The managed agent is `antigravity-preview-05-2026`, with a configurable underlying model. Labels and defaults do not prove which API model an actual CLI run used. Sources: guide lines 16–39, 463–466, 661–671; AGY lines 1589–1625; models lines 86–89, 240–242.

2. **Use exact-model thinking controls.** Gemini 3.8 supports `low`, `medium` and `high`, defaults to `medium`, and rejects `minimal`. The guide positions medium for most code/agent work and high for difficult reasoning; generic advice recommending minimal for simple tasks does not apply to 3.8. Sources: guide lines 248–258; card lines 64–68; thinking lines 1184–1201, 1671–1682.

3. **Migrate the request shape.** The 3.8 checklist removes `temperature`, `top_p`, `top_k` and `candidate_count`, replaces `thinking_budget` with `thinking_level`, removes prefilled model turns and recommends server-side `previous_interaction_id`. It places multimodal function assets within response payloads, calls for double-newline inline instructions, and requires `call_id` and `name` on `FunctionResponse` specifically when using generateContent. Source: guide lines 680–727.

4. **Budget thinking and output together.** Exact model caps are 1,048,576 input tokens and 65,536 output tokens. The thinking guide says `max_output_tokens` includes thoughts and is a hard cutoff, which can produce incomplete or empty output while charging for generated thoughts. Lower thinking effort to reduce cost/latency without forcing that cutoff. Billing uses full thoughts plus output, not summary length. Sources: card lines 34–39; thinking lines 1441–1466, 1491–1496, 1653–1667.

5. **Preserve the correct state representation.** Interactions uses dedicated thought steps; thought signatures are always present, summaries optional. Stateful `store=true` plus `previous_interaction_id` preserves state on the server. Stateless callers must resend all thought blocks exactly, including across model changes, and retain built-in-tool signatures. generateContent stores signatures on different parts; its handling differs from Interactions. Sources: thinking lines 21–56, 659–668, 1467–1490; function calling lines 3473–3486.

6. **The application executes custom tools.** Function declarations describe names, parameters and purpose; model calls do not themselves execute those functions. Return an Interactions `function_result` containing matching `name`, `call_id` and typed result blocks. Independent calls can run in parallel; dependent calls need composition through returned results. Assemble streamed argument deltas into complete calls before execution. Sources: function calling lines 2078–2100, 3133–3200, 4226–4253, 4940–4941, 7449–7455.

7. **Constrain and validate tool calls.** `tool_choice` supports auto/any/none/validated and allowed-tool restrictions. The generic guide recommends 10–20 active tools, explicit types and descriptions, pre-execution validation and error handling. Large or deeply nested schemas can be rejected; only an OpenAPI subset is supported. Sources: function calling lines 5597–5630, 8413–8429, 8583–8590.

8. **Avoid fragile structured pre-tool narration.** Required XML/YAML/JSON immediately before a call can trigger `Malformed_Function_Call`. The documented preferred workaround is an `update` function for working notes; alternatives use Markdown headings or remove the requirement. This is a conditional workaround, not proof every existing prompt needs changing. Sources: function calling lines 8430–8449, 8501–8582; guide lines 713–717.

9. **Respect capability and transport boundaries.** 3.8 accepts text/image/video/audio/PDF but outputs text; image/audio generation and Live API are unsupported. Computer use is Preview. Generic Interactions supports combined built-in/custom tools, multimodal result blocks and remote MCP using Streamable HTTP, excluding SSE server transport; MCP names cannot contain hyphens. Sources: card lines 28–70; function calling lines 5895–5900, 6584–6599, 7085–7125.

10. **Treat AGY status and telemetry precisely.** Unknown pinned models fail nonzero; approval-dependent tools can instead be soft-denied while the run exits 0. Check intended effects and tool/error events alongside response/status. `init.model` appears only when overridden. Streaming `result.response` is per-turn, while usage, turns and duration are cumulative; summing all result usages double-counts. Sources: AGY lines 594–630, 748–775, 1284–1294, 1623–1672, 1763–1794.

11. **Follow AGY's persistent input protocol.** Stream input requires stream output, text-only `user` messages and one result per turn. Wait for the current result before sending another prompt; close stdin to end. CLI `/model` and `/usage` are unavailable inside this stream, and `-p` prompts are dropped in this mode. Source: AGY lines 862–959, 1321–1330, 1427–1486, 1541–1588.

12. **Pin versions and timestamp prices.** Generic `latest` aliases hot-swap; stable identifiers are recommended for most production apps. The captured 3.8 guide lists introductory $0.75/$3.75 per million input/output tokens through 2026-12-31 and $1.50/$7.50 from 2027-01-01. These are documentation claims, not measured task costs. Sources: models lines 282–325; guide lines 243–247.

## Evidence limitations and graph treatment

The vendor's improved coding, agent reliability and factual-rigor statements are positioning claims, not independently measured results. The guide itself acknowledges potentially higher token consumption from more iterative verification (guide lines 229–247).

Some documentation examples do not demonstrate their section's claimed behavior. Several Java function-calling examples use `gemini-3.6-flash` and repeat generic `custom_function` snippets (for example function calling lines 651–664, 2377–2392 and 3370–3383). The 3.8 guide's Java thinking/managed-agent snippets also resemble its basic quickstart instead of expressing the corresponding settings (guide lines 354–439, 559–644). Treat prose, exact-model tables and applicable request examples separately; examples were read, not executed.

The graph fragment contains 75 nodes, 102 edges and 3 hyperedges. Node provenance copies the originating URL and capture time; author/contributor are absent in frontmatter and remain null. IDs use stems relative to the parent-confirmed corpus scan root; source paths remain absolute. The CLI-slug/API-ID correspondence is explicitly ambiguous as runtime identity evidence. Generic and exact-model concepts remain separately attributable. Actual extraction token usage is unavailable and recorded as null, never inferred from sample telemetry or corpus size.
