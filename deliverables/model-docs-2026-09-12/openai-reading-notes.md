# OpenAI corpus reading notes

Complete-read coverage: all three assigned corpus wrappers were read in full:
`corpus/openai-cli-model-retrieve.md` (135 lines),
`corpus/openai-gpt6-astra-guidance.md` (223 lines), and
`corpus/openai-gpt6-astra-model.md` (176 lines).

- Select the documented model with `gpt-6-astra`. The OpenAI CLI reference
  shows `openai models retrieve --model gpt-6-astra`; it returns identifier,
  creation time, owner, and optional shutdown date. [CLI source](https://developers.openai.com/api/reference/cli/resources/models/methods/retrieve), lines 18-67 and 82-112.
- The model card documents a 1,050,000-token context window, 128,000 maximum
  output tokens, reasoning efforts from `low` through `max`, image input, text
  output, and Responses-API support for web/file search, image generation, code
  interpreter, hosted shell, apply patch, skills, computer use, MCP, and tool
  search. [Model card](https://developers.openai.com/api/docs/models/gpt-6-astra), lines 29-48 and 113-143.
- For tool calls, OpenAI’s model guide directs GPT-6 Astra workloads to the
  Responses API. It describes asynchronous custom/function tools with `async:
  true` and returning a result against the original `call_id`. [Model guidance](https://developers.openai.com/api/docs/guides/latest-model?model=gpt-6-astra), lines 27-42 and 170-173.
- To change reasoning effort during a standard single-agent conversation while
  preserving the prompt prefix for caching, use a `configuration_update` input
  item and keep request-level `reasoning.effort` unchanged. `none` is not a
  supported effort. [Model guidance](https://developers.openai.com/api/docs/guides/latest-model?model=gpt-6-astra), lines 47-54, 61-66, and 196-203.
- Migration guidance says to remove `temperature`, `top_p`, and `top_logprobs`;
  it also distinguishes `reasoning.effort` in Responses from
  `reasoning_effort` in Chat Completions. EU data residency does not support
  the listed fast/priority service tiers. [Model guidance](https://developers.openai.com/api/docs/guides/latest-model?model=gpt-6-astra), lines 150-195.
- The guide recommends making task completion, instruction priority, writing
  style, delegation expectations, and testing scope explicit in prompts where a
  harness needs those behaviors. [Model guidance](https://developers.openai.com/api/docs/guides/latest-model?model=gpt-6-astra), lines 86-140.

These notes report only the captured documentation. They do not establish local
CLI access, account entitlement, live availability, or usage metrics.
