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

[Extending and automating](#extending-and-automating)
=====================================================

Table of contents

Take Muse Code past a single interactive session: distribute work across parallel agents, package reusable workflows as skills, wire your own commands into the lifecycle with hooks, connect external tools through MCP, and run the agent non-interactively in CI.

For structured orchestration with parallel groups, dependent stages, monitoring, and reusable JavaScript definitions, see [workflows](/docs/muse-code/workflows). To coordinate independent live sessions, use [session messaging](/docs/muse-code/session-messaging).

[Subagents and multi-agent](#multi-agent)
-----------------------------------------

Distribute a large job across a team of agents that work in parallel, and steer them from one place. A lead session spawns child agents, hands each a bounded task, and keeps managing the group while they run.

A **subagent** is a child agent that the lead spawns for one bounded task. Children share the lead's checkout unless the lead requests worktree isolation for that child. Ask for isolated worktrees when parallel children may write, and keep read-only children in the shared checkout.

An isolated child gets a Muse Code-managed Git worktree. The request can be rejected when the current profile, workspace, provider, or Git state cannot support isolation. It never silently falls back to the shared checkout.

The `--subagent-worktree-isolation` launch flag remains accepted for compatibility. It does not force every child into a worktree; isolation is still selected per child.

Use subagents when a job splits into tasks that are each bounded, independently verifiable, and would otherwise contend for the same files. Keep the work on one agent when the steps are strictly sequential.

**Steer the agents.** Run `/subagents` to view running and past subagents. Run `/tasks` to manage direct subagent tasks and use the actions available for the selected row. The lead also manages children through its native subagent tools.

A few properties worth knowing:

* One agent tree can execute eight agents at once by default, including the root agent. Set `agents.execution_capacity` from 1 through 64 in `settings.json` to change the limit. An unconfigured `ultra` root uses 64.
* A spawn attempted when the tree is full is rejected. An accepted child can still wait for a host scheduler slot before it starts.
* Children can spawn grandchildren. Every descendant shares the same root-tree capacity.
* Cancellation is cooperative: a cancelled child that never reaches a checkpoint keeps running, and one mid-write finishes that write.
* The runtime journals every spawn, status change, and control action, so you can answer "which child did what, and when" after the run. A child commits changes only when its task explicitly asks it to.

### [Background observer agents](#observer-agents)

Alongside the main session, Muse Code runs a team of background observer agents. Each one watches a single axis of quality, and can insert a short advisory into the main agent's next turn without an interruption. An observer never answers for you: it proposes, a reconciler decides, and only an accepted proposal reaches the main agent.

* **Memory recall**: surface a note from local project [memory](/docs/muse-code/configuration#local-memory) relevant to the next reply.
* **Skill recall**: surface a project [skill](#skills) the task should load first.
* **Goal tracking**: hold the agent to a declared [goal](/docs/muse-code/interactive#goals-loops) and decline to close the turn until the work is done.
* **Verification**: check that the agent ran the work it claims it finished.

All four observers, including verification, are on by default. Rollout gates and your [settings file](/docs/muse-code/configuration#settings-file) can still disable them.

Observers add token usage

Each enabled observer makes its own model calls, so the default set of four adds token usage in addition to the main session.

[Skills](#skills)
-----------------

A skill packages a repeatable workflow the agent can load on demand: a set of instructions, and optionally tools and files, that turn "explain how we do X" into a single invocation. Muse Code ships built-in skills, and you can add your own or import them from other agents.

Skills load from four sources:

* **Built-in**: skills that ship with Muse Code.
* **User**: your account-wide skills in `$XDG_CONFIG_HOME/muse/skills` and `~/.agents/skills`, available in every project. Muse Code also discovers `~/.claude/skills` and `$CODEX_HOME/skills` by default, falling back to `~/.codex/skills` when `CODEX_HOME` is unset. A user preference and rollout gate can disable these foreign personal skill roots.
* **Project**: skills committed to a repo under `<repo>/.agents/skills/<skill-id>/SKILL.md`, shared with anyone who clones it. Muse Code also scans repo-local `.codex/skills` and `.claude/skills`.
* **Plugin**: skills contributed by enabled plugin bundles.

Manage and invoke skills:

```
bash



muse skills list                    # every skill, all sources



muse skills inspect <skill-id>



muse skills enable <skill-id> --scope project



muse skills install ./my-skill --scope user



muse skills validate ./my-skill     # check it before installing



muse skills import --from claude    # or: --from codex
```

In an interactive session, invoke a skill with its slash shortcut. Built-in skills include `/plan` (turn a task into a grounded, decision-complete plan, then stop for approval), `/grill` (stress-test a plan or design before you build, recording each settled decision and a written scope contract in your project docs), `/taste` (a design-quality gate for frontend work), and `/threejs` (references for building Three.js scenes). The agent can also load a relevant skill when a [background observer](#observer-agents) surfaces one.

The bundled `migrate` skill imports your Claude Code or Codex memory notes and MCP servers into Muse Code. Invoke it with `/migrate`. It complements `muse skills import`, which brings in skills from those agents.

[Hooks](#hooks)
---------------

Wire your own shell commands into Muse Code's lifecycle. A hook binds a shell command to a lifecycle event. When the event fires, Muse Code runs the command and acts on its result: enforce a check, format code, or block an action before it happens, without a change to the agent itself.

Hooks come from three sources:

* **Project**: committed to the repo at `<project-root>/.muse/hooks.json`.
* **User**: your machine-wide hooks, defined in your [settings](/docs/muse-code/configuration#settings-file).
* **Managed**: a file that the `managed_hooks_path` setting points to, for centrally administered hooks.

Project hooks run only after you trust the project folder. User hooks run from your own settings without a separate hook-level trust step. Managed hooks also run without a project-trust step, so whoever controls the managed hooks file controls what executes.

Hooks run outside the sandbox

Only add hooks whose commands you've read. A hook's command runs directly through your shell, **outside** the [sandbox and approval](/docs/muse-code/permissions) that govern the agent's own tools. The only hardening is a cleared environment with a small allowlist.

### [Lifecycle events](#hook-events)

A hook binds to exactly one event. The available events are `SessionStart`, `UserPromptSubmit`, `PreToolUse`, `PermissionRequest`, `PostToolUse`, `PostToolUseFailure`, `PostToolBatch`, `PreLLMCall`, `PostLLMCall`, `PreCompact`, `PostCompact`, `SubagentStart`, `SubagentStop`, `Notification`, `Stop`, `StopFailure`, and `SessionEnd`. `PostToolUseFailure` runs when a tool call fails or crashes. `PostToolBatch` runs once after each committed batch of tool calls and receives them in order; it can block the batch with a reason, stop the turn, or add context. `StopFailure` runs once when a turn ends in a final model failure, and receives the failure category and safe details. `Notification` fires when the agent needs your approval, so a hook can alert you. `SessionEnd` runs during orderly session termination. It is observational: its output cannot block termination, inject context for a later request, or stop the session.

Muse Code discovers and validates hooks at session startup. A malformed project or managed hook file contributes no handlers from that source and produces a startup warning. A malformed user settings file fails settings validation. Unsupported events, matchers, or handlers skip the affected entry and produce a source-specific warning.

There is no active `muse hooks` command family or per-hook trust command. Fix the reported configuration and start a new session to reload it.

[MCP servers](#mcp)
-------------------

Connect external tools through the Model Context Protocol (MCP). Declare servers in the `mcp_servers` block of your [settings file](/docs/muse-code/configuration#settings-file). Each server takes a `transport`: either `stdio` (with `command`, `args`, and `env`) or `streamable_http` (with `url` and `headers`). Every server also takes `enabled` and `mode`. For `stdio`, the optional `framing` setting controls message framing. A non-default `framing` value on `streamable_http` fails validation.

```
json



{ "mcp_servers": {



"my-tools": { "transport": "stdio", "command": "my-mcp-server", "args": [] }



} }
```

A server's `mode` defaults to `required`. If a required server fails to start, the whole run aborts. Set `mode` to `optional` for a server that Muse Code should skip with a warning when it's unavailable.

Server configs support `${VAR}` environment-variable interpolation, and Muse Code passes `MUSE_SESSION_ID` to `stdio` servers. A tool whose server declares it read-only runs without an approval prompt under [on-request approvals](/docs/muse-code/permissions#approval-modes). Run `/mcp` in an interactive session to see a live inventory of connected servers and their tools.

A named toolset (`run.toolset`) can select MCP tools by their full name, such as `mcp__docs__search`, or with a trailing `*`: `mcp__docs__*` selects every tool from the `docs` server, and `mcp__docs__search*` selects its tools whose names start with `search`. If a selected MCP tool is still missing after the MCP servers start, the run stops before contacting the model and lists every missing name.

MCP tools aren't sandboxed

Only connect servers you trust. **MCP tools are not sandboxed.** Unlike shell commands, an MCP server runs as an ordinary child process (stdio) or a direct network connection, outside the filesystem and network [sandbox](/docs/muse-code/permissions#sandbox). Approval still applies, but containment does not.

### [Sign in to remote MCP servers](#mcp-oauth)

A remote server can require an OAuth 2.1 sign-in. Authenticate in the browser or headless, and Muse Code keeps and refreshes the tokens for you:

```
bash



muse mcp login <server>



muse mcp logout <server>
```

A sign-in done in another terminal is picked up without a restart, and a startup or mid-session 401 tells you the exact `muse mcp login` command to run. To turn off automatic client registration, set `"mcp_oauth_dynamic_client_registration": false` in your [settings file](/docs/muse-code/configuration#settings-file).

[Headless and CI](#headless)
----------------------------

Run Muse Code without a terminal UI. `muse exec` takes one prompt, runs it to completion, and exits, so you can drive the agent from a script, a job, or a CI pipeline:

```
bash



muse exec "Update the changelog for the latest release and run the tests."



muse exec --prompt-file ./task.txt



muse exec --json "Refactor the auth module and run the tests."   # JSONL events on stdout
```

`muse exec` prints the agent's output to stdout and returns a process exit code that reflects **how the run ended, not whether the work is correct**: `0` when the turn completes, `1` when it fails or is cancelled (including a `--max-model-steps` limit), `2` for a usage error, and `130` or `143` on SIGINT or SIGTERM. An agent can finish its turn (exit `0`) and still report failing tests, so gate on your own test command, not on the exit code alone.

**Control approvals for automation.** A non-interactive run has no one to answer an approval prompt, so choose a posture in advance:

* **`--disable-approval`**: skip approval prompts, but keep the [sandbox](/docs/muse-code/permissions#sandbox) on to contain what runs.
* **`--yolo`**: disable approval and the sandbox for a fully unattended run. It also trusts the workspace, so use it only on trusted code in a disposable, isolated container. See [run without guardrails](/docs/muse-code/permissions#run-without-guardrails).

Cap a run's work with `--max-model-steps` so a stuck task can't loop indefinitely.

CI sandbox requirements

The [sandbox](/docs/muse-code/permissions#sandbox) must be able to run on the CI runner, which on Linux needs a working bubblewrap and a non-musl build. Without it, every sandboxed shell command aborts as an environment failure.

**Resume and audit non-interactively.** Headless runs are [sessions](/docs/muse-code/interactive#session-model) like any other. To continue an interrupted job non-interactively, use `exec` with the session id. `muse resume` opens the interactive UI and is not a headless surface:

```
bash



muse exec --session-id <uuid> "Continue the task."



muse export --session <uuid> --out run.json
```

Muse Code refuses a workspace mismatch unless you pass `--allow-workspace-switch`. The export document embeds the CLI version that produced it, so pin the version if you gate on its hash.

[Next steps](#next-steps)
-------------------------

* Keep the whole team inside the same [permissions and sandbox boundary](/docs/muse-code/permissions).
* Build repeatable multi-agent runs with [workflows](/docs/muse-code/workflows).
* Give skills and subagents the project facts they rely on in [configuration and context](/docs/muse-code/configuration).
* Connect external agent tooling to the model from the [coding agents](/docs/coding-agents) guide.

On this page

Please confirm your identity
----------------------------

For added security, you need to confirm your identity. You will be taken to your single sign-on provider, where you can enter your password.

CancelContinue