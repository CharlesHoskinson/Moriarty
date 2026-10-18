# Grok implementation revision 01

Author: GPT-6 Astra at high effort. Status: source-grounded execution design, revised 2026-09-11 UTC (September 10 America/Denver at acquisition). The user's latest instruction selects **Grok 4.6 as implementation author**, with **fresh Claude Opus and fresh GPT-6 Astra medium as independent reviewers**. Root's real Opus availability receipt identifies `claude-opus-5`; pin that model for reviews. Grok uses explicit `grok-4.6` at high effort. This revision supersedes Terra-author/Grok-auditor routing in the retained [original design](../DESIGN.md), original trigger and older repository prose. It preserves their substantive acceptance, historical evidence and resource gates.

## Reuse the running goal

Apply [TRIGGER.txt](TRIGGER.txt) as durable steering for the existing native full-roadmap goal. The parent reports the current native goal already exists and its API only permits status changes, not objective editing. Do not recreate it, falsely complete it, or modify the historical objective merely to change models. Current user steering and this revision control future dispatch. The original design/trigger stay intact. No Grok `/loop`, daemon, timer, new harness or scheduling framework is added. If a future fresh session has no goal, the bounded trigger is suitable for its supported `create_goal` objective; creation must be explicit and observed, never inferred from a file.

The next implementation target is a **source and offline-test loan exit-retention repair**. Revalidate the current checkout and `/tmp/moriarty-sp05-next-terra-followup.md` as a preserved task input; its old author label and recorded HEAD do not control today's routing or actual HEAD. Inspect the previous loan's missing exit observation and the successful swap mechanism. Grok implements the smallest source candidate that retains loaded/exited process evidence before stop and rejects unloaded/default/mismatched/post-stop observations. `FINANCIAL_COMPLETE` cannot establish raw main exit zero. No service, K/native/prover, wallet or public run is part of this source slice. The preserved K candidate `ed7a14687bdf14977d25a17fc29b3a9e4c620faf` remains historical Terra work under separate audits; do not relabel its author or overwrite it.

## Why the prompt is structured this way

[SOURCES.md](SOURCES.md) records the exact model card, complete official corpus inventory and installed CLI observations. [ALL-DOCS-READING.md](ALL-DOCS-READING.md) and its per-section inventory confirm actual reading of all 181 captured bodies and the entire 42-page card, with explicit gaps for content absent from the corpus. Grok is suited to tool-assisted multi-step coding, but its provider's benchmark results do not promise full task closure. The worker therefore receives a concrete public behavior, owned paths, independent acceptance examples, current failure evidence and executable allowed checks. It must act, reproduce, patch and demonstrate, then leave broader gates open. This is a design inference from the documentation and repository postmortems, not an official model-specific magic prompt.

Keep a stable short instruction prefix and append the task delta. Link current artifacts instead of pasting the entire research corpus into every worker. Grok's 500k model context is capacity, not a reason to fill it. Preserve an exact session ID for a continuing author, compact using the CLI's existing mechanism when appropriate, and reload current state after compaction. Fresh reviewers receive their own new contexts and no author conversation or other review verdict before their initial verdict.

