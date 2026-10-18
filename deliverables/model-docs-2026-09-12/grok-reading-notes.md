# Grok 4.6 reading notes

## Coverage

Read in full on 2026-09-12: the [Grok 4.6 API guide](https://docs.x.ai/developers/grok-4-6), [Grok 4.6 model page](https://docs.x.ai/developers/models/grok-4.6), [reasoning guide](https://docs.x.ai/developers/model-capabilities/text/reasoning), and the official [Grok 4.6 Model Card PDF](https://media.x.ai/v1/website/card-4p6-4cd2dc57.pdf). The four captures are retained with raw files, extracted text, per-file metadata, and SHA-256s in `sources/`; their corpus wrappers preserve capture provenance.

The exact documented request ID is `grok-4.6`. The API guide shows it in Python, OpenAI-compatible Responses, and curl examples; the model page repeats that exact name. [API guide, lines 12–18 and 42–47](corpus/xai-grok-46-api-guide.md#L12-L47); [model page, lines 7–11 and 51–63](corpus/xai-grok-46-model-card.md#L7-L63).

No snapshot identifier or alias-to-snapshot mapping appears in this bounded four-source corpus. `Grok Build` is a named product surface: xAI calls Grok 4.6 its default coding-agent model on the API and CLI, but that statement does not supply a response-level runtime identity. An orchestrator should send `grok-4.6` when it needs that documented model, record the model identity returned by its actual provider or CLI, and treat any product label, gateway alias, or CLI default as separate from proof of the runtime resolution. This last operational rule is an inference from the sources, not a vendor guarantee. [API guide, lines 201–212](corpus/xai-grok-46-api-guide.md#L201-L212); [Model Card, lines 168–182](corpus/xai-grok-46-safety-model-card.md#L168-L182).

## Practical orchestration findings

- The documented capacity is a 500,000-token context, text/image input and text output, with Responses API and Chat Completions support. It lists function calling, web search, X search, and code execution. Use the exact API/tool surface required by a task; do not assume that a third-party CLI exposes every listed capability. [API guide, lines 148–178](corpus/xai-grok-46-api-guide.md#L148-L178).

- Pin `reasoning_effort` in an agent profile. It defaults to `high`, cannot be disabled, and accepts `low`, `medium`, `high`, and `xhigh` for Grok 4.6. `xhigh` trades response time for maximum reasoning depth. The same field means agent count for `grok-4.20-multi-agent`, so configuration must be model-specific. [Reasoning guide, lines 34–80 and 311–357](corpus/xai-grok-reasoning.md#L34-L80).

- Reject or strip `presencePenalty`, `frequencyPenalty`, and `stop` before sending a Grok reasoning request: xAI says they produce an error with reasoning models. Capture `reasoning_tokens` when usage telemetry is available. This corpus has no local run or usage measurement, so usage values remain unknown. [Reasoning guide, lines 19–24 and 43–53](corpus/xai-grok-reasoning.md#L19-L53).

- For repeated turns, set `prompt_cache_key` with the Responses API or `x-grok-conv-id` with Chat Completions. xAI says the value routes requests to the same server for reliable cache hits; long agent loops also benefit from context compaction. The model page lists a separate cached-input price and says requests above 200K context receive different rates. [API guide, lines 187–199](corpus/xai-grok-46-api-guide.md#L187-L199); [model page, lines 32–52](corpus/xai-grok-46-model-card.md#L32-L52).

- Do not plan Batch API work for this model: its model page lists Batch API as unsupported. Its listed rate limits are 150 requests/second and 50,000,000 tokens/minute for the page's region; actual account limits need a live check. [Model page, lines 53–68](corpus/xai-grok-46-model-card.md#L53-L68).

- The model card says its evaluations normally use the final deployed checkpoint unless stated otherwise, and says xAI does not silently downgrade or fall back to other models. Treat both as provider statements about that card's evaluation/deployment context, not as a substitute for recording a CLI or gateway's returned runtime identity. [Model Card, lines 132–152](corpus/xai-grok-46-safety-model-card.md#L132-L152).

- Preserve human approval and domain validation for high-stakes outputs. The model card says Grok 4.6 is not intended for autonomous high-stakes decisions in medicine, law, finance, or safety-critical systems without human oversight and domain-expert validation. It also describes layered safety tuning, system prompts, and, on some surfaces, runtime input/topical filters; behaviors can therefore depend on the serving surface. [Model Card, lines 150–152 and 1490–1507](corpus/xai-grok-46-safety-model-card.md#L150-L152).

## Limits

This is an official, bounded source set. It contains an official model card with safety sections, but no separately named Grok 4.6 system card was identified and captured. It does not establish a generic alias policy, a particular third-party gateway mapping, or a local CLI probe result. The performance figures in the model card are vendor-reported evaluation results under named harnesses and efforts; they are not production reliability or usage measurements.
