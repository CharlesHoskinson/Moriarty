[Skip to content](#astryx-app-shell-main)

[Meta Model API](/docs/overview)

Search

[Dashboard](/platform)

Start here

[Overview](/docs/overview)

[Quickstart](/docs/quickstart)

[Authentication](/docs/authentication)

[Models](/docs/models)

[Pricing and rate limits](/docs/pricing-rate-limits)

[SDKs and libraries](/docs/sdks)

Capabilities

[Tool calling](/docs/tool-calling)

[Tool search](/docs/tool-search)

[Search grounding](/docs/search-grounding)

[Image understanding](/docs/image-understanding)

[Image generation](/docs/image-generation)

[Speech to text](/docs/speech-to-text)

[Video understanding](/docs/video-understanding)

[Media segmentation](/docs/media-segmentation)

[File handling](/docs/file-handling)

[Reasoning](/docs/reasoning)

[Structured output](/docs/structured-output)

[Prompt caching](/docs/prompt-caching)

[Token counting](/docs/token-counting)

Protocols

[Choosing an API](/docs/protocols)

[Responses API](/docs/protocols/responses)

[Chat Completions API](/docs/protocols/chat-completions)

[Messages API](/docs/protocols/messages)

Cookbook

[All recipes](/docs/cookbook)

[API fundamentals](/docs/cookbook/api-fundamentals)

[Agent patterns](/docs/cookbook/agent-patterns)

[Use cases](/docs/cookbook/use-cases)

[Building with Muse Code](/docs/cookbook/muse-code)

[Muse Image](/docs/cookbook/muse-image)

[Muse Voice Transcribe](/docs/cookbook/muse-voice-transcribe)

Muse Code

[Overview](/docs/muse-code)

[Authentication and billing](/docs/muse-code/auth)

[Subscriptions](/docs/muse-code/subscriptions)

[Permissions and safety](/docs/muse-code/permissions)

[Working with the agent](/docs/muse-code/interactive)

[Workflows](/docs/muse-code/workflows)

[Session messaging](/docs/muse-code/session-messaging)

[Rewind a conversation](/docs/muse-code/rewind)

[Configuration and context](/docs/muse-code/configuration)

[Extending and automating](/docs/muse-code/extending)

[Changelog](/docs/muse-code/changelog)

Open-weight models

[Overview](/docs/open-weight-models)

Muse Glimmer

Agent guides

[Coding agents](/docs/coding-agents)

[Agent frameworks](/docs/agent-frameworks)

[Computer use](/docs/computer-use)

SAM

[Overview](/docs/sam/overview)

[Segmenting with prompts](/docs/sam/segmenting)

[Reading segmentation output](/docs/sam/reading-segmentation)

[Client libraries](/docs/sam/client-libraries)

API reference

[Introduction](/docs/api-reference)

[Error handling](/docs/error-handling)

Responses

Chat completions

Messages

Files

Images

Voice

Models

[Status](/docs/api-reference/status)

[Help Center](/help)

[Configuration and context](#configuration-and-context)
=======================================================

Table of contents

Set Muse Code up once for your machine and once per project, and give the agent the durable project knowledge it needs. This way, you don't repeat yourself every session. User settings live in a JSON file. Project instructions and memory travel with the repository.

[The settings file](#settings-file)
-----------------------------------

User settings live at `~/.config/muse/settings.json`. The file holds:

* model defaults
* terminal-UI preferences, including [voice](/docs/muse-code/interactive#voice), [reasoning display](/docs/muse-code/interactive#reasoning), and the terminal theme (`tui.terminal_background`, set with `/theme`)
* the default [permission profile](/docs/muse-code/permissions#profiles) (`permissions.default_profile`)
* tool configuration, including [workflow selection](/docs/muse-code/workflows#configure) and [session messaging](/docs/muse-code/session-messaging#configure)
* [MCP servers](/docs/muse-code/extending#mcp)
* a first-class `hooks` block, plus a `managed_hooks_path` pointer (see [hooks](/docs/muse-code/extending#hooks))
* a `runtime_capabilities` map that toggles capabilities such as the [observer agents](/docs/muse-code/extending#observer-agents)
* telemetry options

To delete saved sessions you haven't opened for a while, set `session_retention_days` to a number of days. Leave it unset to keep sessions forever. Retention works on macOS and Linux only for now. To set it for a single run, pass `-c session_retention_days=N`.

schema\_version must be 1

`settings.json` must set `"schema_version": 1`. A file that omits that key fails every command at startup with `malformed settings file`. An unrecognized value fails with `unsupported settings schema version`. A **missing** file is fine, because Muse Code applies defaults. Create the file only when you have something to set, and always include `"schema_version": 1`.

Enterprise administrators can validate a managed configuration document with `muse config validate --plane <defaults|policy> --file <path>`, or check the active managed configuration with `muse config status`.

[Project instructions with AGENTS.md](#agents-md)
-------------------------------------------------

`muse init` seeds your project's agent rules. It writes a single `AGENTS.md` into the current directory. It creates no other files, and does **not** change `settings.json`:

```
bash



muse init            # write AGENTS.md in the current directory



muse init --dry-run  # show what it would write, change nothing
```

Without `--force`, it stops if `AGENTS.md` already exists. With `--force`, it overwrites the file completely, so save any existing content first.

**How Muse Code loads instruction files.** From your workspace root, Muse Code walks up to the nearest `.git` boundary. At each directory level, it checks `AGENTS.md`, `CLAUDE.md`, `.agents/AGENTS.md`, then `.claude/CLAUDE.md`; the first existing file in that order wins for that level. Precedence when guidance conflicts:

* **Project rules win over user rules.**
* Among project files, the **deeper file wins** over a shallower one.

Your machine-wide user rules always load. **Project rules load only after you [trust the workspace](/docs/muse-code/permissions#trust-scopes).** On an untrusted checkout, Muse Code ignores project `AGENTS.md` and `CLAUDE.md` until you trust it.

Conservative by default

Muse Code is designed to follow these files, but it stays conservative about your repository by default: it won't commit, amend, or push your work unless you ask it to in the session, it keeps scratch files outside your checkout, and it reverts incidental edits it didn't need. The end of a task is not a request to commit.

[Select a model](#model)
------------------------

The default model is `muse-spark-1.2`. Override it per run with `--model`, or switch mid-session with the `/models` slash command:

```
bash



muse --model muse-spark-1.2
```

[Set reasoning effort](#reasoning-effort)
-----------------------------------------

`--reasoning-effort` trades latency for depth: `none`, `minimal`, `low`, `medium`, `high` (default), `xhigh`, `max`, `ultra`. Change it mid-session with `/effort`.

```
bash



muse --reasoning-effort medium
```

Premium reasoning levels

`xhigh` is an optional premium-precision level, and `max` is the deepest reasoning level — extended reasoning beyond `xhigh`. Muse Code offers `max` in both Standard and Contributor mode. `ultra` is a client-side setting that maps to each provider's highest supported reasoning level and can make Muse Code delegate more aggressively; where a provider doesn't support it, a request for `ultra` runs at `xhigh`. The Meta provider does not accept `none`; other providers can support it.

[Launch flags](#flags)
----------------------

Muse Code has two launch surfaces with **separate** flag sets: the interactive TUI (`muse …`) and headless (`muse exec …`). Some flags exist on only one.

Common to both:

* **`--model <id>`**, **`--reasoning-effort <level>`**: model and reasoning depth.
* **`--sandbox-network <mode>`**, **`--disable-sandbox`**, **`--disable-approval`**, **`--yolo`**, **`--trust-workspace`**: [permissions and sandbox](/docs/muse-code/permissions).
* **`--approval-mode <mode>`**, **`--approval-judge <on|off>`**: tune [approvals](/docs/muse-code/permissions#approval-modes).
* **`--subagent-worktree-isolation`**: compatibility flag. Isolation is requested per [subagent](/docs/muse-code/extending#multi-agent); the flag does not isolate every child.
* **`--workspace <path>`**: root policy-gated workspace tools at a path.
* **`--no-session-log`**: don't retain the [session](/docs/muse-code/interactive#session-model) event log.

Headless only (`muse exec …`):

* **`--json`** (emit JSONL events), **`--prompt-file <path>`** (read the prompt from a file), **`--max-model-steps <n>`** (cap the run).

`muse exec` accepts both approval flags. They configure the policy before the run starts. A headless run has no interactive UI for answering a human approval prompt, so choose a policy that can complete unattended or expect the held action to remain unresolved.

Both `muse --no-session-log` and `muse exec --no-session-log` run without retained session state, which [disables resume and the other retained-session features](/docs/muse-code/interactive#session-model). Generated inline workflow scripts then use process-lifetime temporary storage, which cannot support retained recovery.

[Debug provider calls](#transport-trace)
----------------------------------------

Set `MUSE_TRANSPORT_TRACE=1` to print raw provider request and response lines to stderr while debugging model calls. Credentials are scrubbed from the output.

[Local memory](#local-memory)
-----------------------------

Memory is Markdown you keep for the agent, so it recalls durable facts on its own. There are three scopes:

* **Personal project memory** (the default): stored on your machine outside the repo, private to you, scoped to this project.
* **Project memory**: committed to the repo under `<repo>/.agents/memory/`, shared with everyone who clones it.
* **Personal memory**: your machine-wide notes, across all projects.

Keep an index in `MEMORY.md` and one Markdown file per topic:

```
.agents/memory/



├── MEMORY.md        # index: one line per topic file



└── deploy.md        # a durable note the agent should know
```

Put durable, project-specific facts here: deployment procedures, the reason a workaround exists, a service's unusual behavior. Facts the agent could get wrong from general knowledge alone are the ones worth recording.

Memory loads in untrusted workspaces

Muse Code reads committed project memory into the model's context even in an **untrusted** workspace. Skills, rules, and hooks differ: they load only after you trust it. Treat a repo's `MEMORY.md` as a prompt-injection surface, and review it on checkouts you don't control.

[How the agent recalls memory](#recall)
---------------------------------------

At the start of a session, Muse Code injects an *index* of your memory: `MEMORY.md` plus a list of the other Markdown files (their paths, not their contents), up to 48 files. The agent reads individual files on demand, and a [background observer](/docs/muse-code/extending#observer-agents) can insert a relevant note into a turn before the agent answers.

[Next steps](#next-steps)
-------------------------

* Save repeatable multi-agent orchestration as [workflows](/docs/muse-code/workflows#save).
* Package reusable instructions and tools as [skills](/docs/muse-code/extending#skills) the agent can load on demand.
* Lock down what the agent can do in [permissions and safety](/docs/muse-code/permissions).

On this page

Please confirm your identity
----------------------------

For added security, you need to confirm your identity. You will be taken to your single sign-on provider, where you can enter your password.

CancelContinue