The [exact-4.6 reasoning guide](https://docs.x.ai/developers/model-capabilities/text/reasoning) supports high; no extra “think forever” instruction, private-reasoning export, xhigh escalation or invented sampling default is required. API compaction, caching and output fields do not become CLI switches. In particular, the [Responses reference](https://docs.x.ai/developers/rest-api-reference/inference/responses) says automatic `context_management` directives are not executed. Use existing CLI behavior rather than building an API orchestrator.

## Concrete installed CLI contract

Use **`--prompt-file` or `--single`**, not a positional prompt. Installed help calls positional input interactive. Root's [first native-JSON canary](grok-native-json-availability.json) failed with a device error; the [corrected `--single` canary](grok-native-json-availability-02.json) returned exit zero, `READY`, terminal `end_turn` in 3.799 seconds. That isolates the headless-entry correction; it does not establish a successful coding run. Requested `grok-4.6` returned usage identity `grok-4.6-build`; keep both exact strings. No inspected official source pins that internal suffix to an immutable checkpoint.

Installed 1.0.13 help and local headless docs support the following **template**, to be filled with real current paths, existing permission/resource admission and a justified turn limit before launch. These are not default resource grants:

```text
grok --cwd <existing-isolated-checkout>
  --model grok-4.6 --reasoning-effort high
  --prompt-file <filled-GROK-WORKER-PROMPT>
  --no-plan --no-subagents --disable-web-search
  --tools read_file,grep,list_dir,search_replace,write_file,run_terminal_cmd
  --permission-mode dontAsk
  --allow 'Read(<needed-source-path-glob>)'
  --allow 'Grep(<needed-source-path-glob>)'
  --allow 'Edit(<owned-path-glob>)' --allow 'Write(<owned-path-glob>)'
  --allow 'Bash(<exact-reviewed-offline-check-command>)'
  --deny MCPTool --deny WebSearch --deny WebFetch
  --max-turns <admitted-main-agent-turns>
  --output-format json
```

Repeat scoped rules for actual paths/commands, including guarded startup status/next if the worker runs them. `run_terminal_cmd` is the documented internal shell ID; `Bash(...)` is its permission class. `write_file` is named in installed configuration and filesystem docs; the root verified these IDs and scoped rules with an actual harmless read/edit/new-file/offline-check fixture; retain the same scoped pattern and revalidate task-specific paths/commands. The [source smoke](grok-source-smoke.json) completed in 28.637 seconds/four turns with exit 0 and end_turn; [root verification](grok-source-smoke-root-check.json) confirms changed sample, new report and unchanged passing verifier. A configured tool in another scope can still be unavailable. Do not create broad shell grants to repair that uncertainty. `--tools` does not remove all MCP meta-tools; explicit MCP denial remains useful. A catch-all `--deny Bash` would override allowed shell commands, so do not combine them as an allowlist. Unmatched actions in `dontAsk` fail rather than waiting for input; inspect a denial and correct the scope only when already authorized.

JSON is the root's actually observed format. Native `streaming-json` is documented and preferable for tool progress **once the existing supervisor's parser is verified on it**; it has `type: tool_call/tool_call_update/usage/end/error`, not the older Messages `result` schema. Do not pass native events through a parser that expects Messages fields. No new orchestration framework is needed to use the existing verified capture path. A `--max-turns` limit is not a bound on total tool calls, subprocess lifetime, tokens or money.

The root retains actual non-private output, stderr, process exit, session/request IDs, elapsed time, usage/incompleteness and requested/returned model identity. Exclude `thought`/thinking blocks and private witness/secrets from repository evidence. Native JSON may be silent until termination; quiet stdout is not a hang. Use actual process/tool activity and the admitted deadline. If using streaming, retain the terminal `end` with `stopReason: end_turn` and the substantive author report. A `max_tokens`, `max_turn_requests`, cancellation, error or timeout is incomplete work, not success. Preserve useful files and charges before bounded recovery.

Reuse exact `--resume <captured-id>` only for the same author/task when useful; never `--restore-code`, `--continue` against an ambiguous recent session, or a reused `--session-id`. Headless `--worktree` does not create isolation: the orchestrator must create/inspect the checkout before launch. Keep the existing kernel/process/cgroup/resource containment. Permission rules and prompt text cannot prove offline isolation, enforce every child action, or substitute for the guarded campaign dispatcher. Source-inspect every allowed test command because a permissive script can perform more than its name says. Heavy/public actions remain root-only, separately admitted operations.

## Review and continuation

Use [GROK-WORKER-PROMPT.md](GROK-WORKER-PROMPT.md) for implementation and [REVIEW-PROMPTS.md](REVIEW-PROMPTS.md) for both fresh result audits. Root's [Opus canary](opus-availability-02.json) returned `claude-opus-5`/firstParty; this is availability evidence, not approval. Both actual candidate reviews must complete substantively, identify exact bytes and inspect the production predicate. Use the root’s corrected `dontAsk` no-tools route for a self-contained Opus review packet; plan mode produced an `ExitPlanMode` fragment without a verdict in one actual source audit. Exit zero and valid JSON alone are not review approval. Supply complete required source and contract contents when tools are disabled, or explicitly admit read-only tools when source inspection requires them. Independent reviewers do not share initial opinions, approve their own changes or inherit the author's session. Fix findings through Grok, recompute hashes/checks and obtain current reviews; preserve originals.

The parent owns integration, publication, transaction notifications, campaign resources and the current goal. After an accepted slice it immediately selects another eligible capability. Historical SP01.6 admission trouble blocks only dependent campaigns. Missing heavy/native/network admission never licenses bypass and never blocks unrelated authorized source work. When a design/resource decision changes, use the standing two-agreeing-substantive-vote authority with the current participating models and preserved dissent; result review still requires both non-author reviewers.

All original stop rules remain: reproduce/change approach after two same-defect failed cycles; after two process-only cycles or thirty administration minutes, return to executable diagnosis/implementation. Preserve total historical charges and limits; justified amendments precede new spending. Complete only after all SP01–SP12/MC01–MC08/G01–G24 and adopted financial/asset/report predicates, final independent audits and publication. Mark the host goal blocked only after the same genuine blocker persists for three consecutive goal turns and every meaningful independent path is exhausted. Record actual liveness, not a saved prompt's existence.

## Verification scope

This revision changes research/design artifacts only. The source inventory is reproducible from the captured official corpus and local documentation hashes. The trigger is checked against the native 4,000-character limit. Root's canaries are cited with their actual scope. The design agent performed no product source edit, K/native/prover/service/public dispatch, goal mutation or external model call. Adoption and actual product-writing liveness remain root-owned evidence; the disposable source-writing canary passed with its stated narrow scope.
