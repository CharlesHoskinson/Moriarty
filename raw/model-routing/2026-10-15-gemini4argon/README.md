# Gemini 4 Argon through AGY — source receipt

Retrieved 2026-10-15 at the user's request. These are provider facts and
transport observations, not Moriarty correctness or security acceptance.

- [Official model card](https://deepmind.google/models/model-cards/gemini-4-argon/),
  published 2026-10-13: up to 1M combined input/output tokens; text output;
  text/image/audio/video inputs; software engineering and cybersecurity defense
  are stated uses. The card lists a March 2026 knowledge cutoff and possible
  hallucinations, latency and timeouts. Supply current pinned source and evidence.
- [AGY headless documentation](https://www.antigravity.google/docs/cli/headless/):
  use `agy models`, select the exact slug and use print mode for a bounded job.
- [AGY model documentation](https://www.antigravity.google/docs/models/): Argon
  is available through Antigravity CLI. Do not infer other-provider availability.

`receipts.json` binds requested/canonical URLs, retrieval times, status and
content digests. `.html.gz` files retain exact response-body bytes compressed
with deterministic metadata; `.txt` files are separate main-element extracts.
Acquisition used Scrapling 0.4.15, public requests and checked robots rules;
no authentication cookies were retained. Existing static-document extraction
patterns sufficed; no new browser or site-bypass pattern was needed.

`agy-model-list.txt` is the fresh authenticated list. The requested high-effort
slug is **`gemini-4-argon-high`**; no separate effort flag is needed.
`agy-readiness.json` retains the actual successful `ARGON_READY` response and
final usage/status. It does not constitute a review. The observed AGY JSON
did not expose a separate returned model ID, so the receipt records the exact
requested catalogue selection without inventing canonical-ID metadata.

The readiness-only invocation combined `--mode plan` with
`--disable-slash-commands`; AGY warned that this disables the mode effect.
Subsequent source/security jobs use plan mode without that flag. The canary
requested and performed no tools. Keep command/result scope explicit.

The [user assignment](../../assignments/moriarty-argon-temporary-security-2026-10-15.md)
makes this a temporary replacement for Opus-owned work. GPT-6.1 Sol high
implementation and Muse Wonder1.0 high nonsecurity audits remain in place.
Native authority, historical counters and all language acceptance gates remain
unchanged. Restore Opus after availability is observed; preserve Argon authorship
and audit identity on every result it produces.
