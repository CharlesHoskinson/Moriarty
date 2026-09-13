# Reviewed source findings

Each statement below is a source fact from the local source named in
parentheses. These notes do not report a local model probe or establish account
availability.

## Gemini 3.8 Flash

- The official name is Gemini 3.8 Flash and the stable model code is
  `gemini-3.8-flash`. Google describes it as generally available and ready for
  production. ([model card](sources/google-gemini-38-model-card.txt),
  [guide](sources/google-gemini-38-guide.txt))
- Its documented limits are 1,048,576 input tokens and 65,536 output tokens.
  Supported inputs are text, image, video, audio, and PDF; output is text.
  ([model card](sources/google-gemini-38-model-card.txt))
- Its card lists caching, code execution, preview computer use, file search,
  function calling, Google Maps grounding, Search grounding, structured
  outputs, URL context, Batch, Flex, and Priority inference. It lists audio and
  image generation and Live API as unsupported. ([model card](sources/google-gemini-38-model-card.txt))
- Thinking defaults to medium and permits `low`, `medium`, and `high`;
  `minimal` is rejected for this model. The thinking guide says thought
  summaries can be enabled and that output-token limits apply to the combined
  thought and final-output token count. ([thinking guide](sources/google-gemini-thinking.txt))
- AGY’s official headless CLI guide says to inspect choices with `agy models`
  and pin a returned slug with `--model`; it shows 3.8 Flash high and medium
  slugs. An unknown model exits nonzero rather than silently falling back.
  ([AGY guide](sources/agy-headless-model-selection.txt))

## Claude Opus 5

- Anthropic identifies Claude Opus 5 with the API string `claude-opus-5`.
  Its product page says the model is available natively on the Claude Platform
  and its source states a starting price of $5 per million input tokens and $25
  per million output tokens. ([product page](sources/anthropic-opus-product.txt))
- The captured system card calls it an upgrade to Opus 4.8, records a May 2026
  knowledge cutoff, and assesses it across software engineering, scientific and
  mathematical reasoning, long context, agentic search, multi-agent
  orchestration, multimodal and computer-use tasks, and professional work.
  ([full system card](sources/anthropic-opus-5-system-card.txt))
- Anthropic’s prompting guidance says Opus 5 has thinking enabled by default
  when omitted. It may be disabled only at `high` effort or lower; it advises
  using adaptive thinking and effort settings instead of the older manual
  `budget_tokens` mechanism for newer models. ([prompting guide](sources/anthropic-prompting.txt))
- Anthropic’s tool-use guide gives `claude-opus-5` examples. It distinguishes
  client tools, whose calls the application executes and returns as
  `tool_result`, from Anthropic server tools such as web search, code execution,
  and tool search. ([tool-use guide](sources/anthropic-tool-use.txt))
- Claude Code documents `--model` for the current session, accepting aliases or
  a full model name. Its `--effort` options are model-dependent. ([CLI reference](sources/anthropic-claude-code-cli.txt))

## GPT-6 Astra and the requested “GPT 6” label

- The official OpenAI material calls the model GPT-6 Astra and gives the model
  ID `gpt-6-astra`. The card lists a 1,050,000-token context window, 128,000
  maximum output tokens, and reasoning efforts `low`, `medium`, `high`,
  `xhigh`, and `max`. ([model card](sources/openai-gpt6-astra-model.txt))
- The card lists Responses-API support for web search, file search, image
  generation, code interpreter, hosted shell, apply patch, skills, computer use,
  MCP, and tool search. Fine-tuning is listed as unsupported. ([model card](sources/openai-gpt6-astra-model.txt))
- The model guide says to select `gpt-6-astra`, use the Responses API for tool
  calling, and remove `temperature`, `top_p`, and `top_logprobs`. It says Astra
  does not support `none` reasoning effort and describes changing effort with a
  `configuration_update` item. ([model guidance](sources/openai-gpt6-astra-guidance.txt))
- The OpenAI CLI reference uses `openai models retrieve --model gpt-6-astra` as
  its model-identifier example. ([CLI reference](sources/openai-cli-model-retrieve.txt))

## Limitations and provenance

- These are vendor documentation claims, not independent performance,
  reliability, safety, or availability findings.
- OpenAI has no separate GPT-6 system card in the bounded captured set. The
  source used is its official model card and model guidance. Google’s
  model-specific page is titled as a model card; Anthropic provides a separate
  full system card.
- Raw public page/PDF receipts, status codes, canonical URLs, and hashes are in
  [the manifest](MANIFEST.json). The collection method is described in
  [the source-coverage report](SOURCE-COVERAGE.md).
