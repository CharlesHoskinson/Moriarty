# Official model-documentation coverage

Collection times: 2026-09-12T15:51:54Z and 2026-09-12T17:29:46Z. The collector
made only public, unauthenticated requests with Scrapling Fetcher 0.4.15 after
checking the Google, AGY, Anthropic, OpenAI, and xAI robots files. `MANIFEST.json` records the
requested and canonical URL, HTTP status, retrieval time, raw-content SHA-256,
and text-extract SHA-256 for every capture.

| Requested label | Official display name | Official model identifier | Coverage |
| --- | --- | --- | --- |
| Gemini Flash 3.8 | Gemini 3.8 Flash | `gemini-3.8-flash` | Model-specific card, release/migration guide, thinking, function calling, and AGY CLI selection |
| Claude Opus 5 | Claude Opus 5 | `claude-opus-5` | Product and release pages, full system card, prompting, tool use, and Claude Code CLI selection |
| GPT 6 | GPT-6 Astra | `gpt-6-astra` | OpenAI model card, model guidance, and OpenAI CLI identifier reference |
| Grok 4.6 | Grok 4.6 | `grok-4.6` | API guide, model page, reasoning guide, and official model-card PDF with safety sections |

The third row is a supplied-family mapping. OpenAI’s captured documentation
uses the distinct display name **GPT-6 Astra** and model identifier
`gpt-6-astra`; it does not establish an official standalone model name or ID
spelled exactly `GPT 6`. The corpus does not silently rename that distinction.

All 20 saved official documents returned HTTP 200. The full Opus 5 system-card
PDF and full Grok 4.6 Model Card PDF, with complete `pdftotext -layout`
extractions, are retained. The graph
input should use the `sources/*.txt` extracts; corresponding `.html` or `.pdf`
files are immutable provenance captures.

The bounded Grok collection establishes `grok-4.6` as the documented request
ID. It does not establish an alias-to-snapshot mapping or prove the runtime
identity selected by a particular CLI or gateway. `grok-reading-notes.md`
separates the documented ID from Grok Build and provider-surface labels.

No credentials, cookies, authorization headers, account data, or model probes
were collected by this source-acquisition task.
