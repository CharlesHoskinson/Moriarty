# Bounded Grok timeout diagnosis

Two bounded calls tested the same small absolute-deadline counterexample with requested `grok-4.6`, high effort, no tools and streaming response events. Both gave the correct answer. No wallet, chain, proof or financial operation occurred.

| Observation | Existing system prompt | Explicit static-review system prompt |
| --- | ---: | ---: |
| Elapsed seconds | 28.953 | 33.790 |
| Reported input tokens | 12,140 | 12,444 |
| Reported output tokens | 1,423 | 1,165 |
| Terminal exit | 0 | 0 |

[Baseline observation](observation.json), [timing summary](timing-summary.json), [one-variable comparison](override-observation.json). The baseline emitted its first thinking event at 6.007s and first answer-text event at 26.446s. Public observations retain event kinds, times, lengths, final answers and provider usage; private deliberation remains outside Git. Initial zero-valued usage events are placeholders; use the terminal usage record, not a sum of repeated cumulative events. The model metadata reported `grok-4.6` for these diagnostic calls.

What this establishes: the selected client/model route can complete a small substantive review, and streaming exposes progress before the final response. Buffered JSON alone concealed that distinction in the prior timed-out attempts. The default-role override did not reduce observed latency or input tokens in this one comparison, so it is not an adopted fix. One sample is not a performance distribution.

[Configuration discovery](discovered-context.json) found global instructions, 119 skills, 17 agents, five hooks and five plugins even from `/tmp`; Goldbach appears among the configured plugins. This is an observed configuration-relevance problem to investigate, not proof that those components ran during the timed-out request or caused its delay. No global settings were changed.

Root cause of the larger source-review timeouts remains unresolved. The proposed replay has now run: [unchanged section A with streaming](source-A-start.json) emitted 44 events between 5.720 and 177.194 seconds, but supplied no final answer before local termination at 180.214 seconds ([observation](source-A-observation.json)). Streaming revealed continued provider output without yielding a review verdict. It therefore does not fix the source-review failure. No broad queue resumed. Upstream cancellation and missing usage for terminated calls remain unverified. These diagnostic answers are not approvals of the full launcher or its execution allocation.

The separate [corrected local execution proposal](../sp05-financial-integration-2026-09-09/local-execution-02/commands.md) fixes the known late-launch deadline gap and has [GPT-6 correction approval](../sp05-financial-integration-2026-09-09/local-execution-02/correction-vote-gpt6.json). Required Grok source/resource reviews and live execution admission remain open.
