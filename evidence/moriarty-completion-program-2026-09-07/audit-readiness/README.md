# Audit lane readiness, 2026-09-07

Experiment observation: installed Foreman verification returned `Pass`; the artifact manifest is in [foreman-install.json](foreman-install.json). This checks the installed runtime, not actionful dispatch admission.

Experiment observation: one bounded Claude canary requested exactly `claude-fable-5-1`, with no tools, plan permission mode, no session persistence, safe mode and no browser, from a disposable operating-system temporary directory. It exited 1 after approximately 2.4 seconds. The provider result reports exhausted usage credits. It returned no `modelUsage` entry, so canonical identity is **unverified** and the Fable audit lane is **blocked by the reported usage-credit limit**. This is not evidence that the user is signed out. No model was substituted and no second canary was attempted.

See [raw JSON output](fable-canary.stdout.json), [raw stderr](fable-canary.stderr.txt) and [sanitized readiness receipt](fable-readiness.json). No audit verdict exists from this canary. GPT-6 audit availability was outside this subtask.

The installed CLI accepts this advisory invocation. Supply the complete cold audit packet through stdin; use a temporary cwd and an external bounded process supervisor:

```sh
claude --print --model claude-fable-5-1 --tools '' --permission-mode plan --no-session-persistence --safe-mode --no-chrome --output-format json --effort high < /absolute/path/to/audit-packet.txt
```

Before admitting a later result, require an exit-zero non-error result and `modelUsage["claude-fable-5-1"].canonicalModel == "claude-fable-5-1"`. A requested model name or self-description alone is insufficient. Treat Fable-required result promotion as blocked until the exact lane returns an admissible audit. Keep implementation and audit bounds in the execution contract; do not convert this provider limit into an unlimited retry loop.
